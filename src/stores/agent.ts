// src/stores/agent.ts

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export type AgentStatus = 'idle' | 'starting' | 'running' | 'stopping' | 'error';

export interface Skill {
  name: string;
  description: string;
  default_render: string;
  supported_renders: string[];
  input_schema?: Record<string, unknown>;
  output_schema?: Record<string, unknown>;
}

export const useAgentStore = defineStore('agent', () => {
  const status = ref<AgentStatus>('idle');
  const skills = ref<Skill[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const isRunning = computed(() => status.value === 'running');
  const canStart = computed(() => status.value === 'idle' || status.value === 'error');
  const canStop = computed(() => status.value === 'running');
  const skillCount = computed(() => skills.value.length);

  async function startAgent() {
    if (!canStart.value) {
      throw new Error('Agent 状态为 ' + status.value + '，无法启动');
    }

    try {
      loading.value = true;
      status.value = 'starting';
      error.value = null;

      await window.__TAURI__.invoke('start_agent');

      status.value = 'running';
      console.log('Agent 启动成功');
    } catch (e) {
      status.value = 'error';
      error.value = e instanceof Error ? e.message : String(e);
      console.error('Agent 启动失败:', error.value);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function stopAgent() {
    if (!canStop.value) {
      throw new Error('Agent 状态为 ' + status.value + '，无法停止');
    }

    try {
      loading.value = true;
      status.value = 'stopping';
      error.value = null;

      await window.__TAURI__.invoke('stop_agent');

      status.value = 'idle';
      skills.value = [];
      console.log('Agent 停止成功');
    } catch (e) {
      status.value = 'error';
      error.value = e instanceof Error ? e.message : String(e);
      console.error('Agent 停止失败:', error.value);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function getSkills() {
    if (!isRunning.value) {
      throw new Error('Agent 未运行，无法获取 Skill 列表');
    }

    try {
      loading.value = true;
      error.value = null;

      const result = await window.__TAURI__.invoke('get_skills');
      skills.value = result as Skill[];

      console.log('获取到 ' + skills.value.length + ' 个 Skill');
      return skills.value;
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      console.error('获取 Skill 列表失败:', error.value);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function refreshSkills() {
    if (!isRunning.value) {
      throw new Error('Agent 未运行，无法刷新 Skill 列表');
    }

    try {
      loading.value = true;
      error.value = null;

      const result = await window.__TAURI__.invoke('get_skills');
      skills.value = result as Skill[];

      console.log('刷新 Skill 列表成功，共 ' + skills.value.length + ' 个 Skill');
      return skills.value;
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      console.error('刷新 Skill 列表失败:', error.value);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  function clearError() {
    error.value = null;
  }

  function getSkillByName(name: string): Skill | undefined {
    return skills.value.find(skill => skill.name === name);
  }

  function reset() {
    status.value = 'idle';
    skills.value = [];
    loading.value = false;
    error.value = null;
  }

  return {
    status,
    skills,
    loading,
    error,
    isRunning,
    canStart,
    canStop,
    skillCount,
    startAgent,
    stopAgent,
    getSkills,
    refreshSkills,
    clearError,
    getSkillByName,
    reset,
  };
});
