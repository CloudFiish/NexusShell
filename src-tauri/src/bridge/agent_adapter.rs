// src-tauri/src/bridge/agent_adapter.rs

use crate::bridge::error::{AgentError, AgentResult};
use crate::bridge::protocol::{SkillInfo, SessionId, SkillInput};
use crate::bridge::event_emitter::EventEmitter;
use futures::Stream;

/// Agent 适配器 trait - 所有 Agent CLI 的统一接口
///
/// 这个 trait 定义了所有 Agent 必须实现的核心功能，包括启动/停止、
/// Skill 发现、执行以及事件订阅。通过这个接口，Bridge 可以无缝切换
/// 不同的 Agent 实现 (如 CodeBuddy Code、Claude Code、Aider 等)。
#[async_trait::async_trait]
pub trait AgentAdapter {
    /// 启动 Agent
    ///
    /// 启动 Agent 进程，建立双通道通信 (stdout/stderr 和 Control Channel)，
    /// 初始化必要的资源。
    ///
    /// # 错误
    /// - 进程启动失败 (二进制文件不存在、权限不足等)
    /// - 端口冲突 (Control Channel 端口被占用)
    /// - 通信初始化失败
    async fn start(&mut self) -> AgentResult<()>;

    /// 停止 Agent
    ///
    /// 优雅地停止 Agent 进程，清理所有资源，关闭所有连接。
    /// 确保所有正在执行的 Skill 被正确终止，所有状态被持久化。
    ///
    /// # 错误
    /// - 进程无法终止
    /// - 资源清理失败
    async fn stop(&mut self) -> AgentResult<()>;

    /// 获取可用的 Skill 列表
    ///
    /// 查询 Agent 当前加载的所有 Skill，包括名称、描述、支持的渲染模式、
    /// 输入输出 schema 等元数据。
    ///
    /// # 返回
    /// 返回 Skill 信息向量，每个 Skill 包含元数据。
    ///
    /// # 错误
    /// - Agent 未启动
    /// - Control Channel 通信失败
    /// - 协议解析错误
    async fn get_skills(&self) -> AgentResult<Vec<SkillInfo>>;

    /// 执行指定的 Skill
    ///
    /// 异步执行指定的 Skill，立即返回唯一的 session_id。
    /// Skill 的执行结果和进度通过事件流异步返回。
    ///
    /// # 参数
    /// - `skill_name`: 要执行的 Skill 名称
    /// - `input`: Skill 的输入参数
    ///
    /// # 返回
    /// 返回会话 ID，用于后续查询状态和接收事件。
    ///
    /// # 错误
    /// - Skill 不存在
    /// - 输入参数无效
    /// - Agent 未启动
    /// - 执行启动失败
    async fn execute_skill(
        &self,
        skill_name: &str,
        input: SkillInput,
    ) -> AgentResult<SessionId>;

    /// 订阅执行事件
    ///
    /// 返回一个事件流，用于接收 Skill 执行过程中的所有事件，包括：
    /// - `data_chunk`: 流式数据
    /// - `progress`: 进度更新
    /// - `error`: 错误信息
    /// - `execution_complete`: 执行完成
    ///
    /// # 返回
    /// 返回事件流，可以异步迭代读取事件。
    fn subscribe_events(&self) -> std::pin::Pin<Box<dyn futures::Stream<Item = AgentEvent> + Send>>;
}

/// Agent 事件类型
///
/// 定义了所有 Agent 可能产生的事件类型，包括数据、进度、错误等。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum AgentEvent {
    /// 流式数据块
    DataChunk {
        event_id: u64,
        session_id: SessionId,
        chunk_index: Option<u64>, // Changed to Option to match usage
        data: serde_json::Value,
        is_final: bool,
    },

    /// 进度更新
    Progress {
        event_id: u64,
        session_id: SessionId,
        current: u64,
        total: u64,
        message: String,
    },

    /// 执行错误
    Error {
        event_id: u64,
        session_id: SessionId,
        code: String, // Changed from error_code to match usage
        message: String,
        suggestion: String,
    },

    /// 执行完成
    ExecutionComplete {
        event_id: u64,
        session_id: SessionId,
        success: bool,
        summary: String,
    },

    /// 执行开始
    ExecutionStart {
        event_id: u64,
        session_id: SessionId,
        skill_name: String,
        render_mode: String,
    },
}
