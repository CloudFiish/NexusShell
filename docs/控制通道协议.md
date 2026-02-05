# Control Channel 协议规范

本文档定义了 Nexus Shell 中 Bridge (Tauri 后端) 和 Agent CLI 之间通过 Control Channel 传输的结构化数据协议。

## 概述

Control Channel 是一个专用的通信通道，用于传输结构化的 UI 指令和数据。它与 stdout/stderr 通道并行，互不干扰。

- **stdout/stderr**: 面向用户的日志输出，用于显示执行进度和调试信息
- **Control Channel**: 面向 UI 的结构化数据，用于 Skill 发现、执行和结果传输

## 消息格式

所有 Control Channel 消息都是 JSON 格式，必须包含 `type` 字段标识消息类型。

### 基本结构

```json
{
  "type": "message_type",
  "data": { /* 消息特定数据 */ }
}
```

## 消息类型

### 1. Skill Discovery (技能发现)

#### GetSkills

请求获取当前可用的 Skill 列表。

**请求**:
```json
{
  "type": "get_skills"
}
```

**响应**: SkillList
```json
{
  "type": "skill_list",
  "skills": [
    {
      "name": "security-review",
      "description": "审查代码安全性，检测潜在漏洞和安全风险",
      "default_render": "table",
      "supported_renders": ["table", "log", "markdown"],
      "input_schema": { /* JSON Schema */ },
      "output_schema": { /* JSON Schema */ },
      "category": "security",
      "requires_filesystem": true,
      "requires_network": false
    }
  ]
}
```

### 2. Skill Execution (技能执行)

#### ExecuteSkill

执行指定的 Skill。

**请求**:
```json
{
  "type": "execute_skill",
  "skill_name": "security-review",
  "input": {
    "type": "text",
    "content": "审查 src/ 目录下的所有代码"
  }
}
```

或者使用结构化输入:
```json
{
  "type": "execute_skill",
  "skill_name": "generate-test",
  "input": {
    "type": "structured",
    "content": {
      "files": ["src/main.ts"],
      "framework": "vitest",
      "coverage": true
    }
  }
}
```

#### ExecutionStart

Skill 开始执行的确认消息。

```json
{
  "type": "execution_start",
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "skill_name": "security-review",
  "render_mode": "table"
}
```

#### ExecutionComplete

Skill 执行完成。

```json
{
  "type": "execution_complete",
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "success": true,
  "summary": "审查完成，发现 3 个安全问题"
}
```

#### CancelExecution

取消正在执行的 Skill。

