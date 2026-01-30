<script setup lang="ts">
import { useToast } from '@/composables/useToast'

const { toasts, removeToast } = useToast()

function getIcon(type: string): string {
  const icons: Record<string, string> = {
    'success': '✅',
    'error': '❌',
    'warning': '⚠️',
    'info': 'ℹ️',
  }
  return icons[type] || 'ℹ️'
}

function getColors(type: string): { bg: string; border: string; text: string } {
  const colors: Record<string, { bg: string; border: string; text: string }> = {
    'success': {
      bg: 'bg-green-500/10',
      border: 'border-green-500/30',
      text: 'text-green-400'
    },
    'error': {
      bg: 'bg-red-500/10',
      border: 'border-red-500/30',
      text: 'text-red-400'
    },
    'warning': {
      bg: 'bg-yellow-500/10',
      border: 'border-yellow-500/30',
      text: 'text-yellow-400'
    },
    'info': {
      bg: 'bg-blue-500/10',
      border: 'border-blue-500/30',
      text: 'text-blue-400'
    },
  }
  return colors[type] || colors.info
}
</script>

<template>
  <Teleport to="body">
    <div class="fixed top-4 right-4 z-[9999] flex flex-col items-end space-y-2 pointer-events-none">
      <TransitionGroup name="toast">
        <div
          v-for="toast in toasts"
          :key="toast.id"
          class="pointer-events-auto"
        >
          <div
            class="flex items-start p-4 rounded-lg shadow-2xl backdrop-blur-md border min-w-[300px] max-w-md"
            :class="getColors(toast.type)"
          >
            <span class="text-xl mr-3">{{ getIcon(toast.type) }}</span>
            <div class="flex-1 min-w-0">
              <p class="text-sm text-gray-200 break-words">{{ toast.message }}</p>
            </div>
            <button
              @click="removeToast(toast.id)"
              class="ml-3 text-gray-400 hover:text-white transition-colors"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<style scoped>
.toast-enter-active {
  transition: all 0.3s ease-out;
}

.toast-enter-from {
  transform: translateX(100%);
  opacity: 0;
}

.toast-enter-to {
  transform: translateX(0);
  opacity: 1;
}

.toast-leave-active {
  transition: all 0.3s ease-in;
}

.toast-leave-from {
  transform: translateX(0);
  opacity: 1;
}

.toast-leave-to {
  transform: translateX(100%);
  opacity: 0;
}

.toast-move {
  transition: transform 0.3s ease;
}
</style>
