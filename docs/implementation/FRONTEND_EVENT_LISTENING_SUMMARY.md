# 前端事件监听实现文档

## 概述

本文档描述了 P1.2: 实现前端事件监听的完整实现细节。

## 实现时间
2026-01-30

## 完成的任务

### 1. 创建 useEvent Composable

#### 文件: `src/composables/useEvent.ts`

**功能**:
- 提供统一的 Tauri 事件监听接口
- 支持持久化监听和一次性监听
- 自动清理监听器,防止内存泄漏
- 提供便捷的事件监听函数

**核心 API**:

```typescript
interface UseEventReturn {
  // 监听事件
  listen: (eventName: string, callback: TauriEventListener) => () => void;

  // 一次性事件监听
  once: (eventName: string, callback: TauriEventListener) => () => void;

  // 发送事件到后端
  emit: (eventName: string, payload?: any) => Promise<void>;

  // 移除所有事件监听器
  removeAllListeners: () => void;
}
```

**便捷函数**:

```typescript
// 监听 Agent 事件
useAgentEvent(callback)

// 监听 Session 更新事件
useSessionEvent(callback)

// 监听 Agent 状态变化事件
useAgentStatusEvent(callback)

// 监听错误事件
useErrorEvent(callback)
```

### 2. 更新 Agent Store

#### 文件: `src/stores/agent.ts`

**新增功能**:

1. **事件监听集成**:
   - 导入 `useAgentEvent` 和 `useErrorEvent`
   - 实现 `setupEventListeners()` 方法
   - 实现 `cleanupEventListeners()` 方法
   - 在 `onMounted` 生命周期中自动设置监听器
   - 在 `onUnmounted` 生命周期中自动清理

2. **事件处理**:
   - 监听 `execution_start` 事件
   - 监听 `execution_complete` 事件
   - 监听错误事件并更新 `error` 状态

**代码示例**:

```typescript
function setupEventListeners() {
  // 监听 Agent 事件
  unlistenAgentEvent = useAgentEvent((event) => {
    switch (event.type) {
      case 'execution_start':
        console.log('[Agent Store] Skill 执行开始:', event);
        break;
      case 'execution_complete':
        console.log('[Agent Store] Skill 执行完成:', event);
        break;
      default:
        console.log('[Agent Store] 收到未处理的 Agent 事件:', event.type);
    }
  });

  // 监听错误事件
  unlistenErrorEvent = useErrorEvent((errorEvent) => {
    console.error('[Agent Store] 收到错误事件:', errorEvent);
    error.value = `${errorEvent.code}: ${errorEvent.message}`;

    if (errorEvent.suggestion) {
      console.log('[Agent Store] 建议解决方案:', errorEvent.suggestion);
    }
  });
}
```

### 3. 更新 Session Store

#### 文件: `src/stores/session.ts`

**新增功能**:

1. **事件监听集成**:
   - 导入 `useAgentEvent` 和 `useErrorEvent`
   - 实现 `setupEventListeners()` 方法
   - 实现 `cleanupEventListeners()` 方法
   - 在生命周期钩子中自动管理监听器

2. **ACP 协议消息处理**:
   - 实现 `handleAgentEvent()` 方法,处理所有 ACP 消息类型
   - 支持 `execution_start` 消息
   - 支持 `data_chunk` 消息 (流式数据)
   - 支持 `progress` 消息 (进度更新)
   - 支持 `execution_complete` 消息
   - 支持 `error` 消息

**代码示例**:

```typescript
function handleAgentEvent(event: any) {
  const { type, session_id, ...rest } = event;

  if (!session_id) {
    console.warn('[Session Store] 事件缺少 session_id:', event);
    return;
  }

  switch (type) {
    case 'execution_start': {
      const { skill_name } = rest;
      handleSessionUpdated(session_id, {
        status: 'running',
        started_at: new Date().toISOString()
      });
      break;
    }

    case 'data_chunk': {
      const { chunk_index, data, is_final } = rest;
      const chunk: DataChunk = {
        index: chunk_index || 0,
        data,
        is_final: is_final || false,
        received_at: new Date().toISOString(),
        size: JSON.stringify(data).length
      };
      handleDataChunk(session_id, chunk);
      break;
    }

    case 'progress': {
      const { current, total, message } = rest;
      const progress: ProgressInfo = {
        current,
        total,
        message,
        percentage: total > 0 ? Math.round((current / total) * 100) : 0,
        updated_at: new Date().toISOString()
      };
      handleProgress(session_id, progress);
      break;
    }

    case 'execution_complete': {
      const { success, summary } = rest;
      handleExecutionComplete(session_id, summary || '', success);
      break;
    }

    default:
      console.log('[Session Store] 未处理的事件类型:', type);
  }
}
```

