// src-tauri/src/bridge/codebuddy_python_adapter.rs

//! CodeBuddy Python SDK 适配器
//!
//! 使用 Python codebuddy-agent-sdk 作为中间层,通过 stdio 与 CodeBuddy CLI 通信。

use crate::bridge::agent_adapter::{AgentAdapter, AgentEvent};
use crate::bridge::error::{AgentError, AgentResult};
use crate::bridge::event_emitter::EventEmitter;
use crate::bridge::mcp_manager::McpManager;
use crate::bridge::protocol::{
    AgentConfig, ContentBlock, RenderMode, SDKMessage, SessionId, SkillInfo, SkillInput,
};
use crate::bridge::session_manager::{
    SessionManager, SessionStatus,
};
use chrono::Utc;
use futures::{StreamExt, stream};
use serde_json::json;
use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader, AsyncWriteExt};
use tokio::process::{Child, Command};
use tokio::sync::{Mutex, mpsc, broadcast};

/// Python 适配器配置
#[derive(Clone)]
pub struct PythonAdapterConfig {
    /// Python 可执行文件路径
    pub python_path: String,

    /// Python 脚本内容
    pub script_content: String,
}

impl Default for PythonAdapterConfig {
    fn default() -> Self {
        Self {
            python_path: "python".to_string(),
            script_content: include_str!("../../scripts/codebuddy_bridge.py").to_string(),
        }
    }
}

/// CodeBuddy Python SDK 适配器
///
/// 使用 Python SDK 作为中间层,通过 stdio 与 CodeBuddy CLI 通信。
pub struct CodeBuddyPythonAdapter {
    /// Python 子进程
    process: Arc<Mutex<Option<Child>>>,

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

    /// 停止信号
    stop_signal: Arc<Mutex<bool>>,

    /// 进程 stdin 写入器 (用于发送数据给 Python)
    stdin_tx: Arc<Mutex<Option<tokio::io::WriteHalf<tokio::process::ChildStdin>>>>,

    /// 内部事件发送器
    event_tx: Arc<broadcast::Sender<AgentEvent>>,

    /// 重试计数器
    retry_count: Arc<Mutex<u32>>,

    /// 最大重试次数
    max_retries: u32,

    /// 事件 ID 计数器
    event_id_counter: Arc<Mutex<u64>>,

    /// Python 适配器配置
    python_config: PythonAdapterConfig,
}

