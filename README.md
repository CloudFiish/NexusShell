# Nexus Shell

Nexus Shell 是一个融合 CLI 强大功能与现代 UI 体验的智能命令行工具,通过垂直分层架构将自然语言指令转换为可视化操作。

## 核心特性

- **可插拔 Agent 架构**: 支持多种 Agent CLI (Claude Code、Aider、Cursor CLI 等)
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
Claude Code   Aider   Cursor CLI  MCP Server
```

## 项目状态

当前项目处于**架构设计阶段**。

详细架构请参考 [架构.md](./架构.md)。

## 技术栈

- **前端**: Vue 3 + TypeScript + Tailwind CSS
- **中间层**: Tauri 2.0 (Rust)
- **Agent**: Claude Code (默认)、Aider、Cursor CLI 等

## 开发计划

- [ ] Phase 1: 基础架构 (Tauri + Vue 3 项目搭建)
- [ ] Phase 2: Claude Code 集成
- [ ] Phase 3: UI 和功能完善
- [ ] Phase 4: 多 Agent 支持
- [ ] Phase 5: 高级功能

## 贡献指南

项目目前处于早期阶段,欢迎提出建议和改进意见。

## 许可证

待定
# NexusShell
