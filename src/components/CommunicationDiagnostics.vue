<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { useCommunicationDiagnostics } from '@/composables/useCommunicationDiagnostics'

const {
  diagnosticsResults,
  isRunningDiagnostics,
  communicationStatus,
  runFullDiagnostics,
  startAgentAndWait,
  getStatusSummary,
  clearResults,
  cleanup
} = useCommunicationDiagnostics()

// 组件挂载时自动运行一次诊断
onMounted(() => {
  runFullDiagnostics()
})

// 组件卸载时清理资源
onUnmounted(() => {
  cleanup()
})

function getStatusIcon(success: boolean) {
  return success ? '✅' : '❌'
}

function getStatusClass(success: boolean) {
  return success ? 'text-green-400' : 'text-red-400'
}
</script>

<template>
  <div class="communication-diagnostics bg-black/80 backdrop-blur-xl border border-white/10 rounded-xl p-4 text-sm">
    <!-- 标题栏 -->
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-gray-200 font-semibold flex items-center">
        <svg class="w-4 h-4 mr-2 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
        </svg>
        通信诊断
      </h3>
      <div class="flex items-center space-x-2">
        <span v-if="isRunningDiagnostics" class="text-xs text-yellow-400 animate-pulse">
          诊断中...
        </span>
        <button
          @click="runFullDiagnostics"
          :disabled="isRunningDiagnostics"
          class="px-3 py-1 bg-indigo-500/20 text-indigo-400 border border-indigo-500/30 rounded hover:bg-indigo-500/30 disabled:opacity-50 disabled:cursor-not-allowed text-xs transition-colors"
        >
          重新诊断
        </button>
        <button
          @click="clearResults"
          class="px-3 py-1 bg-gray-500/20 text-gray-400 border border-gray-500/30 rounded hover:bg-gray-500/30 text-xs transition-colors"
        >
          清除
        </button>
      </div>
    </div>

    <!-- 状态摘要 -->
    <div class="mb-4 p-3 bg-white/5 rounded-lg border border-white/5">
      <div class="text-xs text-gray-400 mb-2">状态摘要</div>
      <div class="text-gray-200 font-mono text-xs">
        {{ getStatusSummary() }}
      </div>
    </div>

    <!-- 详细状态 -->
    <div class="grid grid-cols-2 gap-3 mb-4">
      <!-- Agent 状态 -->
      <div class="p-3 bg-white/5 rounded-lg border border-white/5">
        <div class="flex items-center justify-between mb-2">
          <span class="text-xs text-gray-400">Agent</span>
          <span
            class="w-2 h-2 rounded-full"
            :class="communicationStatus.agentRunning ? 'bg-green-500' : 'bg-red-500'"
          ></span>
        </div>
        <div class="text-xs text-gray-300">
          {{ communicationStatus.agentRunning ? '运行中' : '未运行' }}
        </div>
        <div v-if="communicationStatus.agentError" class="text-xs text-red-400 mt-1">
          {{ communicationStatus.agentError }}
        </div>
      </div>

      <!-- 事件系统 -->
      <div class="p-3 bg-white/5 rounded-lg border border-white/5">
        <div class="flex items-center justify-between mb-2">
          <span class="text-xs text-gray-400">事件系统</span>
          <span
            class="w-2 h-2 rounded-full"
            :class="communicationStatus.eventListenerActive ? 'bg-green-500' : 'bg-yellow-500'"
          ></span>
        </div>
        <div class="text-xs text-gray-300">
          {{ communicationStatus.eventCount }} 个事件
        </div>
        <div v-if="communicationStatus.lastEventReceived" class="text-xs text-gray-500 mt-1">
          最后: {{ new Date(communicationStatus.lastEventReceived).toLocaleTimeString() }}
        </div>
      </div>

      <!-- 会话状态 -->
      <div class="p-3 bg-white/5 rounded-lg border border-white/5">
        <div class="flex items-center justify-between mb-2">
          <span class="text-xs text-gray-400">会话</span>
          <span class="text-xs text-gray-500">
            {{ communicationStatus.activeSessions }}/{{ communicationStatus.totalSessions }}
          </span>
        </div>
        <div class="text-xs text-gray-300">
          {{ communicationStatus.activeSessions }} 个活跃
        </div>
      </div>

      <!-- 延迟 -->
      <div class="p-3 bg-white/5 rounded-lg border border-white/5">
        <div class="flex items-center justify-between mb-2">
          <span class="text-xs text-gray-400">延迟</span>
          <span class="text-xs text-gray-500">RTT</span>
        </div>
        <div class="text-xs text-gray-300">
          {{ communicationStatus.latency ? `${communicationStatus.latency}ms` : '未测试' }}
        </div>
        <div v-if="communicationStatus.lastPingTime" class="text-xs text-gray-500 mt-1">
          最后: {{ new Date(communicationStatus.lastPingTime).toLocaleTimeString() }}
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="flex items-center space-x-2 mb-4">
      <button
        v-if="!communicationStatus.agentRunning"
        @click="startAgentAndWait"
        :disabled="isRunningDiagnostics"
        class="flex-1 px-4 py-2 bg-green-500/20 text-green-400 border border-green-500/30 rounded-lg hover:bg-green-500/30 disabled:opacity-50 disabled:cursor-not-allowed text-xs transition-colors flex items-center justify-center"
      >
        <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        启动 Agent
      </button>
    </div>

    <!-- 诊断结果列表 -->
    <div class="border-t border-white/10 pt-4">
      <div class="text-xs text-gray-400 mb-2">诊断日志</div>
      <div class="max-h-48 overflow-y-auto space-y-1">
        <div
          v-for="(result, index) in diagnosticsResults"
          :key="index"
          class="flex items-start space-x-2 text-xs p-2 rounded"
          :class="result.success ? 'bg-green-500/10' : 'bg-red-500/10'"
        >
          <span class="flex-shrink-0">{{ getStatusIcon(result.success) }}</span>
          <div class="flex-1 min-w-0">
            <div class="flex items-center space-x-2">
              <span class="font-mono text-gray-500">[{{ result.stage }}]</span>
              <span :class="getStatusClass(result.success)">{{ result.message }}</span>
            </div>
            <div v-if="result.details" class="mt-1 text-gray-500 text-[10px] font-mono">
              {{ JSON.stringify(result.details, null, 2) }}
            </div>
            <div class="mt-1 text-gray-600 text-[10px]">
              {{ new Date(result.timestamp).toLocaleTimeString() }}
            </div>
          </div>
        </div>

        <div v-if="diagnosticsResults.length === 0" class="text-center text-gray-500 text-xs py-4">
          暂无诊断结果
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.communication-diagnostics {
  max-width: 500px;
}

/* 自定义滚动条 */
::-webkit-scrollbar {
  width: 6px;
}

::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.05);
  border-radius: 3px;
}

::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
