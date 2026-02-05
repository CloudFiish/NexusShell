<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useAgentStore, type Skill } from '@/stores/agent'
import { useSessionStore, type Session } from '@/stores/session'

const agentStore = useAgentStore()
const sessionStore = useSessionStore()

const loading = ref(false)
const error = ref<string | null>(null)
const refreshing = ref(false)
const expandedSkills = ref<Set<string>>(new Set())
const expandedSessions = ref<Set<string>>(new Set())

// 获取数据
const skills = computed(() => agentStore.skills)
const sessions = computed(() => sessionStore.sessions)
const activeSessions = computed(() => sessionStore.activeSessions)
const completedSessions = computed(() => sessionStore.completedSessions)
const activeSession = computed(() => sessionStore.activeSession)
const isAgentRunning = computed(() => agentStore.isRunning)
const canStart = computed(() => agentStore.canStart)
const canStop = computed(() => agentStore.canStop)
const skillCount = computed(() => agentStore.skillCount)

// 加载数据
async function loadData() {
  loading.value = true
  error.value = null

  try {
    await Promise.all([
      agentStore.getSkills(),
      sessionStore.getAllSessions()
    ])
  } catch (err) {
    error.value = err instanceof Error ? err.message : '加载数据失败'
    console.error('加载数据失败:', err)
  } finally {
    loading.value = false
  }
}

// 刷新数据
async function refreshData() {
  refreshing.value = true
  try {
    await loadData()
  } finally {
    refreshing.value = false
  }
}

// 切换 Skill 展开状态
function toggleSkill(skillName: string) {
  if (expandedSkills.value.has(skillName)) {
    expandedSkills.value.delete(skillName)
  } else {
    expandedSkills.value.add(skillName)
  }
}

// 切换 Session 展开状态
function toggleSession(sessionId: string) {
  if (expandedSessions.value.has(sessionId)) {
    expandedSessions.value.delete(sessionId)
  } else {
    expandedSessions.value.add(sessionId)
  }
}

// 选择 Session
function selectSession(session: Session) {
  sessionStore.setActiveSession(session.id)
}

// 获取状态颜色
function getStatusColor(status: string): string {
  const colors: Record<string, string> = {
    'pending': 'bg-yellow-500/20 text-yellow-400',
    'running': 'bg-green-500/20 text-green-400',
    'paused': 'bg-blue-500/20 text-blue-400',
    'cancelled': 'bg-gray-500/20 text-gray-400',
    'completed': 'bg-green-500/20 text-green-400',
    'error': 'bg-red-500/20 text-red-400',
  }
  return colors[status] || 'bg-gray-500/20 text-gray-400'
}

// 获取状态图标
function getStatusIcon(status: string): string {
  const icons: Record<string, string> = {
    'pending': '⏳',
    'running': '▶️',
    'paused': '⏸️',
    'cancelled': '🚫',
    'completed': '✅',
    'error': '❌',
  }
  return icons[status] || '⏸️'
}

// 监听 Agent 状态变化
watch(() => agentStore.status, (newStatus) => {
  if (newStatus === 'running') {
    // Agent 启动后加载数据
    loadData()
  }
})

// 监听 Session 变化 (自动更新)
// Session Store 已经通过事件监听自动更新,这里不需要额外处理

onMounted(() => {
  // 如果 Agent 已经在运行,立即加载数据
  if (isAgentRunning.value) {
    loadData()
  }
})
</script>

