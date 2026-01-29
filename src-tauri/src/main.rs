// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bridge;
mod commands;

use bridge::{AgentAdapter, AgentConfig, CodeBuddyAdapter};
use tauri::Manager;

#[tokio::main]
async fn main() {
    // 初始化日志
    env_logger::init();

    log::info!("启动 Nexus Shell 应用...");

    // 创建 Agent 配置
    let agent_config = AgentConfig {
        agent_type: "codebuddy".to_string(),
        binary_path: "codebuddy".to_string(),
        control_port: 0, // 0 表示自动分配
        timeout_ms: 30000,
        env: std::collections::HashMap::new(),
        mcp_servers: Vec::new(),
    };

    // 创建 CodeBuddy 适配器
    let codebuddy_adapter = std::sync::Arc::new(std::sync::Mutex::new(CodeBuddyAdapter::new(agent_config)));
    let adapter_clone = codebuddy_adapter.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            // 在应用启动时初始化 Agent
            {
                let mut adapter = codebuddy_adapter.lock().unwrap();

                // 启动 Agent
                #[cfg(debug_assertions)]
                {
                    // 在调试模式下，我们暂时不启动真实的 Agent
                    log::warn!("调试模式: 跳过 Agent 启动");
                }

                #[cfg(not(debug_assertions))]
                {
                    if let Err(e) = tauri::async_runtime::block_on(adapter.start()) {
                        log::error!("启动 CodeBuddy Code 失败: {}", e);
                    }
                }
            } // Drop lock here

            // 将适配器存储到应用状态中
            app.manage(adapter_clone);

            log::info!("Nexus Shell 应用启动成功");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_agent,
            commands::stop_agent,
            commands::get_skills,
            commands::execute_skill,
            commands::cancel_session,
            commands::get_sessions,
            commands::get_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
