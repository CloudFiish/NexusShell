# CodeBuddy Code 通信方案分析与实施计划

**验证日期**: 2026-01-29
**CodeBuddy Code 版本**: 2.41.6
**验证方式**: 手动执行命令

---

## 📊 验证结果分析

### ✅ CodeBuddy Code 基础信息

**版本**: 2.41.6
**安装状态**: ✅ 已安装

### ✅ 发现的关键通信能力

#### 1. 输出格式控制
- **命令**: `--output-format <format>` (第104行)
- **支持格式**:
  - `"text"` - 纯文本输出
  - `"json"` - 单个 JSON 结果
  - `"stream-json"` - 实时流式 JSON 输出
  - 默认: `"text"`

#### 2. 输入格式控制
- **命令**: `--input-format <format>` (第106行)
- **支持格式**:
  - `"text"` - 纯文本输入
  - `"stream-json"` - 实时流式 JSON 输入

#### 3. JSON Schema 验证
- **命令**: `--json-schema <schema>` (第108行)
- **功能**: 定义输出的 JSON Schema 进行验证
- **示例**: `{"type":"object","properties":{"name":{"type":"string"}},"required":["name"]}}`

#### 4. 流式消息支持
- **命令**: `--include-partial-messages` (第110行)
- **功能**: 包含来自模型请求的原始 SSE 增量消息
- **配合**: `--output-format stream-json`

#### 5. MCP 协议支持
- **命令**: `--mcp-config <fileOrString>` (第120行)
- **功能**: 从 JSON 文件或字符串加载 MCP 服务器配置
- **相关命令**:
  - `codebuddy mcp add` - 添加 MCP 服务器
  - `codebuddy mcp remove` - 删除 MCP 服务器
  - `codebuddy mcp list` - 列出 MCP 服务器
  - `codebuddy mcp get <name>` - 获取服务器详情

#### 6. ACP (Agent Client Protocol) 模式 ⭐⭐⭐
- **命令**: `--acp` (第142行)
- **功能**: 启用 ACP 模式，通过 stdin/stdout 使用 ndJsonStream 通信
- **传输方式**: `--acp-transport` (第144行)
  - 支持:
    - `"stdio"` - 使用 stdin/stdout
    - `"streamable-http"` - 使用 HTTP 流式传输

#### 7. 非交互式输出
- **命令**: `-p` / `--print` (第103行)
- **功能**: 打印响应并退出（用于管道和自动化）

#### 8. HTTP 服务器模式
- **命令**: `--serve` (第139行)
- **功能**: 启动 HTTP 服务器模式（非交互式）
- **相关参数**:
  - `--port <number>` - 指定端口
  - `--host <string>` - 指定主机地址
  - `--acp` - ACP 模式下可以同时使用

---

## 🎯 最佳通信方案选择

### ⭐ 推荐方案：ACP + Stdio 双向通信

**选择理由**:
1. ✅ **标准协议** - ACP 是标准化的 Agent Client Protocol
2. ✅ **双向通信** - 支持实时双向通信
3. ✅ **实时流式** - 支持 ndJsonStream 实时流式传输
4. ✅ **稳定可靠** - CodeBuddy Code 官方支持
5. ✅ **易于实现** - 不需要额外封装，直接通信
6. ✅ **高性能** - 低延迟，高吞吐量

**技术栈**:
- **传输方式**: `--acp-transport "stdio"`
- **协议**: ACP (Agent Client Protocol) + ndJsonStream
- **序列化**: JSON (ndjson 库)
- **通道**: stdin/stdout (双向)

**命令示例**:
```bash
# 启动 ACP 模式
codebuddy --acp --acp-transport "stdio"

# 执行 Skill 并获取流式响应
codebuddy --acp --acp-transport "stdio" -p '{"command":"execute_skill","skill_name":"security-review"}'
```

---

## 📋 实施计划

### Phase 1: 完善 CodeBuddyAdapter ACP 集成 (2-3 天)

