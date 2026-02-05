<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useSessionStore, type Session, type RenderMode } from '@/stores/session'
import type { DataChunk } from '@/stores/session'

const sessionStore = useSessionStore()

const activeTab = ref<number>(0)
const renderMode = ref<RenderMode>('log')
const showTabs = ref(true)

// 从 Session Store 获取数据
const sessions = computed(() => sessionStore.sessions)
const activeSession = computed(() => sessionStore.activeSession)

// Tab 列表 (只显示活跃和最近完成的会话)
const tabs = computed(() => {
  const active = sessionStore.activeSessions
  const recent = sessionStore.completedSessions.slice(-5).reverse()
  return [...active, ...recent]
})

// 监听 activeSession 变化,自动切换到对应的 Tab
watch(() => activeSession.value?.id, (newId) => {
  if (newId) {
    const index = tabs.value.findIndex(t => t.id === newId)
    if (index !== -1) {
      activeTab.value = index
    }
  }
})

// 监听 tabs 变化,如果 activeSession 不在列表中,切换到第一个
watch(tabs, (newTabs) => {
  if (activeSession.value) {
    const index = newTabs.findIndex(t => t.id === activeSession.value.id)
    if (index !== -1) {
      activeTab.value = index
    }
  }
})

// 渲染模式选项
const renderModes: { value: RenderMode; label: string; icon: string }[] = [
  { value: 'log', label: '日志', icon: '📝' },
  { value: 'code', label: '代码', icon: '💻' },
  { value: 'table', label: '表格', icon: '📊' },
  { value: 'json', label: 'JSON', icon: '{ }' },
  { value: 'markdown', label: 'Markdown', icon: '📄' },
]

// 切换 Tab
function switchTab(index: number) {
  activeTab.value = index
  const session = tabs.value[index]
  if (session) {
    sessionStore.setActiveSession(session.id)
  }
}

// 关闭 Tab
function closeTab(index: number, event: Event) {
  event.stopPropagation()

  // 只能关闭已完成的会话
  if (tabs.value[index].status === 'running' || tabs.value[index].status === 'pending') {
    return
  }

  // 清理会话资源
  const sessionId = tabs.value[index].id
  sessionStore.cleanupSession(sessionId)

  // 更新 activeTab
  if (activeTab.value >= tabs.value.length - 1 && tabs.value.length > 1) {
    activeTab.value = tabs.value.length - 2
  }
}

// 获取会话内存统计
function getMemoryStats(session: Session) {
  return sessionStore.getSessionMemoryStats(session.id)
}

// 格式化时间
function formatTime(isoString: string): string {
  const date = new Date(isoString)
  const now = new Date()
  const diff = now.getTime() - date.getTime()

  if (diff < 60000) {
    return `${Math.floor(diff / 1000)} 秒前`
  } else if (diff < 3600000) {
    return `${Math.floor(diff / 60000)} 分钟前`
  } else if (diff < 86400000) {
    return `${Math.floor(diff / 3600000)} 小时前`
  } else {
    return date.toLocaleDateString()
  }
}

// 辅助函数：获取状态颜色
function getStatusColor(status: string) {
  switch (status) {
    case 'running': return 'text-blue-400 bg-blue-400/10'
    case 'completed': return 'text-green-400 bg-green-400/10'
    case 'error': return 'text-red-400 bg-red-400/10'
    case 'cancelled': return 'text-gray-400 bg-gray-400/10'
    default: return 'text-gray-400 bg-gray-400/10'
  }
}

// 辅助函数：获取状态图标
function getStatusIcon(status: string) {
  switch (status) {
    case 'running': return '⚡'
    case 'completed': return '✅'
    case 'error': return '❌'
    case 'cancelled': return '🚫'
    default: return '⏳'
  }
}

