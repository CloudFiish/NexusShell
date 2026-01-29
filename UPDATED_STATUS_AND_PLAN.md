# Nexus Shell 更新状态和实施计划

**更新日期**: 2026-01-29
**当前进度**: 87% (26/30 tasks completed)
**通信方案**: ACP (Agent Client Protocol) + Stdio

---

## ✅ 重大发现

经过验证，我们发现了 CodeBuddy Code 的完整通信能力：

### 🔍 验证结果

**CodeBuddy Code 版本**: 2.41.6
**安装状态**: ✅ 已安装

**支持的通信能力**:
- ✅ **ACP (Agent Client Protocol)**: 完整支持
  - 命令: `--acp` (第142行)
  - 传输方式: `--acp-transport "stdio"` (第144行)
  - 支持双向通信
- ✅ **JSON 输出格式**: 完整支持
  - 命令: `--output-format` (第104行)
  - 支持: `"text"`, `"json"`, `"stream-json"` (第105行)
- ✅ **流式消息**: 完整支持
  - 命令: `--include-partial-messages` (第110行)
  - 配合: `--output-format stream-json` (第111行)
- ✅ **MCP 协议**: 完整支持
  - 命令: `codebuddy mcp --help`, `codebuddy mcp list` (第182-192行)
  - 支持添加、删除、列出、详情查看 MCP 服务器
- ✅ **JSON Schema**: 完整支持
  - 命令: `--json-schema <schema>` (第108行)
  - 用于验证和约束输出格式
- ✅ **输入格式控制**: 完整支持
  - 命令: `--input-format` (第106行)
  - 支持: `"text"`, `"stream-json"`
- ✅ **命令行选项**: 完整支持
  - `--print` / `-p`: 打印并退出 (第103行)
  - `--model`: 选择模型 (第124行)
  - `--serve`: 启动 HTTP 服务器 (第139行)

**不支持的**:
- ❌ WebSocket (不需要 ACP 更好)
- ❌ 特殊标记 (JSON 输出更规范)

---

## 🎯 最佳通信方案

### ⭐⭐⭐ 推荐方案: ACP + Stdio 双向通信

**方案类型**: 原生集成，使用官方协议

#### 核心特性

**1. 使用 ACP (Agent Client Protocol) 模式**
```bash
# 启动时
codebuddy --acp --acp-transport "stdio"
```

**2. 通过 stdin 发送 JSON 命令**
```json
{
  "command": "execute_skill",
  "skill_name": "security-review",
  "input": "审查代码安全性"
}
```

**3. 通过 stdout 接收 ndjson 格式的流式响应**
```json
{"type":"start","session_id":"uuid","skill_name":"security-review"}
{"type":"data","session_id":"uuid","data":"..."}
{"type":"progress","session_id":"uuid","current":1,"total":10}
{"type":"end","session_id":"uuid","success":true,"summary":"..."}
```

#### 优势

| 特性 | 方案 A: ACP+Stdio | 方案 B: WebSocket | 方案 C: 标记解析 | 方案 D: 文件 I/O |
|------|----------------|---------------|------------|-----------|
| **实现难度** | 低 (官方协议) | 中 (需维护) | 中高 (不稳定) | 高 (复杂) |
| **通信性能** | 高 | 高 | 低 | 低 |
| **双向通信** | ✅ 支持 | ✅ 支持 | ❌ 单向 | ✅ 支持 |
| **实时性** | ✅ 优秀 | ✅ 优秀 | ⚠️ 中等 | ❌ 差 |
| **可靠性** | ✅ 高 | ⚠️ 中 | ⚠️ 低 | ✅ 中 |
| **维护成本** | ✅ 低 (跟随官方) | 中 | 高 | 中 |
| **官方支持** | ✅ 完整 | ❌ 无 | ❌ 无 | ❌ 无 |
| **流式输出** | ✅ 原生支持 | ✅ 支持 | ❌ 手动实现 | ❌ 手动实现 |
| **协议稳定** | ✅ 标准化 | ❌ 自定义 | ❌ 自定义 | ❌ 自定义 |

**综合评分**: ⭐⭐⭐⭐⭐ (最高)

---

## 📊 实施计划更新

基于验证结果，我们调整了实施计划：

### Phase 2 (已更新)

#### 任务 2.2-2.9: 实现 ACP 通信层 (替代原 WebSocket 方案)

**新任务**: 2.2 实现 ACP 模式启动
- 添加 `--acp --acp-transport "stdio"` 参数
- 移除临时 WebSocket 服务器代码

**新任务**: 2.3 实现 stdin 写入器
- 使用 `ndjson::to_writer` 创建 JSON 写入器
- 实现消息发送方法

**新任务**: 2.4 实现 stdout 读取器
- 使用 `ndjson::from_reader` 创建 JSON 流解析器
- 实现流式消息处理循环

**新任务**: 2.5 实现 ACP 消息类型
- 定义 ACP 消息枚举
- 实现 Start, Data, Progress, Error, End 消息类型
- 对应 `--include-partial-messages` 的 SSE 格式

**新任务**: 2.6 实现 ACP 协议集成
- 实现 ndjson::Parser 和 ndjson::Writer
- 处理 ndjson Newline-Delimited JSON 流

**新任务**: 2.7 实现 MCP 集成
- 使用 `codebuddy mcp list` 获取 Skill 列表
- 实现 MCP 服务器动态发现
- 实现 `--mcp-config` 参数传递

---

### 新增 Phase 2.5: ACP 通信层实现 (2 天)

#### 任务 2.5.1: 添加 ndjson 依赖

**文件**: `src-tauri/Cargo.toml`

