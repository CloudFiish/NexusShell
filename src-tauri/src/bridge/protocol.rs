// src-tauri/src/bridge/protocol.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 会话 ID
///
/// 每次 Skill 执行都有唯一的会话 ID，用于跟踪执行状态和接收事件。
pub type SessionId = String;

/// 渲染模式
///
/// 定义了不同的数据渲染方式，前端根据这个字段选择合适的渲染器。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum RenderMode {
    /// 结构化数据、列表
    Table,

    /// 代码片段、diff
    Code,

    /// 复杂嵌套数据
    #[default]
    Json,

    /// 流式文本、日志
    Log,

    /// 数值统计、图表
    Chart,

    /// 文件系统、目录树
    FileTree,

    /// 文档、说明
    Markdown,

    /// 文件差异、变更
    Diff,
}

impl RenderMode {
    /// 获取所有支持的渲染模式
    pub fn all() -> Vec<RenderMode> {
        vec![
            RenderMode::Table,
            RenderMode::Code,
            RenderMode::Json,
            RenderMode::Log,
            RenderMode::Chart,
            RenderMode::FileTree,
            RenderMode::Markdown,
            RenderMode::Diff,
        ]
    }
}

/// ContentBlock 类型
///
/// CodeBuddy SDK 支持的内容块类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ContentBlock {
    /// 纯文本内容
    Text(TextBlock),

    /// 内部推理过程
    Thinking(ThinkingBlock),

    /// 工具使用请求
    ToolUse(ToolUseBlock),

    /// 工具执行结果
    ToolResult(ToolResultBlock),
}

/// 文本块
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextBlock {
    /// 文本内容
    pub text: String,
}

/// 思考块 (内部推理)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThinkingBlock {
    /// 思考过程
    pub thinking: String,

    /// 签名
    pub signature: String,
}

/// 工具使用块
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolUseBlock {
    /// 工具 ID
    pub id: String,

    /// 工具名称
    pub name: String,

    /// 输入参数
    pub input: serde_json::Value,
}

/// 工具结果块
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolResultBlock {
    /// 对应的工具使用 ID
    #[serde(rename = "tool_use_id")]
    pub tool_use_id: String,

    /// 结果内容
    pub content: Option<String>,

    /// 是否出错
    #[serde(rename = "is_error")]
    pub is_error: Option<bool>,
}

/// Skill 信息
///
/// 包含 Skill 的元数据，包括名称、描述、支持的渲染模式等。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInfo {
    /// Skill 名称 (唯一标识符)
    pub name: String,

    /// Skill 描述
    pub description: String,

    /// 默认渲染模式
    #[serde(default)]
    pub default_render: RenderMode,

    /// 支持的渲染模式列表
    pub supported_renders: Vec<RenderMode>,

    /// 输入参数的 JSON Schema
    pub input_schema: Option<serde_json::Value>,

    /// 输出数据的 JSON Schema
    pub output_schema: Option<serde_json::Value>,

    /// Skill 分类
    pub category: Option<String>,

    /// 是否需要文件系统访问
    #[serde(default)]
    pub requires_filesystem: bool,

    /// 是否需要网络访问
    #[serde(default)]
    pub requires_network: bool,
}

/// Skill 输入
///
/// 执行 Skill 时传入的参数，可以是字符串或结构化 JSON 数据。
/// 注意: 与 CodeBuddy SDK 不同，这里保留向后兼容性
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SkillInput {
    /// 纯文本输入 (如自然语言描述)
    Text(String),

    /// 结构化输入 (如 JSON 对象)
    Structured(serde_json::Value),
}

impl SkillInput {
    /// 将输入转换为字符串
    pub fn as_text(&self) -> String {
        match self {
            SkillInput::Text(s) => s.clone(),
            SkillInput::Structured(v) => serde_json::to_string(v).unwrap_or_default(),
        }
    }

    /// 转换为 CodeBuddy SDK prompt 格式
    pub fn to_prompt(&self) -> String {
        self.as_text()
    }
}

/// CodeBuddy SDK 消息类型
///
/// 匹配 CodeBuddy Python SDK 的消息类型定义
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SDKMessage {
    /// 用户消息
    User(UserMessage),

    /// 助手消息
    Assistant(AssistantMessage),

    /// 系统消息
    System(SystemMessage),

    /// 结果消息 (表示请求完成)
    Result(ResultMessage),

    /// 流式事件 (实时更新)
    Stream(StreamEvent),
}

