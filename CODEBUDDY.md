# CODEBUDDY.md

This file provides guidance to CodeBuddy Code when working with code in this repository.

## 项目概述

Nexus Shell 是一个融合 CLI 强大功能与现代 UI 体验的智能命令行工具,通过垂直分层架构将自然语言指令转换为可视化操作。核心特点是**可插拔的 Agent 架构**,支持多种 Agent CLI (CodeBuddy Code、Claude Code、Aider、Cursor CLI 等)。

## 核心架构

本项目采用**垂直分层架构**,分为三个关键层级:

### 1. 视觉交互层 (Visual Frontend)
- **技术栈**: Vue 3 + TypeScript + Tailwind CSS
- **核心组件**:
  - **Skill Dock (技能坞)**: 实时显示已加载的 Skill 状态和活跃会话
  - **Semantic Canvas (语义画布)**: 动态渲染区,支持多种渲染模式(表格、代码、图表、日志、文件树等)
  - **Omni-Box (全局搜索框)**: 用户输入自然语言指令的入口

### 2. 通信桥梁 (Communication Bridge)
- **技术栈**: Tauri 2.0 (Rust)
- **核心职责**:
  - **双通道通信**:
    - stdout/stderr: 传输用户可见的日志和进度信息
    - Control Channel (WebSocket/IPC): 传输结构化的 UI 指令和数据
  - **Protocol Parser (协议解析器)**: 解析 Control Channel 中的 JSON 协议,转化为前端状态
  - **Session Management (会话管理)**: 管理多个并发 Skill 执行会话
  - **Agent Adapter (Agent 适配器)**: 支持多种 Agent CLI 的统一接口

### 3. 智能内核 (Agent CLI Core)
- **可插拔设计**: 支持多种 Agent CLI,通过统一接口接入
- **默认实现**: CodeBuddy Code (腾讯云官方智能编码工具)
- **其他候选**: Claude Code、Aider、Cursor CLI、OpenAI Code Interpreter、自定义 MCP Server
- **核心职责**:
  - **推理决策 (Model Decision)**: 判断用户意图
  - **Skill 调用**: 通过 MCP 协议调用本地或远程的技能
  - **结构化输出**: 通过 Control Channel 返回可渲染的数据

## 可插拔的 Agent 架构

### 核心设计原则

Agent CLI 核心是可以替换的,通过统一的适配器接口支持多种 Agent 实现。

### Agent 适配器接口

每个 Agent CLI 需要实现以下能力:

1. **双通道通信**
   - stdout/stderr: 面向用户的日志输出
   - Control Channel: 面向 UI 的结构化数据传输

2. **统一的协议格式**
   - `get_skills`: 获取可用 Skill 列表
   - `execute_skill`: 执行指定 Skill
   - `data_chunk`: 流式传输执行结果
   - `progress`: 更新执行进度
   - `error`: 错误信息和建议

3. **Agent 特性声明**
   - 支持的 Skill 类型
   - 流式输出能力
   - 进度报告能力
   - 会话管理能力

### 支持的 Agent 实现

| Agent CLI                   | 描述                             | 集成难度 | 特性支持  |
| --------------------------- | ------------------------------ | ---- | ----- |
| **CodeBuddy Code**          | **默认实现**,腾讯云官方智能编码工具,完整 MCP 支持 | 低    | ⭐⭐⭐⭐⭐ |
| **Claude Code**             | Anthropic AI 编程助手,完整的 Skill 系统 | 中等   | ⭐⭐⭐⭐⭐ |
| **Aider**                   | AI 编程助手,支持 Git 集成              | 低    | ⭐⭐⭐   |
| **Cursor CLI**              | Cursor 的命令行版本                  | 低    | ⭐⭐⭐   |
| **OpenAI Code Interpreter** | OpenAI 的代码执行环境                 | 中等   | ⭐⭐⭐⭐  |
| **Custom MCP Server**       | 自定义 MCP 服务器                    | 低    | ⭐⭐⭐⭐  |

### Agent 适配器实现 (Rust)

定义统一的 `AgentAdapter` trait,每种 Agent CLI 实现自己的适配器:

