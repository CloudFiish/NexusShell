<script setup lang="ts">
import { ref, onMounted } from 'vue'

const skills = ref<any[]>([])
const sessions = ref<any[]>([])
const loading = ref(false)

onMounted(async () => {
  await loadSkills()
  await loadSessions()
})

async function loadSkills() {
  loading.value = true
  try {
    // TODO: 调用 Tauri 命令获取 Skill 列表
    // const result = await invoke('get_skills')
    // skills.value = result
    console.log('加载 Skill 列表...')
  } catch (error) {
    console.error('加载 Skill 列表失败:', error)
  } finally {
    loading.value = false
  }
}

async function loadSessions() {
  try {
    // TODO: 调用 Tauri 命令获取会话列表
    // const result = await invoke('get_sessions')
    // sessions.value = result
    console.log('加载会话列表...')
  } catch (error) {
    console.error('加载会话列表失败:', error)
  }
}
</script>

<template>
  <div class="flex-1 flex flex-col overflow-hidden select-none">
    <!-- Section Header -->
    <div class="px-4 py-2 flex items-center justify-between" style="border-bottom: 1px solid var(--border-color)">
      <span class="text-xs font-bold text-gray-400 tracking-widest uppercase">Explorer</span>
      <div class="flex space-x-1">
         <div class="w-2 h-2 rounded-full bg-gray-600"></div>
         <div class="w-2 h-2 rounded-full bg-gray-600"></div>
      </div>
    </div>

    <!-- Skill 列表 -->
    <div class="flex-1 overflow-y-auto custom-scrollbar">
      <div class="py-2">
        <div class="px-4 py-1 mb-1 text-xs font-semibold text-blue-400 uppercase tracking-wider flex items-center cursor-pointer hover:text-blue-300">
          <span class="mr-1">▼</span> INSTALLED SKILLS
        </div>
        
        <div v-if="loading" class="px-8 py-2 text-xs text-gray-500 italic">
          Scanning modules...
        </div>
        
        <div v-else-if="skills.length === 0" class="px-8 py-2 text-xs text-gray-600">
          No skills detected.
        </div>
        
        <div v-else class="space-y-0.5">
          <div
            v-for="skill in skills"
            :key="skill.name"
            class="group px-4 py-1.5 flex items-center hover:bg-[#2a2d2e] cursor-pointer border-l-2 border-transparent hover:border-blue-500 transition-all duration-150"
          >
            <div class="w-4 h-4 mr-2 text-gray-400 group-hover:text-blue-400">
               <svg fill="none" viewBox="0 0 24 24" stroke="currentColor">
                 <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.384-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
               </svg>
            </div>
            <div class="overflow-hidden">
              <div class="text-sm text-gray-300 group-hover:text-white font-medium truncate">{{ skill.name }}</div>
              <div class="text-[10px] text-gray-500 truncate">{{ skill.description }}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- 活跃会话 -->
      <div class="py-2 border-t border-[#333]">
        <div class="px-4 py-1 mb-1 text-xs font-semibold text-green-500 uppercase tracking-wider flex items-center cursor-pointer hover:text-green-400">
          <span class="mr-1">▼</span> ACTIVE SESSIONS
        </div>
        
        <div v-if="sessions.length === 0" class="px-8 py-2 text-xs text-gray-600">
          No active processes.
        </div>
        
        <div v-else class="space-y-0.5">
          <div
            v-for="session in sessions"
            :key="session.id"
            class="group px-4 py-1.5 hover:bg-[#2a2d2e] cursor-pointer border-l-2 border-transparent hover:border-green-500 transition-all duration-150"
          >
            <div class="flex items-center justify-between mb-0.5">
              <span class="text-sm font-medium text-gray-300 group-hover:text-white truncate">{{ session.skill_name }}</span>
              <span class="text-[10px] px-1.5 py-0.5 rounded bg-gray-700 text-gray-300 font-mono">
                {{ session.status }}
              </span>
            </div>
            <div class="text-[10px] text-gray-500 font-mono truncate pl-1 border-l border-gray-600">
              > {{ session.input }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #424242;
  border-radius: 2px;
}
</style>