### 4. 创建事件处理器

#### 文件: `src-tauri/src/event_handlers.rs`

**功能**:

1. **AgentEventHandler**:
   - 监听 Agent 事件流
   - 将事件转换为 JSON 格式
   - 通过 Tauri Event API 发送到前端

2. **事件类型支持**:
   - `execution_start`: Session 开始执行
   - `data_chunk`: 流式数据块
   - `progress`: 进度更新
   - `execution_complete`: 执行完成
   - `error`: 错误信息

3. **事件分发**:
   - 发送通用的 `agent-event` 事件
   - 根据事件类型发送特定事件:
     - `session-updated`: Session 状态更新
     - `progress`: 进度更新
     - `error`: 错误发生

**核心方法**:

```rust
pub struct AgentEventHandler {
    app_handle: AppHandle,
    adapter: Arc<tokio::sync::Mutex<CodeBuddyAdapter>>,
}

impl AgentEventHandler {
    // 启动事件监听 (在后台任务中运行)
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>>;

    // 将 AgentEvent 转换为 JSON
    fn convert_event_to_json(event: &AgentEvent) -> Result<serde_json::Value, Box<dyn std::error::Error>>;

    // 根据事件类型发送特定事件
    async fn emit_specialized_events(
        app_handle: &AppHandle,
        event: &AgentEvent,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
```

### 5. 集成到主应用

#### 文件: `src-tauri/src/main.rs`

**修改内容**:

1. **导入事件处理器模块**:
   ```rust
   mod event_handlers;
   use event_handlers::AgentEventHandler;
   ```

2. **在应用启动时创建事件处理器**:
   ```rust
   let event_handler = AgentEventHandler::new(app_handle, adapter_clone.clone());
   if let Err(e) = tauri::async_runtime::block_on(event_handler.start()) {
       log::error!("启动事件处理器失败: {}", e);
   }
   ```

## 事件流程图

```
┌─────────────────────────────────────────────────────────────────┐
│                         CodeBuddy Code                          │
│                     (ACP 协议输出)                             │
└──────────────────────┬────────────────────────────────────────┘
                       │
                       │ ndjson messages
                       │ (start, data, progress, error, end)
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                    CodeBuddyAdapter                            │
│                   (Rust 后端)                                 │
│                                                                  │
│  - AcpTransport 接收消息                                        │
│  - handle_acp_message() 处理                                   │
│  - 更新 SessionManager                                          │
│  - 发送 AgentEvent 到事件流                                      │
└──────────────────────┬────────────────────────────────────────┘
                       │
                       │ AgentEvent
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                   AgentEventHandler                             │
│                   (Rust 后端)                                   │
│                                                                  │
│  - 监听事件流                                                    │
│  - convert_event_to_json() 转换                                 │
│  - app_handle.emit() 发送到前端                                  │
└──────────────────────┬────────────────────────────────────────┘
                       │
                       │ Tauri Events
                       │ (agent-event, session-updated, progress, error)
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                     useEvent Composable                          │
│                   (Vue 前端)                                    │
│                                                                  │
│  - listen() 监听事件                                             │
│  - once() 一次性监听                                             │
│  - 自动清理监听器                                                │
└──────────────────────┬────────────────────────────────────────┘
                       │
                       │ Callbacks
                       │
                       ▼
         ┌─────────────┴─────────────┐
         │                           │
         ▼                           ▼
┌─────────────────┐       ┌──────────────────┐
│  Agent Store    │       │  Session Store   │
│                 │       │                  │
│  - 错误处理      │       │  - handleAgentEvent() │
│  - 状态更新      │       │  - 数据块处理      │
│  - 日志记录      │       │  - 进度更新        │
└─────────────────┘       │  - 执行完成        │
                          │  - 错误处理        │
                          └──────────────────┘
```

## 测试指南

### 1. 测试 useEvent Composable

```typescript
// 测试基本事件监听
import { useEvent } from '@/composables/useEvent';

const { listen, emit } = useEvent();

// 监听事件
const unlisten = listen('test-event', (payload) => {
  console.log('收到事件:', payload);
});

// 发送事件
await emit('test-event', { data: 'hello' });

// 取消监听
unlisten();
```

### 2. 测试 Agent Store 事件

```typescript
import { useAgentStore } from '@/stores/agent';

const agentStore = useAgentStore();

// Store 会自动设置事件监听器
// 在组件挂载时自动启动
// 在组件卸载时自动清理
```

### 3. 测试 Session Store 事件

```typescript
import { useSessionStore } from '@/stores/session';

const sessionStore = useSessionStore();

// 执行一个 Skill 后,观察 Session 状态变化
// - data_chunk 事件会自动更新 data_chunks
// - progress 事件会自动更新 progress
// - execution_complete 事件会更新 status 为 completed
```