/// 用户消息
///
/// 用户发送的输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    // 用户消息可能没有显式字段，SDK 会自动处理
}

/// 助手消息
///
/// AI 的响应内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessage {
    /// 内容块列表 (TextBlock, ThinkingBlock, ToolUseBlock, ToolResultBlock)
    pub content: Vec<ContentBlock>,

    /// 使用的模型
    pub model: String,

    /// 父级工具使用 ID
    #[serde(rename = "parent_tool_use_id")]
    pub parent_tool_use_id: Option<String>,

    /// 错误信息
    pub error: Option<String>,
}

/// 系统消息
///
/// 系统级别的消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessage {
    /// 消息内容
    pub content: String,
}

/// 结果消息
///
/// 表示请求完成，包含最终状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultMessage {
    /// 子类型
    pub subtype: String,

    /// 执行时长 (毫秒)
    #[serde(rename = "duration_ms")]
    pub duration_ms: i64,

    /// API 执行时长 (毫秒)
    #[serde(rename = "duration_api_ms")]
    #[serde(default)]
    pub duration_api_ms: i64,

    /// 是否出错
    #[serde(rename = "is_error")]
    pub is_error: bool,

    /// 交互轮数
    #[serde(rename = "num_turns")]
    pub num_turns: i64,

    /// 会话 ID
    #[serde(rename = "session_id")]
    pub session_id: String,

    /// 总成本 (美元)
    #[serde(rename = "total_cost_usd")]
    pub total_cost_usd: Option<f64>,

    /// 结果文本
    pub result: Option<String>,

    /// 使用统计
    pub usage: Option<serde_json::Value>,
}

/// 流式事件
///
/// 实时更新事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    /// 事件 UUID
    pub uuid: String,

    /// 会话 ID
    #[serde(rename = "session_id")]
    pub session_id: String,

    /// 事件数据
    pub event: serde_json::Value,

    /// 父级工具使用 ID
    #[serde(rename = "parent_tool_use_id")]
    pub parent_tool_use_id: Option<String>,
}

/// Control Channel 消息类型 (保留向后兼容性)
///
/// 定义了 Bridge 和 Agent 之间传输的所有消息类型。
/// 注意: 这个是旧的 ACP 协议,将与 CodeBuddy SDK 消息类型一起使用
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ControlMessage {
    /// 获取 Skill 列表请求 (旧)
    GetSkills,

    /// Skill 列表响应 (旧)
    SkillList {
        skills: Vec<SkillInfo>,
    },

    /// 执行 Skill 请求 (旧)
    ExecuteSkill {
        skill_name: String,
        input: SkillInput,
    },

    /// 执行开始通知 (旧)
    ExecutionStart {
        session_id: SessionId,
        skill_name: String,
        render_mode: RenderMode,
    },

    /// 流式数据块 (旧)
    DataChunk {
        session_id: SessionId,
        chunk_index: u64,
        data: serde_json::Value,
        is_final: bool,
    },

    /// 进度更新 (旧)
    Progress {
        session_id: SessionId,
        current: u64,
        total: u64,
        message: String,
    },

    /// 错误信息 (旧)
    Error {
        session_id: SessionId,
        error_code: String,
        message: String,
        suggestion: String,
    },

    /// 执行完成 (旧)
    ExecutionComplete {
        session_id: SessionId,
        success: bool,
        summary: String,
    },

    /// 取消执行请求 (旧)
    CancelExecution {
        session_id: SessionId,
    },

    /// 心跳 (旧)
    Heartbeat,

    /// 心跳响应 (旧)
    HeartbeatAck,
}

impl ControlMessage {
    /// 创建 GetSkills 请求
    pub fn get_skills() -> Self {
        ControlMessage::GetSkills
    }

    /// 创建 ExecuteSkill 请求
    pub fn execute_skill(skill_name: String, input: SkillInput) -> Self {
        ControlMessage::ExecuteSkill { skill_name, input }
    }

    /// 创建 CancelExecution 请求
    pub fn cancel_execution(session_id: SessionId) -> Self {
        ControlMessage::CancelExecution { session_id }
    }

    /// 创建心跳消息
    pub fn heartbeat() -> Self {
        ControlMessage::Heartbeat
    }

