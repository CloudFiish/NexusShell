<script setup lang="ts">
import { ref, computed } from 'vue'

const input = ref('')
const isFocused = ref(false)
const suggestions = ref<string[]>([])

const placeholderText = computed(() => {
  return isFocused.value ? '输入自然语言指令...' : '例如: 审查代码安全性 / 生成单元测试'
})

async function handleSubmit() {
  if (!input.value.trim()) return

  const command = input.value.trim()
  console.log('执行指令:', command)

  // TODO: 调用 Tauri 命令执行 Skill
  // const result = await invoke('execute_skill', { skillName: command, input: command })
  
  input.value = ''
  suggestions.value = []
}

function handleFocus() {
  isFocused.value = true
}

function handleBlur() {
  // 延迟隐藏建议，以便点击建议项
  setTimeout(() => {
    isFocused.value = false
  }, 200)
}

function handleKeyDown(event: KeyboardEvent) {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault()
    handleSubmit()
  }
}
const inputRef = ref<HTMLInputElement | null>(null)

function focus() {
  inputRef.value?.focus()
}

defineExpose({
  focus
})
</script>

<template>
  <div class="relative w-full max-w-3xl group">
    <div class="relative flex items-center transition-all duration-300">
      <div class="absolute left-0 pl-4 flex items-center pointer-events-none text-gray-400 group-focus-within:text-indigo-400 transition-colors">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
        <span class="ml-2 font-mono text-gray-500 font-bold select-none">></span>
      </div>
      
      <input
        ref="inputRef"
        v-model="input"
        type="text"
        :placeholder="placeholderText"
        @focus="handleFocus"
        @blur="handleBlur"
        @keydown="handleKeyDown"
        class="w-full h-12 pl-12 pr-20 bg-black/20 backdrop-blur-md border border-white/10 rounded-2xl text-sm text-gray-200 placeholder-gray-500 focus:outline-none focus:border-indigo-500/50 focus:bg-black/40 transition-all font-mono shadow-lg"
        spellcheck="false"
        autocomplete="off"
      />
      
      <div class="absolute right-3 flex items-center space-x-2 pointer-events-none">
        <span v-if="!isFocused" class="text-[10px] text-gray-500 font-mono border border-white/10 rounded px-1.5 py-0.5 bg-white/5">Ctrl+K</span>
        <span v-else class="text-[10px] text-indigo-400 font-mono">ENTER</span>
      </div>
    </div>

    <!-- 建议列表 -->
    <Transition
      enter-active-class="transition ease-out duration-100"
      enter-from-class="transform opacity-0 -translate-y-2"
      enter-to-class="transform opacity-100 translate-y-0"
      leave-active-class="transition ease-in duration-75"
      leave-from-class="transform opacity-100 translate-y-0"
      leave-to-class="transform opacity-0 -translate-y-2"
    >
      <div
        v-if="isFocused && suggestions.length > 0"
        class="absolute z-50 w-full mt-2 bg-black/80 backdrop-blur-xl border border-white/10 rounded-xl shadow-2xl overflow-hidden"
      >
        <div class="px-3 py-2 text-[10px] font-semibold text-gray-500 uppercase tracking-wider border-b border-white/5 bg-white/5">
          Suggestions
        </div>
        <button
          v-for="(suggestion, index) in suggestions"
          :key="index"
          @click="input = suggestion; handleSubmit()"
          class="w-full text-left px-4 py-3 text-sm text-gray-300 hover:bg-indigo-500/20 hover:text-white transition-colors flex items-center group font-mono"
        >
          <span class="w-4 h-4 mr-3 flex items-center justify-center text-gray-500 group-hover:text-indigo-400">
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
          </span>
          {{ suggestion }}
        </button>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
/* No extra styles needed, Tailwind covers it */
</style>
