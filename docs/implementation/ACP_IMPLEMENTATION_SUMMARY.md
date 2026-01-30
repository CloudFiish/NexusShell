# ACP 通信功能实现总结

## 实现时间
2026-01-30

## 实施范围
P0.2: 实现 ACP 通信层 (2 天) - 完成

## 完成的任务

### 1. 添加依赖 (src-tauri/Cargo.toml:302-303)
- ✅ 添加 `ndjson = "0.8"` 依赖
- ✅ 添加 `tokio-util = "0.7"` 依赖

### 2. 创建 ACP 传输模块 (src-tauri/src/bridge/acp_transport.rs)
新文件,包含以下功能:

#### 核心功能
- ✅ `AcpTransport` 结构体: 管理 ACP 传输连接
- ✅ 同步模式支持: `from_child_sync()` 方法
- ✅ 异步模式支持: `from_child_async()` 方法
- ✅ 连接状态管理: `is_connected()` 方法
- ✅ 优雅关闭: `close()` 和 `close_async()` 方法

#### 消息发送/接收
- ✅ `send_sync()` / `send_async()`: 发送 JSON 消息到 stdin
- ✅ `receive_sync()` / `receive_async()`: 从 stdout 读取 JSON 消息
- ✅ `stream_reader()`: 创建流式读取器 (FramedRead)
- ✅ `stream_writer()`: 创建流式写入器 (FramedWrite)

#### 错误处理
- ✅ 连接未检查处理
- ✅ 序列化/反序列化错误处理
- ✅ IO 错误处理
- ✅ 资源清理 (Drop trait)

#### 日志记录
- ✅ 所有关键操作都有 debug/info 级别日志

### 3. 更新模块导出 (src-tauri/src/bridge/mod.rs:3,19)
- ✅ 添加 `pub mod acp_transport;`
- ✅ 添加 `pub use acp_transport::AcpTransport;`

### 4. 重构 CodeBuddyAdapter (src-tauri/src/bridge/codebuddy_adapter.rs)
完全重写,移除 WebSocket,使用 ACP:

#### 主要变更
- ✅ 移除 WebSocket 服务器代码 (`establish_control_channel`)
- ✅ 移除 WebSocket 连接 (`WebSocketStream`, `control_channel`)
- ✅ 添加 `AcpTransport` 字段
- ✅ 添加 ACP 消息类型 (`AcpMessage` enum)

#### 启动流程更新
- ✅ `start_process()`: 添加 `--acp --acp-transport "stdio"` 参数
- ✅ 使用 `tokio::process::Command` 而不是 `std::process::Command`
- ✅ 创建 ACP 传输层: `AcpTransport::from_child_async(child)`

#### 通信实现
- ✅ `send_acp_message()`: 使用 ACP 传输层发送消息
- ✅ `wait_for_acp_response()`: 使用 oneshot 通道等待响应
- ✅ `start_message_loop()`: 启动异步消息接收循环

#### ACP 消息处理
- ✅ `handle_acp_message()`: 处理所有 ACP 消息类型
- ✅ `handle_execution_start()`: 处理 Start 消息
- ✅ `handle_data_chunk()`: 处理 Data 消息 (流式数据)
- ✅ `handle_progress()`: 处理 Progress 消息
- ✅ `handle_execution_complete()`: 处理 End 消息
- ✅ `handle_error()`: 处理 Error 消息

#### 错误恢复
- ✅ 进程退出监控
- ✅ 自动重启机制 (指数退避)
- ✅ 最大重试次数限制 (3 次)

#### 日志监听
- ✅ `monitor_stderr()`: 监听 stderr 输出 (用户可见日志)

### 5. 创建测试文档 (ACP_COMMUNICATION_TEST.md)
完整的测试指南,包含:
- ✅ 前提条件
- ✅ 测试步骤 (编译检查、手动测试、单元测试、集成测试)
- ✅ 前端测试代码示例
- ✅ 流式数据测试
- ✅ 预期的 ACP 消息格式
- ✅ 常见问题和解决方案
- ✅ 性能基准指标

## 技术细节

### ACP 协议实现

