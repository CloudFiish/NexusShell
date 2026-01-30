<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

const appWindow = getCurrentWindow()
const isMaximized = ref(false)

const minimize = () => appWindow.minimize()
const toggleMaximize = async () => {
  await appWindow.toggleMaximize()
  isMaximized.value = await appWindow.isMaximized()
}
const close = () => appWindow.close()

onMounted(async () => {
  isMaximized.value = await appWindow.isMaximized()
  // 监听窗口大小变化以更新状态（可选，视 API 支持情况而定）
  // Tauri v2 可能有更简便的方法，这里先只做基本状态获取
  appWindow.listen('tauri://resize', async () => {
    isMaximized.value = await appWindow.isMaximized()
  })
})
</script>

<template>
  <div data-tauri-drag-region class="h-8 flex justify-between items-center select-none bg-black/20 backdrop-blur-sm border-b border-white/5">
    <!-- 左侧：Logo 和 标题 -->
    <div class="flex items-center pl-3 space-x-2 pointer-events-none">
      <svg class="w-4 h-4 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
      </svg>
      <span class="text-xs text-gray-300 font-medium" style="font-family: 'Segoe UI', sans-serif">Nexus Shell</span>
    </div>

    <!-- 右侧：窗口控制按钮 -->
    <div class="flex h-full">
      <div 
        class="titlebar-button" 
        @click="minimize"
        title="最小化"
      >
        <svg width="10" height="1" viewBox="0 0 10 1" fill="currentColor">
          <path d="M0 0h10v1H0z"/>
        </svg>
      </div>

      <div 
        class="titlebar-button" 
        @click="toggleMaximize"
        :title="isMaximized ? '还原' : '最大化'"
      >
        <svg v-if="!isMaximized" width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1">
          <rect x="0.5" y="0.5" width="9" height="9"/>
        </svg>
        <svg v-else width="10" height="10" viewBox="0 0 10 10" fill="none" stroke="currentColor" stroke-width="1">
          <path d="M2.5 2.5h-2v-2h2v2zM2.5 2.5h5v5h-5v-5z" fill="none"/>
          <path d="M2.5 0.5h7v7" stroke-dasharray="0 2 0"/>
          <rect x="2.5" y="2.5" width="7" height="7" fill="transparent" stroke="currentColor"/>
          <path d="M0.5 7.5v-5h5" stroke="currentColor"/>
        </svg>
      </div>

      <div 
        class="titlebar-button titlebar-close" 
        @click="close"
        title="关闭"
      >
        <svg width="10" height="10" viewBox="0 0 10 10" fill="currentColor">
          <path d="M0 0l10 10m0-10L0 10" stroke="currentColor" stroke-width="1"/>
        </svg>
      </div>
    </div>
  </div>
</template>

<style scoped>
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 46px;
  height: 100%;
  color: #cccccc;
  transition: background-color 0.1s;
  cursor: default;
}

.titlebar-button:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.titlebar-close:hover {
  background-color: #e81123;
  color: white;
}
</style>
