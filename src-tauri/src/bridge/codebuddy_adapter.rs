// src-tauri/src/bridge/codebuddy_adapter.rs

use crate::bridge::agent_adapter::{AgentAdapter, AgentEvent};
use crate::bridge::error::{AgentError, AgentResult};
use crate::bridge::event_emitter::EventEmitter;
use crate::bridge::mcp_manager::McpManager;
use crate::bridge::protocol::{
    AgentConfig, ControlMessage, RenderMode, SessionId, SkillInfo, SkillInput,
};
use crate::bridge::session_manager::{
    Session, SessionStatus, DataChunk, ProgressInfo, ErrorInfo,
};
use chrono::Utc;
use futures::{SinkExt, StreamExt};
use futures_util::stream;
use std::collections::HashMap;
use std::io::BufRead;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc, RwLock};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use tokio::net::TcpListener;

/// WebSocket 连接类型
type WebSocketStream = tokio_tungstenite::WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

/// CodeBuddy Code 适配器
///
/// 实现 AgentAdapter trait，封装 CodeBuddy Code 进程管理和通信逻辑。
/// 使用双通道通信: stdout/stderr 用于日志，Control Channel 用于结构化数据。
pub struct CodeBuddyAdapter {
    /// CodeBuddy Code 进程
    process: Option<Child>,

    /// 配置
    config: AgentConfig,

    /// 会话管理器
    session_manager: Arc<SessionManager>,

    /// 事件发射器
    event_emitter: Arc<EventEmitter>,

    /// MCP 服务器管理器
    mcp_manager: Arc<McpManager>,

    /// 运行状态
    is_running: Arc<Mutex<bool>>,

    /// Control Channel WebSocket 连接
    control_channel: Option<Arc<Mutex<WebSocketStream>>>,

    /// 响应通道 (用于等待响应)
    response_channels: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<ControlMessage>>>>,

    /// 进程 stdout 接收器
    stdout_rx: Option<Arc<Mutex<mpsc::UnboundedReceiver<String>>>>,

    /// 进程 stderr 接收器
    stderr_rx: Option<Arc<Mutex<mpsc::UnboundedReceiver<String>>>>,

    /// 停止信号
    stop_signal: Arc<Mutex<bool>>,
}

impl CodeBuddyAdapter {
    /// 创建新的 CodeBuddy 适配器
    pub fn new(config: AgentConfig) -> Self {
        let binary_path = config.binary_path.clone();
        CodeBuddyAdapter {
            process: None,
            config,
            session_manager: Arc::new(SessionManager::new()),
            event_emitter: Arc::new(EventEmitter::new()),
            mcp_manager: Arc::new(McpManager::new(binary_path)),
            is_running: Arc::new(Mutex::new(false)),
            control_channel: None,
            response_channels: Arc::new(RwLock::new(HashMap::new())),
            stdout_rx: None,
            stderr_rx: None,
            stop_signal: Arc::new(Mutex::new(false)),
        }
    }

    /// 获取会话管理器
    pub fn session_manager(&self) -> Arc<SessionManager> {
        self.session_manager.clone()
    }

    /// 获取事件发射器
    pub fn event_emitter(&self) -> Arc<EventEmitter> {
        self.event_emitter.clone()
    }

    /// 获取 MCP 管理器
    pub fn mcp_manager(&self) -> Arc<McpManager> {
        self.mcp_manager.clone()
    }

    /// 检查是否正在运行
    pub async fn is_running(&self) -> bool {
        *self.is_running.lock().await
    }

    /// 启动 CodeBuddy Code 进程
    async fn start_process(&mut self) -> AgentResult<Child> {
        log::info!("启动 CodeBuddy Code 进程: {}", self.config.binary_path);

        // 检查二进制文件是否存在
        if which::which(&self.config.binary_path).is_err() {
            return Err(AgentError::process_start_failed(format!(
                "CodeBuddy Code 未找到，请先安装: {}",
                self.config.binary_path
            )));
        }

        // 构建命令
        let mut cmd = Command::new(&self.config.binary_path);

        // 设置环境变量
        for (key, value) in &self.config.env {
            cmd.env(key, value);
        }

        // 重定向标准流
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        // 启动进程
        let child = cmd.spawn().map_err(|e| {
            AgentError::process_start_failed(format!("无法启动进程: {}", e))
        })?;

        log::info!("进程已启动，PID: {}", child.id());
        Ok(child)
    }

