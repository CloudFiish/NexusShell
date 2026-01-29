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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum RenderMode {
    /// 结构化数据、列表
    Table,

    /// 代码片段、diff
    Code,

    /// 复杂嵌套数据
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
    pub requires_filesystem: bool,

    /// 是否需要网络访问
    pub requires_network: bool,
}

/// Skill 输入
///
/// 执行 Skill 时传入的参数，可以是字符串或结构化 JSON 数据。
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
}

/// Control Channel 消息类型
///
/// 定义了 Bridge 和 Agent 之间传输的所有消息类型。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ControlMessage {
    /// 获取 Skill 列表请求
    GetSkills,

    /// Skill 列表响应
    SkillList {
        skills: Vec<SkillInfo>,
    },

    /// 执行 Skill 请求
    ExecuteSkill {
        skill_name: String,
        input: SkillInput,
    },

    /// 执行开始通知
    ExecutionStart {
        session_id: SessionId,
        skill_name: String,
        render_mode: RenderMode,
    },

    /// 流式数据块
    DataChunk {
        session_id: SessionId,
        chunk_index: u64,
        data: serde_json::Value,
        is_final: bool,
    },

    /// 进度更新
    Progress {
        session_id: SessionId,
        current: u64,
        total: u64,
        message: String,
    },

    /// 错误信息
    Error {
        session_id: SessionId,
        error_code: String,
        message: String,
        suggestion: String,
    },

    /// 执行完成
    ExecutionComplete {
        session_id: SessionId,
        success: bool,
        summary: String,
    },

    /// 取消执行请求
    CancelExecution {
        session_id: SessionId,
    },

    /// 心跳 (用于保持连接活跃)
    Heartbeat,

    /// 心跳响应
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
    /// Agent 类型 (如 "codebuddy", "claude-code")
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
            agent_type: "codebuddy".to_string(),
            binary_path: "codebuddy".to_string(),
            control_port: 0,
            timeout_ms: 30000,
            env: HashMap::new(),
            mcp_servers: Vec::new(),
        }
    }
}
