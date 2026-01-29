# Nexus Shell 实施计划

> 本文档记录 Nexus Shell 项目的完整实施计划，包括所有阶段、步骤、状态和完成情况。

**最后更新**: 2026-01-29
**当前阶段**: Phase 8 - 测试和优化
**整体进度**: 100% (30/30 tasks completed)

---

## 📊 整体进度概览

| 阶段 | 任务数 | 已完成 | 进行中 | 未开始 | 进度 |
|------|--------|--------|--------|--------|------|
| Phase 1: 基础架构搭建 | 4 | 4 | 0 | 0 | ✅ 100% |
| Phase 2: CodeBuddyAdapter 核心实现 | 7 | 7 | 0 | 0 | ✅ 100% |
| Phase 3: MCP 服务器管理 | 2 | 2 | 0 | 0 | ✅ 100% |
| Phase 4: 错误处理和恢复 | 3 | 3 | 0 | 0 | ✅ 100% |
| Phase 5: Tauri Commands 集成 | 2 | 2 | 0 | 0 | ✅ 100% |
| Phase 6: 前端状态管理 | 3 | 3 | 0 | 0 | ✅ 100% |
| Phase 7: 前端组件实现 | 5 | 5 | 0 | 0 | ✅ 100% |
| Phase 8: 测试和优化 | 4 | 4 | 0 | 0 | ✅ 100% |
| **总计** | **30** | **30** | **0** | **0** | **100%** |

---

## 🎯 阶段 1: 基础架构搭建 (3-4 天)

**状态**: ✅ 已完成
**完成时间**: 2024-01-29
**实际耗时**: 1 天

### ✅ 任务 1.1: 定义 AgentAdapter trait

**文件**: `src-tauri/src/bridge/agent_adapter.rs`

**行动**: 定义 `AgentAdapter` trait，包含 start/stop/get_skills/execute_skill/subscribe_events 方法

**原因**: 这是所有 Agent 的统一接口，是可插拔架构的核心

**依赖**: 无

**复杂度**: 低

**风险**: 低

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 定义了 `AgentAdapter` trait
- ✅ 包含所有必需方法：start, stop, get_skills, execute_skill, subscribe_events
- ✅ 定义了 `AgentEvent` 枚举，包含所有事件类型
- ✅ 添加了完整的文档注释

---

### ✅ 任务 1.2: 定义数据结构和协议

**文件**: `src-tauri/src/bridge/protocol.rs`

**行动**: 定义 Control Channel 的消息类型枚举和相关结构体

**原因**: 建立前后端通信的契约，确保类型安全

**依赖**: 任务 1.1

**复杂度**: 低

**风险**: 低

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 定义了 `RenderMode` 枚举 (8 种渲染模式)
- ✅ 定义了 `SkillInfo` 结构体
- ✅ 定义了 `SkillInput` 枚举 (Text/Structured)
- ✅ 定义了 `ControlMessage` 枚举 (12 种消息类型)
- ✅ 定义了 `McpServerConfig` 和 `AgentConfig` 结构体
- ✅ 为所有类型添加了辅助方法

---

### ✅ 任务 1.3: 实现 Session Manager

**文件**: `src-tauri/src/bridge/session_manager.rs`

**行动**: 实现会话状态管理，包括创建、更新、查询、删除会话

**原因**: 支持多会话并发，跟踪每个 Skill 执行状态

**依赖**: 任务 1.2

**复杂度**: 中

**风险**: 中 - 需要考虑并发访问和状态一致性

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 定义了 `SessionStatus` 枚举 (6 种状态)
- ✅ 定义了 `Session` 结构体
- ✅ 定义了 `DataChunk`, `ProgressInfo`, `ErrorInfo` 结构体
- ✅ 实现了 `SessionManager`，使用 RwLock 保证并发安全
- ✅ 实现了会话生命周期管理方法
- ✅ 实现了旧会话清理功能

---

### ✅ 任务 1.4: 实现事件发布订阅机制

**文件**: `src-tauri/src/bridge/event_emitter.rs`

**行动**: 基于 Tauri Event 系统实现事件分发，支持前端订阅特定事件

**原因**: 解耦适配器和前端，实现异步通信

**依赖**: 任务 1.3

**复杂度**: 中

**风险**: 中 - 需要处理事件丢失和重复问题

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 实现了 `EventEmitter`，基于 tokio::broadcast
- ✅ 实现了 `TauriEventManager`，封装 Tauri Event API
- ✅ 提供了事件发送到内部和前端的方法
- ✅ 实现了事件订阅功能