// 渲染内容组件
const RenderComponents = {
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
  },
  code: {
    name: '代码渲染器',
    render: (chunks: DataChunk[]) => {
      const code = chunks.map(c => typeof c.data === 'string' ? c.data : JSON.stringify(c.data, null, 2)).join('\n')
      return `<pre class="code-block"><code>${escapeHtml(code)}</code></pre>`
    }
  },
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
  },
  json: {
    name: 'JSON 渲染器',
    render: (chunks: DataChunk[]) => {
      const data = chunks.map(c => c.data)
      return `<pre class="json-block">${escapeHtml(JSON.stringify(data, null, 2))}</pre>`
    }
  },
  markdown: {
    name: 'Markdown 渲染器',
    render: (chunks: DataChunk[]) => {
      const text = chunks.map(c => typeof c.data === 'string' ? c.data : JSON.stringify(c.data, null, 2)).join('\n')
      return `<div class="markdown-block">${escapeHtml(text).replace(/\n/g, '<br>')}</div>`
    }
  },
}

// HTML 转义
function escapeHtml(text: string): string {
  const div = document.createElement('div')
  div.textContent = text
  return div.innerHTML
}

// 渲染当前内容
const renderedContent = computed(() => {
  const currentSession = activeTab.value !== -1 ? tabs.value[activeTab.value] : null

  if (!currentSession || currentSession.data_chunks.length === 0) {
    return '<p class="text-gray-500 italic">等待数据...</p>'
  }

  const renderer = RenderComponents[renderMode.value]
  if (!renderer) {
    return '<p class="text-red-500">不支持的渲染模式</p>'
  }

  return renderer.render(currentSession.data_chunks)
})

// 初始化
onMounted(() => {
  // 如果有活跃的 Session,自动切换到对应的 Tab
  if (activeSession.value) {
    const index = tabs.value.findIndex(t => t.id === activeSession.value.id)
    if (index !== -1) {
      activeTab.value = index
    }
  }
})
</script>

