# Nexus Shell 测试文档

## 目录
1. [概述](#概述)
2. [测试策略](#测试策略)
3. [单元测试](#单元测试)
4. [集成测试](#集成测试)
5. [端到端测试](#端到端测试)
6. [手动测试](#手动测试)
7. [性能测试](#性能测试)
8. [测试覆盖率](#测试覆盖率)
9. [测试环境配置](#测试环境配置)
10. [常见问题及解决方案](#常见问题及解决方案)

## 概述

本文档详细介绍了 Nexus Shell 项目的测试策略、方法和流程。Nexus Shell 是一个融合 CLI 强大功能与现代 UI 体验的智能命令行工具，采用可插拔的 Agent 架构，支持多种 Agent CLI（如 CodeBuddy Code、Claude Code、Aider、Cursor CLI 等）。

### 测试目标
- 确保核心功能按预期工作
- 验证错误处理和恢复机制
- 验证多 Agent 支持
- 确保前端组件正常渲染
- 验证通信协议正确性
- 保证系统稳定性

## 测试策略

### 测试层次
1. **单元测试**: 测试单个函数、组件和模块
2. **集成测试**: 测试模块间的交互
3. **端到端测试**: 测试完整用户流程
4. **性能测试**: 验证系统性能指标
5. **压力测试**: 验证系统在高负载下的表现

### 测试类型
- **功能测试**: 验证功能是否按需求工作
- **回归测试**: 确保新代码不会破坏现有功能
- **错误处理测试**: 验证错误场景下的行为
- **兼容性测试**: 验证跨平台兼容性
- **安全测试**: 验证数据安全和权限控制

## 单元测试

### Rust 后端单元测试

#### 运行命令
```bash
# 进入 src-tauri 目录
cd src-tauri

# 运行所有单元测试
cargo test

# 运行特定模块的测试
cargo test bridge::protocol
cargo test bridge::session_manager
cargo test bridge::error
cargo test bridge::mcp_manager
```

#### 测试覆盖范围

##### 1. 协议模块测试 (`protocol_test.rs`)
- 验证 `AgentConfig` 默认值
- 验证 `RenderMode` 枚举方法
- 验证 `SkillInput` 类型转换
- 验证 `ControlMessage` 类型和方法
- 验证消息类型识别和会话 ID 提取

##### 2. 会话管理测试 (`session_manager_test.rs`)
- 验证会话创建和基本属性
- 验证会话生命周期管理
- 验证数据块处理
- 验证进度更新
- 验证会话列表过滤

##### 3. 错误处理测试 (`error_test.rs`)
- 验证错误创建方法
- 验证可重试错误判断
- 验证错误代码映射
- 验证本地化消息

### Vue 前端单元测试

#### 运行命令
```bash
# 安装测试依赖
npm install --save-dev @vue/test-utils @testing-library/vue jest

# 运行前端测试
npm run test:unit
```

#### 测试覆盖范围
- 组件渲染测试
- 事件处理测试
- 状态管理测试
- API 调用测试

## 集成测试

### 后端集成测试

#### 运行命令
```bash
# 运行集成测试
cargo test --test integration

# 或运行所有测试
cargo test --workspace
```

#### 测试场景
- Agent 适配器与协议解析器集成
- 会话管理器与事件发射器集成
- MCP 管理器与 Agent 配置集成
- WebSocket 通信与消息路由集成

### 前后端集成测试

#### 测试场景
- Tauri 命令调用
- 前端状态与后端数据同步
- 事件订阅和推送机制
- 错误传播和处理

## 端到端测试

### 测试环境准备
```bash
# 安装 E2E 测试工具
npm install --save-dev @playwright/test
```

### 测试场景

#### 1. Agent 生命周期测试
```typescript
// test/agent-lifecycle.spec.ts
import { test, expect } from '@playwright/test';

test('should start and stop agent', async ({ page }) => {
  await page.goto('/');
  
  // 启动 Agent
  await page.click('[data-testid="start-agent"]');
  await expect(page.locator('[data-testid="agent-status"]')).toContainText('running');
  
  // 停止 Agent
  await page.click('[data-testid="stop-agent"]');
  await expect(page.locator('[data-testid="agent-status"]')).toContainText('idle');
});
```

#### 2. Skill 执行测试
```typescript
// test/skill-execution.spec.ts
import { test, expect } from '@playwright/test';

test('should execute skill and show results', async ({ page }) => {
  await page.goto('/');
  
  // 启动 Agent
  await page.click('[data-testid="start-agent"]');
  await page.waitForSelector('[data-testid="agent-status"]:has-text("running")');
  
  // 获取 Skill 列表
  await page.click('[data-testid="get-skills"]');
  await page.waitForSelector('[data-testid="skill-list"] .skill-item');
  
  // 执行第一个 Skill
  const firstSkill = await page.locator('[data-testid="skill-list"] .skill-item').first();
  await firstSkill.click();
  
  // 验证会话创建和结果显示
  await expect(page.locator('[data-testid="active-session"]')).toBeVisible();
});
```

#### 3. 渲染器测试
```typescript
// test/renderers.spec.ts
import { test, expect } from '@playwright/test';

test('should render different data types', async ({ page }) => {
  await page.goto('/');
  
  // 测试表格渲染
  await page.click('[data-testid="render-table"]');
  await expect(page.locator('.data-table')).toBeVisible();
  
  // 测试代码渲染
  await page.click('[data-testid="render-code"]');
  await expect(page.locator('pre code')).toBeVisible();
  
  // 测试 JSON 渲染
  await page.click('[data-testid="render-json"]');
  await expect(page.locator('.json-renderer')).toBeVisible();
});
```

## 手动测试

### 功能测试清单

#### 1. Agent 管理
- [ ] 启动 Agent 按钮功能正常
- [ ] 停止 Agent 按钮功能正常
- [ ] Agent 状态显示正确
- [ ] 错误状态处理正确

#### 2. Skill 管理
- [ ] 获取 Skill 列表功能正常
- [ ] Skill 信息显示正确
- [ ] 执行 Skill 功能正常
- [ ] 取消会话功能正常

#### 3. 会话管理
- [ ] 会话创建成功
- [ ] 会话状态更新正确
- [ ] 数据流式更新
- [ ] 进度显示正确
- [ ] 错误处理正确

#### 4. 渲染器
- [ ] 表格渲染器显示数据
- [ ] 代码渲染器语法高亮
- [ ] JSON 渲染器树形展示
- [ ] 日志渲染器流式显示
- [ ] 渲染器切换功能

#### 5. 错误处理
- [ ] Agent 进程崩溃自动重启
- [ ] WebSocket 断线重连
- [ ] Skill 执行失败处理
- [ ] 网络错误处理
- [ ] 超时错误处理

### 边界条件测试

#### 1. 大数据量测试
- [ ] 大量日志数据渲染
- [ ] 大型 JSON 数据解析
- [ ] 大表格数据滚动
- [ ] 内存使用监控

#### 2. 并发测试
- [ ] 多会话同时执行
- [ ] 多用户访问（如适用）
- [ ] 并发 API 调用
- [ ] 资源竞争处理

## 性能测试

### 前端性能测试

#### 运行命令
```bash
# 使用 Lighthouse 进行性能测试
npm install -g @lhci/cli
lhci autorun
```

#### 性能指标
- 首屏加载时间 < 2s
- 页面完全加载时间 < 5s
- 交互响应时间 < 100ms
- 内存使用 < 200MB
- CPU 使用 < 50%

### 后端性能测试

#### 运行命令
```bash
# 运行性能基准测试
cargo bench
```

#### 性能指标
- WebSocket 连接建立时间 < 100ms
- 消息处理延迟 < 10ms
- 会话创建时间 < 50ms
- 协议解析时间 < 1ms

## 测试覆盖率

### Rust 代码覆盖率
```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 运行覆盖率测试
cargo tarpaulin --out Html

# 查看覆盖率报告
open tarpaulin-report.html
```

### 目标覆盖率
- 协议模块: 95%
- 会话管理: 90%
- 错误处理: 85%
- 通信桥接: 80%
- 总体覆盖率: 85%

## 测试环境配置

### 开发环境
```bash
# 安装依赖
npm install
cd src-tauri && cargo build

# 启动开发服务器
npm run dev
```

### 测试环境
```bash
# CI/CD 配置
NODE_ENV=test
RUST_TEST_THREADS=1
TAURI_SKIP_UI=true
```

### 环境变量
- `TEST_MODE`: 启用测试模式
- `MOCK_AGENT`: 使用 Mock Agent
- `LOG_LEVEL`: 设置日志级别
- `TIMEOUT_MS`: 设置测试超时

## 常见问题及解决方案

### 1. Tauri 构建失败
**问题**: `cargo test` 因 Tauri 构建问题失败
**解决方案**: 
```bash
# 检查图标文件是否存在
ls src-tauri/icons/

# 重新生成图标（如果需要）
# 或使用 cargo check 替代 cargo test
cargo check
```

### 2. WebSocket 连接失败
**问题**: WebSocket 连接测试失败
**解决方案**:
- 检查端口是否被占用
- 验证防火墙设置
- 确认 Agent 进程正在运行

### 3. 前端测试失败
**问题**: Vue 组件测试失败
**解决方案**:
```bash
# 清理缓存
npm run clean
# 重新安装依赖
rm -rf node_modules package-lock.json
npm install
```

### 4. 内存泄漏
**问题**: 长时间运行测试导致内存增长
**解决方案**:
- 检查事件监听器清理
- 验证 WebSocket 连接关闭
- 确认定时器清理

### 5. 并发测试失败
**问题**: 并发测试出现竞态条件
**解决方案**:
- 使用串行执行测试
- 添加适当锁机制
- 隔离测试数据

## 测试报告

### 自动生成报告
```bash
# 生成测试报告
npm run test:report

# 上传到 CI/CD
npm run test:upload
```

### 报告内容
- 测试执行统计
- 通过/失败/跳过数量
- 性能指标
- 覆盖率统计
- 错误详情

## 维护和更新

### 测试维护责任
- 新功能开发必须包含对应测试
- 重构代码时更新相关测试
- 定期审查和清理过时测试
- 保持测试环境同步

### 测试更新流程
1. 分析代码变更影响
2. 更新受影响的测试
3. 添加新功能测试
4. 运行完整测试套件
5. 验证测试结果