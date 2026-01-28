// src-tauri/src/commands.rs

use crate::bridge::{
    AgentAdapter, AgentError, AgentResult, SessionId, SkillInfo, SkillInput,
};
use tauri::State;

// 定义 Agent 类型别名，用于 State
type AgentState = std::sync::Arc<std::sync::Mutex<crate::bridge::CodeBuddyAdapter>>;

/// 启动 Agent
#[tauri::command]
pub async fn start_agent(state: State<'_, AgentState>) -> Result<String, String> {
    let mut adapter = state.lock().map_err(|e| format!("无法获取 Agent 锁: {}", e))?;
    
    adapter
        .start()
        .await
        .map_err(|e| format!("启动 Agent 失败: {}", e))?;

    Ok("Agent 启动成功".to_string())
}

/// 停止 Agent
#[tauri::command]
pub async fn stop_agent(state: State<'_, AgentState>) -> Result<String, String> {
    let mut adapter = state.lock().map_err(|e| format!("无法获取 Agent 锁: {}", e))?;
    
    adapter
        .stop()
        .await
        .map_err(|e| format!("停止 Agent 失败: {}", e))?;

    Ok("Agent 已停止".to_string())
}

/// 获取 Skill 列表
#[tauri::command]
pub async fn get_skills(state: State<'_, AgentState>) -> Result<Vec<SkillInfo>, String> {
    let adapter = state.lock().map_err(|e| format!("无法获取 Agent 锁: {}", e))?;
    
    adapter
        .get_skills()
        .await
        .map_err(|e| format!("获取 Skill 列表失败: {}", e))
}

/// 执行 Skill
#[tauri::command]
pub async fn execute_skill(
    state: State<'_, AgentState>,
    skill_name: String,
    input: String,
) -> Result<SessionId, String> {
    let adapter = state.lock().map_err(|e| format!("无法获取 Agent 锁: {}", e))?;
    
    let skill_input = SkillInput::Text(input);
    
    adapter
        .execute_skill(&skill_name, skill_input)
        .await
        .map_err(|e| format!("执行 Skill 失败: {}", e))
}

/// 取消会话
#[tauri::command]
pub async fn cancel_session(
    state: State<'_, AgentState>,
    session_id: String,
) -> Result<String, String> {
    // TODO: 实现取消会话逻辑
    Ok(format!("会话 {} 已取消", session_id))
}

/// 获取所有会话
#[tauri::command]
pub async fn get_sessions(state: State<'_, AgentState>) -> Result<Vec<crate::bridge::session_manager::Session>, String> {
    let adapter = state.lock().map_err(|e| format!("无法获取 Agent 锁: {}", e))?;
    
    let session_manager = adapter.session_manager();
    let sessions = session_manager.list().await;
    
    Ok(sessions)
}

/// 获取指定会话
#[tauri::command]
pub async fn get_session(
    state: State<'_, AgentState>,
    session_id: String,
) -> Result<Option<crate::bridge::session_manager::Session>, String> {
    let adapter = state.lock().map_err(|e| format!("无法获取 Agent 锁: {}", e))?;
    
    let session_manager = adapter.session_manager();
    let session = session_manager.get(&session_id).await;
    
    Ok(session)
}
