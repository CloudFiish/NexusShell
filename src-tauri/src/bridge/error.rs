// src-tauri/src/bridge/error.rs

use thiserror::Error;

/// Agent 操作结果类型
pub type AgentResult<T> = Result<T, AgentError>;

/// Agent 错误类型
///
/// 定义了所有 Agent 操作可能遇到的错误，包括进程错误、通信错误、协议错误等。
#[derive(Error, Debug)]
pub enum AgentError {
    /// 进程启动失败
    #[error("进程启动失败: {message}")]
    ProcessStartFailed {
        message: String,
        suggestion: String,
    },

    /// 进程已退出
    #[error("进程已退出，退出码: {exit_code:?}, 原因: {reason}")]
    ProcessExited {
        exit_code: Option<i32>,
        reason: String,
    },

    /// 进程执行失败
    #[error("进程执行失败: {message}")]
    ProcessExecutionFailed {
        message: String,
        suggestion: String,
    },

    /// 端口冲突
    #[error("端口冲突: 端口 {port} 已被占用")]
    PortConflict {
        port: u16,
        suggestion: String,
    },

    /// 通信错误
    #[error("通信错误: {message}")]
    CommunicationError {
        message: String,
        suggestion: String,
    },

    /// WebSocket 连接失败
    #[error("WebSocket 连接失败: {message}")]
    WebSocketConnectionFailed {
        message: String,
        suggestion: String,
    },

    /// WebSocket 断开
    #[error("WebSocket 连接断开: {reason}")]
    WebSocketDisconnected {
        reason: String,
        suggestion: String,
    },

    /// 消息发送失败
    #[error("消息发送失败: {message}")]
    MessageSendFailed {
        message: String,
        suggestion: String,
    },

    /// 消息接收超时
    #[error("消息接收超时: 超过 {timeout_ms}ms")]
    MessageReceiveTimeout {
        timeout_ms: u64,
        suggestion: String,
    },

    /// 协议解析错误
    #[error("协议解析错误: {message}")]
    ProtocolParseError {
        message: String,
        suggestion: String,
    },

    /// 协议错误 (无效的消息格式或字段)
    #[error("协议错误: {message}")]
    ProtocolError {
        message: String,
        suggestion: String,
    },

    /// Skill 不存在
    #[error("Skill 不存在: {skill_name}")]
    SkillNotFound {
        skill_name: String,
        suggestion: String,
    },

    /// Skill 执行失败
    #[error("Skill 执行失败: {skill_name}, 原因: {reason}")]
    SkillExecutionFailed {
        skill_name: String,
        reason: String,
        suggestion: String,
    },

    /// 会话不存在
    #[error("会话不存在: {session_id}")]
    SessionNotFound {
        session_id: String,
        suggestion: String,
    },

    /// 会话已存在
    #[error("会话已存在: {session_id}")]
    SessionAlreadyExists {
        session_id: String,
        suggestion: String,
    },

    /// 输入参数无效
    #[error("输入参数无效: {message}")]
    InvalidInput {
        message: String,
        suggestion: String,
    },

    /// MCP 服务器错误
    #[error("MCP 服务器错误: {message}")]
    McpServerError {
        message: String,
        suggestion: String,
    },

    /// 超时错误
    #[error("操作超时: {message}")]
    Timeout {
        message: String,
        suggestion: String,
    },

    /// IO 错误
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    /// JSON 序列化/反序列化错误
    #[error("JSON 错误: {0}")]
    Json(#[from] serde_json::Error),

    /// 其他未知错误
    #[error("未知错误: {0}")]
    Other(String),
}

impl AgentError {
    /// 创建进程启动失败错误
    pub fn process_start_failed(message: impl Into<String>) -> Self {
        let msg = message.into();
        AgentError::ProcessStartFailed {
            message: msg.clone(),
            suggestion: format!(
                "请检查: 1) CodeBuddy Code 是否已安装; 2) 路径是否正确; 3) 是否有执行权限。错误: {}",
                msg
            ),
        }
    }