#### 任务 1.1: 实现 ACP 模式启动
**文件**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**行动**:
1. [ ] 启动时添加 `--acp` 参数
2. [ ] 添加 `--acp-transport "stdio"` 参数
3. [ ] 移除临时 WebSocket 服务器代码
4. [ ] 修改进程启动逻辑

**代码修改**:
```rust
// 在 start_process 方法中
let mut cmd = Command::new(&self.config.binary_path);
cmd.arg("--acp")
   .arg("--acp-transport")
   .arg("stdio");
```

**预期耗时**: 0.5 天

#### 任务 1.2: 实现 ACP 通信层
**文件**: `src-tauri/src/bridge/acp_transport.rs` (新建)

**行动**:
1. [ ] 创建 ACP 传输层模块
2. [ ] 实现 ndJsonStream 编码器
3. [ ] 实现 ndJsonStream 解码器
4. [ ] 实现消息发送 (stdin)
5. [ ] 实现消息接收 (stdout)
6. [ ] 实现流式消息处理

**依赖**: ndjson 库

**预计耗时**: 1.5 天

#### 任务 1.3: 更新 Session Manager
**文件**: `src-tauri/src/bridge/session_manager.rs`

**行动**:
1. [ ] 调整 DataChunk 类型以支持 ACP 格式
2. [ ] 添加流式消息类型 (SSE 消息)
3. [ ] 优化并发性能
4. [ ] 添加消息去重逻辑

**预计耗时**: 0.5 天

---

### Phase 2: 前端 ACP 事件集成 (1-2 天)

#### 任务 2.1: 更新前端事件监听
**文件**: `src/stores/agent.ts`, `src/stores/session.ts`, `src/composables/useAgent.ts`

**行动**:
1. [ ] 监听 `acp-message` 事件
2. [ ] 处理 ACP 协议消息
3. [ ] 更新 Session Store 状态
4. [ ] 实现流式数据渲染

**预计耗时**: 1 天

#### 任务 2.2: 实现前端 ACP 协议解析
**文件**: `src/utils/acp-protocol.ts` (新建)

**行动**:
1. [ ] 定义 ACP 消息类型
2. [ ] 实现消息解析器
3. [ ] 实现消息序列化器
4. [ ] 实现 ndjson-stream 解析

**依赖**: ndjson-stream 库

**预计耗时**: 0.5 天

---

### Phase 3: 完善流式数据处理 (1 天)

#### 任务 3.1: 实现流式事件处理
**文件**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**行动**:
1. [ ] 处理 SSE 风格事件
2. [ ] 处理增量数据
3. [ ] 更新 Session Manager
4. [ ] 触发前端事件

**预计耗时**: 1 天

---

### Phase 4: 测试和优化 (1-2 天)

#### 任务 4.1: 编写单元测试
**文件**: `src-tauri/src/bridge/acp_transport_test.rs` (新建)

**行动**:
1. [ ] 测试 ACP 消息序列化
2. [ ] 测试 ACP 消息反序列化
3. [ ] 测试流式数据解析
4. [ ] 测试错误处理

**目标覆盖率**: 90%

**预计耗时**: 0.5 天

#### 任务 4.2: 编写集成测试
**文件**: `src-tauri/tests/acp_integration.rs` (新建)

**行动**:
1. [ ] 测试完整的 ACP 通信流程
2. [ ] 测试 Skill 发现
3. [ ] 测试 Skill 执行
4. [ ] 测试流式数据传输
5. [ ] 测试错误恢复

**预计耗时**: 1 天

#### 任务 4.3: 性能优化
**行动**:
1. [ ] 实现消息批处理
2. [ ] 优化内存使用
3. [ ] 性能基准测试
4. [ ] 目标: 流式输出延迟 < 100ms

**预计耗时**: 0.5 天

---

## 📊 详细时间估算

| 阶段 | 任务 | 预计耗时 | 累计 |
|------|------|---------|------|
| Phase 1: ACP 集成 | 1.1 + 1.2 + 1.3 | 2.5 天 | 2.5 天 |
| Phase 2: 前端集成 | 2.1 + 2.2 | 1.5 天 | 4 天 |
| Phase 3: 流式处理 | 3.1 | 1 天 | 5 天 |
| Phase 4: 测试优化 | 4.1 + 4.2 + 4.3 | 2 天 | 7 天 |

