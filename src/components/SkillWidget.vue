<script setup lang="ts">
import { ref, nextTick } from 'vue'

const props = defineProps<{
  session: {
    id: string
    skillName: string
    status: string
    input: string
    type: string // 'log', 'code', 'table', etc.
    data: any
  }
}>()

const emit = defineEmits(['close', 'submit'])

const localInput = ref('')
const isMaximized = ref(false)

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault()
    if (!localInput.value.trim()) return
    emit('submit', { sessionId: props.session.id, input: localInput.value })
    localInput.value = ''
  }
}

function toggleMaximize() {
  isMaximized.value = !isMaximized.value
}
</script>

<template>
  <div 
    class="flex flex-col rounded-2xl overflow-hidden transition-all duration-300 glass-panel border border-white/10"
    :class="[
      isMaximized ? 'fixed inset-4 z-50' : 'w-full h-96 min-h-[300px]'
    ]"
  >
    <!-- Header -->
    <div class="h-10 px-4 flex items-center justify-between bg-white/5 border-b border-white/5 select-none">
      <div class="flex items-center space-x-2">
        <div class="w-2 h-2 rounded-full" 
          :class="{
            'bg-green-500 shadow-[0_0_5px_rgba(34,197,94,0.6)]': session.status === 'running',
            'bg-gray-500': session.status === 'completed',
            'bg-red-500': session.status === 'error'
          }"
        ></div>
        <span class="text-sm font-semibold text-white tracking-wide">{{ session.skillName }}</span>
        <span class="text-xs text-gray-400 font-mono opacity-60">#{{ session.id.slice(0, 4) }}</span>
      </div>
      
      <div class="flex items-center space-x-2">
        <button @click="toggleMaximize" class="p-1.5 text-gray-400 hover:text-white hover:bg-white/10 rounded-lg transition-colors">
          <svg v-if="!isMaximized" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4" /></svg>
          <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
        </button>
        <button @click="$emit('close', session.id)" class="p-1.5 text-gray-400 hover:text-red-400 hover:bg-red-400/10 rounded-lg transition-colors">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
        </button>
      </div>
    </div>

    <!-- Content Area -->
    <div class="flex-1 overflow-auto bg-black/20 relative">
      <div v-if="!session.data" class="absolute inset-0 flex items-center justify-center text-gray-500 text-sm">
        Waiting for output...
      </div>
      
      <!-- Slot for dynamic content renderer -->
      <slot name="content">
        <div class="p-4 text-gray-300 font-mono text-sm whitespace-pre-wrap">
          {{ session.data || 'No data' }}
        </div>
      </slot>
    </div>

    <!-- Footer / Input -->
    <div class="p-3 bg-white/5 border-t border-white/5">
      <div class="relative">
        <input 
          v-model="localInput"
          type="text" 
          placeholder="Send input to this session..." 
          @keydown="handleKeydown"
          class="w-full bg-black/20 border border-white/10 rounded-xl px-4 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all"
        />
        <div class="absolute right-3 top-1/2 -translate-y-1/2 text-[10px] text-gray-500 font-mono">
          ⏎
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Scrollbar within widget */
.flex-1::-webkit-scrollbar {
  width: 4px;
}
.flex-1::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
}
</style>