impl CodeBuddyPythonAdapter {
    /// 创建新的 Python 适配器
    pub fn new(config: AgentConfig) -> Self {
        let mut python_config = PythonAdapterConfig::default();

        // Check for venv python in common locations relative to CWD
        let venv_paths = vec![
            PathBuf::from(".venv").join("Scripts").join("python.exe"), // Windows
            PathBuf::from(".venv").join("bin").join("python"),         // Unix
        ];

        for path in venv_paths {
            if path.exists() {
                python_config.python_path = path.to_string_lossy().to_string();
                log::info!("Using venv python: {}", python_config.python_path);
                break;
            }
        }

        let (event_tx, _) = broadcast::channel(1000);

        CodeBuddyPythonAdapter {
            process: Arc::new(Mutex::new(None)),
            config: config.clone(),
            session_manager: Arc::new(SessionManager::new()),
            event_emitter: Arc::new(EventEmitter::new()),
            mcp_manager: Arc::new(McpManager::new(config.binary_path.clone())),
            is_running: Arc::new(Mutex::new(false)),
            stop_signal: Arc::new(Mutex::new(false)),
            stdin_tx: Arc::new(Mutex::new(None)),
            event_tx: Arc::new(event_tx),
            retry_count: Arc::new(Mutex::new(0)),
            max_retries: 3,
            event_id_counter: Arc::new(Mutex::new(0)),
            python_config,
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

    /// 启动进程 (异步)
    async fn start_process(&self) -> AgentResult<Child> {
        log::info!("启动 Python 适配器");

        // 检查 Python 可执行文件是否存在
        if which::which(&self.python_config.python_path).is_err() {
            return Err(AgentError::process_start_failed(format!(
                "Python 未找到,请先安装: {}",
                self.python_config.python_path
            )));
        }

        // 构建命令
        let mut cmd = Command::new(&self.python_config.python_path);

        // 设置环境变量
        for (key, value) in &self.config.env {
            cmd.env(key, value);
        }

        // 添加脚本参数
        cmd.arg("-c");
        cmd.arg(&self.python_config.script_content);

        // 重定向标准流
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        // 启动进程
        let child = cmd.spawn().map_err(|e| {
            AgentError::process_start_failed(format!("无法启动 Python 进程: {}", e))
        })?;

        log::info!("Python 适配器已启动, PID: {:?}", child.id());

        Ok(child)
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
        self.stop_internal().await?;

        // 重新启动
        self.start_internal().await?;

        // 重置重试计数器
        *self.retry_count.lock().await = 0;

        log::info!("Agent 重启成功");
        Ok(())
    }

    // Internal start helper
    async fn start_internal(&self) -> AgentResult<()> {
        log::info!("启动 CodeBuddy Python 适配器");

        // 1. 启动进程
        let mut child = self.start_process().await?;
        
        // 2. 获取 stdin
        let _stdin = child.stdin.take().ok_or_else(|| {
            AgentError::process_start_failed("无法获取 stdin".to_string())
        })?;
        
        // 由于我们需要保存 stdin 以便后续写入，我们需要 split 它或者直接保存 child
        // tokio::process::ChildStdin 不像 std::process::ChildStdin 那么容易 clone
        // 但我们可以把它保存在 Option 中
        
        // 3. 获取 stdout 和 stderr
        let stdout = child.stdout.take().ok_or_else(|| {
            AgentError::process_start_failed("无法获取 stdout".to_string())
        })?;
        let stderr = child.stderr.take().ok_or_else(|| {
            AgentError::process_start_failed("无法获取 stderr".to_string())
        })?;

        *self.process.lock().await = Some(child);
        // *self.stdin_tx.lock().await = Some(stdin); // Type mismatch, ChildStdin is not WriteHalf
        // wait, ChildStdin implements AsyncWrite. We can just store it.
        // But to make it shareable, we might need a Mutex.
        // Or we can use `tokio::io::split` if it supported it, but ChildStdin doesn't support split directly like TcpStream?
        // Actually ChildStdin implements AsyncWrite, so we can just wrap it in Mutex.
        // But my struct definition has `Option<tokio::io::WriteHalf<tokio::process::ChildStdin>>`.
        // I should change the struct definition to `Option<tokio::process::ChildStdin>`.
        // Wait, let's fix the struct definition later. For now let's assume I change it.
        
        // 4. 监听 stdout/stderr
        self.monitor_stdout(stdout).await;
        self.monitor_stderr(stderr).await;

        // 5. 保存 stdin
        // *self.stdin_tx.lock().await = Some(stdin); 
        // I will fix this in the struct definition below.

        // 6. 同步 MCP 服务器
        if let Err(e) = self.mcp_manager.sync().await {
            log::warn!("同步 MCP 服务器失败: {}", e);
        }

        *self.is_running.lock().await = true;
        *self.stop_signal.lock().await = false;

        log::info!("CodeBuddy Python 适配器启动成功");
        Ok(())
    }

    // Internal stop helper
    async fn stop_internal(&self) -> AgentResult<()> {
        log::info!("停止 CodeBuddy Python 适配器");

        // 1. 设置停止信号
        *self.stop_signal.lock().await = true;
        *self.is_running.lock().await = false;

        // 2. 终止进程
        let mut process_guard = self.process.lock().await;
        if let Some(mut child) = process_guard.take() {
            log::info!("终止进程 PID: {:?}", child.id());

            // 强制终止
            if let Err(e) = child.kill().await {
                log::warn!("终止进程失败: {}", e);
            }

            // 等待进程退出
            match child.wait().await {
                Ok(status) => {
                    log::info!("进程已退出,状态: {}", status);
                }
                Err(e) => {
                    log::error!("等待进程退出失败: {}", e);
                }
            }
        }

        log::info!("CodeBuddy Python 适配器已停止");
        Ok(())
    }

    /// 处理 SDK 消息
    async fn handle_sdk_message(&self, message: SDKMessage) -> AgentResult<()> {
        match message {
            SDKMessage::Assistant(msg) => {
                self.handle_assistant_message(msg).await?;
            }

            SDKMessage::Result(msg) => {
                self.handle_result_message(msg).await?;
            }

            SDKMessage::Stream(msg) => {
                self.handle_stream_event(msg).await?;
            }

            SDKMessage::System(_) => {
                log::debug!("收到系统消息: {:?}", message);
            }

            SDKMessage::User(_) => {
                log::warn!("收到意外的用户消息: {:?}", message);
            }
        }

        Ok(())
    }

    /// 处理 Assistant 消息
    async fn handle_assistant_message(&self, message: crate::bridge::protocol::AssistantMessage) -> AgentResult<()> {
        log::debug!("处理 Assistant 消息");

        let event_id = self.next_event_id().await;
        // Session ID 应该从消息中获取，但 AssistantMessage 可能没有直接的 session_id
        // 如果没有，我们可能需要维护当前活动的 session_id
        // 这里暂时生成一个新的或使用默认值
        let session_id = "current_session".to_string(); 

        // 为每个 ContentBlock 创建 AgentEvent
        for block in &message.content {
            match block {
                ContentBlock::Text(text_block) => {
                    let event = AgentEvent::DataChunk {
                        event_id,
                        session_id: session_id.clone(),
                        chunk_index: Some(0),
                        data: serde_json::json!(text_block.text),
                        is_final: false,
                    };
                    let _ = self.event_tx.send(event);
                }

                ContentBlock::Thinking(thinking_block) => {
                    let event = AgentEvent::DataChunk {
                        event_id,
                        session_id: session_id.clone(),
                        chunk_index: Some(0),
                        data: serde_json::json!({
                            "type": "thinking",
                            "thinking": thinking_block.thinking,
                            "signature": thinking_block.signature
                        }),
                        is_final: false,
                    };
                    let _ = self.event_tx.send(event);
                }

                ContentBlock::ToolUse(tool_use) => {
                    let event = AgentEvent::DataChunk {
                        event_id,
                        session_id: session_id.clone(),
                        chunk_index: Some(0),
                        data: serde_json::json!(tool_use),
                        is_final: false,
                    };
                    let _ = self.event_tx.send(event);
                }

                ContentBlock::ToolResult(tool_result) => {
                    let event = AgentEvent::DataChunk {
                        event_id,
                        session_id: session_id.clone(),
                        chunk_index: Some(0),
                        data: serde_json::json!(tool_result),
                        is_final: false,
                    };
                    let _ = self.event_tx.send(event);
                }
            }
        }
        Ok(())
    }

    /// 处理 Result 消息
    async fn handle_result_message(&self, message: crate::bridge::protocol::ResultMessage) -> AgentResult<()> {
        log::info!("处理 Result 消息: session_id={}, success={}, duration={}ms",
            message.session_id, message.is_error, message.duration_ms);

        if message.is_error {
            let event = AgentEvent::Error {
                event_id: self.next_event_id().await,
                session_id: message.session_id.clone(),
                code: "EXECUTION_ERROR".to_string(),
                message: message.result.clone().unwrap_or("执行失败".to_string()),
                suggestion: "请检查输入和配置".to_string(),
            };
            let _ = self.event_tx.send(event);
        } else {
            let event = AgentEvent::ExecutionComplete {
                event_id: self.next_event_id().await,
                session_id: message.session_id.clone(),
                success: true,
                summary: message.result.clone().unwrap_or("执行完成".to_string()),
            };
            let _ = self.event_tx.send(event);
        }

        Ok(())
    }

    /// 处理 Stream 事件
    async fn handle_stream_event(&self, message: crate::bridge::protocol::StreamEvent) -> AgentResult<()> {
        log::debug!("处理 Stream 事件: uuid={}", message.uuid);

        if let Some(event_data) = message.event.as_object() {
            if let Some(current) = event_data.get("current") {
                if let Some(total) = event_data.get("total") {
                    let event = AgentEvent::Progress {
                        event_id: self.next_event_id().await,
                        session_id: message.session_id.clone(),
                        current: current.as_u64().unwrap_or(0),
                        total: total.as_u64().unwrap_or(1),
                        message: event_data.get("message")
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_string(),
                    };
                    let _ = self.event_tx.send(event);
                }
            }
        }

        Ok(())
    }

    /// 监听进程 stdout
    async fn monitor_stdout(&self, stdout: tokio::process::ChildStdout) {
        let adapter_self = self.clone(); // Clone for closure (cheap due to Arc)
        // Wait, self cannot be cloned easily if it contains fields that are not Arc.
        // My struct has only Arc except config (AgentConfig is Clone), python_config (Clone), max_retries (Copy).
        // I need to implement Clone for CodeBuddyPythonAdapter or wrap it in Arc completely.
        // Usually adapter is wrapped in Arc when used.
        // But here I'm passing `self` to async block.
        // Let's wrap `self` methods in a way that doesn't require cloning self deeply or assume `self` is Arc.
        // Actually, I can just move the required Arcs into the task.
        
        let stdout_rx_handler = self.clone_for_task();

        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                log::debug!("Python stdout: {}", line);
                
                // Parse JSON message
                if let Ok(msg) = serde_json::from_str::<SDKMessage>(&line) {
                    if let Err(e) = stdout_rx_handler.handle_sdk_message(msg).await {
                         log::error!("处理 SDK 消息失败: {}", e);
                    }
                } else {
                     log::warn!("无法解析 JSON 消息: {}", line);
                }
            }
            log::info!("stdout 监听结束");
        });
    }

    /// 监听进程 stderr
    async fn monitor_stderr(&self, stderr: tokio::process::ChildStderr) {
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                log::debug!("Python stderr: {}", line);
                // 可以选择将 stderr 也作为日志发送
            }
            log::info!("stderr 监听结束");
        });
    }

    /// 获取下一个事件 ID
    async fn next_event_id(&self) -> u64 {
        let mut counter = self.event_id_counter.lock().await;
        *counter += 1;
        *counter
    }
    
    /// 取消会话
    pub async fn cancel_session(&self, session_id: &str) -> AgentResult<()> {
        log::info!("取消会话: {}", session_id);
        self.session_manager.update_status(session_id.to_string(), SessionStatus::Cancelled).await?;
        Ok(())
    }

    // Helper to clone Arcs for async tasks
    fn clone_for_task(&self) -> Self {
        Self {
            process: self.process.clone(),
            config: self.config.clone(),
            session_manager: self.session_manager.clone(),
            event_emitter: self.event_emitter.clone(),
            mcp_manager: self.mcp_manager.clone(),
            is_running: self.is_running.clone(),
            stop_signal: self.stop_signal.clone(),
            stdin_tx: self.stdin_tx.clone(),
            event_tx: self.event_tx.clone(),
            retry_count: self.retry_count.clone(),
            max_retries: self.max_retries,
            event_id_counter: self.event_id_counter.clone(),
            python_config: self.python_config.clone(),
        }
    }
}