    /// 创建端口冲突错误
    pub fn port_conflict(port: u16) -> Self {
        AgentError::PortConflict {
            port,
            suggestion: format!(
                "请尝试: 1) 关闭占用该端口的程序; 2) 在配置中更换其他端口; 3) 设置 port=0 使用动态端口。",
            ),
        }
    }

    /// 创建 WebSocket 连接失败错误
    pub fn websocket_connection_failed(message: impl Into<String>) -> Self {
        let msg = message.into();
        AgentError::WebSocketConnectionFailed {
            message: msg.clone(),
            suggestion: format!(
                "请检查: 1) Agent 是否正常运行; 2) Control Channel 端口是否正确; 3) 防火墙是否阻止连接。错误: {}",
                msg
            ),
        }
    }

    /// 创建 Skill 不存在错误
    pub fn skill_not_found(skill_name: impl Into<String>) -> Self {
        let name = skill_name.into();
        AgentError::SkillNotFound {
            skill_name: name.clone(),
            suggestion: format!(
                "请检查: 1) Skill 名称是否正确; 2) MCP 服务器是否已加载; 3) 使用 get_skills 查看可用 Skill。",
            ),
        }
    }

    /// 创建超时错误
    pub fn timeout(message: impl Into<String>, timeout_ms: u64) -> Self {
        let msg = message.into();
        AgentError::Timeout {
            message: msg.clone(),
            suggestion: format!(
                "操作超时 ({}ms)。请尝试: 1) 增加配置中的 timeout_ms; 2) 检查网络连接; 3) 查看 Agent 日志。",
                timeout_ms
            ),
        }
    }

    /// 判断错误是否可重试
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            AgentError::WebSocketDisconnected { .. }
                | AgentError::CommunicationError { .. }
                | AgentError::Timeout { .. }
                | AgentError::MessageReceiveTimeout { .. }
        )
    }

    /// 获取错误代码 (用于前端显示和国际化)
    pub fn error_code(&self) -> &'static str {
        match self {
            AgentError::ProcessStartFailed { .. } => "PROCESS_START_FAILED",
            AgentError::ProcessExited { .. } => "PROCESS_EXITED",
            AgentError::ProcessExecutionFailed { .. } => "PROCESS_EXECUTION_FAILED",
            AgentError::PortConflict { .. } => "PORT_CONFLICT",
            AgentError::CommunicationError { .. } => "COMMUNICATION_ERROR",
            AgentError::WebSocketConnectionFailed { .. } => "WEBSOCKET_CONNECTION_FAILED",
            AgentError::WebSocketDisconnected { .. } => "WEBSOCKET_DISCONNECTED",
            AgentError::MessageSendFailed { .. } => "MESSAGE_SEND_FAILED",
            AgentError::MessageReceiveTimeout { .. } => "MESSAGE_RECEIVE_TIMEOUT",
            AgentError::ProtocolParseError { .. } => "PROTOCOL_PARSE_ERROR",
            AgentError::ProtocolError { .. } => "PROTOCOL_ERROR",
            AgentError::SkillNotFound { .. } => "SKILL_NOT_FOUND",
            AgentError::SkillExecutionFailed { .. } => "SKILL_EXECUTION_FAILED",
            AgentError::SessionNotFound { .. } => "SESSION_NOT_FOUND",
            AgentError::SessionAlreadyExists { .. } => "SESSION_ALREADY_EXISTS",
            AgentError::InvalidInput { .. } => "INVALID_INPUT",
            AgentError::McpServerError { .. } => "MCP_SERVER_ERROR",
            AgentError::Timeout { .. } => "TIMEOUT",
            AgentError::Io(_) => "IO_ERROR",
            AgentError::Json(_) => "JSON_ERROR",
            AgentError::Other(_) => "UNKNOWN_ERROR",
        }
    }
}
