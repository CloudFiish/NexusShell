<script setup lang="ts">
import { ref, onMounted, onUnmounted, provide } from 'vue'
import SkillDock from './components/SkillDock.vue'
import SemanticCanvas from './components/SemanticCanvas.vue'
import OmniBox from './components/OmniBox.vue'

const loading = ref(true)
const error = ref<string | null>(null)
const omniBoxRef = ref()

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
  <div class="flex flex-col h-screen bg-gray-900 text-gray-100">
    <!-- 顶部导航栏 -->
    <header class="flex items-center justify-between px-6 py-3 bg-gray-800 border-b border-gray-700">
      <div class="flex items-center space-x-3">
        <div class="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center">
          <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
          </svg>
        </div>
        <h1 class="text-xl font-bold text-white">Nexus Shell</h1>
      </div>
      <OmniBox ref="omniBoxRef" />
    </header>

    <!-- 主内容区 -->
    <div v-if="loading" class="flex items-center justify-center flex-1">
      <div class="text-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
        <p class="mt-4 text-gray-400">正在加载...</p>
      </div>
    </div>

    <div v-else-if="error" class="flex items-center justify-center flex-1">
      <div class="text-center">
        <div class="text-red-500 text-6xl mb-4">⚠️</div>
        <h2 class="text-xl font-semibold text-red-400 mb-2">应用启动失败</h2>
        <p class="text-gray-400">{{ error }}</p>
      </div>
    </div>

    <div v-else class="flex flex-1 overflow-hidden">
      <!-- 左侧边栏 -->
      <aside class="w-64 bg-gray-800 border-r border-gray-700 flex flex-col">
        <SkillDock />
      </aside>

      <!-- 主画布 -->
      <main class="flex-1 overflow-hidden">
        <SemanticCanvas />
      </main>
    </div>

    <!-- Toast Notification -->
    <Transition
      enter-active-class="transition ease-out duration-300"
      enter-from-class="transform translate-y-2 opacity-0"
      enter-to-class="transform translate-y-0 opacity-100"
      leave-active-class="transition ease-in duration-200"
      leave-from-class="transform translate-y-0 opacity-100"
      leave-to-class="transform translate-y-2 opacity-0"
    >
      <div v-if="toast.show" class="fixed bottom-4 right-4 px-4 py-2 rounded-lg shadow-lg text-white text-sm flex items-center space-x-2 z-50"
        :class="{
          'bg-blue-600': toast.type === 'info',
          'bg-red-600': toast.type === 'error',
          'bg-green-600': toast.type === 'success'
        }"
      >
        <span>{{ toast.message }}</span>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
/* 组件特定样式 */
</style>
