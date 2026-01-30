// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bridge;
mod commands;
mod event_handlers;

use bridge::{AgentAdapter, AgentConfig, CodeBuddyPythonAdapter};
use event_handlers::AgentEventHandler;
use tauri::Manager;

#[tokio::main]
async fn main() {
    // 初始化日志
    env_logger::init();

    log::info!("启动 Nexus Shell 应用...");

    // 创建 Agent 配置
    let agent_config = AgentConfig {
        agent_type: "codebuddy-python".to_string(),
        binary_path: "codebuddy".to_string(),
        control_port: 0, // 0 表示自动分配
        timeout_ms: 30000,
        env: std::collections::HashMap::new(),
        mcp_servers: Vec::new(),
    };

    // 创建 CodeBuddy 适配器
    let codebuddy_adapter = std::sync::Arc::new(tokio::sync::Mutex::new(CodeBuddyPythonAdapter::new(agent_config)));
    let adapter_clone = codebuddy_adapter.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            // 获取应用句柄
            let app_handle = app.handle();

            // 在应用启动时初始化 Agent
            {
                let adapter_for_start = codebuddy_adapter.clone();

                // 启动 Agent
                #[cfg(debug_assertions)]
                {
                    // 在调试模式下，我们暂时不启动真实的 Agent
                    log::warn!("调试模式: 跳过 Agent 启动");
                }

                #[cfg(not(debug_assertions))]
                {
                    // 使用 spawn 代替 block_on，避免运行时嵌套错误
                    tauri::async_runtime::spawn(async move {
                        let mut adapter = adapter_for_start.lock().await;
                        if let Err(e) = adapter.start().await {
                            log::error!("启动 CodeBuddy Code 失败: {}", e);
                        }
                    });
                }
            }

            // 创建并启动事件处理器
            {
                let adapter_for_handler = adapter_clone.clone();
                let app_handle_for_handler = app_handle.clone();
                
                tauri::async_runtime::spawn(async move {
                    let adapter_inner = adapter_for_handler.lock().await.clone();
                    let event_handler = AgentEventHandler::new(app_handle_for_handler, adapter_inner);
                    if let Err(e) = event_handler.start().await {
                        log::error!("启动事件处理器失败: {}", e);
                    }
                });
            }

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
            commands::is_agent_running,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
