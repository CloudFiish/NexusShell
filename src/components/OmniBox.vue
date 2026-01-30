<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useAgent } from '@/composables/useAgent'
import { useAgentStore } from '@/stores/agent'
import { useSessionStore } from '@/stores/session'

const { executeSkill, isReady, isLoading } = useAgent()
const agentStore = useAgentStore()
const sessionStore = useSessionStore()

const input = ref('')
const isFocused = ref(false)
const suggestions = ref<Array<{ name: string; description: string }>>([])
const isLoadingSkill = ref(false)
const errorMessage = ref<string | null>(null)

const placeholderText = computed(() => {
  if (!isReady.value) {
    return 'Agent 未运行，请先启动 Agent...'
  }
  if (isLoading.value || isLoadingSkill.value) {
    return '处理中...'
  }
  return isFocused.value ? '输入自然语言指令...' : '例如: 审查代码安全性 / 生成单元测试'
})

// 输入验证
const isValidInput = computed(() => {
  return input.value.trim().length > 0 && input.value.trim().length <= 1000
})

// 监听输入变化，生成建议 - 已禁用，直接发送 Prompt
watch(input, async (newValue) => {
  // Suggestions logic removed for CodeBuddy SDK integration
  suggestions.value = []
})

async function handleSubmit() {
  if (!isValidInput.value) {
    if (input.value.trim().length === 0) {
      return
    }
    errorMessage.value = '输入内容过长，请缩短后重试'
    setTimeout(() => {
      errorMessage.value = null
    }, 3000)
    return
  }

  if (!isReady.value) {
    errorMessage.value = 'Agent 未运行，请先启动 Agent'
    setTimeout(() => {
      errorMessage.value = null
    }, 3000)
    return
  }

  const command = input.value.trim()
  console.log('发送 Prompt:', command)

  try {
    isLoadingSkill.value = true
    errorMessage.value = null

    // 直接发送 prompt 给 assistant, 由 Assistant 决定是否使用工具
    const sessionId = await executeSkill('assistant', { input: command })

    console.log('Session ID:', sessionId)

    // 清空输入
    input.value = ''
    suggestions.value = []

    // 自动切换到新的 Session
    sessionStore.setActiveSession(sessionId)

  } catch (error) {
    console.error('执行指令失败:', error)
    errorMessage.value = error instanceof Error ? error.message : '执行指令失败'
    setTimeout(() => {
      errorMessage.value = null
    }, 5000)
  } finally {
    isLoadingSkill.value = false
  }
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
  } else if (event.key === 'Escape') {
    isFocused.value = false
    suggestions.value = []
  }
}

const inputRef = ref<HTMLInputElement | null>(null)

function focus() {
  inputRef.value?.focus()
}

// 监听键盘快捷键 Ctrl+K
onMounted(() => {
  const handleShortcut = (e: KeyboardEvent) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
      e.preventDefault()
      focus()
    }
  }

  window.addEventListener('keydown', handleShortcut)

  // 组件卸载时移除监听器
  return () => {
    window.removeEventListener('keydown', handleShortcut)
  }
})

defineExpose({
  focus
})
</script>

<template>
  <div class="relative w-full max-w-3xl group">
    <div class="relative flex items-center transition-all duration-300">
      <!-- 状态指示器 -->
      <div class="absolute left-0 pl-4 flex items-center pointer-events-none transition-colors"
           :class="isReady ? 'text-gray-400 group-focus-within:text-indigo-400' : 'text-red-400'">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
        <span class="ml-2 font-mono text-gray-500 font-bold select-none">></span>
      </div>

      <!-- 加载指示器 -->
      <div v-if="isLoadingSkill" class="absolute left-10 pointer-events-none">
        <svg class="animate-spin h-4 w-4 text-indigo-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
      </div>

      <!-- 输入框 -->
      <input
        ref="inputRef"
        v-model="input"
        type="text"
        :placeholder="placeholderText"
        :disabled="!isReady"
        @focus="handleFocus"
        @blur="handleBlur"
        @keydown="handleKeyDown"
        class="w-full h-12 pl-12 pr-20 bg-black/20 backdrop-blur-md border border-white/10 rounded-2xl text-sm text-gray-200 placeholder-gray-500 focus:outline-none focus:border-indigo-500/50 focus:bg-black/40 transition-all font-mono shadow-lg disabled:opacity-50 disabled:cursor-not-allowed"
        :class="{ 'border-red-500/50': errorMessage }"
        spellcheck="false"
        autocomplete="off"
      />

      <!-- 右侧提示 -->
      <div class="absolute right-3 flex items-center space-x-2 pointer-events-none">
        <span v-if="!isFocused && isReady" class="text-[10px] text-gray-500 font-mono border border-white/10 rounded px-1.5 py-0.5 bg-white/5">Ctrl+K</span>
        <span v-else-if="isFocused && isValidInput" class="text-[10px] text-indigo-400 font-mono">ENTER</span>
        <span v-else-if="!isReady" class="text-[10px] text-red-400 font-mono">未就绪</span>
      </div>
    </div>

    <!-- 错误消息 -->
    <Transition
      enter-active-class="transition ease-out duration-100"
      enter-from-class="transform opacity-0 translate-y-2"
      enter-to-class="transform opacity-100 translate-y-0"
      leave-active-class="transition ease-in duration-75"
      leave-from-class="transform opacity-100 translate-y-0"
      leave-to-class="transform opacity-0 -translate-y-2"
    >
      <div
        v-if="errorMessage"
        class="absolute z-50 w-full mt-2 px-4 py-2 bg-red-500/10 backdrop-blur-md border border-red-500/30 rounded-lg text-red-400 text-sm"
      >
        <div class="flex items-center">
          <svg class="w-4 h-4 mr-2 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <span>{{ errorMessage }}</span>
        </div>
      </div>
    </Transition>

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
          @click="input = suggestion.name; handleSubmit()"
          class="w-full text-left px-4 py-3 text-sm hover:bg-indigo-500/20 hover:text-white transition-colors flex flex-col group font-mono border-b border-white/5 last:border-0"
        >
          <div class="flex items-center">
            <span class="w-4 h-4 mr-3 flex items-center justify-center text-gray-500 group-hover:text-indigo-400 flex-shrink-0">
              <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
            </span>
            <span class="text-gray-300 group-hover:text-white font-semibold">{{ suggestion.name }}</span>
          </div>
          <div class="ml-7 mt-1 text-xs text-gray-500 group-hover:text-gray-400 truncate">
            {{ suggestion.description }}
          </div>
        </button>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
/* No extra styles needed, Tailwind covers it */
</style>
