// src-tauri/src/event_handlers.rs

//! 事件处理模块
//!
//! 负责监听 Agent 事件并转发到前端

use crate::bridge::agent_adapter::AgentEvent;
use crate::bridge::CodeBuddyPythonAdapter;
use crate::bridge::AgentAdapter; // Import trait to use subscribe_events
use tauri::{AppHandle, Emitter};

/// Agent 事件处理器
///
/// 监听 Agent 事件并通过 Tauri Event API 发送到前端
pub struct AgentEventHandler {
    app_handle: AppHandle,
    adapter: CodeBuddyPythonAdapter,
}

impl AgentEventHandler {
    /// 创建新的事件处理器
    pub fn new(app_handle: AppHandle, adapter: CodeBuddyPythonAdapter) -> Self {
        Self {
            app_handle,
            adapter,
        }
    }

    /// 启动事件监听
    ///
    /// 这个方法会在后台任务中持续监听 Agent 事件并转发到前端
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("启动 Agent 事件处理器");

        // 获取 Adapter 的事件流
        // CodeBuddyAdapter is thread-safe and cloneable, so we can just use it directly
        let event_stream = self.adapter.subscribe_events();

        // 克隆必要的数据
        let app_handle = self.app_handle.clone();

        // 在后台任务中处理事件
        tokio::spawn(async move {
            log::info!("Agent 事件监听任务已启动");

            use futures::StreamExt;

            // event_stream is already a Pin<Box<dyn Stream>>, so we can iterate it directly
            let mut stream = event_stream;

            while let Some(event) = stream.next().await {
                log::info!("收到 Agent 事件: {:?}", event);

                // 将事件转换为 JSON 并发送到前端
                match Self::convert_event_to_json(&event) {
                    Ok(event_json) => {
                        log::info!("事件 JSON: {}", event_json);

                        // 发送到前端
                        match app_handle.emit("agent-event", &event_json) {
                            Ok(_) => {
                                log::info!("✅ 事件已发送到前端: agent-event");
                            }
                            Err(e) => {
                                log::error!("❌ 发送事件到前端失败: {}", e);
                            }
                        }

                        // 根据事件类型,发送特定事件
                        if let Err(e) = Self::emit_specialized_events(&app_handle, &event).await {
                            log::error!("发送特定事件失败: {}", e);
                        }
                    }
                    Err(e) => {
                        log::error!("转换事件为 JSON 失败: {}", e);
                    }
                }
            }

            log::warn!("Agent 事件监听任务已结束");
        });

        Ok(())
    }

    /// 将 AgentEvent 转换为 JSON 兼容的对象
    fn convert_event_to_json(event: &AgentEvent) -> Result<serde_json::Value, String> {
        match event {
            AgentEvent::ExecutionStart {
                event_id,
                session_id,
                skill_name,
                render_mode,
            } => Ok(serde_json::json!({
                "type": "execution_start",
                "event_id": event_id,
                "session_id": session_id,
                "skill_name": skill_name,
                "render_mode": render_mode,
            })),

            AgentEvent::DataChunk {
                event_id,
                session_id,
                chunk_index,
                data,
                is_final,
            } => Ok(serde_json::json!({
                "type": "data_chunk",
                "event_id": event_id,
                "session_id": session_id,
                "chunk_index": chunk_index,
                "data": data,
                "is_final": is_final,
            })),

            AgentEvent::Progress {
                event_id,
                session_id,
                current,
                total,
                message,
            } => Ok(serde_json::json!({
                "type": "progress",
                "event_id": event_id,
                "session_id": session_id,
                "current": current,
                "total": total,
                "message": message,
            })),

            AgentEvent::ExecutionComplete {
                event_id,
                session_id,
                success,
                summary,
            } => Ok(serde_json::json!({
                "type": "execution_complete",
                "event_id": event_id,
                "session_id": session_id,
                "success": success,
                "summary": summary,
            })),

            AgentEvent::Error {
                event_id,
                session_id,
                code,
                message,
                suggestion,
            } => Ok(serde_json::json!({
                "type": "error",
                "event_id": event_id,
                "session_id": session_id,
                "code": code,
                "message": message,
                "suggestion": suggestion,
            })),
        }
    }

    /// 根据事件类型发送特定事件
    async fn emit_specialized_events(
        app_handle: &AppHandle,
        event: &AgentEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            AgentEvent::ExecutionStart { session_id, .. } => {
                // 发送 session-updated 事件
                app_handle.emit(
                    "session-updated",
                    serde_json::json!({
                        "session_id": session_id,
                        "status": "running",
                    }),
                )?;
            }

            AgentEvent::Progress { session_id, current, total, message, .. } => {
                // 发送 progress 事件
                app_handle.emit(
                    "progress",
                    serde_json::json!({
                        "session_id": session_id,
                        "current": current,
                        "total": total,
                        "message": message,
                    }),
                )?;
            }

            AgentEvent::ExecutionComplete { session_id, success, .. } => {
                // 发送 session-updated 事件
                app_handle.emit(
                    "session-updated",
                    serde_json::json!({
                        "session_id": session_id,
                        "status": if *success { "completed" } else { "error" },
                    }),
                )?;
            }

            AgentEvent::Error { session_id, code, message, suggestion, .. } => {
                // 发送 error 事件
                app_handle.emit(
                    "error",
                    serde_json::json!({
                        "session_id": session_id,
                        "code": code,
                        "message": message,
                        "suggestion": suggestion,
                    }),
                )?;
            }

            _ => {
                // 其他事件不需要特殊处理
            }
        }

        Ok(())
    }
}
