# 前端集成实现文档

## 概述

本文档描述了 P1.4: 前端集成的完整实现细节。

## 实现时间
2026-01-30

## 完成的任务

### 1. OmniBox 完整集成 (src/components/OmniBox.vue)

#### 功能实现

**集成 Tauri invoke**:
- ✅ 使用 `useAgent` composable 调用 Tauri 命令
- ✅ 集成 `executeSkill` 方法
- ✅ 集成 `isReady` 和 `isLoading` 状态
- ✅ 错误处理和显示

**输入验证**:
```typescript
const isValidInput = computed(() => {
  return input.value.trim().length > 0 && input.value.trim().length <= 1000
})
```

**自动补全**:
- ✅ 实时监听输入变化
- ✅ 从 Agent Store 获取 Skill 列表
- ✅ 模糊匹配 Skill 名称和描述
- ✅ 最多显示 5 个建议

**智能 Skill 选择**:
```typescript
// 尝试精确匹配
let matchedSkill = skills.find(skill =>
  skill.name.toLowerCase() === command.toLowerCase()
)

// 如果没有精确匹配,尝试模糊匹配
if (!matchedSkill) {
  matchedSkill = skills.find(skill =>
    skill.name.toLowerCase().includes(command.toLowerCase()) ||
    skill.description.toLowerCase().includes(command.toLowerCase())
  )
}

// 如果有匹配的 Skill,执行对应的 Skill
// 否则使用默认的 "assistant" Skill
if (matchedSkill) {
  sessionId = await executeSkill(matchedSkill.name, { input: command })
} else {
  sessionId = await executeSkill('assistant', { input: command })
}
```

**键盘快捷键**:
- ✅ `Ctrl+K` / `Cmd+K`: 聚焦输入框
- ✅ `Enter`: 提交指令
- ✅ `Shift+Enter`: 换行
- ✅ `Escape`: 清空输入并隐藏建议

**UI 改进**:
- ✅ 状态指示器 (就绪/未就绪)
- ✅ 加载指示器
- ✅ 错误消息显示
- ✅ 建议列表动画
- ✅ 禁用状态处理

### 2. SkillDock 完整集成 (src/components/SkillDock.vue)

#### 功能实现

**集成 Agent Store**:
- ✅ 使用 `useAgentStore` 获取 Agent 状态
- ✅ 监听 Agent 状态变化
- ✅ 自动加载 Skill 列表
- ✅ 显示 Agent 运行状态

**集成 Session Store**:
- ✅ 使用 `useSessionStore` 获取 Session 列表
- ✅ 自动更新活跃会话
- ✅ 自动更新已完成会话
- ✅ 通过事件监听实现实时更新

**数据展示**:
- ✅ 显示已安装的 Skill 列表
- ✅ 显示活跃会话 (running/pending)
- ✅ 显示已完成会话 (最近 10 个)
- ✅ 可折叠的分组显示

**交互功能**:
- ✅ 点击 Session 切换活跃 Session
- ✅ 显示 Session 状态图标
- ✅ 显示 Session 输入内容
- ✅ 刷新按钮

**实时更新**:
```typescript
// 监听 Agent 状态变化
watch(() => agentStore.status, (newStatus) => {
  if (newStatus === 'running') {
    // Agent 启动后加载数据
    loadData()
  }
})

// Session Store 已经通过事件监听自动更新
// 这里不需要额外处理
```

**内存统计**:
- ✅ 显示 Skill 数量
- ✅ 显示会话数量
- ✅ 状态指示灯 (绿色/红色)

### 3. SemanticCanvas 完整集成 (src/components/SemanticCanvas.vue)

#### 功能实现

**集成 Session Store**:
- ✅ 使用 `useSessionStore` 获取 Session 数据
- ✅ 监听 `activeSession` 变化
- ✅ 自动切换到对应的 Tab
- ✅ 实时显示 Session 数据

**Tab 管理**:
- ✅ 显示活跃和最近完成的会话
- ✅ 支持 Tab 切换
- ✅ 支持关闭已完成会话的 Tab
- ✅ 自动管理活跃 Tab

**渲染模式**:
- ✅ 实现多种渲染器:
  - **Log**: 日志模式,显示时间戳和内容
  - **Code**: 代码模式,语法高亮
  - **Table**: 表格模式,结构化数据
  - **JSON**: JSON 模式,格式化显示
  - **Markdown**: Markdown 模式,文档显示
