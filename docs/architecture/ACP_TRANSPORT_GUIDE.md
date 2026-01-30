# ACP 通信层详细实施指南

**创建日期**: 2026-01-29
**通信方案**: ACP (Agent Client Protocol) + Stdio
**预计耗时**: 2 天

---

## 📋 实施步骤概览

### 步骤 1: 添加依赖 (15 分钟)
- [ ] 在 `src-tauri/Cargo.toml` 中添加 ndjson 依赖
- [ ] 在 `package.json` 中添加 ndjson-stream 前端依赖

### 步骤 2: 创建 ACP 传输层模块 (1 小时)
- [ ] 创建 `src-tauri/src/bridge/acp_transport.rs` 文件
- [ ] 定义 ACP 消息类型结构体
- [ ] 实现 ndjson::Parser
- [ ] 实现 ndjson::Writer
- [ ] 实现错误处理

### 步骤 3: 更新 CodeBuddyAdapter (1 小时)
- [ ] 移除 WebSocket 服务器代码
- [ ] 添加 `--acp` 和 `--acp-transport "stdio"` 参数
- [ ] 实现 stdin 写入器 (使用 ndjson::Writer)
- [ ] 实现 stdout 读取器 (使用 ndjson::Parser)
- [ ] 更新 send_message 方法
- [ ] 更新 receive 循环

### 步骤 4: 更新 Session Manager (30 分钟)
- [ ] 更新 DataChunk 类型以支持 ACP 格式
- [ ] 添加 ACP 消息类型
- [ ] 更新会话状态处理逻辑

### 步骤 5: 测试 ACP 通信 (30 分钟)
- [ ] 创建简单的测试命令
- [ ] 测试双向通信
- [ ] 验证消息格式

---

## 📝 步骤 1: 添加依赖

### Rust 后端依赖

**文件**: `src-tauri/Cargo.toml`

在 `[dependencies]` 部分添加:
```toml
[dependencies]
# ... 现有依赖 ...
ndjson = "0.8"
tokio-util = { version = "0.7", features = ["io"] }
```

### 前端依赖

**文件**: `package.json`

在 `dependencies` 部分添加:
```json
{
  "dependencies": {
    "ndjson-stream": "^0.6.0"
  }
}
```

---

## 📝 步骤 2: 创建 ACP 传输层模块

**文件**: `src-tauri/src/bridge/acp_transport.rs`

### 完整代码实现

```rust
use ndjson::{Ndjson, Parser, Stream};
use crate::bridge::error::{AgentError, AgentResult};
use crate::bridge::protocol::ControlMessage;
use tokio::process::{ChildStdin, ChildStdout};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

/// ACP (Agent Client Protocol) 传输层
///
/// 使用 ndjson 库实现与 CodeBuddy Code 的 ACP 双向通信
pub struct AcpTransport {
    stdin: ChildStdin,
    stdout: ChildStdout,
    parser: Parser<Stream>,
    writer: Ndjson,
}

impl AcpTransport {
    /// 创建新的 ACP 传输层
    pub fn new(stdin: ChildStdin, stdout: ChildStdout) -> Self {
        let parser = Parser::new(stdout);
        let writer = Ndjson::new(stdin);
        
        Self {
            stdin,
            stdout,
            parser,
            writer,
        }
    }

    /// 发送消息到 stdin
    pub async fn send_message(&mut self, message: &ControlMessage) -> AgentResult<()> {
        let json = serde_json::to_string(message)
            .map_err(|e| AgentError::Other(format!("序列化消息失败: {}", e)))?;
        
        self.writer
            .adv(ndjson::json(json)?)
            .map_err(|e| AgentError::CommunicationError {
                message: format!("发送消息到 stdin 失败: {}", e),
                suggestion: "检查 CodeBuddy Code 是否正在运行且支持 ACP 模式".to_string(),
            })?;
        
        self.writer
            .await_flush()
            .map_err(|e| AgentError::CommunicationError {
                message: format!("刷新 stdin 失败: {}", e),
                suggestion: "可能需要调整缓冲区大小".to_string(),
            })?;
        
        Ok(())
    }

    /// 接收来自 stdout 的消息流
    pub async fn receive_messages(&mut self) -> AgentResult<Vec<ControlMessage>> {
        let mut messages = Vec::new();
        
        // 解析流中的所有消息
        loop {
            match self.parser.next() {
                Some(Ok(msg)) => {
                    // 将 ndjson 消息转换为我们的 ControlMessage
                    if let Ok(control_msg) = self.convert_ndjson_to_control(msg) {
                        messages.push(control_msg);
                    }
                }
                Some(Err(e)) => {
                    // ndjson 错误，记录日志但不停止
                    log::warn!("解析 ndjson 消息失败: {}", e);
                    break;
                }
                None => {
                    // 流结束
                    break;
                }
            }
        }
        
        Ok(messages)
    }

    /// 将 ndjson 消息转换为 ControlMessage
    fn convert_ndjson_to_control(&self, msg: ndjson::Msg) -> AgentResult<ControlMessage> {
        // 这里需要根据 ndjson 消息的格式进行转换
        // 假设 ndjson 消息包含 type 字段和数据
        let json_str = msg.to_string();
        
        // 解析 ndjson 的自定义字段
        let msg_type = match msg.get("type") {
            Some(val) => val.as_str().unwrap_or("unknown"),
            None => return Err(AgentError::Other("消息缺少 type 字段".to_string())),
        };
        
        match msg_type {
            "start" => {
                let session_id = msg.get("session_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "".to_string())?;
                let skill_name = msg.get("skill_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| "".to_string())?;
                let render_mode = msg.get("render_mode")
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| "log".to_string())?;
                
                Ok(ControlMessage::ExecutionStart {
                    session_id: session_id.to_string(),
                    skill_name: skill_name.to_string(),
                    render_mode: match render_mode.as_str() {
                        "table" => crate::bridge::protocol::RenderMode::Table,
                        "code" => crate::bridge::protocol::RenderMode::Code,
                        "json" => crate::bridge::protocol::RenderMode::Json,
                        "log" => crate::bridge::protocol::RenderMode::Log,
                        "chart" => crate::bridge::protocol::RenderMode::Chart,
                        "file_tree" => crate::bridge::protocol::RenderMode::FileTree,
                        "markdown" => crate::bridge::protocol::RenderMode::Markdown,
                        "diff" => crate::bridge::protocol::RenderMode::Diff,
                        _ => crate::bridge::protocol::RenderMode::Log,
                    },
                })
            }
            "data" => {
                let session_id = msg.get("session_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "".to_string())?;
                let chunk_index = msg.get("index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| 0)?;
                let data = msg.get("data")
                    .and_then(|v| serde_json::from_value(v.clone()))
                    .ok_or_else(serde_json::Value::Null)?;
                let is_final = msg.get("is_final")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                
                Ok(ControlMessage::DataChunk {
                    session_id: session_id.to_string(),
                    chunk_index,
                    data,
                    is_final,
                })
            }
            "progress" => {
                let session_id = msg.get("session_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "".to_string())?;
                let current = msg.get("current")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| 0)?;
                let total = msg.get("total")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| 0)?;
                let message = msg.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| "".to_string())?;
                
                Ok(ControlMessage::Progress {
                    session_id: session_id.to_string(),
                    current,
                    total,
                    message: message.to_string(),
                })
            }
            "error" => {
                let session_id = msg.get("session_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "".to_string())?;
                let error_code = msg.get("code")
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| "UNKNOWN".to_string())?;
                let error_message = msg.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| "未知错误".to_string())?;
                let suggestion = msg.get("suggestion")
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| "请联系管理员".to_string())?;
                
                Ok(ControlMessage::Error {
                    session_id: session_id.to_string(),
                    error_code: error_code.to_string(),
                    message: error_message.to_string(),
                    suggestion: suggestion.to_string(),
                })
            }
            "end" => {
                let session_id = msg.get("session_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| "".to_string())?;
                let success = msg.get("success")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)?;
                let summary = msg.get("summary")
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| "".to_string())?;
                
                Ok(ControlMessage::ExecutionComplete {
                    session_id: session_id.to_string(),
                    success,
                    summary: summary.to_string(),
                })
            }
            _ => Err(AgentError::ProtocolError {
                message: format!("未知的 ACP 消息类型: {}", msg_type),
                suggestion: "请检查消息类型是否正确".to_string(),
            }),
        }
    }
}
```

