<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import type { Session } from '@/stores/session'

const props = defineProps<{
  session: Session
}>()

const emit = defineEmits(['close', 'submit'])

const localInput = ref('')
const isMaximized = ref(false)
const contentRef = ref<HTMLElement | null>(null)

// Debug: Monitor content blocks
watch(() => props.session, (newSession) => {
  console.log(`[SkillWidget] Session ${newSession.id} updated. Blocks: ${newSession.content_blocks?.length || 0}`, newSession.content_blocks);
}, { deep: true, immediate: true })

// Auto-scroll to bottom when new content arrives
watch(() => props.session.content_blocks.length, () => {
  nextTick(() => {
    if (contentRef.value) {
      contentRef.value.scrollTop = contentRef.value.scrollHeight
    }
  })
})

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
      isMaximized ? 'fixed inset-4 z-50 bg-[#1a1a1a]' : 'w-full h-96 min-h-[300px]'
    ]"
  >
    <!-- Header -->
    <div class="h-10 px-4 flex items-center justify-between bg-white/5 border-b border-white/5 select-none flex-shrink-0">
      <div class="flex items-center space-x-2 overflow-hidden">
        <div class="w-2 h-2 rounded-full flex-shrink-0" 
          :class="{
            'bg-green-500 shadow-[0_0_5px_rgba(34,197,94,0.6)]': session.status === 'running',
            'bg-gray-500': session.status === 'completed',
            'bg-red-500': session.status === 'error',
            'bg-yellow-500': session.status === 'pending'
          }"
        ></div>
        <span class="text-sm font-semibold text-white tracking-wide truncate">{{ session.skill_name }}</span>
        <span class="text-xs text-gray-400 font-mono opacity-60 hidden sm:inline">#{{ session.id.slice(0, 4) }}</span>
      </div>
      
      <div class="flex items-center space-x-2 flex-shrink-0">
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
    <div ref="contentRef" class="flex-1 overflow-auto bg-black/20 p-4 custom-scrollbar relative">
      <div v-if="!session.content_blocks || session.content_blocks.length === 0" class="absolute inset-0 flex items-center justify-center text-gray-500 text-sm">
        Waiting for output...
      </div>
      
      <div v-else class="space-y-4">
         <div v-for="(block, index) in session.content_blocks" :key="index" class="animate-fade-in">
             <!-- User Input -->
             <div v-if="block.type === 'user'" class="flex justify-end">
               <div class="max-w-[85%] bg-indigo-500/20 border border-indigo-500/30 rounded-2xl rounded-tr-sm px-4 py-3">
                 <div class="text-xs text-indigo-400 font-medium mb-1 flex items-center">
                   <span class="mr-1">👤</span> You
                 </div>
                 <div class="text-gray-200 text-sm whitespace-pre-wrap leading-relaxed">{{ block.text }}</div>
               </div>
             </div>

             <!-- AI Text -->
             <div v-else-if="block.type === 'text'" class="flex justify-start">
               <div class="max-w-[90%]">
                 <div class="text-xs text-gray-500 font-medium mb-1 flex items-center">
                   <span class="mr-1">🤖</span> Assistant
                 </div>
                 <div class="text-gray-300 text-sm whitespace-pre-wrap font-mono leading-relaxed bg-white/5 rounded-2xl rounded-tl-sm px-4 py-3">{{ block.text }}</div>
               </div>
             </div>
             
             <!-- Thinking -->
             <div v-else-if="block.type === 'thinking'" class="flex justify-start">
               <div class="max-w-[90%]">
                 <div class="text-xs text-gray-500 font-medium mb-1">💭 Thinking</div>
                 <div class="text-xs text-gray-500 italic border-l-2 border-gray-600 pl-3 py-2 bg-white/5 rounded-r">
                   {{ block.thinking }}
                 </div>
               </div>
             </div>
             
             <!-- Tool Use -->
             <div v-else-if="block.type === 'tooluse'" class="flex justify-start">
               <div class="max-w-[90%] w-full">
                 <div class="text-xs text-blue-400 font-medium mb-1 flex items-center">
                   <span class="mr-1">🔧</span> Tool: {{ block.name }}
                 </div>
                 <div class="bg-blue-500/10 border border-blue-500/20 rounded-lg p-3 text-xs">
                   <pre class="text-gray-400 overflow-x-auto custom-scrollbar bg-black/20 p-2 rounded">{{ JSON.stringify(block.input, null, 2) }}</pre>
                 </div>
               </div>
             </div>

             <!-- Tool Result -->
             <div v-else-if="block.type === 'toolresult'" class="flex justify-start">
               <div class="max-w-[90%] w-full">
                 <div class="text-xs text-green-400 font-medium mb-1 flex items-center">
                   <span class="mr-1">✅</span> Result
                   <span v-if="block.is_error" class="ml-2 text-red-400 bg-red-900/20 px-1 rounded">Error</span>
                 </div>
                 <div class="bg-green-500/10 border border-green-500/20 rounded-lg p-3 text-xs">
                   <pre class="text-gray-400 overflow-x-auto custom-scrollbar bg-black/20 p-2 rounded">{{ block.content }}</pre>
                 </div>
               </div>
             </div>
         </div>
         
         <!-- AI Typing Indicator -->
         <div v-if="session.status === 'running'" class="flex justify-start animate-fade-in">
           <div class="max-w-[90%]">
             <div class="text-xs text-gray-500 font-medium mb-1 flex items-center">
               <span class="mr-1">🤖</span> Assistant is typing
             </div>
             <div class="bg-white/5 rounded-2xl rounded-tl-sm px-4 py-3 flex items-center space-x-2">
               <div class="flex space-x-1">
                 <div class="w-2 h-2 bg-indigo-400 rounded-full animate-bounce" style="animation-delay: 0ms"></div>
                 <div class="w-2 h-2 bg-indigo-400 rounded-full animate-bounce" style="animation-delay: 150ms"></div>
                 <div class="w-2 h-2 bg-indigo-400 rounded-full animate-bounce" style="animation-delay: 300ms"></div>
               </div>
               <span class="text-xs text-gray-500 ml-2">thinking...</span>
             </div>
           </div>
         </div>
      </div>
    </div>

    <!-- Footer / Input -->
    <div class="p-3 bg-white/5 border-t border-white/5 flex-shrink-0">
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
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.animation-delay-200 {
  animation-delay: 200ms;
}
.animation-delay-400 {
  animation-delay: 400ms;
}

@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-fade-in {
  animation: fade-in 0.3s ease-out;
}
</style>