<template>
  <div class="flex-1 flex flex-col overflow-hidden select-none">
    <!-- Section Header -->
    <div class="px-4 py-2 flex items-center justify-between border-b border-white/10">
      <span class="text-xs font-bold text-gray-400 tracking-widest uppercase">Explorer</span>
      <div class="flex items-center space-x-2">
        <div class="w-2 h-2 rounded-full" :class="isAgentRunning ? 'bg-green-500 animate-pulse' : 'bg-red-500'"></div>
        <span v-if="isAgentRunning" class="text-[10px] text-gray-500 font-mono">{{ skillCount }} Skills</span>
        <button
          @click="refreshData"
          :disabled="refreshing"
          class="p-1 hover:bg-white/10 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          title="刷新"
        >
          <svg class="w-3 h-3" :class="{ 'animate-spin': refreshing }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
        </button>
      </div>
    </div>

    <!-- 错误提示 -->
    <Transition
      enter-active-class="transition ease-out duration-100"
      enter-from-class="transform opacity-0 -translate-y-2"
      enter-to-class="transform opacity-100 translate-y-0"
      leave-active-class="transition ease-in duration-75"
      leave-from-class="transform opacity-100 translate-y-0"
      leave-to-class="transform opacity-0 -translate-y-2"
    >
      <div v-if="error" class="mx-4 mt-2 px-3 py-2 bg-red-500/10 border border-red-500/30 rounded-lg text-red-400 text-xs">
        {{ error }}
      </div>
    </Transition>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto custom-scrollbar">
      <div class="py-2">
        <!-- Skill 列表 -->
        <div class="mb-4">
          <div
            class="px-4 py-1 mb-1 text-xs font-semibold text-blue-400 uppercase tracking-wider flex items-center cursor-pointer hover:text-blue-300 transition-colors"
            @click="expandedSkills.add('all')"
          >
            <span class="mr-1">{{ expandedSkills.has('all') ? '▼' : '▶' }}</span> INSTALLED SKILLS ({{ skills.length }})
          </div>

          <div v-if="loading && skills.length === 0" class="px-8 py-2 text-xs text-gray-500 italic">
            扫描模块中...
          </div>

          <div v-else-if="skills.length === 0" class="px-8 py-2 text-xs text-gray-600">
            未检测到 Skill
          </div>

          <Transition
            enter-active-class="transition ease-out duration-100"
            enter-from-class="transform opacity-0 -translate-y-1"
            enter-to-class="transform opacity-100 translate-y-0"
            leave-active-class="transition ease-in duration-75"
            leave-from-class="transform opacity-100 translate-y-0"
            leave-to-class="transform opacity-0 -translate-y-1"
          >
            <div v-show="expandedSkills.has('all')" class="space-y-0.5">
              <div
                v-for="skill in skills"
                :key="skill.name"
                class="group px-4 py-1.5 flex items-start hover:bg-white/5 cursor-pointer border-l-2 border-transparent hover:border-blue-500 transition-all duration-150"
                :class="{ 'border-blue-500 bg-white/5': activeSession?.skill_name === skill.name }"
              >
                <div class="w-4 h-4 mt-0.5 mr-2 text-gray-400 group-hover:text-blue-400 flex-shrink-0">
                  <svg fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
                  </svg>
                </div>
                <div class="overflow-hidden flex-1 min-w-0">
                  <div class="text-sm text-gray-300 group-hover:text-white font-medium truncate">{{ skill.name }}</div>
                  <div class="text-[10px] text-gray-500 truncate">{{ skill.description }}</div>
                </div>
              </div>
            </div>
          </Transition>
        </div>

        <!-- 活跃会话 -->
        <div class="mb-4 border-t border-white/10 pt-2">
          <div
            class="px-4 py-1 mb-1 text-xs font-semibold text-green-500 uppercase tracking-wider flex items-center cursor-pointer hover:text-green-400 transition-colors"
            @click="expandedSessions.add('active')"
          >
            <span class="mr-1">{{ expandedSessions.has('active') ? '▼' : '▶' }}</span> ACTIVE SESSIONS ({{ activeSessions.length }})
          </div>

          <div v-if="activeSessions.length === 0" class="px-8 py-2 text-xs text-gray-600">
            无活跃进程
          </div>

          <Transition
            enter-active-class="transition ease-out duration-100"
            enter-from-class="transform opacity-0 -translate-y-1"
            enter-to-class="transform opacity-100 translate-y-0"
            leave-active-class="transition ease-in duration-75"
            leave-from-class="transform opacity-100 translate-y-0"
            leave-to-class="transform opacity-0 -translate-y-1"
          >
            <div v-show="expandedSessions.has('active')" class="space-y-0.5">
              <div
                v-for="session in activeSessions"
                :key="session.id"
                class="group px-4 py-1.5 hover:bg-white/5 cursor-pointer border-l-2 transition-all duration-150"
                :class="{
                  'border-green-500 bg-white/5': activeSession?.id === session.id,
                  'border-transparent': activeSession?.id !== session.id
                }"
                @click="selectSession(session)"
              >
                <div class="flex items-center justify-between mb-0.5">
                  <span class="text-sm font-medium text-gray-300 group-hover:text-white truncate flex-1">{{ session.skill_name }}</span>
                  <span class="text-[10px] px-1.5 py-0.5 rounded font-mono flex-shrink-0" :class="getStatusColor(session.status)">
                    {{ getStatusIcon(session.status) }} {{ session.status }}
                  </span>
                </div>
                <div class="text-[10px] text-gray-500 font-mono truncate pl-1 border-l border-gray-600">
                  > {{ session.input }}
                </div>
              </div>
            </div>
          </Transition>
        </div>

        <!-- 已完成会话 -->
        <div class="border-t border-white/10 pt-2">
          <div
            class="px-4 py-1 mb-1 text-xs font-semibold text-gray-500 uppercase tracking-wider flex items-center cursor-pointer hover:text-gray-400 transition-colors"
            @click="expandedSessions.add('completed')"
          >
            <span class="mr-1">{{ expandedSessions.has('completed') ? '▼' : '▶' }}</span> COMPLETED SESSIONS ({{ completedSessions.length }})
          </div>

          <div v-if="completedSessions.length === 0" class="px-8 py-2 text-xs text-gray-600">
            无已完成会话
          </div>

          <Transition
            enter-active-class="transition ease-out duration-100"
            enter-from-class="transform opacity-0 -translate-y-1"
            enter-to-class="transform opacity-100 translate-y-0"
            leave-active-class="transition ease-in duration-75"
            leave-from-class="transform opacity-100 translate-y-0"
            leave-to-class="transform opacity-0 -translate-y-1"
          >
            <div v-show="expandedSessions.has('completed')" class="space-y-0.5">
              <div
                v-for="session in completedSessions.slice(-10).reverse()"
                :key="session.id"
                class="group px-4 py-1.5 hover:bg-white/5 cursor-pointer border-l-2 transition-all duration-150"
                :class="{
                  'border-gray-500 bg-white/5': activeSession?.id === session.id,
                  'border-transparent': activeSession?.id !== session.id
                }"
                @click="selectSession(session)"
              >
                <div class="flex items-center justify-between mb-0.5">
                  <span class="text-sm font-medium text-gray-300 group-hover:text-white truncate flex-1">{{ session.skill_name }}</span>
                  <span class="text-[10px] px-1.5 py-0.5 rounded font-mono flex-shrink-0" :class="getStatusColor(session.status)">
                    {{ getStatusIcon(session.status) }} {{ session.status }}
                  </span>
                </div>
                <div class="text-[10px] text-gray-500 font-mono truncate pl-1 border-l border-gray-600">
                  > {{ session.input }}
                </div>
              </div>

              <div v-if="completedSessions.length > 10" class="px-8 py-2 text-xs text-gray-500 italic">
                + {{ completedSessions.length - 10 }} 个较早的会话
              </div>
            </div>
          </Transition>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #424242;
  border-radius: 2px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
</style>