---

## 📝 步骤 3: 更新 CodeBuddyAdapter

### 需要修改的文件

**文件**: `src-tauri/src/bridge/codebuddy_adapter.rs`

### 修改点 1: 移除 WebSocket 服务器代码

删除或注释掉 `establish_control_channel()` 方法中的 WebSocket 服务器相关代码。

### 修改点 2: 更新 start_process 方法

在 `start_process` 方法中，添加 ACP 参数:

```rust
async fn start_process(&mut self) -> AgentResult<Child> {
    log::info!("启动 CodeBuddy Code 进程: {}", self.config.binary_path);
    
    let mut cmd = Command::new(&self.config.binary_path);
    
    // 添加 ACP 模式参数
    cmd.arg("--acp")
       .arg("--acp-transport")
       .arg("stdio");
    
    // 设置环境变量
    for (key, value) in &self.config.env {
        cmd.env(key, value);
    }
    
    // 重定向标准流
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    
    // 启动进程
    let child = cmd.spawn().map_err(|e| {
        AgentError::process_start_failed(format!("无法启动进程: {}", e))
    })?;
    
    log::info!("进程已启动，PID: {}, 使用 ACP 模式", child.id());
    
    Ok(child)
}
```

### 修改点 3: 在 start 方法中创建 ACP 传输层

在 `start` 方法中，创建 AcpTransport 实例并存储：

```rust
async fn start(&mut self) -> AgentResult<()> {
    log::info!("启动 CodeBuddy Code 适配器: {}", self.config.binary_path);
    
    // 1. 启动进程
    let child = self.start_process().await?;
    self.process = Some(child);
    
    // 2. 获取 stdout 和 stderr
    let process_ref = self.process.as_ref().unwrap();
    let stdout = process_ref.stdout.as_ref().ok_or_else(|| {
        AgentError::process_start_failed("无法获取 stdout".to_string())
    })?;
    let stderr = process_ref.stderr.as_ref().ok_or_else(|| {
        AgentError::process_start_failed("无法获取 stderr".to_string())
    })?;
    
    // 3. 创建 ACP 传输层
    let stdin = process_ref.stdin.as_ref().ok_or_else(|| {
        AgentError::process_start_failed("无法获取 stdin".to_string())
    })?;
    let acp_transport = AcpTransport::new(stdin, stdout);
    
    // 存储传输层实例（如果需要）
    // self.acp_transport = Some(acp_transport);
    
    // 4. 监听 stdout/stderr
    self.monitor_stdout(stdout.clone()).await;
    self.monitor_stderr(stderr).await;
    
    // 5. 完成
    *self.is_running.lock().await = true;
    
    log::info!("CodeBuddy Code 适配器启动成功，使用 ACP 模式");
    
    Ok(())
}
```

### 