**总计**: 7 天

---

## 🎯 技术架构

### 通信流程

```
┌─────────────┐
│   Frontend  │
└──────┬──────┘
       │ WebSocket / IPC
┌──────▼──────┐
│   Bridge    │
│  (Tauri)    │
└──────┬──────┘
       │ ACP Protocol
┌──────▼──────┐
│  Adapter    │
│  (ACP)      │
└──────┬──────┘
       │ ndJsonStream (Stdio)
┌──────▼──────┐
│ CodeBuddy  │
│   Code      │
└─────────────┘
```

### 数据流

1. **下行 (Frontend → CodeBuddy)**:
   ```
   Frontend → Bridge → stdin → CodeBuddy
   ```

2. **上行 (CodeBuddy → Frontend)**:
   ```
   CodeBuddy → stdout → Bridge → ndJsonStream → EventEmitter → Frontend
   ```

### 消息格式

**下行消息 (JSON)**:
```json
{
  "command": "execute_skill",
  "skill_name": "security-review",
  "input": "审查代码安全性"
}
```

**上行消息 (ndjson - Newline Delimited JSON)**:
```
{"type":"start","session_id":"...","skill_name":"security-review"}

{"type":"data_chunk","session_id":"...","data":"..."}

{"type":"progress","session_id":"...","current":1,"total":10,"message":"处理中..."}

{"type":"complete","session_id":"...","success":true,"summary":"审查完成"}
```

---

## 🔧 需要添加的依赖

### Rust 后端

**Cargo.toml**:
```toml
[dependencies]
# ... 现有依赖
ndjson = "0.8"
tokio-util = { version = "0.7", features = ["io"] }
async-trait = "0.1"
thiserror = "1"
```

### 前端

**package.json**:
```json
{
  "dependencies": {
    "ndjson-stream": "^0.6.0"
  }
}
```

---

## 🎯 成功标准

### 功能完整性
- [ ] CodeBuddy Code 可以通过 ACP 模式启动
- [ ] 可以获取完整的 Skill 列表和元数据
- [ ] 可以执行 Skill 并获取实时流式输出
- [ ] 支持至少 3 种渲染模式 (log, code, table)
- [ ] 支持同时运行多个 Skill 并独立查看结果
- [ ] 可以通过 MCP Manager 管理服务器

### 稳定性
- [ ] ACP 通信稳定可靠
- [ ] 流式输出延迟 < 100ms
- [ ] 长时间运行 (1 小时+) 不出现内存泄漏
- [ ] 大数据量 (10,000+ 行日志) 不卡顿

### 用户体验
- [ ] 自然语言输入可以正确触发 Skill
- [ ] 流式输出延迟 < 100ms
- [ ] 错误信息清晰且包含解决建议
- [ ] 界面响应流畅，无明显卡顿
- [ ] 支持中英文界面 (至少错误提示中文化)

### 代码质量
- [ ] Rust 单元测试覆盖率 ≥ 80%
- [ ] 集成测试覆盖主要用户流程
- [ ] 无编译警告 (warnings as errors)
- [ ] 符合 Rust Clippy 规范
- [ ] TypeScript 类型安全，无 any 类型

---

## 🔄 下一步行动

### 立即执行

1. [ ] **更新实施计划** - 根据验证结果更新 IMPLEMENTATION_PLAN.md
2. [ ] **更新 README** - 反映 ACP 集成方案
3. [ ] **提交代码** - 提交当前所有代码到 git

### 近期执行 (本周)

1. [ ] **Phase 1.1** - 实现 ACP 模式启动
2. [ ] **Phase 1.2** - 实现 ACP 通信层
3. [ ] **Phase 1.3** - 更新 Session Manager

### 中期执行 (下周)

1. [ ] **Phase 2** - 前端 ACP 事件集成
2. [ ] **Phase 3** - 完善流式数据处理
3. [ ] **Phase 4** - 测试和优化

---

**请确认此方案是否符合您的需求，我将立即开始实施！**