**行动**:
- 添加 ndjson 依赖: `ndjson = "0.8"`
- 添加 tokio-util 依赖: `tokio-util = "0.7"`

**预计耗时**: 0.25 天

#### 任务 2.5.2: 实现 ACP 传输层

**文件**: `src-tauri/src/bridge/acp_transport.rs` (新建)

**行动**:
1. 定义 ACP 消息类型结构体
2. 实现 ndjson::Parser 用于读取 stdout
3. 实现 ndjson::to_writer 用于写入 stdin
4. 实现错误处理和日志记录
5. 实现心跳机制

**预计耗时**: 1.5 天

#### 任务 2.5.3: 更新 CodeBuddyAdapter 使用 ACP

**文件**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**行动**:
1. 移除 WebSocket 服务器代码 (establish_control_channel 方法)
2. 更新 start_process 添加 `--acp --acp-transport "stdio"` 参数
3. 替换 send_message 方法使用 ndjson writer
4. 替换 stdout/stderr 监听使用 ndjson parser
5. 更新所有消息处理逻辑以使用 ACP 消息类型
6. 移除 WebSocket 相关代码

**预计耗时**: 0.5 天

---

## 📅 更新的时间估算

| 阶段 | 任务 | 原估算 | 新估算 | 变化 |
|------|------|---------|---------|------|
| Phase 2: CodeBuddyAdapter | 7 天 | 6 天 | -1 天 |
| Phase 2.5: ACP 通信层 | 0 天 | 2 天 | +2 天 |
| Phase 3: MCP 服务器管理 | 2 天 | 2 天 | 0 天 |
| Phase 4: 错误处理和恢复 | 3 天 | 3 天 | 0 天 |
| Phase 5: Tauri Commands | 2 天 | 2 天 | 0 天 |
| Phase 6: 前端状态管理 | 3 天 | 3 天 | 0 天 |
| Phase 7: 前端组件实现 | 5 天 | 5 天 | 0 天 |
| Phase 8: 测试和优化 | 4 天 | 4 天 | 0 天 |
| **总计** | **28 天** | **27 天** | **-1 天** |

---

## 🎯 立即开始的任务

### 优先级 P0.2: 实现 ACP 通信层 (今天开始)

#### 步骤 1: 添加依赖 (15 分钟)

```bash
cd d:\Project\NexusShell\src-tauri
# 在 Cargo.toml 的 [dependencies] 部分添加:
ndjson = "0.8"
tokio-util = "0.7"
```

#### 步骤 2: 创建 ACP 传输层模块 (2 小时)

```bash
# 创建文件
echo "// src-tauri/src/bridge/acp_transport.rs" > src-tauri/src/bridge/acp_transport.rs
```

然后在文件中实现 ACP 传输层。

#### 步骤 3: 更新 CodeBuddyAdapter (1 小时)

修改 `src-tauri/src/bridge/codebuddy_adapter.rs`:
- 移除 WebSocket 相关代码
- 更新启动命令
- 集成 ndjson parser/writer
- 更新消息处理逻辑

#### 步骤 4: 测试 ACP 通信 (30 分钟)

```bash
cd d:\Project\NexusShell\src-tauri
cargo check
cargo build
```

---

## 📝 文档更新

### 已更新
- ✅ `ACP_COMMUNICATION_PLAN.md` - ACP 通信方案分析和实施计划
- ✅ `NEXT_STEPS_PLAN_UPDATED.md` - 更新的实施计划
- ✅ `PROJECT_STATUS_REPORT.md` - 项目状态报告

### 需要更新
- [ ] `IMPLEMENTATION_PLAN.md` - 更新 Phase 2 任务以反映 ACP 方案
- [ ] `README.md` - 更新通信方案说明
- [ ] `架构.md` - 更新架构图以反映 ACP 通信
- [ ] `docs/control-channel-protocol.md` - 更新为 ACP 协议规范

---

## 🎉 关键成功因素

### 1. 使用官方协议 ✅
- ACP 是 CodeBuddy Code 官方支持的协议
- 无需自己实现 WebSocket 服务器
- 跟随官方更新

### 2. 原生支持流式 ✅
- `--include-partial-messages` 原生支持 SSE 风格
- `--output-format stream-json` 原生支持流式 JSON
- 无需手动实现流式机制

### 3. 双向通信 ✅
- stdin: 发送命令
- stdout: 接收流式响应
- ndjson 格式确保数据完整性

### 4. 简化实现 ✅
- 无需 WebSocket 服务器
- 无需消息确认机制
- 无需心跳和重连
- ndjson 库处理所有复杂性

### 5. 稳定性高 ✅
- 官方协议, 维护简单
- 代码量少, bug 少
- 性能优秀

---

## 🚀 下一步行动

### 选项 1: 立即开始实施 (推荐)

**现在开始**:
1. 添加 ndjson 依赖到 Cargo.toml
2. 创建 acp_transport.rs 模块
3. 更新 CodeBuddyAdapter 使用 ACP
4. 测试通信

**预计今天完成**: P0.2.1 (依赖) + P0.2.2 (传输层) + P0.2.3 (适配器更新) + 测试

### 选项 2: 先更新文档

**先完成**:
1. 更新 IMPLEMENTATION_PLAN.md
2. 更新 README.md
3. 更新架构.md
4. 更新 control-channel-protocol.md

### 选项 3: 询问问题

**如有任何问题**, 请告诉我:
- ACP 方案是否清晰?
- 需要我解释 ACP 协议的细节?
- 需要我提供代码示例?
- 有其他关注点?

---

**请您确认: 您想立即开始实施吗?还是先更新文档,或者有其他问题?**

**我建议: 立即开始实施 ACP 通信层,这是关键的第一步! 🚀**