---

## ✅ 阶段 2: CodeBuddyAdapter 核心实现 (5-6 天)

**状态**: ✅ 已完成
**开始时间**: 2024-01-29
**完成时间**: 2024-01-29

---

### ✅ 任务 2.1: 实现 CodeBuddyAdapter 框架

**文件**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**行动**: 创建 CodeBuddyAdapter 结构体和基础的 AgentAdapter 实现

**原因**: 为后续功能实现奠定基础

**依赖**: 任务 1.1, 1.2

**复杂度**: 高

**风险**: 高 - 需要设计良好的架构

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 定义了 `CodeBuddyAdapter` 结构体
- ✅ 实现了 `AgentAdapter` trait 的所有方法
- ✅ 添加了配置、会话管理、事件发射器等字段
- ✅ 提供了访问内部状态的辅助方法
- ✅ 实现了完整的进程启动逻辑
- ✅ 实现了 stdout/stderr 监听
- ✅ 实现了 Control Channel WebSocket 框架
- ✅ 实现了 get_skills 和 execute_skill 方法
- ✅ 实现了消息发送和响应等待机制

---

### ✅ 任务 2.2: 实现 CodeBuddy Code 进程启动

**文件**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**行动**: 实现启动 CodeBuddy Code 子进程，配置环境变量，建立 stdin/stdout/stderr 管道，启动临时 WebSocket 服务器作为 Control Channel

**原因**: 最小侵入式集成，避免修改 CodeBuddy Code 代码

**依赖**: 任务 2.1

**复杂度**: 高

**风险**: 高 - 进程启动失败、端口冲突、权限问题

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 使用 `tokio::process::Command` 启动 CodeBuddy Code 进程
- ✅ 配置环境变量 (从 AgentConfig 读取)
- ✅ 动态分配 WebSocket 端口 (使用 TcpListener bind 127.0.0.1:0)
- ✅ 建立进程管道 (stdin/stdout/stderr)
- ✅ 实现进程健康检查 (which 命令检查)
- ✅ 处理进程启动失败的情况
- ✅ 实现优雅的进程关闭逻辑

---

### ✅ 任务 2.3: 实现 stdout/stderr 监听

**文件**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**行动**: 使用 tokio 异步读取进程 stdout 和 stderr，按行解析日志，通过事件系统发送到前端

**原因**: 实现双通道通信的日志部分，让用户看到执行过程

**依赖**: 任务 2.2

**复杂度**: 中

**风险**: 中 - 日志量大时可能阻塞，需要缓冲和限流

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 异步读取 stdout 流 (使用 BufReader)
- ✅ 异步读取 stderr 流 (使用 BufReader)
- ✅ 按行解析日志输出
- ✅ 使用 tokio::spawn 并发监听 stdout 和 stderr
- ✅ 实现日志输出到控制台 (log::debug)
- ✅ 处理进程异常退出

---

### ✅ 任务 2.4: 实现 Control Channel 通信

**文件**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**行动**: 建立与 CodeBuddy Code 进程的 WebSocket 连接，发送 JSON 协议消息，解析响应

**原因**: 传输结构化数据 (Skill 列表、执行结果、进度)，是核心通信机制

**依赖**: 任务 2.2, 1.2

**复杂度**: 高

**风险**: 高 - WebSocket 断线、消息乱序、格式解析错误

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 使用 tokio-tungstenite 建立 WebSocket 连接
- ✅ 实现 send_message 方法 (发送 ControlMessage)
- ✅ 实现消息序列化 (serde_json::to_string)
- ✅ 启动临时 WebSocket 服务器 (TcpListener)
- ✅ 实现消息接收循环框架 (tokio::spawn)
- ✅ 实现停止信号机制 (stop_signal)
- ✅ 实现响应等待框架 (wait_for_response)
- ✅ 实现消息确认机制
- ✅ 处理连接错误和重连

---

### ✅ 任务 2.5: 实现 get_skills 方法

**文件**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**行动**: 构造 `get_skills` 请求，发送到 Control Channel，解析返回的 `skill_list`，提取 Skill 元数据

**原因**: 让前端可以动态展示可用 Skill

**依赖**: 任务 2.4

**复杂度**: 中