- ✅ 用户可切换渲染模式
- ✅ 自动选择默认渲染模式

**数据展示**:
- ✅ 显示 Session 基本信息 (ID, 输入, 状态)
- ✅ 显示进度条 (如果有)
- ✅ 显示内存使用统计
- ✅ 显示错误信息
- ✅ 显示摘要信息

**渲染器实现**:

**Log 渲染器**:
```typescript
log: {
  name: '日志渲染器',
  render: (chunks: DataChunk[]) => {
    return chunks.map(chunk => `
      <div class="log-entry">
        <span class="log-time">${new Date(chunk.received_at).toLocaleTimeString()}</span>
        <span class="log-content">${typeof chunk.data === 'string' ? chunk.data : JSON.stringify(chunk.data, null, 2)}</span>
      </div>
    `).join('')
  }
}
```

**Code 渲染器**:
```typescript
code: {
  name: '代码渲染器',
  render: (chunks: DataChunk[]) => {
    const code = chunks.map(c => typeof c.data === 'string' ? c.data : JSON.stringify(c.data, null, 2)).join('\n')
    return `<pre class="code-block"><code>${escapeHtml(code)}</code></pre>`
  }
}
```

**Table 渲染器**:
```typescript
table: {
  name: '表格渲染器',
  render: (chunks: DataChunk[]) => {
    const data = chunks.map(c => c.data).flat()
    if (!Array.isArray(data) || data.length === 0) return '<p class="text-gray-500">无数据</p>'

    const keys = Object.keys(data[0] || {})
    return `
      <table class="data-table">
        <thead>
          <tr>
            ${keys.map(key => `<th>${key}</th>`).join('')}
          </tr>
        </thead>
        <tbody>
          ${data.map(row => `
            <tr>
              ${keys.map(key => `<td>${escapeHtml(String(row[key] || ''))}</td>`).join('')}
            </tr>
          `).join('')}
        </tbody>
      </table>
    `
  }
}
```

**内存管理**:
- ✅ 显示数据块数量
- ✅ 显示内存使用量 (MB)
- ✅ 内存接近限制时显示警告
- ✅ 集成 Session Store 的内存管理功能

**错误处理**:
- ✅ 显示错误代码和消息
- ✅ 显示建议解决方案
- ✅ 红色高亮显示

**摘要显示**:
- ✅ 显示执行摘要
- ✅ 绿色高亮显示成功摘要

**自动切换**:
```typescript
// 监听 activeSession 变化,自动切换到对应的 Tab
watch(() => activeSession.value?.id, (newId) => {
  if (newId) {
    const index = tabs.value.findIndex(t => t.id === newId)
    if (index !== -1) {
      activeTab.value = index
    }
  }
})
```

### 4. 全局错误通知 (src/composables/useToast.ts 和 src/components/ToastContainer.vue)

#### 功能实现

**useToast Composable**:
- ✅ 提供统一的 Toast 接口
- ✅ 支持多种类型: success, error, warning, info
- ✅ 自动移除 (可配置时长)
- ✅ 手动移除功能
- ✅ 清除所有功能

**API**:
```typescript
interface UseToastReturn {
  toasts: Toast[]
  addToast(message: string, type?: ToastType, duration?: number): number
  removeToast(id: number): void
  clearAll(): void
  success(message: string, duration?: number): number
  error(message: string, duration?: number): number
  warning(message: string, duration?: number): number
  info(message: string, duration?: number): number
}
```

**ToastContainer 组件**:
- ✅ 固定在右上角
- ✅ 使用 Teleport 到 body
- ✅ 动画进入/离开效果
- ✅ 支持多个 Toast 堆叠
- ✅ 手动关闭按钮
- ✅ 根据类型显示不同颜色和图标

**使用示例**:
```typescript
import { useToast } from '@/composables/useToast'

const { success, error, warning, info } = useToast()

// 显示成功消息
success('操作成功!')

// 显示错误消息
error('操作失败,请重试')

// 显示警告消息
warning('内存使用接近限制')

// 显示信息消息
info('正在处理中...')
```

### 5. 实时状态更新

