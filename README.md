# Nexus Shell

Nexus Shell 是一个融合 CLI 强大功能与现代 UI 体验的智能命令行工具,通过垂直分层架构将自然语言指令转换为可视化操作。

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

## 项目状态

✅ **Phase 1: 基础架构** - 已完成
- ✅ 定义 AgentAdapter trait 和数据结构
- ✅ 实现 Session Manager 和事件发布订阅机制
- ✅ 创建基础的项目结构和配置文件

🚧 **Phase 2: CodeBuddy Code 集成** - 进行中
- ✅ 创建 CodeBuddyAdapter 占位符
- 🔄 实现 CodeBuddy Code 进程启动和通信
- ⏳ 实现 Skill 发现和执行
- ⏳ 实现流式数据处理

详细架构请参考 [架构.md](./架构.md) 和 [Control Channel 协议规范](./docs/control-channel-protocol.md)。

## 技术栈

- **前端**: Vue 3 + TypeScript + Tailwind CSS
- **中间层**: Tauri 2.0 (Rust)
- **Agent**: CodeBuddy Code (默认)、Claude Code、Aider、Cursor CLI 等
- **状态管理**: Pinia
- **通信**: WebSocket / IPC

## 快速开始

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

## 开发计划

- [x] Phase 1: 基础架构 (Tauri + Vue 3 项目搭建)
- [ ] Phase 2: CodeBuddy Code 集成
- [ ] Phase 3: UI 和功能完善
- [ ] Phase 4: 多 Agent 支持
- [ ] Phase 5: 高级功能

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

## 技术栈

- **前端**: Vue 3 + TypeScript + Tailwind CSS
- **中间层**: Tauri 2.0 (Rust)
- **Agent**: CodeBuddy Code (默认)、Claude Code、Aider、Cursor CLI 等

## 开发计划

- [ ] Phase 1: 基础架构 (Tauri + Vue 3 项目搭建)
- [ ] Phase 2: CodeBuddy Code 集成
- [ ] Phase 3: UI 和功能完善
- [ ] Phase 4: 多 Agent 支持
- [ ] Phase 5: 高级功能

## 贡献指南

项目目前处于早期阶段,欢迎提出建议和改进意见。

## 许可证

待定
