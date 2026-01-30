<script setup lang="ts">
import { ref, onMounted, onUnmounted, provide } from 'vue'
import SideToolbar from './components/SideToolbar.vue'
import WorkspaceGrid from './components/WorkspaceGrid.vue'
import OmniBox from './components/OmniBox.vue'
import TitleBar from './components/TitleBar.vue'
import StatusBar from './components/StatusBar.vue'

import { useAgentStore } from '@/stores/agent'

const loading = ref(true)
const error = ref<string | null>(null)
const omniBoxRef = ref()
const agentStore = useAgentStore()

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
        <SideToolbar />

        <!-- 主区域 -->
        <div class="flex-1 flex flex-col overflow-hidden mr-4">
          <!-- 顶部指令栏 (Floating) -->
          <div class="flex-none flex items-center justify-center py-4">
            <OmniBox ref="omniBoxRef" />
          </div>

          <!-- 窗口网格 -->
          <main class="flex-1 overflow-hidden relative rounded-2xl border border-white/5 glass-panel bg-white/5">
            <WorkspaceGrid />
          </main>
        </div>
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
  </div>
</template>

<style scoped>
/* Scoped styles overrides */
</style>
