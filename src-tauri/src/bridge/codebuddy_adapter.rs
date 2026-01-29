// src-tauri/src/bridge/codebuddy_adapter.rs

use crate::bridge::agent_adapter::{AgentAdapter, AgentEvent};
use crate::bridge::error::{AgentError, AgentResult};
use crate::bridge::event_emitter::EventEmitter;
use crate::bridge::mcp_manager::McpManager;
use crate::bridge::protocol::{
    AgentConfig, ControlMessage, RenderMode, SessionId, SkillInfo, SkillInput,
};
use crate::bridge::session_manager::{
    Session, SessionManager, SessionStatus, DataChunk, ProgressInfo, ErrorInfo,
};
use chrono::Utc;
use futures::{SinkExt, StreamExt, Stream};
use futures_util::stream;
use serde_json::Value;
use std::collections::HashMap;
use std::io::BufRead;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc, RwLock, broadcast};
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
    response_channels: Arc<RwLock<HashMap<String, tokio::sync::oneshot::Sender<ControlMessage>>>>,

    /// 进程 stdout 接收器
    stdout_rx: Option<Arc<Mutex<mpsc::UnboundedReceiver<String>>>>,

    /// 进程 stderr 接收器
    stderr_rx: Option<Arc<Mutex<mpsc::UnboundedReceiver<String>>>>,

    /// 停止信号
    stop_signal: Arc<Mutex<bool>>,

    /// 内部事件发送器 (用于流式事件)
    event_tx: Arc<broadcast::Sender<AgentEvent>>,

    /// 重试计数器
    retry_count: Arc<Mutex<u32>>,

    /// 最大重试次数
    max_retries: u32,

    /// 事件 ID 计数器
    event_id_counter: Arc<Mutex<u64>>,
}

// Manual implementation of Clone for CodeBuddyAdapter
// Note: This clones the shared references but not the actual process
impl Clone for CodeBuddyAdapter {
    fn clone(&self) -> Self {
        CodeBuddyAdapter {
            process: None, // Cannot clone a process, so set to None
            config: self.config.clone(),
            session_manager: self.session_manager.clone(),
            event_emitter: self.event_emitter.clone(),
            mcp_manager: self.mcp_manager.clone(),
            is_running: self.is_running.clone(),
            control_channel: None, // Cannot clone WebSocket connections
            response_channels: self.response_channels.clone(),
            stdout_rx: None, // Cannot clone receivers
            stderr_rx: None, // Cannot clone receivers
            stop_signal: self.stop_signal.clone(),
            event_tx: self.event_tx.clone(),
            retry_count: self.retry_count.clone(),
            max_retries: self.max_retries,
            event_id_counter: self.event_id_counter.clone(),
        }
    }
}

impl CodeBuddyAdapter {
    /// 创建新的 CodeBuddy 适配器
    pub fn new(config: AgentConfig) -> Self {
        let binary_path = config.binary_path.clone();
        let (event_tx, _) = broadcast::channel(1000);
        
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
            event_tx: Arc::new(event_tx),
            retry_count: Arc::new(Mutex::new(0)),
            max_retries: 3,
            event_id_counter: Arc::new(Mutex::new(0)),
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
        
        // 启动进程退出监听
        self.monitor_process_exit(child.id()).await;

        Ok(child)
    }

    /// 监听进程退出事件
    async fn monitor_process_exit(&self, pid: u32) {
        let stop_signal = self.stop_signal.clone();
        let is_running = self.is_running.clone();
        let adapter_self = self.clone(); // Use the implemented Clone

        tokio::spawn(async move {
            log::debug!("启动进程退出监听: PID {}", pid);

            // Monitor the process in a loop
            loop {
                // Check if stop signal is triggered
                if *stop_signal.lock().await {
                    log::debug!("停止信号已触发，退出进程监听: PID {}", pid);
                    break;
                }

                // Check if the process is still running by trying to send a signal (on Unix) or checking existence
                // On Windows, we'll use a different approach
                #[cfg(unix)]
                {
                    use nix::sys::signal::{kill, Signal};
                    use nix::unistd::Pid;

                    match kill(Pid::from_raw(pid as i32), Signal::SIGCONT) {
                        Ok(_) => {
                            // Process exists, continue monitoring
                        }
                        Err(_) => {
                            // Process doesn't exist anymore
                            log::error!("检测到进程已退出: PID {}", pid);

                            // Update running status
                            *is_running.lock().await = false;

                            // Attempt to restart the agent if not intentionally stopped
                            if !*stop_signal.lock().await {
                                log::info!("尝试自动重启 Agent...");
                                if let Err(e) = adapter_self.restart_agent().await {
                                    log::error!("自动重启失败: {}", e);
                                }
                            }
                            break;
                        }
                    }
                }

                #[cfg(windows)]
                {
                    use std::process::Command;

                    // On Windows, check if process exists by running tasklist
                    let output = Command::new("tasklist")
                        .args(&["/FI", &format!("PID eq {}", pid)])
                        .output();

                    if let Ok(output) = output {
                        let output_str = String::from_utf8_lossy(&output.stdout);

                        // If the process is not in the tasklist, it has exited
                        if !output_str.contains(&pid.to_string()) {
                            log::error!("检测到进程已退出: PID {}", pid);

                            // Update running status
                            *is_running.lock().await = false;

                            // Attempt to restart the agent if not intentionally stopped
                            if !*stop_signal.lock().await {
                                log::info!("尝试自动重启 Agent...");
                                if let Err(e) = adapter_self.restart_agent().await {
                                    log::error!("自动重启失败: {}", e);
                                }
                            }
                            break;
                        }
                    }
                }

                // Sleep before next check
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }

            log::debug!("进程退出监听结束: PID {}", pid);
        });
    }

