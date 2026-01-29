// src-tauri/src/bridge/session_manager.rs

use crate::bridge::error::{AgentError, AgentResult};
use crate::bridge::protocol::{SessionId, SkillInfo, RenderMode};
use crate::bridge::agent_adapter::AgentEvent;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use chrono::{DateTime, Utc};
use std::time::Duration;

/// 会话状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    /// 等待开始
    Pending,

    /// 运行中
    Running,

    /// 已暂停
    Paused,

    /// 已取消
    Cancelled,

    /// 已完成 (成功)
    Completed,

    /// 失败
    Error,
}

/// 会话信息
///
/// 记录每个 Skill 执行会话的完整状态和信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// 会话 ID
    pub id: SessionId,

    /// 执行的 Skill 名称
    pub skill_name: String,

    /// Skill 信息
    pub skill_info: Option<SkillInfo>,

    /// 会话状态
    pub status: SessionStatus,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 开始时间
    pub started_at: Option<DateTime<Utc>>,

    /// 完成时间
    pub completed_at: Option<DateTime<Utc>>,

    /// 输入参数
    pub input: String,

    /// 渲染模式
    pub render_mode: Option<RenderMode>,

    /// 流式数据块列表
    pub data_chunks: Vec<DataChunk>,

    /// 进度信息
    pub progress: Option<ProgressInfo>,

    /// 错误信息
    pub error: Option<ErrorInfo>,

    /// 执行摘要
    pub summary: Option<String>,

    /// 是否成功
    pub success: bool,
}

/// 数据块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataChunk {
    /// 块索引
    pub index: u64,

    /// 数据内容
    pub data: Value,

    /// 是否为最后一个块
    pub is_final: bool,

    /// 接收时间
    pub received_at: DateTime<Utc>,

    /// 数据大小 (字节数)
    pub size: usize,
}

/// 进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressInfo {
    /// 当前值
    pub current: u64,

    /// 总值
    pub total: u64,

    /// 消息
    pub message: String,

    /// 进度百分比 (0-100)
    pub percentage: f64,

    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

impl ProgressInfo {
    /// 计算进度百分比
    pub fn percentage(current: u64, total: u64) -> f64 {
        if total == 0 {
            0.0
        } else {
            (current as f64 / total as f64 * 100.0).min(100.0)
        }
    }
}

/// 错误信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// 错误代码
    pub code: String,

    /// 错误消息
    pub message: String,

    /// 建议操作
    pub suggestion: String,

    /// 发生时间
    pub occurred_at: DateTime<Utc>,
}

/// 会话管理器
///
/// 负责管理多个并发会话的状态，提供会话的创建、更新、查询和删除功能。
/// 使用 RwLock 实现线程安全的读写操作。
#[derive(Clone)]
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
    event_tx: Arc<mpsc::UnboundedSender<SessionEvent>>,
    _event_rx: Arc<tokio::task::JoinHandle<()>>,
}

/// 会话事件 (内部使用)
#[derive(Debug)]
enum SessionEvent {
    /// 创建会话
    Create(Session),
    /// 更新会话状态
    UpdateStatus(SessionId, SessionStatus),
    /// 添加数据块
    AddDataChunk(SessionId, DataChunk),
    /// 更新进度
    UpdateProgress(SessionId, ProgressInfo),
    /// 设置错误
    SetError(SessionId, ErrorInfo),
    /// 设置摘要
    SetSummary(SessionId, String, bool),
    /// 更新渲染模式
    UpdateRenderMode(SessionId, RenderMode),
    /// 删除会话
    Remove(SessionId),
}

