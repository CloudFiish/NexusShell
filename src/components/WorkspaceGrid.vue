<script setup lang="ts">
import { ref } from 'vue'
import SkillWidget from './SkillWidget.vue'
import LogRenderer from '../renderers/LogRenderer.vue'
import CodeRenderer from '../renderers/CodeRenderer.vue'

// Mock data for dev - in real app this comes from store/props
const sessions = ref([
  {
    id: 'sess_001',
    skillName: 'Security Scan',
    status: 'running',
    input: 'scan ./src',
    type: 'log',
    data: [{ received_at: new Date().toISOString(), data: 'Scanning directory...' }]
  },
  {
    id: 'sess_002',
    skillName: 'Generate Component',
    status: 'completed',
    input: 'create Button.vue',
    type: 'code',
    data: `<template>\n  <button class="btn">Click me</button>\n</template>`
  }
])

function handleClose(id: string) {
  sessions.value = sessions.value.filter(s => s.id !== id)
}

function handleSubmit({ sessionId, input }: { sessionId: string, input: string }) {
  console.log(`Input for session ${sessionId}: ${input}`)
  // TODO: Send to backend
  const session = sessions.value.find(s => s.id === sessionId)
  if (session && session.type === 'log') {
    session.data.push({ received_at: new Date().toISOString(), data: `> ${input}` })
  }
}
</script>

<template>
  <div class="p-6 h-full overflow-y-auto">
    <div v-if="sessions.length === 0" class="h-full flex flex-col items-center justify-center text-gray-500">
      <div class="w-16 h-16 mb-4 rounded-2xl bg-white/5 flex items-center justify-center">
        <svg class="w-8 h-8 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
        </svg>
      </div>
      <p class="text-lg font-medium text-gray-400">No active widgets</p>
      <p class="text-sm text-gray-600">Use the command bar to launch a skill</p>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6 pb-20">
      <SkillWidget
        v-for="session in sessions"
        :key="session.id"
        :session="session"
        @close="handleClose"
        @submit="handleSubmit"
      >
        <template #content>
          <LogRenderer 
            v-if="session.type === 'log'" 
            :dataChunks="session.data" 
            :autoScroll="true"
            class="h-full"
          />
          <CodeRenderer
            v-else-if="session.type === 'code'"
            :data="session.data"
            language="vue"
            class="h-full"
          />
          <div v-else class="p-4 text-gray-400">
            Unknown renderer type: {{ session.type }}
          </div>
        </template>
      </SkillWidget>
    </div>
  </div>
</template>

<style scoped>
/* Masonry-like layout could be achieved with column-count if needed, 
   but grid is safer for interactive widgets */
</style>