<template>
  <div class="flex flex-col h-full" style="background-color: var(--bg-primary)">
    <!-- Tab 栏 -->
    <Transition
      enter-active-class="transition ease-out duration-100"
      enter-from-class="transform opacity-0 -translate-y-1"
      enter-to-class="transform opacity-100 translate-y-0"
      leave-active-class="transition ease-in duration-75"
      leave-from-class="transform opacity-100 translate-y-0"
      leave-to-class="transform opacity-0 -translate-y-1"
    >
      <div v-if="tabs.length > 0" class="flex items-center px-2" style="background-color: var(--bg-secondary); border-bottom: 1px solid var(--border-color)">
        <button
          v-for="(tab, index) in tabs"
          :key="tab.id"
          @click="switchTab(index)"
          class="flex items-center px-4 py-2 text-sm font-medium border-t-2 transition-colors group relative"
          :style="{
            backgroundColor: activeTab === index ? 'var(--bg-primary)' : 'transparent',
            color: activeTab === index ? 'white' : '#9ca3af',
            borderColor: activeTab === index ? '#2563eb' : 'transparent',
            borderRight: index < tabs.length - 1 ? '1px solid var(--border-color)' : 'none',
            borderLeft: 'none'
          }"
        >
          <span class="truncate max-w-xs">{{ tab.skill_name }}</span>
          <span class="ml-2 text-[10px] px-1.5 py-0.5 rounded font-mono"
                :class="getStatusColor(tab.status)">
            {{ getStatusIcon(tab.status) }} {{ tab.status }}
          </span>
          <button
            v-if="tab.status !== 'running' && tab.status !== 'pending'"
            @click.stop="closeTab(index, $event)"
            class="ml-2 text-gray-500 hover:text-white transition-colors opacity-0 group-hover:opacity-100"
          >
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </button>
      </div>
    </Transition>

    <!-- 内容区 -->
    <div class="flex-1 overflow-auto p-6 custom-scrollbar">
      <!-- 空状态 -->
      <div v-if="tabs.length === 0" class="h-full flex flex-col items-center justify-center text-gray-500">
        <svg class="w-16 h-16 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <p class="text-lg mb-2">暂无活动会话</p>
        <p class="text-sm">在上方输入框中输入指令开始使用</p>
      </div>

      <!-- 会话内容 -->
      <div v-else class="max-w-6xl mx-auto">
        <div class="rounded-lg p-6" style="background-color: var(--bg-secondary)">
          <!-- 头部信息 -->
          <div class="flex items-start justify-between mb-6 pb-4 border-b border-white/10">
            <div class="flex-1">
              <h2 class="text-xl font-bold text-white mb-2 flex items-center">
                {{ activeSession?.skill_name }}
                <span class="ml-3 text-xs px-2 py-1 rounded-full font-mono"
                      :class="getStatusColor(activeSession?.status || 'pending')">
                  {{ getStatusIcon(activeSession?.status || 'pending') }} {{ activeSession?.status }}
                </span>
              </h2>
              <div class="space-y-1 text-sm text-gray-400">
                <p>会话 ID: <span class="font-mono">{{ activeSession?.id }}</span></p>
                <p>输入: <span class="text-gray-300">{{ activeSession?.input }}</span></p>
                <div v-if="activeSession?.progress" class="flex items-center space-x-2">
                  <span>进度: {{ activeSession.progress.current }} / {{ activeSession.progress.total }}</span>
                  <div class="flex-1 max-w-xs h-2 bg-gray-700 rounded-full overflow-hidden">
                    <div class="h-full bg-indigo-500 transition-all"
                         :style="{ width: activeSession.progress.percentage + '%' }"></div>
                  </div>
                  <span class="text-xs font-mono">{{ activeSession.progress.percentage }}%</span>
                </div>
              </div>
            </div>

            <!-- 渲染模式选择 -->
            <div class="flex items-center space-x-2">
              <label class="text-xs text-gray-400">渲染模式:</label>
              <select v-model="renderMode" class="px-2 py-1 text-sm bg-black/30 border border-white/10 rounded text-white focus:outline-none focus:border-indigo-500">
                <option v-for="mode in renderModes" :key="mode.value" :value="mode.value">
                  {{ mode.icon }} {{ mode.label }}
                </option>
              </select>
            </div>
          </div>

          <!-- 内存使用统计 -->
          <Transition
            enter-active-class="transition ease-out duration-100"
            enter-from-class="transform opacity-0 -translate-y-1"
            enter-to-class="transform opacity-100 translate-y-0"
            leave-active-class="transition ease-in duration-75"
            leave-from-class="transform opacity-100 translate-y-0"
            leave-to-class="transform opacity-0 -translate-y-1"
          >
            <div v-if="activeSession" class="mb-4 px-3 py-2 bg-white/5 rounded-lg text-xs">
              <div class="flex items-center justify-between">
                <span class="text-gray-400">数据块: <span class="text-white">{{ activeSession.data_chunks.length }}</span></span>
                <span class="text-gray-400">内存: <span class="text-white">{{ getMemoryStats(activeSession)?.memoryUsageMB.toFixed(2) || '0' }} MB</span></span>
                <span v-if="getMemoryStats(activeSession)?.isNearLimit" class="text-yellow-400">⚠️ 内存接近限制</span>
              </div>
            </div>
          </Transition>

          <!-- 渲染区域 -->
          <div class="rounded-lg p-4 overflow-auto custom-scrollbar" style="background-color: var(--bg-primary); max-height: 600px;">
            <div v-if="activeSession?.content_blocks && activeSession.content_blocks.length > 0" class="content-blocks space-y-4">
                 <div v-for="(block, index) in activeSession.content_blocks" :key="index">
                     <!-- Text -->
                     <div v-if="block.type === 'text'" class="text-block text-white whitespace-pre-wrap font-mono text-sm">{{ block.text }}</div>
                     
                     <!-- Thinking -->
                     <div v-if="block.type === 'thinking'" class="thinking-block">
                        <details class="bg-white/5 rounded p-2">
                           <summary class="text-gray-400 cursor-pointer text-xs hover:text-white transition-colors">Thinking Process</summary>
                           <pre class="text-gray-400 text-xs mt-2 whitespace-pre-wrap font-mono pl-4 border-l-2 border-gray-600">{{ block.thinking }}</pre>
                        </details>
                     </div>
                     
                     <!-- Tool Use -->
                     <div v-if="block.type === 'tooluse'" class="tool-use-block bg-blue-500/10 border border-blue-500/20 rounded p-3">
                        <div class="text-blue-400 text-xs font-bold mb-2 flex items-center">
                            <span class="mr-2">🔧</span> Tool Call: {{ block.name }}
                        </div>
                        <pre class="text-gray-300 text-xs overflow-x-auto font-mono bg-black/20 p-2 rounded">{{ JSON.stringify(block.input, null, 2) }}</pre>
                     </div>
                     
                     <!-- Tool Result -->
                     <div v-if="block.type === 'toolresult'" class="tool-result-block bg-green-500/10 border border-green-500/20 rounded p-3">
                         <div class="text-green-400 text-xs font-bold mb-2 flex items-center">
                            <span class="mr-2">✅</span> Tool Result
                            <span v-if="block.is_error" class="ml-2 text-red-400 bg-red-900/20 px-1 rounded">Error</span>
                         </div>
                         <pre class="text-gray-300 text-xs overflow-x-auto font-mono bg-black/20 p-2 rounded">{{ block.content }}</pre>
                     </div>
                 </div>
            </div>
            <div v-else v-html="renderedContent" class="rendered-content"></div>
          </div>

          <!-- 错误信息 -->
          <Transition
            enter-active-class="transition ease-out duration-100"
            enter-from-class="transform opacity-0 -translate-y-1"
            enter-to-class="transform opacity-100 translate-y-0"
            leave-active-class="transition ease-in duration-75"
            leave-from-class="transform opacity-100 translate-y-0"
            leave-to-class="transform opacity-0 -translate-y-1"
          >
            <div v-if="activeSession?.error" class="mt-4 p-4 bg-red-500/10 border border-red-500/30 rounded-lg">
              <div class="flex items-start">
                <svg class="w-5 h-5 text-red-400 mr-3 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <div class="flex-1">
                  <p class="text-red-400 font-semibold mb-1">{{ activeSession.error.code }}: {{ activeSession.error.message }}</p>
                  <p class="text-gray-400 text-sm">建议: {{ activeSession.error.suggestion }}</p>
                </div>
              </div>
            </div>
          </Transition>

          <!-- 摘要信息 -->
          <Transition
            enter-active-class="transition ease-out duration-100"
            enter-from-class="transform opacity-0 -translate-y-1"
            enter-to-class="transform opacity-100 translate-y-0"
            leave-active-class="transition ease-in duration-75"
            leave-from-class="transform opacity-100 translate-y-0"
            leave-to-class="transform opacity-0 -translate-y-1"
          >
            <div v-if="activeSession?.summary" class="mt-4 p-4 bg-green-500/10 border border-green-500/30 rounded-lg">
              <p class="text-green-400 text-sm">{{ activeSession.summary }}</p>
            </div>
          </Transition>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #424242;
  border-radius: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}

.rendered-content :deep(.log-entry) {
  padding: 0.5rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}
.rendered-content :deep(.log-time) {
  color: #9ca3af;
  margin-right: 0.5rem;
  font-size: 0.75rem;
}
.rendered-content :deep(.code-block) {
  background: #1a1a1a;
  padding: 1rem;
  border-radius: 0.5rem;
  font-family: 'Courier New', monospace;
  white-space: pre-wrap;
  word-break: break-all;
}
.rendered-content :deep(.data-table) {
  width: 100%;
  border-collapse: collapse;
}
.rendered-content :deep(.data-table th),
.rendered-content :deep(.data-table td) {
  padding: 0.5rem;
  text-align: left;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}
.rendered-content :deep(.data-table th) {
  background: rgba(255, 255, 255, 0.05);
  font-weight: 600;
}
.rendered-content :deep(.json-block) {
  background: #1a1a1a;
  padding: 1rem;
  border-radius: 0.5rem;
  font-family: 'Courier New', monospace;
  white-space: pre-wrap;
  word-break: break-all;
}
.rendered-content :deep(.markdown-block) {
  line-height: 1.6;
}
</style>