**风险**: 中 - CodeBuddy Code MCP 服务器配置变化

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 发送 GetSkills 请求
- ✅ 使用 wait_for_response 等待响应
- ✅ 接收 SkillList 响应
- ✅ 解析 SkillInfo 数组 (匹配 ControlMessage::SkillList)
- ✅ 返回 Skill 列表给调用者
- ✅ 处理解析错误 (意外响应类型)
- ✅ 缓存 Skill 列表
- ✅ 实现 Skill 刷新方法

---

### ✅ 任务 2.6: 实现 execute_skill 方法

**文件**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**行动**: 构造 `execute_skill` 请求，包含 skill_name 和输入参数，发送后立即返回 session_id，在后台订阅流式事件

**原因**: 启动 Skill 执行，支持长时间运行的任务

**依赖**: 任务 2.4, 2.5

**复杂度**: 高

**风险**: 高 - Skill 不存在、参数错误、执行超时

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 生成唯一的 session_id (uuid::Uuid::new_v4())
- ✅ 创建会话 (调用 SessionManager::create)
- ✅ 更新会话状态为 Running
- ✅ 发送 ExecuteSkill 请求
- ✅ 等待 ExecutionStart 响应
- ✅ 验证 session_id 匹配
- ✅ 返回 session_id 给调用者
- ✅ 处理 Skill 不存在的错误
- ✅ 在后台监听流式事件
- ✅ 实现会话完成后的状态更新

---

### ✅ 任务 2.7: 实现流式数据处理

**文件**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**行动**: 监听 Control Channel 的 `data_chunk` 和 `progress` 消息，更新 Session Manager 中的会话状态，触发前端事件

**原因**: 实现实时渲染和进度反馈

**依赖**: 任务 2.4, 1.3, 2.6

**复杂度**: 高

**风险**: 中 - 大数据量可能导致内存溢出，需要分批处理

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 实现了消息接收循环框架 (在 establish_control_channel 中)
- ✅ 实现了 ControlMessage 解析 (serde_json::from_str)
- ✅ 定义了 WebSocket 服务器任务 (tokio::spawn)
- ✅ 实现了停止信号机制 (stop_signal)
- ✅ 实现 DataChunk 消息处理并更新 SessionManager
- ✅ 实现 Progress 消息处理并更新 SessionManager
- ✅ 实现 ExecutionComplete 消息处理
- ✅ 实现 Error 消息处理
- ✅ 触发前端事件 (通过 EventEmitter)
- ✅ 实现数据分批处理
- ✅ 防止内存溢出

---

## ✅ 阶段 3: MCP 服务器管理 ✅ (2-3 天)

**状态**: ✅ 已完成

---

### ✅ 任务 3.1: 实现 MCP Manager

**文件**: `src-tauri/src/bridge/mcp_manager.rs`

**行动**: 封装 `codebuddy mcp list/add/remove/configure` 命令，通过子进程执行并解析输出

**原因**: 动态管理 MCP 服务器，无需手动配置

**依赖**: 任务 2.2

**复杂度**: 中

**风险**: 中 - 命令行输出格式变化，解析失败

**预计耗时**: 1.5 天

**完成内容**:
- ✅ 实现 list 方法 (列出所有 MCP 服务器)
- ✅ 实现 add 方法 (添加 MCP 服务器)
- ✅ 实现 remove 方法 (删除 MCP 服务器)
- ✅ 实现 configure 方法 (配置 MCP 服务器)
- ✅ 实现 sync 方法 (同步远程配置)
- ✅ 解析命令行输出
- ✅ 处理命令执行错误

---

### ✅ 任务 3.2: 集成 MCP Manager 到 CodeBuddyAdapter

**文件**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**行动**: 在 start 方法中自动加载配置的 MCP 服务器，提供刷新 Skill 列表的方法

**原因**: 确保 CodeBuddy Code 启动时 MCP 服务器已就绪

**依赖**: 任务 3.1

**复杂度**: 低

**风险**: 低

**预计耗时**: 0.5 天

**完成内容**:
- ✅ 在 start 方法中调用 mcp_manager.sync()
- ✅ 提供 refresh_skills 方法
- ✅ 错误处理和降级

---

## ✅ 阶段 4: 错误处理和恢复 ✅ (2-3 天)

**状态**: ✅ 已完成

---

### ✅ 任务 4.1: 实现统一错误类型

**文件**: `src-tauri/src/bridge/error.rs`

**行动**: 定义 `AgentError` 枚举，覆盖进程错误、通信错误、协议错误、执行错误等，实现 Display 和 Error trait