#### 消息类型 (src-tauri/src/bridge/codebuddy_adapter.rs:27-65)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AcpMessage {
    GetSkills,
    ExecuteSkill { skill_name: String, input: String },
    Start { session_id: String, skill_name: String, render_mode: Option<String> },
    Data { session_id: String, data: Value },
    Progress { session_id: String, current: u64, total: u64, message: String },
    Error { session_id: String, code: String, message: String, suggestion: String },
    End { session_id: String, success: bool, summary: String },
}
```

#### ndjson 格式
每行一个 JSON 对象,以换行符分隔:
```json
{"type":"start","session_id":"uuid","skill_name":"security-review","render_mode":"code"}
{"type":"data","session_id":"uuid","data":"..."}
{"type":"progress","session_id":"uuid","current":1,"total":10,"message":"Processing..."}
{"type":"end","session_id":"uuid","success":true,"summary":"Review complete"}
```

### 流式数据处理

#### 接收循环 (src-tauri/src/bridge/codebuddy_adapter.rs:419-463)
- 异步循环,持续从 ACP 传输层读取消息
- 自动检测连接断开并触发重启
- 支持优雅停止

#### 事件发射 (src-tauri/src/bridge/codebuddy_adapter.rs:465-595)
- 每个消息类型都有对应的处理函数
- 更新 SessionManager 状态
- 通过 broadcast channel 发送事件到前端

### 错误处理

#### 连接错误
- 检测 stdout/stderr 断开
- 自动触发 Agent 重启
- 记录详细错误日志

#### 超时处理
- 使用 `tokio::time::timeout` 防止无限等待
- 默认超时: 5 秒
- 返回友好的错误消息和建议

#### 进程管理
- 监控进程退出 (Windows: tasklist, Unix: kill)
- 优雅关闭 (kill) + 等待退出 (wait)
- 资源清理 (Drop trait)

## 文件变更清单

### 新增文件
1. `src-tauri/src/bridge/acp_transport.rs` - ACP 传输模块 (523 行)
2. `ACP_COMMUNICATION_TEST.md` - 测试文档 (272 行)

### 修改文件
1. `src-tauri/Cargo.toml` - 添加依赖 (2 行)
2. `src-tauri/src/bridge/mod.rs` - 模块导出 (2 行)
3. `src-tauri/src/bridge/codebuddy_adapter.rs` - 完全重写 (1040 行 → 1100 行)

### 删除内容
- WebSocket 服务器代码
- `establish_control_channel()` 方法
- WebSocket 相关字段和方法
- tokio-tungstenite 依赖使用

## 关键决策

### 为什么选择 ACP + Stdio?
1. ✅ **官方支持**: CodeBuddy Code 官方支持 `--acp` 参数
2. ✅ **简单可靠**: stdio 比WebSocket更简单,无需额外服务器
3. ✅ **性能优越**: 直接管道通信,开销最小
4. ✅ **低风险**: 不需要修改 CodeBuddy Code 代码
5. ✅ **易于调试**: 可以直接在终端测试

### 为什么使用异步模式?
1. ✅ **Tauri 要求**: Tauri 后端使用 Tokio 异步运行时
2. ✅ **并发处理**: 可以同时处理多个消息和事件
3. ✅ **非阻塞 I/O**: 不阻塞主线程,提高响应性
4. ✅ **流式支持**: 更好地支持流式数据处理

### 为什么使用 ndjson 而不是其他格式?
1. ✅ **简单**: 每行一个 JSON,易于解析
2. ✅ **流式**: 支持增量读取,不需要等待完整响应
3. ✅ **标准化**: 广泛使用的流式数据格式
4. ✅ **易调试**: 可以直接查看原始输出

## 潜在问题和解决方案

### 问题 1: Stdio 缓冲
**风险**: 大量输出时可能导致缓冲,延迟增加

**解决方案**:
- ✅ 使用 async/await 非阻塞读取
- ✅ 建议实现批处理 (P2.3)
- ✅ 实现虚拟滚动 (P2.3)

### 问题 2: 进程意外退出
**风险**: Agent 崩溃导致连接中断

**解决方案**:
- ✅ 进程退出监控
- ✅ 自动重启机制
- ✅ 最大重试次数限制

### 问题 3: 消息顺序
**风险**: 并发发送可能导致消息乱序

**解决方案**:
- ✅ 使用单个消息通道
- ✅ 使用 oneshot 等待响应
- ✅ Session ID 标识消息归属

### 问题 4: 内存溢出
**风险**: 流式数据大量累积

**解决方案**:
- ⚠️ 待实现: 数据批处理 (P2.3)
- ⚠️ 待实现: 限制内存使用
- ⚠️ 待实现: 及时清理已消费数据

## 下一步行动

### P1 任务 (高优先级)
1. **P1.1**: ✅ 更新 CodeBuddyAdapter 使用 ACP (已完成)
2. **P1.2**: 实现前端事件监听
   - `useEvent` composable
   - Agent store 事件处理
   - Session store 事件处理
3. **P1.3**: 流式数据处理优化
   - 数据批处理
   - 内存管理
   - 防止溢出
4. **P1.4**: 前端集成
   - OmniBox 集成
   - SkillDock 集成
   - SemanticCanvas 集成

### P2 任务 (中等优先级)
1. **P2.1**: 单元测试
   - ACP 传输层测试
   - 协议解析测试
   - SessionManager 测试
2. **P2.2**: 集成测试
   - Agent 启动测试
   - Skill 执行测试
   - 错误场景测试
3. **P2.3**: 性能优化
   - 虚拟滚动
   - 批处理 UI 更新
   - Web Worker 协议解析
4. **P2.4**: 错误场景测试
   - 进程崩溃恢复
   - 连接断开恢复
   - 内存泄漏检查

## 总结

P0.2: 实现 ACP 通信层已成功完成!

### 主要成就
- ✅ 创建了完整的 ACP 传输层模块
- ✅ 重构 CodeBuddyAdapter 使用 ACP 协议
- ✅ 移除了 WebSocket 依赖,简化架构
- ✅ 实现了流式数据处理
- ✅ 添加了完善的错误处理和恢复机制
- ✅ 提供了详细的测试文档

### 代码质量
- ✅ 模块化设计,职责清晰
- ✅ 完善的错误处理
- ✅ 详细的日志记录
- ✅ 异步/非阻塞实现
- ✅ 资源安全管理 (Drop trait)

### 架构改进
- ✅ 从 WebSocket 迁移到 stdio,降低复杂度
- ✅ 使用官方 ACP 协议,提高稳定性
- ✅ 流式数据处理,提升用户体验
- ✅ 自动恢复机制,提高可靠性

### 下一步
继续实施 P1.2 (前端事件监听) 和 P1.3 (流式数据处理优化),预计完成时间: 3 天。
