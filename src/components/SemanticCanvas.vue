<script setup lang="ts">
import { ref, computed } from 'vue'

const activeTab = ref<number>(0)
const tabs = ref<any[]>([])

const activeSession = computed(() => {
  return tabs.value[activeTab.value]
})

function closeTab(index: number) {
  tabs.value.splice(index, 1)
  if (activeTab.value >= tabs.value.length && tabs.value.length > 0) {
    activeTab.value = tabs.value.length - 1
  }
}

function switchTab(index: number) {
  activeTab.value = index
}
</script>

<template>
  <div class="flex flex-col h-full" style="background-color: var(--bg-primary)">
    <!-- Tab 栏 -->
    <div v-if="tabs.length > 0" class="flex items-center px-2" style="background-color: var(--bg-secondary); border-bottom: 1px solid var(--border-color)">
      <button
        v-for="(tab, index) in tabs"
        :key="tab.id"
        @click="switchTab(index)"
        class="flex items-center px-4 py-2 text-sm font-medium border-t-2 transition-colors"
        :style="{
          backgroundColor: activeTab === index ? 'var(--bg-primary)' : 'transparent',
          color: activeTab === index ? 'white' : '#9ca3af',
          borderColor: activeTab === index ? '#2563eb' : 'transparent',
          borderRight: '1px solid var(--border-color)',
          borderLeft: '1px solid var(--border-color)'
        }"
      >
        <span class="truncate">{{ tab.skill_name }}</span>
        <button
          @click.stop="closeTab(index)"
          class="ml-2 text-gray-500 hover:text-white"
        >
          ×
        </button>
      </button>
    </div>

    <!-- 内容区 -->
    <div class="flex-1 overflow-auto p-6">
      <div v-if="tabs.length === 0" class="h-full flex flex-col items-center justify-center text-gray-500">
        <svg class="w-16 h-16 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <p class="text-lg mb-2">暂无活动会话</p>
        <p class="text-sm">在上方输入框中输入指令开始使用</p>
      </div>

      <div v-else class="max-w-4xl mx-auto">
        <div class="rounded-lg p-6" style="background-color: var(--bg-secondary)">
          <h2 class="text-xl font-bold text-white mb-4">{{ activeSession?.skill_name }}</h2>
          
          <!-- 渲染区域 -->
          <div class="text-gray-300">
            <p>会话 ID: {{ activeSession?.id }}</p>
            <p>状态: {{ activeSession?.status }}</p>
            <p>输入: {{ activeSession?.input }}</p>
            
            <!-- TODO: 根据渲染模式选择合适的渲染器 -->
            <div class="mt-4 p-4 rounded-lg font-mono text-sm overflow-auto" style="background-color: var(--bg-primary)">
              <p class="text-gray-500">// 等待数据流...</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 组件特定样式 */
</style>