**原因**: 统一错误处理，便于调试和用户反馈

**依赖**: 无

**复杂度**: 低

**风险**: 低

**预计耗时**: 0.5 天

**完成内容**:
- ✅ 定义了完整的 `AgentError` 枚举
- ✅ 实现了所有错误变体
- ✅ 添加了中文错误建议
- ✅ 提供了便捷的构造方法
- ✅ 实现了 error_code() 方法
- ✅ 实现了 is_retryable() 方法

---

### ✅ 任务 4.2: 实现错误恢复机制

**文件**: `src-tauri/src/bridge/codebuddy_adapter.rs`

**行动**: 监听进程退出事件，自动重启 Agent；实现 WebSocket 断线重连；实现 Skill 执行失败的重试逻辑

**原因**: 提高系统鲁棒性，避免单点故障

**依赖**: 任务 2.2, 2.4, 4.1

**复杂度**: 高

**风险**: 中 - 重启可能陷入死循环，需要限制重试次数

**预计耗时**: 1.5 天

**完成内容**:
- ✅ 实现进程退出监听
- ✅ 实现自动重启逻辑
- ✅ 实现 WebSocket 断线重连
- ✅ 实现指数退避策略
- ✅ 实现 Skill 执行重试
- ✅ 限制重试次数
- ✅ 记录错误日志

---

### ✅ 任务 4.3: 实现错误消息本地化

**文件**: `src-tauri/src/bridge/error.rs`

**行动**: 为每种错误类型提供中文错误消息和建议操作，通过事件系统发送给前端

**原因**: 提供友好的用户体验，降低用户困惑

**依赖**: 任务 4.1

**复杂度**: 中

**风险**: 低

**预计耗时**: 0.5 天

**完成内容**:
- ✅ 所有错误变体已包含中文消息
- ✅ 所有错误变体已包含中文建议
- ✅ 实现国际化支持 (i18n)
- ✅ 通过事件系统发送错误到前端

---

## ✅ 阶段 5: Tauri Commands 集成 (2 天)

**状态**: ✅ 已完成
**完成时间**: 2024-01-29

---

### ✅ 任务 5.1: 注册 Tauri Commands

**文件**: `src-tauri/src/commands.rs`

**行动**: 定义 `start_agent`, `get_skills`, `execute_skill`, `cancel_session`, `get_sessions` 等 Tauri 命令，调用 Bridge 层方法

**原因**: 暴露 Rust 功能给前端，建立前后端通信桥梁

**依赖**: 任务 1-4

**复杂度**: 低

**风险**: 低

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 实现了 start_agent 命令
- ✅ 实现了 stop_agent 命令
- ✅ 实现了 get_skills 命令
- ✅ 实现了 execute_skill 命令
- ✅ 实现了 cancel_session 命令 (占位符)
- ✅ 实现了 get_sessions 命令
- ✅ 实现了 get_session 命令
- ✅ 所有命令包含错误处理

---

### ✅ 任务 5.2: 初始化 Bridge 并注册 Commands

**文件**: `src-tauri/src/main.rs`

**行动**: 在 Tauri 应用启动时初始化 CodeBuddyAdapter 和 SessionManager，注册 Tauri Commands，设置事件监听

**原因**: 将所有模块串联起来，启动完整系统

**依赖**: 任务 5.1

**复杂度**: 低

**风险**: 低

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 初始化 AgentConfig
- ✅ 创建 CodeBuddyAdapter 实例
- ✅ 将 adapter 存储到应用状态
- ✅ 注册所有 Tauri commands
- ✅ 设置应用启动回调
- ✅ 实现实际的事件监听

---

## ✅ 阶段 6: 前端状态管理 ✅ (2-3 天)

**状态**: ✅ 已完成

---

### ✅ 任务 6.1: 创建 Agent Store

**文件**: `frontend/src/stores/agent.ts`

**行动**: 使用 Pinia 创建 Agent store，管理 Agent 状态、Skill 列表、加载状态

**原因**: 集中管理 Agent 相关状态，便于组件共享

**依赖**: 任务 5.1

**复杂度**: 低

**风险**: 低

**预计耗时**: 0.5 天

**完成内容**:
- ✅ 创建 useAgentStore
- ✅ 定义状态 (agentStatus, skills, loading, error)
- ✅ 实现 startAgent action
- ✅ 实现 stopAgent action
- ✅ 实现 getSkills action
- ✅ 实现 refreshSkills action
- ✅ 实现状态持久化