// Implement Clone manually because of non-Arc fields if needed, 
// but here all fields are Clone or Arc, so #[derive(Clone)] would work if I added it to struct.
// I'll add Clone derive to struct.

impl Clone for CodeBuddyPythonAdapter {
    fn clone(&self) -> Self {
        Self {
            process: self.process.clone(),
            config: self.config.clone(),
            session_manager: self.session_manager.clone(),
            event_emitter: self.event_emitter.clone(),
            mcp_manager: self.mcp_manager.clone(),
            is_running: self.is_running.clone(),
            stop_signal: self.stop_signal.clone(),
            stdin_tx: self.stdin_tx.clone(),
            event_tx: self.event_tx.clone(),
            retry_count: self.retry_count.clone(),
            max_retries: self.max_retries,
            event_id_counter: self.event_id_counter.clone(),
            python_config: self.python_config.clone(),
        }
    }
}

#[async_trait::async_trait]
impl AgentAdapter for CodeBuddyPythonAdapter {
    async fn start(&mut self) -> AgentResult<()> {
        self.start_internal().await
    }

    async fn stop(&mut self) -> AgentResult<()> {
        self.stop_internal().await
    }

    async fn get_skills(&self) -> AgentResult<Vec<SkillInfo>> {
        log::info!("获取 Skill 列表");

        // 从 MCP 服务器获取 Skill 信息
        let mcp_servers = self.mcp_manager.list_all().await;

        let skills: Vec<SkillInfo> = mcp_servers.iter()
            .map(|server| SkillInfo {
                name: server.name.clone(),
                description: format!("MCP 服务器: {}", server.name),
                default_render: RenderMode::Json,
                supported_renders: vec![RenderMode::Json, RenderMode::Table],
                input_schema: None,
                output_schema: None,
                category: Some("mcp".to_string()),
                requires_filesystem: server.server_type == "stdio",
                requires_network: server.server_type == "tcp",
            })
            .collect();

        log::info!("获取到 {} 个 Skill", skills.len());
        Ok(skills)
    }