```rust
pub trait AgentAdapter {
    // 启动 Agent
    async fn start(&mut self) -> Result<(), AgentError>;

    // 停止 Agent
    async fn stop(&mut self) -> Result<(), AgentError>;

    // 获取 Skill 列表
    async fn get_skills(&self) -> Result<Vec<SkillInfo>, AgentError>;

    // 执行 Skill
    async fn execute_skill(
        &self,
        skill_name: &str,
        input: SkillInput,
    ) -> Result<SessionId, AgentError>;

    // 订阅执行事件
    fn subscribe_events(&self) -> EventStream;
}

// CodeBuddy Code 适配器实现
pub struct CodeBuddyAdapter {
    process: Child,
    control_channel: WebSocket,
    mcp_servers: Vec<McpServerConfig>,
    // ...
}

impl AgentAdapter for CodeBuddyAdapter {
    async fn start(&mut self) -> Result<(), AgentError> {
        // 启动 CodeBuddy Code 进程
        // 建立 WebSocket 控制通道
        // 发送初始化握手
    }

    async fn get_skills(&self) -> Result<Vec<SkillInfo>, AgentError> {
        // 通过 Control Channel 请求 Skill 列表
        let request = json!({"type": "get_skills"});
        self.control_channel.send(request).await?;
        // 解析响应
    }

    // ... 其他方法实现
}

// Claude Code 适配器实现
pub struct ClaudeCodeAdapter {
    process: Child,
    control_channel: WebSocket,
    // ...
}

impl AgentAdapter for ClaudeCodeAdapter {
    // 实现 Claude Code 特定的逻辑
}

// Aider 适配器实现
pub struct AiderAdapter {
    process: Child,
    control_channel: NamedPipe,  // 使用命名管道
    // ...
}

impl AgentAdapter for AiderAdapter {
    // 实现 Aider 特定的逻辑
}
```

## 数据流向

项目遵循"语义管道"逻辑,具备**双向性**和**双通道**:

### 意图下行 (Intent Downward)
用户在 Omni-Box 输入自然语言指令 → UI 将指令发给 Bridge → Bridge 通过 Control Channel 发送给 Agent → Agent 推理并调用相应 Skill

### 执行上行 (Execution Upward)
Agent 开始执行 Skill → 通过 Control Channel 发送 `execution_start` → Skill 产生中间数据 → 流式发送 `data_chunk` → 实时更新 Semantic Canvas

### 日志并行 (Log Parallel)
Agent 同时向 stdout 输出详细日志 → Bridge 捕获并在侧边栏日志窗口显示

### 渲染反馈 (Visual Feedback)
Bridge 解析 Control Channel 的 JSON 包 → Semantic Canvas 根据 Skill 元数据选择渲染模式 → 实时渲染数据

## 控制通道协议规范

### 消息类型

**Skill Discovery**
- `get_skills`: 请求 Skill 列表
- `skill_list`: 返回 Skill 元数据(名称、描述、渲染模式、输入输出 schema)

**Skill Execution**
- `execute_skill`: 执行指定 Skill
- `execution_start`: 开始执行,返回 session_id 和渲染模式

**Streaming Data**
- `data_chunk`: 流式传输数据,支持增量更新

**Progress Update**
- `progress`: 更新执行进度和状态信息

**Execution Complete**
- `execution_complete`: 执行完成,返回汇总信息

**Error Handling**
- `error`: 错误信息,包含错误代码、消息、建议和重试选项

### 渲染模式

- `table`: 结构化数据、列表
- `code`: 代码片段、diff
- `json`: 复杂嵌套数据
- `log`: 流式文本、日志
- `chart`: 数值统计、图表
- `file_tree`: 文件系统、目录树
- `markdown`: 文档、说明
- `diff`: 文件差异、变更

## 会话管理

支持多会话并发,用户可以同时运行多个 Skill:

- 每个执行会话有唯一的 `session_id`
- 支持会话状态管理: running、completed、error、paused、cancelled
- Skill Dock 显示所有活跃会话
- Semantic Canvas 通过 Tab 切换不同会话的结果

## 技术选型

### 前端层
- **框架**: Vue 3 + TypeScript
  - 学习曲线平缓,开发效率高
  - 组合式 API 适合复杂状态管理
- **样式**: Tailwind CSS
- **状态管理**: Pinia
- **通信库**: Socket.io (支持重连、心跳)
- **UI 组件**: 建议自定义组件,或使用 PrimeVue/Element Plus

### 中间层
- **跨平台框架**: Tauri 2.0
  - 性能好,体积小(几 MB vs Electron 几百 MB)
  - Rust 后端适合处理协议解析
  - 原生文件系统访问,安全性更好
- **通信协议**: WebSocket 或 IPC (Windows Named Pipe / Unix Socket)

### 后端/Agent
- **MCP 协议**: 支持 Model Context Protocol 进行 Skill 发现和调用
- **多 Agent 支持**: 通过 `AgentAdapter` trait 实现可插拔架构

## 开发注意事项

### Agent 适配器的最小侵入原则

对于现有的 Agent CLI (如 CodeBuddy Code、Claude Code),通过以下方式最小化修改:

1. **Feature Gate**: 使用条件编译,UI Bridge 功能默认禁用
2. **独立模块**: UI Bridge 作为独立的 module/crate,通过可选依赖引入
3. **Sidecar 模式**: 通过子进程和 IPC 通信,避免修改核心逻辑
4. **Plugin 架构**: 定义清晰的接口,Agent 可选择性地实现

### MCP 扩展协议设计

- Control Channel 传输的 JSON 数据格式应统一
- 支持流式数据传输,实现实时渲染
- 每条消息包含 `type` 和 `session_id` 字段

### 动态渲染引擎

