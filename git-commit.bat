@echo off
echo ==========================================
echo Nexus Shell Git 提交脚本
echo ==========================================
echo.

echo [1/5] 初始化 Git 仓库...
git init
if %errorlevel% neq 0 (
    echo 初始化失败
    pause
    exit /b 1
)

echo.
echo [2/5] 添加所有文件到暂存区...
git add .
if %errorlevel% neq 0 (
    echo 添加文件失败
    pause
    exit /b 1
)

echo.
echo [3/5] 查看暂存的文件...
git status

echo.
echo [4/5] 创建提交...
git commit -m "完成 Phase 1 和 Phase 2：基础架构和 CodeBuddy Code 适配器

## 新增内容

### 基础架构 (Phase 1)
- 实现 AgentAdapter trait - 统一的 Agent 接口
- 定义协议和数据结构 - Control Channel 消息类型
- 实现 Session Manager - 多会话并发管理
- 实现事件系统 - EventEmitter 和 TauriEventManager
- 定义错误类型 - AgentError 和中文错误消息

### CodeBuddy Code 适配器 (Phase 2)
- 实现进程启动和管理
- 实现 stdout/stderr 监听
- 实现 Control Channel WebSocket 框架
- 实现 get_skills 方法
- 实现 execute_skill 方法
- 实现消息发送和响应等待

### Tauri 集成 (Phase 5)
- 实现所有 Tauri Commands
- 初始化 Bridge 和 Agent

### 前端组件 (Phase 7 部分)
- 实现 OmniBox 组件
- 实现 SkillDock 组件
- 实现 SemanticCanvas 组件
- 实现 App.vue 主应用

### 文档
- 实施计划文档 (IMPLEMENTATION_PLAN.md)
- Control Channel 协议文档 (docs/control-channel-protocol.md)
- 更新 README.md

## 技术栈

- Rust: tokio, tokio-tungstenite, async-trait, thiserror, serde, uuid, chrono
- Vue 3: Composition API, Pinia, TypeScript
- 样式: Tailwind CSS

## 当前状态

整体进度: 31.0% (15/29 tasks completed)
Phase 1: 100%
Phase 2: 100%
Phase 5: 100%
Phase 7: 60% (3/5)

## 下一步

- Phase 3: MCP 服务器管理
- Phase 4: 错误处理和恢复
- Phase 6: 前端状态管理

"
if %errorlevel% neq 0 (
    echo 创建提交失败
    pause
    exit /b 1
)

echo.
echo [5/5] 查看提交历史...
git log --oneline -1

echo.
echo ==========================================
echo Git 提交完成！
echo ==========================================
pause
