// src-tauri/src/bridge/mod.rs

pub mod agent_adapter;
pub mod codebuddy_adapter;
pub mod error;
pub mod event_emitter;
pub mod mcp_manager;
pub mod protocol;
pub mod session_manager;

pub use agent_adapter::AgentAdapter;
pub use codebuddy_adapter::CodeBuddyAdapter;
pub use error::{AgentError, AgentResult};
pub use event_emitter::EventEmitter;
pub use mcp_manager::McpManager;
pub use protocol::*;
pub use session_manager::SessionManager;
