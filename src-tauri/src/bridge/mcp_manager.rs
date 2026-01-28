// src-tauri/src/bridge/mcp_manager.rs

use crate::bridge::error::{AgentError, AgentResult};
use crate::bridge::protocol::McpServerConfig;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::RwLock;

/// MCP 服务器管理器
///
/// 负责管理 CodeBuddy Code 的 MCP 服务器配置，包括列表、添加、删除和配置。
#[derive(Clone)]
pub struct McpManager {
    /// MCP 服务器配置列表
    servers: Arc<RwLock<HashMap<String, McpServerConfig>>>,
    /// CodeBuddy 二进制路径
    codebuddy_path: String,
}

impl McpManager {
    /// 创建新的 MCP 管理器
    pub fn new(codebuddy_path: impl Into<String>) -> Self {
        McpManager {
            servers: Arc::new(RwLock::new(HashMap::new())),
            codebuddy_path: codebuddy_path.into(),
        }
    }

    /// 列出所有 MCP 服务器
    pub async fn list(&self) -> AgentResult<Vec<McpServerConfig>> {
        let result = Command::new(&self.codebuddy_path)
            .args(["mcp", "list"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    // 解析输出
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    self.parse_list_output(&stdout).await
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(AgentError::McpServerError {
                        message: format!("codebuddy mcp list 命令执行失败: {}", stderr),
                        suggestion: "请检查 CodeBuddy Code 是否已正确安装并配置".to_string(),
                    })
                }
            }
            Err(e) => Err(AgentError::Other(format!("执行 codebuddy mcp list 命令失败: {}", e))),
        }
    }

    /// 添加 MCP 服务器
    pub async fn add(&self, config: McpServerConfig) -> AgentResult<()> {
        let result = Command::new(&self.codebuddy_path)
            .args(["mcp", "add", &config.name])
            .args(&config.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    // 添加到本地配置
                    let mut guard = self.servers.write().await;
                    guard.insert(config.name.clone(), config);
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(AgentError::McpServerError {
                        message: format!("添加 MCP 服务器失败: {}", stderr),
                        suggestion: "请检查服务器配置是否正确".to_string(),
                    })
                }
            }
            Err(e) => Err(AgentError::Other(format!("执行 codebuddy mcp add 命令失败: {}", e))),
        }
    }

    /// 删除 MCP 服务器
    pub async fn remove(&self, name: &str) -> AgentResult<()> {
        let result = Command::new(&self.codebuddy_path)
            .args(["mcp", "remove", name])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    // 从本地配置中删除
                    let mut guard = self.servers.write().await;
                    guard.remove(name);
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(AgentError::McpServerError {
                        message: format!("删除 MCP 服务器失败: {}", stderr),
                        suggestion: "请检查服务器名称是否正确".to_string(),
                    })
                }
            }
            Err(e) => Err(AgentError::Other(format!("执行 codebuddy mcp remove 命令失败: {}", e))),
        }
    }

    /// 配置 MCP 服务器
    pub async fn configure(&self, name: &str, config: McpServerConfig) -> AgentResult<()> {
        let result = Command::new(&self.codebuddy_path)
            .args(["mcp", "configure", name])
            .args(&config.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    // 更新本地配置
                    let mut guard = self.servers.write().await;
                    guard.insert(name.to_string(), config);
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(AgentError::McpServerError {
                        message: format!("配置 MCP 服务器失败: {}", stderr),
                        suggestion: "请检查服务器配置是否正确".to_string(),
                    })
                }
            }
            Err(e) => Err(AgentError::Other(format!("执行 codebuddy mcp configure 命令失败: {}", e))),
        }
    }

    /// 获取服务器配置
    pub async fn get(&self, name: &str) -> Option<McpServerConfig> {
        let guard = self.servers.read().await;
        guard.get(name).cloned()
    }

    /// 获取所有服务器配置
    pub async fn list_all(&self) -> Vec<McpServerConfig> {
        let guard = self.servers.read().await;
        guard.values().cloned().collect()
    }

    /// 同步远程配置到本地
    pub async fn sync(&self) -> AgentResult<usize> {
        let remote_servers = self.list().await?;

        let mut guard = self.servers.write().await;
        guard.clear();

        for server in remote_servers {
            guard.insert(server.name.clone(), server);
        }

        Ok(guard.len())
    }

    /// 解析 list 命令的输出
    async fn parse_list_output(&self, output: &str) -> AgentResult<Vec<McpServerConfig>> {
        // TODO: 实现实际的解析逻辑
        // 目前返回空列表，等待 CodeBuddy Code MCP 命令的实际输出格式
        Ok(Vec::new())
    }
}