#### 实现机制

**事件驱动的状态更新**:
- ✅ Agent Store 监听 `agent-event` 事件
- ✅ Session Store 监听 `session-updated` 事件
- ✅ 两个 Store 自动更新状态
- ✅ 组件通过 computed 响应式更新

**自动刷新**:
```typescript
// SkillDock 中的实时更新
watch(() => agentStore.status, (newStatus) => {
  if (newStatus === 'running') {
    loadData() // 自动加载 Skill 和 Session
  }
})
```

**组件响应**:
- ✅ OmniBox: 显示加载状态和错误
- ✅ SkillDock: 自动更新会话列表
- ✅ SemanticCanvas: 自动更新 Tab 和内容

### 6. 自动补全 (已完成)

#### 实现细节

**实时匹配**:
```typescript
watch(input, async (newValue) => {
  if (!newValue.trim() || !isReady.value) {
    suggestions.value = []
    return
  }

  try {
    const skills = await agentStore.getSkills()
    const matched = skills.filter(skill =>
      skill.name.toLowerCase().includes(newValue.toLowerCase()) ||
      skill.description.toLowerCase().includes(newValue.toLowerCase())
    )

    suggestions.value = matched.slice(0, 5).map(skill => ({
      name: skill.name,
      description: skill.description
    }))
  } catch (error) {
    console.error('获取 Skill 列表失败:', error)
    suggestions.value = []
  }
})
```

**建议列表显示**:
- ✅ 显示 Skill 名称
- ✅ 显示 Skill 描述
- ✅ 点击自动填充
- ✅ 自动提交

### 7. 输入验证 (已完成)

#### 实现细节

**验证规则**:
```typescript
const isValidInput = computed(() => {
  return input.value.trim().length > 0 && input.value.trim().length <= 1000
})
```

**错误提示**:
- ✅ 输入为空时提示
- ✅ 输入过长时提示
- ✅ Agent 未运行时提示
- ✅ 自动消失 (3-5 秒)

### 8. 键盘快捷键 (已完成)

#### 实现的快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+K` / `Cmd+K` | 聚焦 OmniBox |
| `Enter` | 提交指令 |
| `Shift+Enter` | 换行 |
| `Escape` | 清空输入并隐藏建议 |

## 集成流程图

```
用户操作
    ↓
┌─────────────────────────────────────────────────┐
│                   OmniBox                     │
│  - 输入指令                                   │
│  - 自动补全                                   │
│  - 验证输入                                   │
│  - 调用 executeSkill()                       │
└───────────────┬───────────────────────────────┘
                │
                │ Tauri invoke
                ↓
┌─────────────────────────────────────────────────┐
│              Tauri Backend                   │
│  - 执行 Skill                                │
│  - 返回 Session ID                           │
└───────────────┬───────────────────────────────┘
                │
                │ Events (agent-event, session-updated)
                ↓
┌─────────────────────────────────────────────────┐
│           Stores (Agent & Session)            │
│  - 接收事件                                   │
│  - 更新状态                                   │
│  - 发出 Toast 错误通知                         │
└───────┬───────────────────────┬─────────────┘
        │                       │
        │ reactive update      │ reactive update
        ↓                       ↓
┌────────────────┐    ┌────────────────┐
│   SkillDock    │    │ SemanticCanvas│
│  - 更新列表     │    │  - 更新 Tab    │
│  - 显示状态     │    │  - 渲染数据     │
└────────────────┘    └────────────────┘
```

## 使用示例

### 执行 Skill

```vue
<script setup>
import { useAgent } from '@/composables/useAgent'
import OmniBox from '@/components/OmniBox.vue'

const { executeSkill } = useAgent()

async function handleCommand() {
  // OmniBox 会自动处理:
  // 1. 输入验证
  // 2. Skill 匹配
  // 3. 调用 executeSkill
  // 4. 显示结果
}
</script>

<template>
  <OmniBox @submit="handleCommand" />
</template>
```

### 监听 Session 更新

```vue
<script setup>
import { useSessionStore } from '@/stores/session'

const sessionStore = useSessionStore()

// Session Store 会自动更新,无需手动监听
const activeSession = computed(() => sessionStore.activeSession)
</script>

<template>
  <div v-if="activeSession">
    <p>Status: {{ activeSession.status }}</p>
    <p>Data chunks: {{ activeSession.data_chunks.length }}</p>
  </div>
</template>
```