- 根据 Skill 的元数据自动选择渲染器
- 支持用户手动切换渲染模式
- 实现 Virtual Scrolling 处理大数据集
- 使用分批渲染优化流式数据更新

### Session 管理

- 使用 WebSocket 管理多会话并发
- 支持暂停/继续/取消正在运行的 Skill
- 保存执行历史,方便回溯
- 实现 Tab 切换查看不同会话结果

### 性能优化

- **虚拟滚动**: 处理大数据集的表格渲染
- **分批渲染**: 流式数据分批更新 UI,避免阻塞
- **Web Worker**: 协议解析在 Worker 线程进行
- **差异更新**: 只更新变化的数据,减少重渲染

### 错误处理和恢复

- **友好的错误提示**: 每个错误包含建议的解决方案
- **重试机制**: 支持自动重试或用户手动重试
- **部分失败处理**: 部分任务失败时,仍然显示成功部分的结果
- **状态回滚**: 支持取消正在执行的 Skill 并恢复状态

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

### 声明式渲染
Semantic Canvas 根据 Skill 的元数据和数据类型选择合适的渲染方式:
- Skill 声明 `default_render` 和 `supported_renders`
- Agent 返回时指定 `render_mode`
- 用户可以手动切换渲染模式

## 项目状态

当前项目处于架构设计阶段,尚未实现代码。

## 未来实现路线

### Phase 1: 基础架构 (2-3 周)
1. **项目初始化**: Tauri + Vue 3 项目搭建
2. **Bridge 核心模块**:
   - 实现 WebSocket 双通道通信
   - 实现 Agent Adapter trait
   - 实现 Protocol Parser
3. **前端基础组件**: Skill Dock、Semantic Canvas 基础框架

### Phase 2: CodeBuddy Code 集成 (2-3 周)
1. **CodeBuddy Code 适配器实现**:
   - 最小侵入式修改 CodeBuddy Code(Feature Gate)
   - 实现 Control Channel 通信
   - 实现 Skill Discovery 和 Execution
2. **基础渲染器实现**: 表格、代码、日志渲染
3. **会话管理**: 多会话并发支持

### Phase 3: 完善 UI 和功能 (2-3 周)
1. **更多渲染器**: 图表、JSON、文件树、Markdown
2. **Omni-Box 集成**: 自然语言输入
3. **错误处理**: 友好的错误提示和重试机制
4. **性能优化**: 虚拟滚动、分批渲染

### Phase 4: 多 Agent 支持 (2-3 周)
1. **Claude Code 适配器实现**
2. **Aider 适配器实现**
3. **Agent 切换 UI**
4. **Agent 配置管理**

### Phase 5: 高级功能 (持续)
1. **Workflow 和 Chain**: Skill 组合和工作流
2. **插件系统**: 第三方 Skill 贡献
3. **权限管理**: 细粒度的权限控制
4. **自定义配置**: 用户偏好设置

## 开发指南

### 如何添加新的 Agent 支持

1. 在 `bridge/agents` 目录创建新的适配器文件
2. 实现 `AgentAdapter` trait
3. 在 Agent 配置中添加新 Agent 的声明
4. 在前端添加 Agent 切换选项

**优先实现的 Agent**:
- **CodeBuddy Code**: 默认实现,优先级最高
- **Claude Code**: 作为备选 Agent,优先级高
- **Aider**: Git 集成功能,优先级中
- **Cursor CLI**: 轻量级备选,优先级低

### 如何添加新的渲染器

1. 在 `frontend/src/renderers` 目录创建新的渲染组件
2. 实现组件逻辑,接收 `data` prop
3. 在 `RenderMode` 枚举中添加新模式
4. 在 Agent 的 Skill 元数据中声明支持的渲染模式

### 如何调试协议通信

1. 在 Bridge 中启用协议日志
2. 使用 WebSocket 调试工具监控 Control Channel
3. 在前端添加事件监听器,打印接收到的消息
4. 使用 `devtools` 查看状态变化

## 术语表

- **Skill**: Agent CLI 可用的技能或工具,如 CodeBuddy Code 的 `backend-patterns`、`tdd-workflow`
- **Session**: 一次 Skill 执行的会话,包含唯一 ID、状态、进度、结果
- **Control Channel**: Bridge 和 Agent 之间传输结构化数据的专用通道
- **Agent**: 智能内核,负责推理决策和 Skill 调用,如 CodeBuddy Code、Claude Code、Aider
- **Agent Adapter**: 统一不同 Agent 接口的适配器,实现 `AgentAdapter` trait
- **Semantic Canvas**: 动态渲染区域,根据数据类型选择合适的渲染模式
- **Skill Dock**: 左侧边栏,显示可用 Skill 和活跃会话
- **Omni-Box**: 全局输入框,用于输入自然语言指令
- **Render Mode**: 渲染模式,如 table、code、log、chart 等
- **MCP Protocol**: Model Context Protocol,用于 Skill 发现和调用的协议