**请求**:
```json
{
  "type": "cancel_execution",
  "session_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### 3. Streaming Data (流式数据)

#### DataChunk

流式传输 Skill 执行产生的数据，支持增量更新。

```json
{
  "type": "data_chunk",
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "chunk_index": 0,
  "data": {
    /* 实际数据，格式由 Skill 决定 */
  },
  "is_final": false
}
```

**数据格式示例**:

表格数据:
```json
{
  "type": "data_chunk",
  "session_id": "...",
  "chunk_index": 0,
  "data": {
    "headers": ["文件", "问题", "严重性"],
    "rows": [
      ["src/auth.ts", "硬编码密钥", "高"],
      ["src/api.ts", "SQL 注入风险", "中"]
    ]
  },
  "is_final": true
}
```

日志数据:
```json
{
  "type": "data_chunk",
  "session_id": "...",
  "chunk_index": 1,
  "data": {
    "level": "info",
    "timestamp": "2024-01-28T10:30:00Z",
    "message": "正在扫描 src/ 目录..."
  },
  "is_final": false
}
```

### 4. Progress Update (进度更新)

#### Progress

更新 Skill 执行进度。

```json
{
  "type": "progress",
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "current": 5,
  "total": 10,
  "message": "正在分析第 5 个文件"
}
```

### 5. Error Handling (错误处理)

#### Error

Skill 执行过程中发生的错误。

```json
{
  "type": "error",
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "error_code": "FILE_NOT_FOUND",
  "message": "文件 src/missing.ts 不存在",
  "suggestion": "请检查文件路径是否正确，或使用 --path 参数指定正确的路径"
}
```

### 6. Heartbeat (心跳)

#### Heartbeat

保持连接活跃的心跳消息。

**请求**:
```json
{
  "type": "heartbeat"
}
```

**响应**: HeartbeatAck
```json
{
  "type": "heartbeat_ack"
}
```

## 渲染模式 (Render Mode)

前端根据 `render_mode` 字段选择合适的渲染器。

| 模式 | 描述 | 使用场景 |
|------|------|---------|
| `table` | 表格渲染 | 结构化数据、列表、统计结果 |
| `code` | 代码高亮 | 代码片段、diff、配置文件 |
| `json` | JSON 树形展示 | 嵌套数据、API 响应 |
| `log` | 日志流 | 执行日志、调试信息 |
| `chart` | 图表渲染 | 数值统计、趋势图 |
| `file_tree` | 文件树 | 目录结构、文件列表 |
| `markdown` | Markdown 渲染 | 文档、说明、报告 |
| `diff` | 差异对比 | 文件变更、代码差异 |

## 会话管理

每次 Skill 执行都有唯一的 `session_id` (UUID v4 格式)。

- **创建**: 收到 `execute_skill` 请求时创建
- **追踪**: 所有后续消息 (`data_chunk`, `progress`, `error`) 都携带 `session_id`
- **完成**: 收到 `execution_complete` 或 `error` 消息后标记为完成

## 错误代码

标准化的错误代码，便于前端处理和国际化。

| 错误代码 | 描述 |
|---------|------|
| `PROCESS_START_FAILED` | 进程启动失败 |
| `PORT_CONFLICT` | 端口冲突 |
| `WEBSOCKET_CONNECTION_FAILED` | WebSocket 连接失败 |
| `WEBSOCKET_DISCONNECTED` | WebSocket 断开连接 |
| `PROTOCOL_PARSE_ERROR` | 协议解析错误 |
| `SKILL_NOT_FOUND` | Skill 不存在 |
| `SKILL_EXECUTION_FAILED` | Skill 执行失败 |
| `SESSION_NOT_FOUND` | 会话不存在 |
| `INVALID_INPUT` | 输入参数无效 |
| `TIMEOUT` | 操作超时 |
| `IO_ERROR` | IO 错误 |
| `JSON_ERROR` | JSON 序列化/反序列化错误 |

## 传输层

Control Channel 支持以下传输方式:

1. **WebSocket** (推荐)
   - 端口: 可配置，默认动态分配
   - 路径: `/control`
   - 支持双向通信

2. **Unix Domain Socket** (Linux/macOS)
   - 路径: `/tmp/nexus-shell-control.sock`

3. **Named Pipe** (Windows)
   - 名称: `\\.\pipe\nexus-shell-control`

## 实现示例

### Rust 客户端

```rust
use serde_json::json;

// 发送 get_skills 请求
let message = json!({
    "type": "get_skills"
});
websocket.send(message.to_string()).await?;

// 发送 execute_skill 请求
let message = json!({
    "type": "execute_skill",
    "skill_name": "security-review",
    "input": {
        "type": "text",
        "content": "审查代码安全性"
    }
});
websocket.send(message.to_string()).await?;
```

### Rust 服务端

```rust
// 解析消息
let message: ControlMessage = serde_json::from_str(&received_string)?;

match message {
    ControlMessage::GetSkills => {
        let skills = load_skills().await?;
        let response = ControlMessage::SkillList { skills };
        websocket.send(serde_json::to_string(&response)?).await?;
    }
    ControlMessage::ExecuteSkill { skill_name, input } => {
        let session_id = start_execution(skill_name, input).await?;
        // 返回 execution_start 消息
    }
    // ... 其他消息类型
}
```

## 版本

当前协议版本: `1.0.0`

向后兼容性保证:
- 新增字段: 兼容
- 修改字段: 新版本号
- 删除字段: 不兼容
