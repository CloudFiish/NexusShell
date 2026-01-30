# ACP 通信功能测试文档

## 概述

本文档描述了如何测试 ACP (Agent Client Protocol) 通信功能。

## 前提条件

1. 安装 CodeBuddy Code 2.41.6 或更高版本
2. 确保 CodeBuddy Code 支持 `--acp --acp-transport "stdio"` 参数
3. 安装 Rust 工具链 (如果需要编译)

## 测试步骤

### 1. 编译检查

首先编译 Rust 代码,确保没有编译错误:

```bash
cd src-tauri
cargo check
```

### 2. 手动测试 ACP 模式

在终端中手动测试 CodeBuddy Code 的 ACP 模式:

```bash
# 启动 CodeBuddy Code 并进入 ACP 模式
codebuddy --acp --acp-transport "stdio"

# 在另一个终端中,通过管道发送命令
echo '{"command":"get_skills"}' | codebuddy --acp --acp-transport "stdio"

# 执行 Skill
echo '{"command":"execute_skill","skill_name":"security-review","input":"Review code security"}' | codebuddy --acp --acp-transport "stdio"
```

### 3. 单元测试

运行单元测试:

```bash
cd src-tauri
cargo test
```

### 4. 集成测试

#### 测试 1: 启动 Agent

```typescript
// 前端测试代码
import { invoke } from '@tauri-apps/api/core';

async function testStartAgent() {
  try {
    await invoke('start_agent', { agentType: 'codebuddy' });
    console.log('Agent 启动成功');
  } catch (error) {
    console.error('Agent 启动失败:', error);
  }
}
```

#### 测试 2: 获取 Skill 列表

```typescript
async function testGetSkills() {
  try {
    const skills = await invoke('get_skills', { agentType: 'codebuddy' });
    console.log('获取到 Skills:', skills);
  } catch (error) {
    console.error('获取 Skills 失败:', error);
  }
}
```

#### 测试 3: 执行 Skill

```typescript
async function testExecuteSkill() {
  try {
    const sessionId = await invoke('execute_skill', {
      agentType: 'codebuddy',
      skillName: 'security-review',
      input: 'Review this code for security issues'
    });
    console.log('Skill 执行开始, Session ID:', sessionId);
  } catch (error) {
    console.error('执行 Skill 失败:', error);
  }
}
```

### 5. 流式数据测试

#### 测试数据块接收

```typescript
async function testStreamingData() {
  const sessionId = await invoke('execute_skill', {
    agentType: 'codebuddy',
    skillName: 'security-review',
    input: 'Review this code for security issues'
  });

  // 监听数据块事件
  const unlisten = await listen('agent-event', (event) => {
    if (event.payload.session_id === sessionId) {
      console.log('收到事件:', event.payload);

      switch (event.payload.type) {
        case 'data_chunk':
          console.log('数据块:', event.payload.data);
          break;
        case 'progress':
          console.log('进度:', event.payload.current, '/', event.payload.total);
          break;
        case 'execution_complete':
          console.log('执行完成:', event.payload.summary);
          unlisten();
          break;
        case 'error':
          console.error('错误:', event.payload.message);
          unlisten();
          break;
      }
    }
  });
}
```

## 预期结果

### ACP 消息格式

#### 请求格式

```json
{"command":"get_skills"}
```

```json
{"command":"execute_skill","skill_name":"security-review","input":"Review code security"}
```

#### 响应格式

**Start 消息:**
```json
{"type":"start","session_id":"uuid","skill_name":"security-review","render_mode":"code"}
```

**Data 消息:**
```json
{"type":"data","session_id":"uuid","data":"..."}
```

**Progress 消息:**
```json
{"type":"progress","session_id":"uuid","current":1,"total":10,"message":"Processing..."}
```

**Error 消息:**
```json
{"type":"error","session_id":"uuid","code":"ERROR","message":"...","suggestion":"..."}
```

**End 消息:**
```json
{"type":"end","session_id":"uuid","success":true,"summary":"..."}
```

## 常见问题

### 1. 编译错误

**问题:** `cargo check` 报错

**解决方案:**
- 检查 Rust 版本是否兼容
- 确保所有依赖已正确安装
- 查看具体的错误信息并修复

### 2. Agent 启动失败

**问题:** Agent 无法启动或连接失败

**解决方案:**
- 检查 CodeBuddy Code 是否已安装
- 验证 `--acp --acp-transport "stdio"` 参数是否支持
- 查看日志输出,定位具体错误

### 3. 消息接收超时

**问题:** 等待响应超时

**解决方案:**
- 检查 Agent 是否正在运行
- 查看日志,确认消息是否发送成功
- 增加超时时间
- 检查是否需要重启 Agent

### 4. 数据格式错误

**问题:** 反序列化失败

**解决方案:**
- 检查 ACP 消息格式是否符合规范
- 确保所有字段都正确
- 查看实际的 JSON 输出,对比预期格式

## 性能基准

### 预期性能指标

| 指标 | 目标值 |
|------|--------|
| Agent 启动时间 | < 2 秒 |
| 获取 Skill 列表 | < 1 秒 |
| 发送消息延迟 | < 50ms |
| 接收消息延迟 | < 100ms |
| 流式数据吞吐 | > 10 MB/s |

### 测试工具

使用以下工具进行性能测试:

```bash
# 使用 Apache Bench (ab) 测试吞吐
ab -n 1000 -c 10 ...

# 使用 wrk 测试并发
wrk -t12 -c400 -d30s ...
```

## 下一步

完成测试后,继续实现以下功能:

1. 前端事件监听 (P1.2)
2. 流式数据处理优化 (P1.3)
3. 完整的前端集成 (P1.4)
4. 单元测试和集成测试 (P2.1, P2.2)
5. 性能优化 (P2.3)
6. 错误场景测试 (P2.4)
