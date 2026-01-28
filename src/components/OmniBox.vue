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
</script>

<template>
  <div class="relative w-full max-w-2xl">
    <div class="relative">
      <input
        v-model="input"
        type="text"
        :placeholder="placeholderText"
        @focus="handleFocus"
        @blur="handleBlur"
        @keydown="handleKeyDown"
        class="w-full px-4 py-2 pl-10 pr-4 bg-gray-700 text-white placeholder-gray-400 rounded-lg border border-gray-600 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 transition-all"
      />
      <svg
        class="absolute left-3 top-1/2 transform -translate-y-1/2 w-5 h-5 text-gray-400"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
        />
      </svg>
      <kbd
        class="absolute right-3 top-1/2 transform -translate-y-1/2 px-2 py-0.5 text-xs text-gray-500 bg-gray-600 rounded"
      >
        Enter
      </kbd>
    </div>

    <!-- 建议列表 -->
    <div
      v-if="isFocused && suggestions.length > 0"
      class="absolute z-10 w-full mt-2 bg-gray-800 border border-gray-700 rounded-lg shadow-lg overflow-hidden"
    >
      <button
        v-for="(suggestion, index) in suggestions"
        :key="index"
        @click="input = suggestion; handleSubmit()"
        class="w-full text-left px-4 py-2 hover:bg-gray-700 transition-colors"
      >
        {{ suggestion }}
      </button>
    </div>
  </div>
</template>

<style scoped>
/* 组件特定样式 */
</style>
