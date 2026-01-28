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
  <div class="flex-1 flex flex-col overflow-hidden">
    <!-- Skill 列表 -->
    <div class="p-4 border-b border-gray-700">
      <h2 class="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-3">
        可用 Skill
      </h2>
      
      <div v-if="loading" class="text-center py-4">
        <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-600 mx-auto"></div>
      </div>
      
      <div v-else-if="skills.length === 0" class="text-center py-4 text-gray-500">
        <p>暂无可用 Skill</p>
      </div>
      
      <div v-else class="space-y-2">
        <button
          v-for="skill in skills"
          :key="skill.name"
          class="w-full text-left px-3 py-2 rounded-lg bg-gray-700 hover:bg-gray-600 transition-colors"
        >
          <div class="font-medium text-white">{{ skill.name }}</div>
          <div class="text-xs text-gray-400 truncate">{{ skill.description }}</div>
        </button>
      </div>
    </div>

    <!-- 活跃会话 -->
    <div class="flex-1 p-4 overflow-y-auto">
      <h2 class="text-sm font-semibold text-gray-400 uppercase tracking-wider mb-3">
        活跃会话
      </h2>
      
      <div v-if="sessions.length === 0" class="text-center py-4 text-gray-500">
        <p>暂无活跃会话</p>
      </div>
      
      <div v-else class="space-y-2">
        <button
          v-for="session in sessions"
          :key="session.id"
          class="w-full text-left px-3 py-2 rounded-lg bg-gray-700 hover:bg-gray-600 transition-colors"
        >
          <div class="flex items-center justify-between">
            <span class="font-medium text-white truncate">{{ session.skill_name }}</span>
            <span class="text-xs px-2 py-0.5 rounded-full bg-blue-600 text-white">
              {{ session.status }}
            </span>
          </div>
          <div class="text-xs text-gray-400 mt-1 truncate">{{ session.input }}</div>
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 组件特定样式 */
</style>