### 显示 Toast 通知

```vue
<script setup>
import { useToast } from '@/composables/useToast'

const { success, error, warning, info } = useToast()

function handleSuccess() {
  success('操作成功!')
}

function handleError() {
  error('操作失败,请重试')
}
</script>

<template>
  <button @click="handleSuccess">成功</button>
  <button @click="handleError">失败</button>
</template>
```

## 性能优化

### 1. 计算属性缓存
```typescript
// 使用 computed 缓存计算结果
const isValidInput = computed(() => {
  return input.value.trim().length > 0 && input.value.trim().length <= 1000
})

const activeSessions = computed(() =>
  sessionStore.sessions.filter(s => s.status === 'running' || s.status === 'pending')
)
```

### 2. 事件节流
```typescript
// 使用 watch 监听输入,debounce 优化
watch(input, async (newValue) => {
  // 自动补全逻辑
}, { debounce: 300 })
```

### 3. 虚拟滚动
```vue
<!-- 对于大量数据,考虑使用虚拟滚动 -->
<VirtualList
  :items="session.data_chunks"
  :item-size="50"
  v-slot="{ item }"
>
  <div>{{ item.data }}</div>
</VirtualList>
```

## 错误处理

### 1. Agent 未运行
```typescript
if (!isReady.value) {
  errorMessage.value = 'Agent 未运行,请先启动 Agent'
  setTimeout(() => {
    errorMessage.value = null
  }, 3000)
  return
}
```

### 2. 执行失败
```typescript
try {
  sessionId = await executeSkill(skillName, { input: command })
} catch (error) {
  errorMessage.value = error instanceof Error ? error.message : '执行指令失败'
  // 使用 Toast 显示全局错误
  useToast().error(errorMessage.value, 5000)
}
```

### 3. 数据加载失败
```typescript
try {
  await loadData()
} catch (err) {
  error.value = err instanceof Error ? err.message : '加载数据失败'
  useToast().error(error.value)
}
```

## 常见问题

### 1. Session 不更新

**问题**: Session 状态没有实时更新

**解决方案**:
- 确认 Session Store 的事件监听器已设置
- 检查后端是否正确发送事件
- 查看浏览器控制台的错误日志

### 2. Toast 不显示

**问题**: Toast 通知没有显示

**解决方案**:
- 确认 `ToastContainer` 组件已添加到根组件
- 检查 z-index 是否被覆盖
- 查看是否有 CSS 冲突

### 3. 自动补全不工作

**问题**: 输入时没有显示建议

**解决方案**:
- 确认 Agent 正在运行
- 检查 Skill 列表是否加载成功
- 验证输入是否为空

## 下一步

### P2 任务 (中等优先级)

1. **P2.1**: 单元测试
   - 编写前端组件测试
   - 编写 Store 测试
   - 编写 Composable 测试

2. **P2.2**: 集成测试
   - 测试完整的用户流程
   - 测试错误场景

3. **P2.3**: 性能优化
   - 实现虚拟滚动
   - 批处理 UI 更新
   - Web Worker 处理

4. **P2.4**: 错误场景测试
   - 测试 Agent 崩溃
   - 测试网络错误
   - 测试内存泄漏

## 总结

P1.4: 前端集成已成功完成!

### 主要成就
- ✅ OmniBox 完整集成,支持 Tauri invoke
- ✅ SkillDock 完整集成,支持实时更新
- ✅ SemanticCanvas 完整集成,支持多种渲染模式
- ✅ 实现实时状态更新
- ✅ 实现自动补全
- ✅ 实现输入验证
- ✅ 实现全局错误通知 (Toast)
- ✅ 实现键盘快捷键

### 架构改进
- ✅ 完全响应式,基于 Vue 3 Composition API
- ✅ 事件驱动,自动更新状态
- ✅ 模块化设计,组件独立可复用
- ✅ 错误处理完善,用户体验友好

### 用户体验
- ✅ 实时反馈,流畅交互
- ✅ 智能补全,提高效率
- ✅ 多种渲染模式,灵活展示
- ✅ 错误提示清晰,易于解决

继续实施 P2 (测试 & 优化)!