    /// 获取消息类型字符串
    pub fn message_type(&self) -> &'static str {
        match self {
            ControlMessage::GetSkills => "get_skills",
            ControlMessage::SkillList { .. } => "skill_list",
            ControlMessage::ExecuteSkill { .. } => "execute_skill",
            ControlMessage::ExecutionStart { .. } => "execution_start",
            ControlMessage::DataChunk { .. } => "data_chunk",
            ControlMessage::Progress { .. } => "progress",
            ControlMessage::Error { .. } => "error",
            ControlMessage::ExecutionComplete { .. } => "execution_complete",
            ControlMessage::CancelExecution { .. } => "cancel_execution",
            ControlMessage::Heartbeat => "heartbeat",
            ControlMessage::HeartbeatAck => "heartbeat_ack",
        }
    }

    /// 获取消息关联的会话 ID (如果有)
    pub fn session_id(&self) -> Option<&SessionId> {
        match self {
            ControlMessage::ExecutionStart { session_id, .. } => Some(session_id),
            ControlMessage::DataChunk { session_id, .. } => Some(session_id),
            ControlMessage::Progress { session_id, .. } => Some(session_id),
            ControlMessage::Error { session_id, .. } => Some(session_id),
            ControlMessage::ExecutionComplete { session_id, .. } => Some(session_id),
            ControlMessage::CancelExecution { session_id, .. } => Some(session_id),
            _ => None,
        }
    }
}

/// MCP 服务器配置
///
/// MCP (Model Context Protocol) 服务器的配置信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// 服务器名称
    pub name: String,

    /// 服务器类型 (如 "stdio", "tcp", "http")
    #[serde(default = "default_server_type")]
    pub server_type: String,

    /// 服务器状态 (如 "Connected", "Disconnected", "Unknown")
    #[serde(default = "default_status")]
    pub status: String,

    /// 命令 (如 "npx" 或绝对路径)
    pub command: String,

    /// 命令参数
    pub args: Vec<String>,

    /// 环境变量
    pub env: Option<HashMap<String, String>>,

    /// 是否启用
    pub enabled: bool,
}

/// 默认服务器类型
fn default_server_type() -> String {
    "stdio".to_string()
}

/// 默认状态
fn default_status() -> String {
    "Unknown".to_string()
}

/// Agent 配置
///
/// Agent CLI 的运行配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent 类型 (如 "codebuddy", "codebuddy-sdk")
    pub agent_type: String,

    /// 二进制文件路径
    pub binary_path: String,

    /// Control Channel 端口 (0 表示自动分配)
    pub control_port: u16,

    /// 超时时间 (毫秒)
    pub timeout_ms: u64,

    /// 环境变量
    pub env: HashMap<String, String>,

    /// MCP 服务器列表
    pub mcp_servers: Vec<McpServerConfig>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        AgentConfig {
            agent_type: "codebuddy-sdk".to_string(),
            binary_path: "codebuddy".to_string(),
            control_port: 0,
            timeout_ms: 30000,
            env: HashMap::new(),
            mcp_servers: Vec::new(),
        }
    }
}

/// 会话状态 (更新)
///
/// 与 SessionManager 中的状态定义保持一致
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionStatus {
    Pending,
    Running,
    Paused,
    Cancelled,
    Completed,
    Error,
}

/// 数据块 (更新)
///
/// 与 SessionManager 中的 DataBlock 定义保持一致
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataChunk {
    /// 块索引
    pub index: u64,

    /// 数据
    pub data: serde_json::Value,

    /// 是否是最后一个块
    pub is_final: bool,

    /// 接收时间
    pub received_at: String,

    /// 数据大小 (字节)
    pub size: usize,
}

/// 进度信息 (新增)
///
/// 详细的进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressInfo {
    /// 当前值
    pub current: u64,

    /// 总值
    pub total: u64,

    /// 百分比 (0-100)
    pub percentage: u64,

    /// 进度消息
    pub message: String,

    /// 更新时间
    pub updated_at: String,
}

/// 错误信息 (新增)
///
/// 详细的错误信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// 错误代码
    pub code: String,

    /// 错误消息
    pub message: String,

    /// 建议解决方案
    pub suggestion: String,

    /// 发生时间
    pub occurred_at: String,
}