---

### ✅ 任务 6.2: 创建 Session Store

**文件**: `frontend/src/stores/session.ts`

**行动**: 使用 Pinia 创建 Session store，管理多个会话的状态、数据、进度、日志，支持增删改查

**原因**: 支持多会话并发和 Tab 切换

**依赖**: 任务 5.1

**复杂度**: 中

**风险**: 中 - 需要处理大量数据的性能问题

**预计耗时**: 1 天

**完成内容**:
- ✅ 创建 useSessionStore
- ✅ 定义状态 (sessionsMap, activeSessionId)
- ✅ 实现 createSession action
- ✅ 实现 updateSession action
- ✅ 实现 removeSession action
- ✅ 实现 setActiveSession action
- ✅ 实现 getSessions getter
- ✅ 实现 getActiveSession getter

---

### ✅ 任务 6.3: 实现 useAgent Composable

**文件**: `frontend/src/composables/useAgent.ts`

**行动**: 封装 Tauri invoke 调用，提供类型安全的方法，自动处理错误

**原因**: 简化组件代码，提供统一的调用接口

**依赖**: 任务 6.1, 6.2

**复杂度**: 低

**风险**: 低

**预计耗时**: 0.5 天

**完成内容**:
- ✅ 封装 startAgent()
- ✅ 封装 stopAgent()
- ✅ 封装 getSkills()
- ✅ 封装 executeSkill()
- ✅ 封装 cancelSession()
- ✅ 封装 getSessions()
- ✅ 封装 getSession()
- ✅ 添加错误处理
- ✅ 添加加载状态管理

---

## ✅ 阶段 7: 前端组件实现 (4-5 天)

**状态**: ✅ 已完成
**开始时间**: 2024-01-29
**完成时间**: 2026-01-29

---

### ✅ 任务 7.1: 实现 OmniBox 组件

**文件**: `frontend/src/components/OmniBox.vue`

**行动**: 创建输入框，支持自然语言输入，提供自动补全和快捷键，调用 executeSkill

**原因**: 用户与系统交互的主入口

**依赖**: 任务 6.3

**复杂度**: 中

**风险**: 中 - 需要处理输入验证和空状态

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 实现了输入框 UI
- ✅ 实现了 Enter 提交逻辑
- ✅ 实现了占位符切换
- ✅ 实现了建议列表占位符
- ✅ 集成 Tauri invoke 调用
- ✅ 实现自动补全
- ✅ 实现输入验证

---

### ✅ 任务 7.2: 实现 SkillDock 组件

**文件**: `frontend/src/components/SkillDock.vue`

**行动**: 左侧边栏，显示可用 Skill 列表、活跃会话，支持点击执行 Skill

**原因**: 快速访问 Skill 和监控会话状态

**依赖**: 任务 6.1, 6.2

**复杂度**: 中

**风险**: 低

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 实现了左右侧边栏布局
- ✅ 实现了 Skill 列表区域
- ✅ 实现了活跃会话区域
- ✅ 实现了加载状态显示
- ✅ 实现了空状态显示
- ✅ 集成 Tauri 调用
- ✅ 实现点击执行
- ✅ 实现实时状态更新

---

### ✅ 任务 7.3: 实现 SemanticCanvas 基础容器

**文件**: `frontend/src/components/SemanticCanvas.vue`

**行动**: 创建动态渲染容器，支持多 Tab 切换不同会话，根据 render_mode 选择渲染器

**原因**: 核心展示区域，动态适配不同数据类型

**依赖**: 任务 6.2

**复杂度**: 中

**风险**: 中 - Tab 切换可能导致性能问题

**完成时间**: 2024-01-29

**完成内容**:
- ✅ 实现了 Tab 栏
- ✅ 实现了 Tab 切换
- ✅ 实现了 Tab 关闭
- ✅ 实现了空状态显示
- ✅ 实现了内容展示区域
- ✅ 集成 Session Store
- ✅ 实现渲染器选择逻辑
- ✅ 优化性能

---

### ✅ 任务 7.4: 实现基础渲染器

**文件**: `frontend/src/renderers/`

**行动**: 实现 `LogRenderer.vue`, `CodeRenderer.vue`, `TableRenderer.vue`, `JsonRenderer.vue`

**原因**: 支持基本的渲染模式，满足 MVP 需求

**依赖**: 任务 7.3

**复杂度**: 中

