<script setup lang="ts">
import { computed, ref } from 'vue'
import { useAgentStore } from '@/stores/agent'
import { useCommunicationDiagnostics } from '@/composables/useCommunicationDiagnostics'

const agentStore = useAgentStore()
const { communicationStatus, runFullDiagnostics, isRunningDiagnostics } = useCommunicationDiagnostics()

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
const memoryUsage = ref('42MB')

// 显示延迟（使用诊断工具的延迟数据）
const displayLatency = computed(() => {
  if (communicationStatus.value.latency) {
    return `${communicationStatus.value.latency}ms`
  }
  return '--'
})

// 显示事件计数
const displayEventCount = computed(() => {
  if (communicationStatus.value.eventCount > 0) {
    return `${communicationStatus.value.eventCount} events`
  }
  return ''
})

async function handleStatusClick() {
  if (agentStore.status === 'idle') {
    await agentStore.startAgent()
  } else if (agentStore.status === 'running') {
    // 运行快速诊断
    await runFullDiagnostics()
  }
}
</script>

<template>
  <div class="status-bar flex items-center justify-between px-3 text-xs select-none">
    <div class="flex items-center space-x-4">
      <!-- Agent 状态指示器 -->
      <div class="flex items-center space-x-2 item-hover px-2 py-0.5 rounded cursor-pointer"
           @click="handleStatusClick"
           :title="agentStore.status === 'running' ? '点击运行通信诊断' : '点击启动 Agent'">
        <span class="w-2 h-2 rounded-full transition-colors duration-300" :class="statusColor"></span>
        <span class="text-gray-300 font-medium">{{ currentAgent }}</span>
        <span v-if="isRunningDiagnostics" class="text-yellow-400 animate-pulse">●</span>
      </div>

      <div class="flex items-center space-x-2 text-gray-500">
        <span>STATUS:</span>
        <span class="text-gray-300">{{ agentStatus }}</span>
        <span v-if="agentStore.error" class="text-red-400 ml-2" :title="agentStore.error">(Error)</span>
      </div>

      <!-- 通信状态指示 -->
      <div v-if="communicationStatus.agentRunning" class="flex items-center space-x-2 text-gray-500">
        <span class="w-1.5 h-1.5 rounded-full"
              :class="communicationStatus.eventListenerActive ? 'bg-green-400' : 'bg-yellow-400'"></span>
        <span class="text-gray-400">{{ displayEventCount }}</span>
      </div>
    </div>

    <div class="flex items-center space-x-4 text-gray-500">
      <div class="flex items-center space-x-1" :title="communicationStatus.lastPingTime ? '最后 ping: ' + new Date(communicationStatus.lastPingTime).toLocaleTimeString() : '未测试'">
        <span>LATENCY:</span>
        <span class="text-gray-300 font-mono">{{ displayLatency }}</span>
      </div>
      <div class="flex items-center space-x-1">
        <span>MEM:</span>
        <span class="text-gray-300 font-mono">{{ memoryUsage }}</span>
      </div>
      <div class="flex items-center space-x-1 item-hover px-2 py-0.5 rounded cursor-pointer"
           @click="runFullDiagnostics"
           title="运行通信诊断">
        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
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
