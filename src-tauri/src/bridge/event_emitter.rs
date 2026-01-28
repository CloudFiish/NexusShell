// src-tauri/src/bridge/event_emitter.rs

use crate::bridge::agent_adapter::AgentEvent;
use crate::bridge::error::AgentResult;
use std::sync::Arc;
use tokio::sync::broadcast;
use tauri::{AppHandle, Emitter};

/// 事件发射器
///
/// 基于 Tauri Event 系统和 tokio::broadcast 实现事件发布订阅机制。
/// 负责将 Agent 事件发送给前端，并提供内部订阅功能。
pub struct EventEmitter {
    /// 广播发送器
    tx: broadcast::Sender<AgentEvent>,
    /// 广播接收器 (用于内部订阅)
    _rx: broadcast::Receiver<AgentEvent>,
}

impl EventEmitter {
    /// 创建新的事件发射器
    pub fn new() -> Self {
        let (tx, rx) = broadcast::channel(100);
        EventEmitter {
            tx,
            _rx: rx,
        }
    }

    /// 发送事件
    ///
    /// 将事件发送给所有订阅者，包括前端窗口。
    pub async fn emit(&self, event: AgentEvent) -> AgentResult<()> {
        // 发送到内部订阅者
        if let Err(e) = self.tx.send(event.clone()) {
            log::warn!("事件发送失败: {}", e);
        }
        Ok(())
    }

    /// 发送事件到 Tauri 前端
    ///
    /// 使用 Tauri 的 Emitter trait 将事件发送给前端。
    pub async fn emit_to_frontend(&self, app: &AppHandle, event: AgentEvent) -> AgentResult<()> {
        // 将事件转换为 JSON
        let event_json = serde_json::to_value(&event)
            .map_err(|e| crate::bridge::error::AgentError::Other(format!("序列化事件失败: {}", e)))?;

        // 发送到前端
        app.emit("agent-event", event_json)
            .map_err(|e| crate::bridge::error::AgentError::Other(format!("发送事件到前端失败: {}", e)))?;

        Ok(())
    }

    /// 同时发送到内部和前端
    pub async fn emit_all(&self, app: &AppHandle, event: AgentEvent) -> AgentResult<()> {
        self.emit(event.clone()).await?;
        self.emit_to_frontend(app, event).await?;
        Ok(())
    }

    /// 订阅事件
    ///
    /// 返回一个事件流，用于接收 Agent 事件。
    pub fn subscribe(&self) -> broadcast::Receiver<AgentEvent> {
        self.tx.subscribe()
    }

    /// 获取订阅者数量
    pub fn subscriber_count(&self) -> usize {
        self.tx.receiver_count()
    }
}

impl Default for EventEmitter {
    fn default() -> Self {
        Self::new()
    }
}

/// Tauri 事件管理器
///
/// 封装 Tauri 事件系统的辅助工具，提供便捷的事件发送方法。
pub struct TauriEventManager {
    app: Arc<AppHandle>,
}

impl TauriEventManager {
    /// 创建新的 Tauri 事件管理器
    pub fn new(app: AppHandle) -> Self {
        TauriEventManager {
            app: Arc::new(app),
        }
    }

    /// 发送 Agent 事件
    pub fn emit_agent_event(&self, event: AgentEvent) -> AgentResult<()> {
        let event_json = serde_json::to_value(&event)
            .map_err(|e| crate::bridge::error::AgentError::Other(format!("序列化事件失败: {}", e)))?;

        self.app
            .emit("agent-event", event_json)
            .map_err(|e| crate::bridge::error::AgentError::Other(format!("发送事件到前端失败: {}", e)))?;

        Ok(())
    }

    /// 发送会话更新事件
    pub fn emit_session_updated(&self, session_id: &str) -> AgentResult<()> {
        self.app
            .emit("session-updated", session_id)
            .map_err(|e| crate::bridge::error::AgentError::Other(format!("发送会话更新事件失败: {}", e)))?;

        Ok(())
    }

    /// 发送 Skill 列表更新事件
    pub fn emit_skills_updated(&self) -> AgentResult<()> {
        self.app
            .emit("skills-updated", ())
            .map_err(|e| crate::bridge::error::AgentError::Other(format!("发送 Skill 更新事件失败: {}", e)))?;

        Ok(())
    }

    /// 发送错误事件
    pub fn emit_error(&self, error: crate::bridge::error::AgentError) -> AgentResult<()> {
        let error_json = serde_json::json!({
            "code": error.error_code(),
            "message": error.to_string(),
            "is_retryable": error.is_retryable(),
        });

        self.app
            .emit("agent-error", error_json)
            .map_err(|e| crate::bridge::error::AgentError::Other(format!("发送错误事件失败: {}", e)))?;

        Ok(())
    }
}

impl Clone for TauriEventManager {
    fn clone(&self) -> Self {
        TauriEventManager {
            app: self.app.clone(),
        }
    }
}
