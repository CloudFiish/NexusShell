<script setup lang="ts">
import { ref } from 'vue'

const activeItem = ref('home')

const menuItems = [
  { id: 'home', icon: 'M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6', label: 'Home' },
  { id: 'skills', icon: 'M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z', label: 'Skills' },
  { id: 'history', icon: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z', label: 'History' },
  { id: 'settings', icon: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z M15 12a3 3 0 11-6 0 3 3 0 016 0z', label: 'Settings' },
]

const emit = defineEmits(['navigate'])

function selectItem(id: string) {
  activeItem.value = id
  emit('navigate', id)
}
</script>

<template>
  <div class="w-16 h-full flex flex-col items-center py-6 glass-panel border-r-0 rounded-r-2xl mr-4 my-4 ml-4">
    <div class="mb-8">
      <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center shadow-lg shadow-indigo-500/30">
        <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
        </svg>
      </div>
    </div>

    <div class="flex-1 flex flex-col space-y-6 w-full px-2">
      <button
        v-for="item in menuItems"
        :key="item.id"
        @click="selectItem(item.id)"
        class="group relative w-full aspect-square rounded-xl flex items-center justify-center transition-all duration-300"
        :class="activeItem === item.id ? 'bg-white/10 text-white shadow-inner' : 'text-gray-400 hover:text-white hover:bg-white/5'"
      >
        <svg class="w-6 h-6 transition-transform group-hover:scale-110" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" :d="item.icon" />
        </svg>
        
        <!-- Tooltip -->
        <div class="absolute left-1/2 -translate-x-1/2 bottom-full mb-2 px-2 py-1 bg-gray-800 text-xs text-white rounded opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap z-50 shadow-lg">
          {{ item.label }}
        </div>

        <!-- Active Indicator -->
        <div v-if="activeItem === item.id" class="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-6 bg-indigo-500 rounded-r-full"></div>
      </button>
    </div>

    <div class="mt-auto">
      <button class="w-10 h-10 rounded-full overflow-hidden border-2 border-white/20 hover:border-white/50 transition-colors">
        <img src="https://api.dicebear.com/7.x/avataaars/svg?seed=Felix" alt="User" />
      </button>
    </div>
  </div>
</template>

<style scoped>
/* Glassmorphism handled by global .glass-panel class */
</style>