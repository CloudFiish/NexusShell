<script setup lang="ts">
import { ref, onMounted, onUnmounted, provide } from 'vue'
import SideToolbar from './components/SideToolbar.vue'
import WorkspaceGrid from './components/WorkspaceGrid.vue'
import OmniBox from './components/OmniBox.vue'
import TitleBar from './components/TitleBar.vue'
import StatusBar from './components/StatusBar.vue'
import CommunicationDiagnostics from './components/CommunicationDiagnostics.vue'
import SkillDock from './components/SkillDock.vue'

import { useAgentStore } from '@/stores/agent'
import { useSessionStore } from '@/stores/session'

const loading = ref(true)
const error = ref<string | null>(null)
const omniBoxRef = ref()
const agentStore = useAgentStore()

// 提前初始化 session store，确保事件监听器在应用启动时就设置
const sessionStore = useSessionStore()
console.log('[App] Session Store 已初始化:', !!sessionStore)

// 通信诊断面板显示状态
const showDiagnostics = ref(false)

// 当前激活的视图
const currentView = ref('home')

// 处理工具栏导航
function handleNavigate(viewId: string) {
  console.log('[App] 切换到视图:', viewId)
  currentView.value = viewId
}

// Toast Logic
const toast = ref({ show: false, message: '', type: 'info' as 'info' | 'error' | 'success' })
let toastTimer: any = null

const showToast = (message: string, type: 'info' | 'error' | 'success' = 'info') => {
  if (toastTimer) clearTimeout(toastTimer)
  toast.value = { show: true, message, type }
  toastTimer = setTimeout(() => {
    toast.value.show = false
  }, 3000)
}

provide('showToast', showToast)

// Global Shortcuts
const handleGlobalKeydown = (e: KeyboardEvent) => {
  // Ctrl/Cmd + K to focus OmniBox
  if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
    e.preventDefault()
    omniBoxRef.value?.focus()
  }

  // Ctrl/Cmd + Shift + D to toggle diagnostics panel
  if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'D') {
    e.preventDefault()
    showDiagnostics.value = !showDiagnostics.value
    console.log(`[App] 通信诊断面板: ${showDiagnostics.value ? '显示' : '隐藏'}`)
  }
}

onMounted(async () => {
  try {
    // 初始化应用
    loading.value = true

    // 检查 Agent 状态
    const isRunning = await agentStore.checkStatus()
    if (!isRunning) {
        console.log('Agent 未运行，尝试启动...')
        await agentStore.startAgent()
    }

    loading.value = false
    window.addEventListener('keydown', handleGlobalKeydown)

    // Global Error Handler
    window.addEventListener('unhandledrejection', (event) => {
      showToast(event.reason?.message || '未知错误', 'error')
    })

    // 显示快捷键提示
    console.log('[App] 快捷键: Ctrl+Shift+D 打开/关闭通信诊断面板')
  } catch (err) {
    error.value = err instanceof Error ? err.message : '未知错误'
    console.error('应用初始化失败:', err)
  }
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleGlobalKeydown)
})
</script>

