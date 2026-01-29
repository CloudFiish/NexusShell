# Nexus Shell - 智能命令行工具

## 项目概述

Nexus Shell 是一个融合 CLI 强大功能与现代 UI 体验的智能命令行工具，通过垂直分层架构将自然语言指令转换为可视化操作。该项目采用可插拔的 Agent 架构，支持多种 Agent CLI（如 CodeBuddy Code、Claude Code、Aider、Cursor CLI 等）。

## 核心特性

- **可插拔 Agent 架构**: 支持多种 Agent CLI (CodeBuddy Code、Claude Code、Aider、Cursor CLI 等)
- **双通道通信**: 同时支持结构化数据传输和用户可见日志
- **智能渲染**: 根据数据类型自动选择最佳渲染模式(表格、图表、代码等)
- **多会话并发**: 同时运行多个 Skill,通过 Tab 切换查看结果
- **流式更新**: 实时显示 Skill 执行进度和结果

## 架构概览

```
┌─────────────────────────────────────┐
│   Visual Frontend (Vue 3 + TS)       │
│  - Skill Dock                        │
│  - Semantic Canvas                  │
│  - Omni-Box                          │
└──────────┬──────────────────────────┘
           │ WebSocket / IPC
┌──────────▼──────────────────────────┐
│   Communication Bridge (Tauri)       │
│  - Protocol Parser                  │
│  - Session Management               │
│  - Agent Adapter                    │
└──────────┬──────────────────────────┘
           │ Control Channel + stdout
    ┌──────┴──────┬──────────┬─────────┐
    ▼             ▼          ▼         ▼
CodeBuddy Code Claude Code Aider  MCP Server
```

## 技术栈

- **前端**: Vue 3 + TypeScript + Tailwind CSS
- **中间层**: Tauri 2.0 (Rust)
- **Agent**: CodeBuddy Code (默认)、Claude Code、Aider、Cursor CLI 等
- **状态管理**: Pinia
- **通信**: WebSocket / IPC

## 项目结构

```
nexus-shell/
├── src/                    # 前端源码 (Vue 3)
│   ├── components/         # Vue 组件
│   │   ├── SkillDock.vue
│   │   ├── SemanticCanvas.vue
│   │   └── OmniBox.vue
│   ├── stores/            # Pinia stores
│   ├── composables/       # Vue composables
│   ├── renderers/        # 渲染器组件
│   ├── types/            # TypeScript 类型定义
│   └── main.ts
├── src-tauri/             # 后端源码 (Rust)
│   ├── src/
│   │   ├── bridge/       # Bridge 模块
│   │   │   ├── mod.rs
│   │   │   ├── agent_adapter.rs
│   │   │   ├── codebuddy_adapter.rs
│   │   │   ├── protocol.rs
│   │   │   ├── session_manager.rs
│   │   │   ├── event_emitter.rs
│   │   │   ├── mcp_manager.rs
│   │   │   └── error.rs
│   │   ├── commands.rs   # Tauri commands
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── docs/                  # 文档
│   └── control-channel-protocol.md
├── 架构.md                # 架构文档
├── CODEBUDDY.md           # CodeBuddy 指导
└── README.md              # 本文件
```

## 核心模块详解

### 1. 通信桥梁 (Bridge)

位于 `src-tauri/src/bridge/` 目录，包含以下核心模块：

- **agent_adapter.rs**: 定义了 `AgentAdapter` trait，为所有 Agent 提供统一接口
- **codebuddy_adapter.rs**: CodeBuddy Code 的具体实现
- **protocol.rs**: 定义 Control Channel 协议和消息格式
- **session_manager.rs**: 管理会话状态和生命周期
- **mcp_manager.rs**: 管理 Model Context Protocol 服务器
- **event_emitter.rs**: 事件发射和订阅机制

### 2. Control Channel 协议

Control Channel 是专门用于传输结构化数据的通信通道，与 stdout/stderr 并行工作：

- **get_skills**: 获取可用的 Skill 列表
- **execute_skill**: 执行指定的 Skill
- **data_chunk**: 流式传输执行结果
- **progress**: 更新执行进度
- **error**: 错误信息和建议
- **execution_complete**: 执行完成通知

### 3. 渲染模式

前端根据数据类型自动选择渲染模式：
- `table`: 表格数据
- `code`: 代码高亮
- `json`: JSON 树形展示
- `log`: 日志流
- `chart`: 图表渲染
- `file_tree`: 文件树
- `markdown`: Markdown 渲染
- `diff`: 差异对比

## 构建和运行

### 环境要求

- Node.js 18+
- Rust 1.70+
- CodeBuddy Code CLI (可选，用于测试)

### 安装依赖

```bash
# 安装前端依赖
npm install

# 安装 Tauri CLI (如果还没有)
cargo install tauri-cli
```

### 开发模式

```bash
# 启动开发服务器 (自动运行前端和 Tauri)
npm run dev

# 或者使用 Tauri CLI
cargo tauri dev
```

### 构建生产版本

```bash
npm run build
cargo tauri build
```

## 开发约定

### 前端开发

- 使用 Vue 3 Composition API
- 类型安全：所有组件和函数都应有适当的 TypeScript 类型注解
- 组件结构：遵循单一职责原则，组件不应过于庞大
- 状态管理：使用 Pinia 进行全局状态管理
- 样式：使用 Tailwind CSS，避免自定义 CSS

### 后端开发 (Rust)

- 错误处理：使用 `thiserror` 库定义错误类型
- 异步编程：使用 `tokio` 进行异步编程
- 类型安全：充分利用 Rust 的类型系统
- 内存安全：遵循 Rust 的所有权规则
- 文档：为公共 API 编写文档注释

### 通信协议

- 消息格式：所有 Control Channel 消息都是 JSON 格式
- 消息类型：使用 `type` 字段标识消息类型
- 会话管理：每个执行会话都有唯一的 `session_id`
- 双通道：stdout/stderr 用于日志，Control Channel 用于结构化数据

## 贡献指南

1. Fork 仓库
2. 创建功能分支
3. 提交更改
4. 发起 Pull Request

### 代码风格

- 前端：遵循 ESLint 和 Prettier 配置
- 后端：使用 rustfmt 和 clippy

### 测试

- 单元测试：为关键功能编写单元测试
- 集成测试：确保各组件间的协作正常

## 未来发展方向

- Phase 2: CodeBuddy Code 集成
- Phase 3: UI 和功能完善
- Phase 4: 多 Agent 支持
- Phase 5: 高级功能

## 关键设计模式

### 垂直分层
每一层职责清晰,通过明确定义的接口进行通信:
- Frontend → Bridge: 用户交互指令
- Bridge → Agent: 格式化后的指令(通过 Control Channel)
- Agent → Bridge: 执行结果 + UI 指令(通过 Control Channel)
- Agent → User: 日志输出(通过 stdout)
- Bridge → Frontend: 可渲染的状态和数据

### 可插拔 Agent
通过 `AgentAdapter` trait 实现多种 Agent CLI 的统一接口:
- 每个 Agent 实现自己的适配器
- Bridge 无需关心 Agent 的具体实现
- 运行时可切换不同的 Agent

### 双通道通信
- stdout/stderr: 面向用户的日志和进度
- Control Channel: 面向 UI 的结构化数据
- 两个通道并行,互不干扰