<script setup lang="ts">
import { computed } from 'vue'
import { useSessionStore } from '@/stores/session'
import { useAgent } from '@/composables/useAgent'
import SkillWidget from './SkillWidget.vue'

const sessionStore = useSessionStore()
const agent = useAgent()

// 获取所有会话，反转顺序使最新的显示在前面
const sessions = computed(() => [...sessionStore.sessions].reverse())

function handleClose(sessionId: string) {
  sessionStore.cleanupSession(sessionId)
  sessionStore.sessionsMap.delete(sessionId)
}

async function handleSubmit(payload: { sessionId: string, input: string }) {
  console.log('[WorkspaceGrid] Submit to session:', payload)
  
  try {
    await agent.sendInputToSession({
      sessionId: payload.sessionId,
      input: payload.input
    })
    console.log('[WorkspaceGrid] 输入发送成功')
  } catch (error) {
    console.error('[WorkspaceGrid] 发送输入失败:', error)
  }
}
</script>

<template>
  <div class="h-full w-full overflow-y-auto custom-scrollbar p-6">
    <!-- 空状态 -->
    <div v-if="sessions.length === 0" class="h-full flex flex-col items-center justify-center text-gray-500">
      <svg class="w-16 h-16 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
      </svg>
      <p class="text-lg">暂无活动会话</p>
      <p class="text-sm">在上方输入框中输入指令开始</p>
    </div>

    <!-- 网格布局 -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 xl:grid-cols-3 gap-6 pb-20">
      <SkillWidget
        v-for="session in sessions"
        :key="session.id"
        :session="session"
        @close="handleClose"
        @submit="handleSubmit"
      />
    </div>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 8px;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
}
</style>