<template>
  <div class="flex flex-col h-screen overflow-hidden bg-[url('https://source.unsplash.com/random/1920x1080/?abstract,dark')] bg-cover bg-center">
    <!-- Backdrop Overlay for Tint -->
    <div class="absolute inset-0 bg-[#0f172a]/90 backdrop-blur-sm pointer-events-none z-0"></div>

    <div class="relative z-10 flex flex-col h-full">
      <TitleBar />

      <div v-if="loading" class="flex items-center justify-center flex-1">
        <div class="text-center">
          <div class="animate-spin rounded-full h-10 w-10 border-b-2 border-indigo-500 mx-auto"></div>
          <p class="mt-6 text-indigo-300 text-sm tracking-widest font-mono">SYSTEM INITIALIZATION...</p>
        </div>
      </div>

      <div v-else-if="error" class="flex items-center justify-center flex-1">
        <div class="text-center glass-panel p-8 rounded-2xl">
          <div class="text-red-500 text-5xl mb-4">⚠️</div>
          <h2 class="text-xl font-bold text-red-400 mb-2">SYSTEM ERROR</h2>
          <p class="text-gray-400 text-sm font-mono">{{ error }}</p>
        </div>
      </div>

      <div v-else class="flex flex-1 overflow-hidden pt-2 pb-2">
        <!-- 左侧工具栏 -->
        <SideToolbar @navigate="handleNavigate" />

        <!-- 主区域 -->
        <div class="flex-1 flex flex-col overflow-hidden mr-4">
          <!-- 顶部指令栏 (Floating) - 只在 home 视图显示 -->
          <div v-if="currentView === 'home'" class="flex-none flex items-center justify-center py-4">
            <OmniBox ref="omniBoxRef" />
          </div>

          <!-- 主内容区 -->
          <main class="flex-1 overflow-hidden relative rounded-2xl border border-white/5 glass-panel bg-white/5">
            <!-- Home 视图 - 会话网格 -->
            <WorkspaceGrid v-if="currentView === 'home'" />
            
            <!-- Skills 视图 -->
            <SkillDock v-else-if="currentView === 'skills'" />
            
            <!-- History 视图 -->
            <div v-else-if="currentView === 'history'" class="h-full flex flex-col items-center justify-center text-gray-500">
              <svg class="w-16 h-16 mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <p class="text-lg">History View</p>
              <p class="text-sm opacity-60">Coming soon...</p>
            </div>
            
            <!-- Settings 视图 -->
            <div v-else-if="currentView === 'settings'" class="h-full flex flex-col items-center justify-center text-gray-500">
              <svg class="w-16 h-16 mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37-2.37a1.724 1.724 0 00-2.572-1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              <p class="text-lg">Settings View</p>
              <p class="text-sm opacity-60">Coming soon...</p>
            </div>
          </main>
        </div>

        <!-- 通信诊断面板 -->
        <Transition
          enter-active-class="transition ease-out duration-300"
          enter-from-class="transform translate-x-full opacity-0"
          enter-to-class="transform translate-x-0 opacity-100"
          leave-active-class="transition ease-in duration-200"
          leave-from-class="transform translate-x-0 opacity-100"
          leave-to-class="transform translate-x-full opacity-0"
        >
          <div
            v-if="showDiagnostics"
            class="fixed right-4 top-20 z-50 w-96 max-h-[80vh] overflow-auto"
          >
            <div class="relative">
              <!-- 关闭按钮 -->
              <button
                @click="showDiagnostics = false"
                class="absolute -top-2 -right-2 w-6 h-6 bg-red-500/20 text-red-400 border border-red-500/30 rounded-full flex items-center justify-center hover:bg-red-500/30 transition-colors z-10"
                title="关闭诊断面板"
              >
                ×
              </button>
              <CommunicationDiagnostics />
            </div>
          </div>
        </Transition>
      </div>

      <!-- 底部状态栏 -->
      <StatusBar class="flex-none bg-black/40 backdrop-blur-md border-t border-white/5" />
    </div>

    <!-- Toast Notification -->
    <Transition
      enter-active-class="transition ease-out duration-300"
      enter-from-class="transform translate-y-2 opacity-0 scale-95"
      enter-to-class="transform translate-y-0 opacity-100 scale-100"
      leave-active-class="transition ease-in duration-200"
      leave-from-class="transform translate-y-0 opacity-100 scale-100"
      leave-to-class="transform translate-y-2 opacity-0 scale-95"
    >
      <div v-if="toast.show" class="fixed bottom-12 right-6 px-6 py-3 rounded-xl shadow-2xl text-white text-sm font-medium flex items-center space-x-3 z-50 glass-panel border-l-4"
        :class="{
          'border-l-blue-500': toast.type === 'info',
          'border-l-red-500': toast.type === 'error',
          'border-l-green-500': toast.type === 'success'
        }"
      >
        <span class="text-lg">
          {{ toast.type === 'success' ? '✓' : toast.type === 'error' ? '!' : 'ℹ' }}
        </span>
        <span>{{ toast.message }}</span>
      </div>
    </Transition>

    <!-- 快捷键提示 -->
    <div class="fixed bottom-12 left-6 text-xs text-gray-500 font-mono opacity-50 hover:opacity-100 transition-opacity">
      <span>Ctrl+Shift+D 诊断面板</span>
    </div>
  </div>
</template>

<style scoped>
/* Scoped styles overrides */
</style>