impl SessionManager {
    /// 创建新的会话管理器
    pub fn new() -> Self {
        let sessions = Arc::new(RwLock::new(HashMap::new()));
        let (event_tx, mut event_rx) = mpsc::unbounded_channel::<SessionEvent>();

        let sessions_clone = sessions.clone();
        let handle = tokio::spawn(async move {
            while let Some(event) = event_rx.recv().await {
                match event {
                    SessionEvent::Create(session) => {
                        let mut guard = sessions_clone.write().await;
                        guard.insert(session.id.clone(), session);
                    }
                    SessionEvent::UpdateStatus(session_id, status) => {
                        let mut guard = sessions_clone.write().await;
                        if let Some(session) = guard.get_mut(&session_id) {
                            session.status = status.clone();
                            match status {
                                SessionStatus::Running => {
                                    if session.started_at.is_none() {
                                        session.started_at = Some(Utc::now());
                                    }
                                }
                                SessionStatus::Completed | SessionStatus::Error | SessionStatus::Cancelled => {
                                    if session.completed_at.is_none() {
                                        session.completed_at = Some(Utc::now());
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    SessionEvent::AddDataChunk(session_id, chunk) => {
                        let mut guard = sessions_clone.write().await;
                        if let Some(session) = guard.get_mut(&session_id) {
                            session.data_chunks.push(chunk);
                        }
                    }
                    SessionEvent::UpdateProgress(session_id, progress) => {
                        let mut guard = sessions_clone.write().await;
                        if let Some(session) = guard.get_mut(&session_id) {
                            session.progress = Some(progress);
                        }
                    }
                    SessionEvent::SetError(session_id, error) => {
                        let mut guard = sessions_clone.write().await;
                        if let Some(session) = guard.get_mut(&session_id) {
                            session.error = Some(error);
                            session.status = SessionStatus::Error;
                            if session.completed_at.is_none() {
                                session.completed_at = Some(Utc::now());
                            }
                        }
                    }
                    SessionEvent::SetSummary(session_id, summary, success) => {
                        let mut guard = sessions_clone.write().await;
                        if let Some(session) = guard.get_mut(&session_id) {
                            session.summary = Some(summary);
                            session.success = success;
                            session.status = if success {
                                SessionStatus::Completed
                            } else {
                                SessionStatus::Error
                            };
                            if session.completed_at.is_none() {
                                session.completed_at = Some(Utc::now());
                            }
                        }
                    }
                    SessionEvent::UpdateRenderMode(session_id, render_mode) => {
                        let mut guard = sessions_clone.write().await;
                        if let Some(session) = guard.get_mut(&session_id) {
                            session.render_mode = Some(render_mode);
                        }
                    }
                    SessionEvent::Remove(session_id) => {
                        let mut guard = sessions_clone.write().await;
                        guard.remove(&session_id);
                    }
                }
            }
        });

        SessionManager {
            sessions,
            event_tx: Arc::new(event_tx),
            _event_rx: Arc::new(handle),
        }
    }

    /// 创建新会话
    pub async fn create(
        &self,
        session_id: SessionId,
        skill_name: String,
        input: String,
        skill_info: Option<SkillInfo>,
    ) -> AgentResult<()> {
        let session = Session {
            id: session_id.clone(),
            skill_name,
            skill_info,
            status: SessionStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            input,
            render_mode: None,
            data_chunks: Vec::new(),
            progress: None,
            error: None,
            summary: None,
            success: false,
        };

        self.event_tx
            .send(SessionEvent::Create(session))
            .map_err(|_| AgentError::Other("无法发送创建会话事件".to_string()))?;

        Ok(())
    }

    /// 更新会话状态
    pub async fn update_status(&self, session_id: SessionId, status: SessionStatus) -> AgentResult<()> {
        self.event_tx
            .send(SessionEvent::UpdateStatus(session_id, status))
            .map_err(|_| AgentError::Other("无法发送更新状态事件".to_string()))?;
        Ok(())
    }

    /// 添加数据块
    pub async fn add_data_chunk(&self, session_id: SessionId, data: Value, is_final: bool) -> AgentResult<u64> {
        // 先获取当前的块数量
        let index = {
            let guard = self.sessions.read().await;
            guard
                .get(&session_id)
                .map(|s| s.data_chunks.len() as u64)
                .unwrap_or(0)
        };

        let chunk = DataChunk {
            index,
            data,
            is_final,
            received_at: Utc::now(),
            size: 0, // TODO: 计算实际大小
        };

        self.event_tx
            .send(SessionEvent::AddDataChunk(session_id, chunk))
            .map_err(|_| AgentError::Other("无法发送数据块事件".to_string()))?;

        Ok(index)
    }

    /// 更新进度
    pub async fn update_progress(
        &self,
        session_id: SessionId,
        current: u64,
        total: u64,
        message: String,
    ) -> AgentResult<()> {
        let progress = ProgressInfo {
            current,
            total,
            message,
            percentage: ProgressInfo::percentage(current, total),
            updated_at: Utc::now(),
        };

        self.event_tx
            .send(SessionEvent::UpdateProgress(session_id, progress))
            .map_err(|_| AgentError::Other("无法发送进度事件".to_string()))?;
        Ok(())
    }

    /// 设置错误
    pub async fn set_error(&self, session_id: SessionId, code: String, message: String, suggestion: String) -> AgentResult<()> {
        let error = ErrorInfo {
            code,
            message,
            suggestion,
            occurred_at: Utc::now(),
        };

        self.event_tx
            .send(SessionEvent::SetError(session_id, error))
            .map_err(|_| AgentError::Other("无法发送错误事件".to_string()))?;
        Ok(())
    }

    /// 设置摘要
    pub async fn set_summary(&self, session_id: SessionId, summary: String, success: bool) -> AgentResult<()> {
        self.event_tx
            .send(SessionEvent::SetSummary(session_id, summary, success))
            .map_err(|_| AgentError::Other("无法发送摘要事件".to_string()))?;
        Ok(())
    }

    /// 更新渲染模式
    pub async fn update_render_mode(&self, session_id: SessionId, render_mode: RenderMode) -> AgentResult<()> {
        self.event_tx
            .send(SessionEvent::UpdateRenderMode(session_id, render_mode))
            .map_err(|_| AgentError::Other("无法发送渲染模式事件".to_string()))?;
        Ok(())
    }

    /// 删除会话
    pub async fn remove(&self, session_id: SessionId) -> AgentResult<()> {
        self.event_tx
            .send(SessionEvent::Remove(session_id))
            .map_err(|_| AgentError::Other("无法发送删除事件".to_string()))?;
        Ok(())
    }

    /// 获取会话
    pub async fn get(&self, session_id: &SessionId) -> Option<Session> {
        let guard = self.sessions.read().await;
        guard.get(session_id).cloned()
    }

    /// 获取所有会话
    pub async fn list(&self) -> Vec<Session> {
        let guard = self.sessions.read().await;
        guard.values().cloned().collect()
    }

    /// 获取活跃会话 (运行中或等待中)
    pub async fn list_active(&self) -> Vec<Session> {
        let guard = self.sessions.read().await;
        guard
            .values()
            .filter(|s| matches!(s.status, SessionStatus::Running | SessionStatus::Pending))
            .cloned()
            .collect()
    }

    /// 获取已完成会话
    pub async fn list_completed(&self) -> Vec<Session> {
        let guard = self.sessions.read().await;
        guard
            .values()
            .filter(|s| matches!(s.status, SessionStatus::Completed | SessionStatus::Error | SessionStatus::Cancelled))
            .cloned()
            .collect()
    }

    /// 清理超过指定时间的旧会话
    pub async fn cleanup_old_sessions(&self, older_than: Duration) -> AgentResult<usize> {
        let now = Utc::now();
        let mut to_remove = Vec::new();

        {
            let guard = self.sessions.read().await;
            for (session_id, session) in guard.iter() {
                if let Some(completed_at) = session.completed_at {
                    if now.signed_duration_since(completed_at).to_std().unwrap_or(Duration::ZERO) > older_than {
                        to_remove.push(session_id.clone());
                    }
                }
            }
        }

        for session_id in to_remove {
            self.remove(session_id).await?;
        }

        Ok(to_remove.len())
    }

    /// 获取会话数量
    pub async fn count(&self) -> usize {
        let guard = self.sessions.read().await;
        guard.len()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