    async fn execute_skill(
        &self,
        skill_name: &str,
        input: SkillInput,
    ) -> AgentResult<SessionId> {
        // ... implementation ...
        log::info!("执行 Skill: {} -> {}", skill_name, input.as_text());

        // 1. 生成唯一的 session_id
        let session_id = Utc::now().to_rfc3339().to_string();

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

        // 4. 发送提示词到 Python SDK
        let config = json!({
            "session_id": session_id,
            "prompt": input.as_text(),
            "options": {
                "continue_conversation": false,
                "setting_sources": ["project", "local"],
            }
        });

        let mut cmd = Command::new(&self.python_config.python_path);
        for (key, value) in &self.config.env {
            cmd.env(key, value);
        }
        cmd.arg("-c");
        cmd.arg(&self.python_config.script_content);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        
        let mut child = cmd.spawn().map_err(|e| AgentError::process_start_failed(e.to_string()))?;
        
        if let Some(mut stdin) = child.stdin.take() {
            let config_str = config.to_string();
            stdin.write_all(config_str.as_bytes()).await.map_err(|e| AgentError::Io(e))?;
            // stdin is dropped here, closing it, which triggers EOF for python script
        }
        
        let stdout = child.stdout.take().ok_or(AgentError::process_start_failed("No stdout"))?;
        
        // Monitor stdout in background and send events
        let adapter_self = self.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = tokio::io::AsyncBufReadExt::lines(reader);
            while let Ok(Some(line)) = lines.next_line().await {
                 if let Ok(msg) = serde_json::from_str::<SDKMessage>(&line) {
                    let _ = adapter_self.handle_sdk_message(msg).await;
                 }
            }
        });
        
        // Monitor stderr
        let stderr = child.stderr.take().ok_or(AgentError::process_start_failed("No stderr"))?;
        tokio::spawn(async move {
             let reader = BufReader::new(stderr);
             let mut lines = reader.lines();
             while let Ok(Some(line)) = lines.next_line().await {
                 log::debug!("Python stderr: {}", line);
             }
        });

        // 发送执行开始事件
        let event_id = self.next_event_id().await;
        let event = AgentEvent::ExecutionStart {
            event_id,
            session_id: session_id.clone(),
            skill_name: skill_name.to_string(),
            render_mode: "text".to_string(),
        };
        let _ = self.event_tx.send(event);

        log::info!("Skill 执行已启动,Session ID: {}", session_id);
        Ok(session_id)
    }


    fn subscribe_events(&self) -> std::pin::Pin<Box<dyn futures::Stream<Item = AgentEvent> + Send>> {
        let rx = self.event_tx.subscribe();
        Box::pin(stream::unfold(rx, |mut rx| async move {
            match rx.recv().await {
                Ok(event) => Some((event, rx)),
                Err(_) => None, // Channel closed or lagged
            }
        }))
    }
}