    /// 尝试重启 Agent
    async fn restart_agent(&mut self) -> AgentResult<()> {
        log::info!("尝试重启 Agent");
        
        // 获取当前重试次数
        let mut retry_count = self.retry_count.lock().await;
        
        if *retry_count >= self.max_retries {
            log::error!("达到最大重试次数 ({}), 停止重试", self.max_retries);
            return Err(AgentError::Other(format!(
                "达到最大重试次数 ({}), 请检查 Agent 配置",
                self.max_retries
            )));
        }
        
        *retry_count += 1;
        let current_retry = *retry_count;
        drop(retry_count);
        
        log::info!("重试第 {} 次", current_retry);
        
        // 指数退避
        let delay = std::time::Duration::from_millis(1000 * 2_u64.pow(current_retry - 1));
        tokio::time::sleep(delay).await;
        
        // 停止当前 Agent
        self.stop().await?;
        
        // 重新启动
        self.start().await?;
        
        // 重置重试计数器
        *self.retry_count.lock().await = 0;
        
        log::info!("Agent 重启成功");
        Ok(())
    }

    /// 尝试重连 WebSocket
    async fn reconnect_websocket(&mut self) -> AgentResult<()> {
        log::info!("尝试重连 WebSocket");
        
        // 获取当前重试次数
        let mut retry_count = self.retry_count.lock().await;
        
        if *retry_count >= self.max_retries {
            log::error!("达到最大重试次数 ({}), 停止重连", self.max_retries);
            return Err(AgentError::Other(format!(
                "达到最大重试次数 ({}), 请检查网络连接",
                self.max_retries
            )));
        }
        
        *retry_count += 1;
        let current_retry = *retry_count;
        drop(retry_count);
        
        log::info!("重连第 {} 次", current_retry);
        
        // 指数退避
        let delay = std::time::Duration::from_millis(500 * 2_u64.pow(current_retry - 1));
        tokio::time::sleep(delay).await;
        
        // 重新建立 Control Channel
        let control_channel = self.establish_control_channel().await?;
        self.control_channel = Some(control_channel);
        
        // 重置重试计数器
        *self.retry_count.lock().await = 0;
        
        log::info!("WebSocket 重连成功");
        Ok(())
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

        // 创建消息处理通道
        let (msg_tx, mut msg_rx) = mpsc::unbounded_channel::<ControlMessage>();
        let stop_signal = self.stop_signal.clone();
        let adapter_self = Arc::new(Mutex::new(self.clone_for_spawn()));

        // 启动 WebSocket 服务器任务
        tokio::spawn({
            let stop_signal = stop_signal.clone();
            let adapter_self_clone = adapter_self.clone();
            let msg_tx_clone = msg_tx.clone();

            async move {
                // 等待连接
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        log::info!("Control Channel 连接来自: {}", addr);
                        let ws_stream = tokio_tungstenite::accept_async(stream).await;

                        match ws_stream {
                            Ok(ws) => {
                                let (mut write, mut read) = ws.split();

                                // 消息接收循环
                                let read_task = tokio::spawn(async move {
                                    let mut counter = 0;
                                    while !*stop_signal.lock().await {
                                        match read.next().await {
                                            Some(Ok(msg)) => {
                                                if let Message::Text(text) = msg {
                                                    counter += 1;
                                                    // 解析 ControlMessage
                                                    match serde_json::from_str::<ControlMessage>(&text) {
                                                        Ok(control_msg) => {
                                                            // 发送到主处理逻辑
                                                            log::debug!("收到 ControlMessage ({}): {:?}", counter, control_msg);

                                                            // Send to the message processing loop
                                                            if msg_tx_clone.send(control_msg).is_err() {
                                                                log::error!("无法发送消息到处理队列");
                                                                break;
                                                            }
                                                        }
                                                        Err(e) => {
                                                            log::error!("解析 ControlMessage 失败: {}", e);
                                                        }
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
            }
        });

        // 启动消息处理循环
        tokio::spawn({
            let stop_signal = stop_signal.clone();
            let adapter_self_clone = adapter_self.clone();

            async move {
                while !*stop_signal.lock().await {
                    if let Some(msg) = msg_rx.recv().await {
                        // Process the message using the adapter instance
                        let adapter = adapter_self_clone.lock().await;
                        adapter.handle_control_message_sync(msg).await;
                    }
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

    /// 处理 Control Channel 消息
    async fn handle_control_message(&self, message: ControlMessage) {
        log::debug!("处理 ControlMessage: {:?}", message);

        // Check if this is a response to a pending request
        match &message {
            ControlMessage::SkillList { .. } => {
                // This is a response to a get_skills request
                self.route_response_to_waiter(message).await;
                return;
            }
            ControlMessage::ExecutionStart { .. } => {
                // This is a response to an execute_skill request
                self.route_response_to_waiter(message).await;
                return;
            }
            _ => {
                // Not a response message, continue with streaming handling
            }
        }

        // Handle streaming messages
        match message {
            ControlMessage::DataChunk {
                session_id,
                data,
                is_final,
                chunk_index,
            } => {
                self.handle_data_chunk(session_id, data, is_final, chunk_index).await;
            }
            ControlMessage::Progress {
                session_id,
                current,
                total,
                message: progress_message,
            } => {
                self.handle_progress(session_id, current, total, progress_message).await;
            }
            ControlMessage::ExecutionComplete {
                session_id,
                summary,
                success,
            } => {
                self.handle_execution_complete(session_id, summary, success).await;
            }
            ControlMessage::Error {
                session_id,
                code,
                message,
                suggestion,
            } => {
                self.handle_error(session_id, code, message, suggestion).await;
            }
            _ => {
                // 其他消息类型在 wait_for_response 中处理
                log::debug!("忽略非流式消息: {:?}", message);
            }
        }
    }

    /// 将响应路由到等待的请求
    async fn route_response_to_waiter(&self, message: ControlMessage) {
        // Get all response channels
        let channels_map = self.response_channels.read().await;

        // Send the message to all waiting channels (they will filter by type)
        // In a more sophisticated implementation, we'd have a way to correlate specific requests/responses
        for (_, sender) in channels_map.iter() {
            // Try to send the message, ignore errors if the receiver has been dropped
            let _ = sender.send(message.clone());
        }
    }
    
    /// 处理 DataChunk 消息
    async fn handle_data_chunk(
        &self,
        session_id: String,
        data: Value,
        is_final: bool,
        chunk_index: Option<u64>,
    ) {
        log::debug!(
            "处理 DataChunk: session_id={}, is_final={}, chunk_index={:?}",
            session_id,
            is_final,
            chunk_index
        );
        
        // 生成事件 ID
        let event_id = self.next_event_id().await;
        
        // 添加数据块到会话
        match self
            .session_manager
            .add_data_chunk(session_id.clone(), data.clone(), is_final)
            .await
        {
            Ok(index) => {
                log::debug!("数据块已添加，索引: {}", index);
                
                // 发送事件到前端
                let event = AgentEvent::DataChunk {
                    event_id,
                    session_id,
                    chunk_index: Some(index),
                    data,
                    is_final,
                };
                
                let _ = self.event_tx.send(event);
            }
            Err(e) => {
                log::error!("添加数据块失败: {}", e);
            }
        }
    }
    
    /// 处理 Progress 消息
    async fn handle_progress(
        &self,
        session_id: String,
        current: u64,
        total: u64,
        message: String,
    ) {
        log::debug!(
            "处理 Progress: session_id={}, current={}, total={}, message={}",
            session_id,
            current,
            total,
            message
        );
        
        // 生成事件 ID
        let event_id = self.next_event_id().await;
        
        // 更新会话进度
        match self
            .session_manager
            .update_progress(session_id.clone(), current, total, message.clone())
            .await
        {
            Ok(_) => {
                log::debug!("进度已更新");
                
                // 发送事件到前端
                let event = AgentEvent::Progress {
                    event_id,
                    session_id,
                    current,
                    total,
                    message,
                };
                
                let _ = self.event_tx.send(event);
            }
            Err(e) => {
                log::error!("更新进度失败: {}", e);
            }
        }
    }
    
    /// 处理 ExecutionComplete 消息
    async fn handle_execution_complete(
        &self,
        session_id: String,
        summary: Option<String>,
        success: bool,
    ) {
        log::info!(
            "处理 ExecutionComplete: session_id={}, success={}, summary={:?}",
            session_id,
            success,
            summary
        );
        
        // 生成事件 ID
        let event_id = self.next_event_id().await;
        
        // 设置会话摘要
        let summary_str = summary.unwrap_or_else(|| {
            if success {
                "执行成功".to_string()
            } else {
                "执行失败".to_string()
            }
        });
        
        match self
            .session_manager
            .set_summary(session_id.clone(), summary_str.clone(), success)
            .await
        {
            Ok(_) => {
                log::info!("会话已完成: {}", session_id);
                
                // 发送事件到前端
                let event = AgentEvent::ExecutionComplete {
                    event_id,
                    session_id,
                    summary: summary_str,
                    success,
                };
                
                let _ = self.event_tx.send(event);
            }
            Err(e) => {
                log::error!("设置摘要失败: {}", e);
            }
        }
    }
    
    /// 处理 Error 消息
    async fn handle_error(
        &self,
        session_id: String,
        code: String,
        message: String,
        suggestion: String,
    ) {
        log::error!(
            "处理 Error: session_id={}, code={}, message={}, suggestion={}",
            session_id,
            code,
            message,
            suggestion
        );
        
        // 生成事件 ID
        let event_id = self.next_event_id().await;
        
        // 设置会话错误
        match self
            .session_manager
            .set_error(session_id.clone(), code.clone(), message.clone(), suggestion.clone())
            .await
        {
            Ok(_) => {
                log::error!("会话错误已设置: {}", session_id);
                
                // 发送事件到前端
                let event = AgentEvent::Error {
                    event_id,
                    session_id,
                    code,
                    message,
                    suggestion,
                };
                
                let _ = self.event_tx.send(event);
            }
            Err(e) => {
                log::error!("设置错误失败: {}", e);
            }
        }
    }
    
    /// 生成下一个事件 ID
    async fn next_event_id(&self) -> u64 {
        let mut counter = self.event_id_counter.lock().await;
        *counter += 1;
        *counter
    }

    /// 刷新 Skill 列表
    pub async fn refresh_skills(&self) -> AgentResult<Vec<SkillInfo>> {
        log::info!("刷新 Skill 列表");

        // 重新同步 MCP 服务器
        if let Err(e) = self.mcp_manager.sync().await {
            log::warn!("同步 MCP 服务器失败: {}", e);
            // 不阻塞刷新，MCP 错误不是致命的
        }

        // 获取最新的 Skill 列表
        self.get_skills().await
    }

    /// 尝试重启 Agent
    async fn restart_agent(&self) -> AgentResult<()> {
        log::info!("尝试重启 Agent");

        // 获取当前重试次数
        let mut retry_count = self.retry_count.lock().await;

        if *retry_count >= self.max_retries {
            log::error!("达到最大重试次数 ({}), 停止重试", self.max_retries);
            return Err(AgentError::Other(format!(
                "达到最大重试次数 ({}), 请检查 Agent 配置",
                self.max_retries
            )));
        }

        *retry_count += 1;
        let current_retry = *retry_count;
        drop(retry_count);

        log::info!("重试第 {} 次", current_retry);

        // 指数退避
        let delay = std::time::Duration::from_millis(1000 * 2_u64.pow(current_retry - 1));
        tokio::time::sleep(delay).await;

        // 停止当前 Agent
        self.stop().await?;

        // 重新启动
        self.start().await?;

        // 重置重试计数器
        *self.retry_count.lock().await = 0;

        log::info!("Agent 重启成功");
        Ok(())
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
            self.session_manager
                .update_render_mode(session_id.clone(), render_mode)
                .await?;
            
            log::info!("Skill 执行已启动，Session ID: {}", session_id);
            Ok(session_id)
        } else {
            Err(AgentError::ProtocolError {
                message: format!("收到意外响应: {:?}", response),
                suggestion: "请检查 Skill 是否存在".to_string(),
            })
        }
    }

    fn subscribe_events(&self) -> Box<dyn futures::Stream<Item = AgentEvent> + Unpin + Send> {
        // 返回事件流
        Box::new(stream::unfold(self.event_tx.subscribe(), |mut rx| async move {
            match rx.recv().await {
                Ok(event) => Some((event, rx)),
                Err(_) => None,
            }
        }))
    }
}