### 4. 集成测试

```typescript
// 完整的事件流测试
import { useAgent } from '@/composables/useAgent';
import { useAgentStore } from '@/stores/agent';
import { useSessionStore } from '@/stores/session';

async function testEventFlow() {
  const { startAgent, executeSkill } = useAgent();
  const agentStore = useAgentStore();
  const sessionStore = useSessionStore();

  // 1. 启动 Agent
  await startAgent();

  // 2. 执行 Skill
  const sessionId = await executeSkill('security-review', {
    input: 'Review this code'
  });

  // 3. 观察 Session 状态变化
  const checkSession = setInterval(() => {
    const session = sessionStore.getSession(sessionId);
    if (session) {
      console.log('Session 状态:', session.status);
      console.log('数据块数量:', session.data_chunks.length);
      console.log('进度:', session.progress);

      if (session.status === 'completed' || session.status === 'error') {
        clearInterval(checkSession);
      }
    }
  }, 1000);
}
```

## 常见问题

### 1. 事件未触发

**可能原因**:
- 事件处理器未启动
- Agent 未运行
- 事件名称不匹配

**解决方案**:
- 检查 Rust 后端日志,确认事件处理器已启动
- 确认 Agent 状态为 running
- 检查事件名称拼写

### 2. 内存泄漏

**可能原因**:
- 未正确清理事件监听器
- 组件重复挂载/卸载

**解决方案**:
- 使用 `useEvent` composable,自动清理
- 在组件卸载时调用 `removeAllListeners()`
- 检查是否有重复的监听器

### 3. 事件延迟

**可能原因**:
- Agent 处理速度慢
- 网络延迟 (如果使用 WebSocket)
- 前端渲染阻塞

**解决方案**:
- 优化 Agent 处理逻辑
- 使用异步处理避免阻塞 UI
- 实现虚拟滚动优化渲染

## 性能优化建议

### 1. 事件批处理

当收到大量 `data_chunk` 事件时,可以进行批处理:

```typescript
// 在 Session Store 中实现批处理
const chunkBuffer: DataChunk[] = [];
let batchTimer: number | null = null;

function handleDataChunk(sessionId: string, chunk: DataChunk) {
  chunkBuffer.push(chunk);

  if (batchTimer) {
    clearTimeout(batchTimer);
  }

  batchTimer = setTimeout(() => {
    const session = sessionsMap.value.get(sessionId);
    if (session) {
      session.data_chunks.push(...chunkBuffer);
      chunkBuffer.length = 0;
    }
  }, 100); // 100ms 批处理
}
```

### 2. 虚拟滚动

对于大量数据,使用虚拟滚动:

```vue
<template>
  <VirtualList
    :items="session.data_chunks"
    :item-size="50"
    v-slot="{ item }"
  >
    <div>{{ item.data }}</div>
  </VirtualList>
</template>
```

### 3. 使用 Web Worker

将繁重的数据处理移到 Web Worker:

```typescript
// worker.ts
self.onmessage = (e) => {
  const data = e.data;
  const processed = processData(data);
  self.postMessage(processed);
};

// 组件中
const worker = new Worker('./worker.ts');
worker.onmessage = (e) => {
  console.log('处理结果:', e.data);
};
```

## 下一步

### P1.3: 流式数据处理优化 (2 天)

1. 实现数据批处理
2. 实现内存管理
3. 防止内存溢出
4. 实现虚拟滚动

### P1.4: 前端集成 (2 天)

1. OmniBox 完整集成
2. SkillDock 完整集成
3. SemanticCanvas 完整集成
4. 实时状态更新
5. 全局错误通知

## 总结

P1.2: 实现前端事件监听已成功完成!

### 主要成就
- ✅ 创建了 `useEvent` composable,提供统一的事件监听接口
- ✅ 更新了 Agent Store,支持 Agent 事件监听
- ✅ 更新了 Session Store,支持 ACP 协议消息处理
- ✅ 创建了 `AgentEventHandler`,将 Rust 事件转发到前端
- ✅ 集成到主应用,自动启动事件处理器
- ✅ 实现了自动清理机制,防止内存泄漏

### 架构改进
- ✅ 解耦事件处理逻辑,提高可维护性
- ✅ 支持多种事件类型,灵活扩展
- ✅ 自动资源管理,降低内存泄漏风险
- ✅ 完整的日志记录,便于调试

### 代码质量
- ✅ 模块化设计,职责清晰
- ✅ TypeScript 类型安全
- ✅ Vue 3 Composition API
- ✅ 自动清理机制
- ✅ 详细的日志记录

继续实施 P1.3 (流式数据处理优化) 和 P1.4 (前端集成)!
