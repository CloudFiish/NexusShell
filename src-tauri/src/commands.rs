// src-tauri/src/commands.rs

use crate::bridge::{
    agent_adapter::AgentAdapter,
    codebuddy_python_adapter::CodeBuddyPythonAdapter,
    error::{AgentError, AgentResult},
    mcp_manager::McpManager,
    protocol::{AgentConfig, SkillInfo, SkillInput, SessionId, RenderMode},
};
use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;

// 定义 Agent 类型别名，用于 State
type PythonAdapterState = Arc<Mutex<crate::bridge::codebuddy_python_adapter::CodeBuddyPythonAdapter>>;

/// 启动 Agent
#[tauri::command]
pub async fn start_agent(state: State<'_, PythonAdapterState>) -> Result<String, String> {
    let mut adapter = state.lock().await;
    
    adapter
        .start()
        .await
        .map_err(|e| format!("启动 Agent 失败: {}", e))?;

    Ok("Agent 启动成功".to_string())
}

/// 停止 Agent
#[tauri::command]
pub async fn stop_agent(state: State<'_, PythonAdapterState>) -> Result<String, String> {
    let mut adapter = state.lock().await;
    
    adapter
        .stop()
        .await
        .map_err(|e| format!("停止 Agent 失败: {}", e))?;

    Ok("Agent 已停止".to_string())
}

/// 获取 Skill 列表
#[tauri::command]
pub async fn get_skills(state: State<'_, PythonAdapterState>) -> Result<Vec<SkillInfo>, String> {
    let adapter = state.lock().await;
    
    adapter
        .get_skills()
        .await
        .map_err(|e| format!("获取 Skill 列表失败: {}", e))
}

/// 执行 Skill
#[tauri::command]
pub async fn execute_skill(
    state: State<'_, PythonAdapterState>,
    skill_name: String,
    input: String,
) -> Result<SessionId, String> {
    let adapter = state.lock().await;
    
    let skill_input = SkillInput::Text(input);
    
    adapter
        .execute_skill(&skill_name, skill_input)
        .await
        .map_err(|e| format!("执行 Skill 失败: {}", e))
}

/// 取消会话
#[tauri::command]
pub async fn cancel_session(
    state: State<'_, PythonAdapterState>,
    session_id: String,
) -> Result<String, String> {
    let adapter = state.lock().await;
    
    adapter
        .cancel_session(&session_id)
        .await
        .map_err(|e| format!("取消会话失败: {}", e))?;

    Ok(format!("会话 {} 已取消", session_id))
}

/// 获取所有会话
#[tauri::command]
pub async fn get_sessions(
    state: State<'_, PythonAdapterState>
) -> Result<Vec<crate::bridge::session_manager::Session>, String> {
    let adapter = state.lock().await;
    
    let session_manager = adapter.session_manager();
    let sessions = session_manager.list().await;
    
    Ok(sessions)
}

/// 获取指定会话
#[tauri::command]
pub async fn get_session(
    state: State<'_, PythonAdapterState>,
    session_id: String,
) -> Result<Option<crate::bridge::session_manager::Session>, String> {
    let adapter = state.lock().await;
    
    let session_manager = adapter.session_manager();
    let session = session_manager.get(&session_id).await;
    
    Ok(session)
}

/// 检查 Agent 是否运行中
#[tauri::command]
pub async fn is_agent_running(state: State<'_, PythonAdapterState>) -> Result<bool, String> {
    let adapter = state.lock().await;
    Ok(adapter.is_running().await)
}