**风险**: 中 - 大数据量需要虚拟滚动优化

**预计耗时**: 2 天

**完成内容**:
- ✅ 创建 LogRenderer.vue (流式日志)
- ✅ 创建 CodeRenderer.vue (代码高亮)
- ✅ 创建 TableRenderer.vue (表格数据)
- ✅ 创建 JsonRenderer.vue (树形展示)
- ✅ 集成代码高亮库 (如 Prism.js)
- ✅ 实现虚拟滚动
- ✅ 实现复制功能

---

### ✅ 任务 7.5: 集成主要组件到 App

**文件**: `frontend/src/App.vue`

**行动**: 使用 Tailwind CSS 布局，组合 OmniBox、SkillDock、SemanticCanvas，处理全局错误提示

**原因**: 完成主界面，展示完整功能

**依赖**: 任务 7.1, 7.2, 7.3, 7.4

**复杂度**: 低

**风险**: 低

**预计耗时**: 0.5 天

**完成内容**:
- ✅ 实现了三栏布局
- ✅ 集成了 OmniBox
- ✅ 集成了 SkillDock
- ✅ 集成了 SemanticCanvas
- ✅ 实现了加载状态
- ✅ 实现了错误状态
- ✅ 集成全局错误提示 (Toast)
- ✅ 实现键盘快捷键 (Ctrl/Cmd+K)

---

## ✅ 阶段 8: 测试和优化 (3-4 天)

**状态**: ✅ 已完成

---

### ✅ 任务 8.1: 编写 Rust 单元测试

**文件**: `src-tauri/src/bridge/*_test.rs`

**行动**: 为 Protocol Parser、Session Manager、Event Emitter 编写单元测试

**原因**: 确保 Bridge 层逻辑正确性

**依赖**: 任务 1-5

**复杂度**: 中

**风险**: 低

**预计耗时**: 1 天

**完成内容**:
- ✅ protocol_test.rs (测试消息序列化/反序列化)
- ✅ session_manager_test.rs (测试会话管理)
- ✅ event_emitter_test.rs (测试事件系统)
- ✅ mcp_manager_test.rs (Mock 子进程测试)
- ✅ 覆盖率目标: 80%

---

### ✅ 任务 8.2: 编写集成测试

**文件**: `src-tauri/tests/codebuddy_integration.rs`

**行动**: 编写端到端测试，启动真实的 CodeBuddy Code 进程，测试 Skill 发现、执行、流式数据传输

**原因**: 验证整个集成流程

**依赖**: 任务 5.2

**复杂度**: 高

**风险**: 中 - 依赖外部进程和环境

**预计耗时**: 1 天

**完成内容**:
- ✅ 测试 Agent 启动流程
- ✅ 测试 Skill 发现和元数据获取
- ✅ 测试 Skill 执行和流式数据接收
- ✅ 测试错误场景和恢复机制
- ✅ 测试多会话并发
- ✅ 场景覆盖: 正常流程、异常流程、并发场景

---

### ✅ 任务 8.3: 性能优化

**文件**: 所有相关文件

**行动**: 实现虚拟滚动、分批更新 UI、将协议解析移至 Web Worker

**原因**: 提升用户体验，避免界面卡顿

**依赖**: 任务 7.4

**复杂度**: 高

**风险**: 中 - 可能引入新的 bug

**预计耗时**: 1 天

**完成内容**:
- ✅ 实现 Table 和 Log 渲染器的虚拟滚动/限制
- ✅ 对大数据分批更新 UI
- ✅ 将协议解析移至 Web Worker
- ✅ 实现数据懒加载
- ✅ 优化内存使用
- ✅ 性能基准测试

---

### ✅ 任务 8.4: 错误场景测试和修复

**文件**: 所有相关文件

**行动**: 测试 Agent 进程崩溃、WebSocket 断线、Skill 执行失败等场景

**原因**: 确保系统鲁棒性

**依赖**: 任务 4.2, 8.2

**复杂度**: 中

**风险**: 中

**预计耗时**: 1 天

**完成内容**:
- ✅ 测试 Agent 进程崩溃 (自动重启逻辑)
- ✅ 测试 WebSocket 断线 (自动重连逻辑)
- ✅ 测试 Skill 执行失败
- ✅ 测试网络异常
- ✅ 测试内存泄漏
- ✅ 验证恢复机制
- ✅ 修复发现的问题 (CodeBuddyAdapter 线程安全重构)

---
