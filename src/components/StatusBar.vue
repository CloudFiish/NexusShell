<script setup lang="ts">
import { computed, ref } from 'vue'
import { useAgentStore } from '@/stores/agent'

const agentStore = useAgentStore()

const agentStatus = computed(() => {
  switch (agentStore.status) {
    case 'running': return 'Online'
    case 'starting': return 'Starting...'
    case 'stopping': return 'Stopping...'
    case 'error': return 'Error'
    default: return 'Offline'
  }
})

const statusColor = computed(() => {
  switch (agentStore.status) {
    case 'running': return 'bg-green-500 shadow-[0_0_5px_rgba(34,197,94,0.6)]'
    case 'starting': return 'bg-yellow-500 shadow-[0_0_5px_rgba(234,179,8,0.6)]'
    case 'error': return 'bg-red-500 shadow-[0_0_5px_rgba(239,68,68,0.6)]'
    default: return 'bg-gray-500'
  }
})

const currentAgent = ref('CodeBuddy Code')
const latency = ref('12ms')
const memoryUsage = ref('42MB')
</script>

<template>
  <div class="status-bar flex items-center justify-between px-3 text-xs select-none">
    <div class="flex items-center space-x-4">
      <div class="flex items-center space-x-2 item-hover px-2 py-0.5 rounded cursor-pointer"
           @click="agentStore.status === 'idle' ? agentStore.startAgent() : null">
        <span class="w-2 h-2 rounded-full transition-colors duration-300" :class="statusColor"></span>
        <span class="text-gray-300 font-medium">{{ currentAgent }}</span>
      </div>
      
      <div class="flex items-center space-x-2 text-gray-500">
        <span>STATUS:</span>
        <span class="text-gray-300">{{ agentStatus }}</span>
        <span v-if="agentStore.error" class="text-red-400 ml-2" :title="agentStore.error">(Error)</span>
      </div>
    </div>

    <div class="flex items-center space-x-4 text-gray-500">
      <div class="flex items-center space-x-1">
        <span>LATENCY:</span>
        <span class="text-gray-300 font-mono">{{ latency }}</span>
      </div>
      <div class="flex items-center space-x-1">
        <span>MEM:</span>
        <span class="text-gray-300 font-mono">{{ memoryUsage }}</span>
      </div>
      <div class="flex items-center space-x-1 item-hover px-2 py-0.5 rounded cursor-pointer">
        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
        </svg>
      </div>
    </div>
  </div>
</template>

<style scoped>
.status-bar {
  height: 22px;
  background-color: var(--bg-accent, #0078d4); /* Default to accent if variable missing, but typically separate */
  background-color: #007acc; /* VS Code Blue for status bar */
  color: white;
}

.item-hover:hover {
  background-color: rgba(255, 255, 255, 0.12);
}
</style>