    /// 监听进程 stdout
    async fn monitor_stdout(&self, mut stdout: std::process::ChildStdout) {
        let stdout_rx = Arc::new(Mutex::new(mpsc::unbounded_channel::<String>().1));
        self.stdout_rx = Some(stdout_rx.clone());

        tokio::spawn(async move {
            let reader = std::io::BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(log_line) => {
                        log::debug!("CodeBuddy stdout: {}", log_line);
                        // 发送到前端显示
                        // TODO: 通过事件系统发送日志到前端
                    }
                    Err(e) => {
                        log::error!("读取 stdout 失败: {}", e);
                        break;
                    }
                }
            }
            log::info!("stdout 监听结束");
        });
    }

    /// 监听进程 stderr
    async fn monitor_stderr(&self, mut stderr: std::process::ChildStderr) {
        let stderr_rx = Arc::new(Mutex::new(mpsc::unbounded_channel::<String>().1));
        self.stderr_rx = Some(stderr_rx.clone());

        tokio::spawn(async move {
            let reader = std::io::BufReader::new(stderr);
            for line in reader.lines() {
                match line {
                    Ok(log_line) => {
                        log::debug!("CodeBuddy stderr: {}", log_line);
                        // 发送到前端显示
                        // TODO: 通过事件系统发送日志到前端
                    }
                    Err(e) => {
                        log::error!("读取 stderr 失败: {}", e);
                        break;
                    }
                }
            }
            log::info!("stderr 监听结束");
        });
    }

    /// 建立 Control Channel (WebSocket 连接)
    async fn establish_control_channel(&mut self) -> AgentResult<Arc<Mutex<WebSocketStream>>> {
        // CodeBuddy Code 可能不内置 WebSocket 支持
        // 我们可以采用几种方案：
        // 方案 1: 通过临时文件/命名管道通信
        // 方案 2: 启动一个辅助 WebSocket 服务器，通过 IPC 与 CodeBuddy 通信
        // 方案 3: 解析 stdout/stderr 中的特殊标记提取结构化数据

        // 这里我们使用方案 2：启动临时 WebSocket 服务器
        let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| {
            AgentError::port_conflict(e.port().unwrap_or(0))
        })?;
        let local_addr = listener.local_addr().map_err(|e| {
            AgentError::Other(format!("获取本地地址失败: {}", e))
        })?;

        log::info!("Control Channel 监听端口: {}", local_addr.port());

        // 启动 WebSocket 服务器任务
        let (ws_tx, ws_rx) = mpsc::channel(100);
        let stop_signal = self.stop_signal.clone();

        tokio::spawn(async move {
            // 等待连接
            match listener.accept().await {
                Ok((stream, addr)) => {
                    log::info!("Control Channel 连接来自: {}", addr);
                    let ws_stream = tokio_tungstenite::accept_async(stream).await;

                    match ws_stream {
                        Ok(ws) => {
                            let (mut write, mut read) = ws.split();
                            let stop_signal = stop_signal.clone();

                            // 消息接收循环
                            let read_task = tokio::spawn(async move {
                                while !*stop_signal.lock().await {
                                    match read.next().await {
                                        Some(Ok(msg)) => {
                                            if let Message::Text(text) = msg {
                                                // 解析 ControlMessage
                                                if let Ok(control_msg) =
                                                    serde_json::from_str::<ControlMessage>(&text)
                                                {
                                                    // 发送到主处理逻辑
                                                    // TODO: 实现
                                                }
                                            }
                                        }
                                        Some(Err(e)) => {
                                            log::error!("接收消息错误: {}", e);
                                            break;
                                        }
                                        None => break,
                                    }
                                }
                            });

                            // 等待读取任务完成
                            read_task.await.ok();
                        }
                        Err(e) => {
                            log::error!("WebSocket 握手失败: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("接受连接失败: {}", e);
                }
            }
        });

        // 等待连接建立
        // 这里简化处理，实际需要等待真正的连接
        // 我们返回一个占位符
        Ok(Arc::new(Mutex::new(tokio_tungstenite::WebSocketStream::from_raw_socket(
            tokio::net::TcpStream::connect(local_addr).await.map_err(|e| {
                AgentError::Other(format!("连接 WebSocket 服务器失败: {}", e))
            })?,
            tokio_tungstenite::tungstenite::protocol::Role::Server,
            None,
        ).await)))
    }

    /// 发送消息到 Control Channel
    async fn send_message(&self, message: ControlMessage) -> AgentResult<()> {
        let channel = self.control_channel.as_ref().ok_or_else(|| {
            AgentError::CommunicationError {
                message: "Control Channel 未建立".to_string(),
                suggestion: "请先启动 Agent".to_string(),
            }
        })?;

        let json = serde_json::to_string(&message).map_err(|e| {
            AgentError::Other(format!("序列化消息失败: {}", e))
        })?;

        let ws = channel.lock().await;
        // TODO: 发送消息到 WebSocket
        // 这里简化处理
        drop(ws);

        log::debug!("发送消息: {}", json);
        Ok(())
    }

    /// 等待特定类型的响应
    async fn wait_for_response(
        &self,
        message_type: &str,
        timeout_ms: u64,
    ) -> AgentResult<ControlMessage> {
        // TODO: 实现响应等待逻辑
        // 使用 timeout 机制
        tokio::time::timeout(
            std::time::Duration::from_millis(timeout_ms),
            async {
                // 等待特定类型的消息
                // 这里简化处理
                Ok(ControlMessage::Heartbeat)
            }
        )
        .await
        .map_err(|_| AgentError::timeout("等待响应超时", timeout_ms))?
    }
}

#[async_trait::async_trait]
impl AgentAdapter for CodeBuddyAdapter {
    async fn start(&mut self) -> AgentResult<()> {
        log::info!("启动 CodeBuddy Code 适配器: {}", self.config.binary_path);

        // 1. 启动进程
        let child = self.start_process().await?;
        self.process = Some(child);

        // 2. 获取 stdout 和 stderr
        let process_ref = self.process.as_ref().unwrap();
        let stdout = process_ref.stdout.as_ref().ok_or_else(|| {
            AgentError::process_start_failed("无法获取 stdout".to_string())
        })?;
        let stderr = process_ref.stderr.as_ref().ok_or_else(|| {
            AgentError::process_start_failed("无法获取 stderr".to_string())
        })?;

        // 3. 监听 stdout/stderr
        let stdout_clone = stdout.try_clone().map_err(|e| {
            AgentError::Other(format!("克隆 stdout 失败: {}", e))
        })?;
        let stderr_clone = stderr.try_clone().map_err(|e| {
            AgentError::Other(format!("克隆 stderr 失败: {}", e))
        })?;

        self.monitor_stdout(stdout).await;
        self.monitor_stderr(stderr).await;

        // 4. 建立 Control Channel
        let control_channel = self.establish_control_channel().await?;
        self.control_channel = Some(control_channel);

        // 5. 同步 MCP 服务器
        if let Err(e) = self.mcp_manager.sync().await {
            log::warn!("同步 MCP 服务器失败: {}", e);
            // 不阻塞启动，MCP 错误不是致命的
        }

        *self.is_running.lock().await = true;
        *self.stop_signal.lock().await = false;

        log::info!("CodeBuddy Code 适配器启动成功");
        Ok(())
    }

    async fn stop(&mut self) -> AgentResult<()> {
        log::info!("停止 CodeBuddy Code 适配器");

        // 1. 设置停止信号
        *self.stop_signal.lock().await = true;
        *self.is_running.lock().await = false;

        // 2. 关闭 Control Channel
        self.control_channel = None;

        // 3. 终止进程
        if let Some(mut child) = self.process.take() {
            log::info!("终止进程 PID: {}", child.id());

            // 先尝试优雅关闭
            if let Err(e) = child.kill() {
                log::warn!("终止进程失败: {}", e);
            }

            // 等待进程退出
            match child.wait() {
                Ok(status) => {
                    log::info!("进程已退出，状态: {}", status);
                }
                Err(e) => {
                    log::error!("等待进程退出失败: {}", e);
                }
            }
        }

        // 4. 清理资源
        self.stdout_rx = None;
        self.stderr_rx = None;

        log::info!("CodeBuddy Code 适配器已停止");
        Ok(())
    }

    async fn get_skills(&self) -> AgentResult<Vec<SkillInfo>> {
        log::info!("获取 Skill 列表");

        // 1. 发送 GetSkills 请求
        self.send_message(ControlMessage::GetSkills).await?;

        // 2. 等待 SkillList 响应
        let response = self.wait_for_response("skill_list", 5000).await?;

        // 3. 解析响应
        if let ControlMessage::SkillList { skills } = response {
            log::info!("获取到 {} 个 Skill", skills.len());
            Ok(skills)
        } else {
            Err(AgentError::ProtocolError {
                message: format!("收到意外响应: {:?}", response),
                suggestion: "请检查 Agent 是否正常工作".to_string(),
            })
        }
    }

    async fn execute_skill(
        &self,
        skill_name: &str,
        input: SkillInput,
    ) -> AgentResult<SessionId> {
        log::info!("执行 Skill: {} -> {}", skill_name, input.as_text());

        // 1. 生成唯一的 session_id
        let session_id = uuid::Uuid::new_v4().to_string();

        // 2. 创建会话
        self.session_manager
            .create(
                session_id.clone(),
                skill_name.to_string(),
                input.as_text(),
                None,
            )
            .await?;

        // 3. 更新会话状态为运行中
        self.session_manager
            .update_status(session_id.clone(), SessionStatus::Running)
            .await?;

        // 4. 发送 ExecuteSkill 请求
        self.send_message(ControlMessage::ExecuteSkill {
            skill_name: skill_name.to_string(),
            input: input.clone(),
        })
        .await?;

        // 5. 等待 ExecutionStart 响应
        let response = self.wait_for_response("execution_start", 5000).await?;

        if let ControlMessage::ExecutionStart {
            session_id: resp_session_id,
            skill_name: _,
            render_mode,
        } = response
        {
            if resp_session_id != session_id {
                return Err(AgentError::ProtocolError {
                    message: "Session ID 不匹配".to_string(),
                    suggestion: "请检查 Agent 是否正确处理请求".to_string(),
                });
            }

            // 更新会话的渲染模式
            // TODO: 更新会话的 render_mode

            log::info!("Skill 执行已启动，Session ID: {}", session_id);
            Ok(session_id)
        } else {
            Err(AgentError::ProtocolError {
                message: format!("收到意外响应: {:?}", response),
                suggestion: "请检查 Skill 是否存在".to_string(),
            })
        }
    }

    fn subscribe_events(&self) -> impl Stream<Item = AgentEvent> + Send {
        // TODO: 实现事件流
        // 目前返回空流
        stream::empty()
    }
}
