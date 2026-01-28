<script setup lang="ts">
import { ref, onMounted } from 'vue'
import SkillDock from './components/SkillDock.vue'
import SemanticCanvas from './components/SemanticCanvas.vue'
import OmniBox from './components/OmniBox.vue'

const loading = ref(true)
const error = ref<string | null>(null)

onMounted(async () => {
  try {
    // 初始化应用
    loading.value = false
  } catch (err) {
    error.value = err instanceof Error ? err.message : '未知错误'
    console.error('应用初始化失败:', err)
  }
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
      <OmniBox />
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
  </div>
</template>

<style scoped>
/* 组件特定样式 */
</style>
