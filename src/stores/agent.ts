// src/stores/agent.ts

import { defineStore } from 'pinia';
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useAgentEvent, useErrorEvent } from '@/composables/useEvent';

export type AgentStatus = 'idle' | 'starting' | 'running' | 'stopping' | 'error';

export interface Skill {
  name: string;
  description: string;
  default_render: string;
  supported_renders: string[];
  input_schema?: Record<string, unknown>;
  output_schema?: Record<string, unknown>;
  category?: string;
  requires_filesystem: boolean;
  requires_network: boolean;
}

export const useAgentStore = defineStore('agent', () => {
  const status = ref<AgentStatus>('idle');
  const skills = ref<Skill[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const agentType = ref<'codebuddy-sdk' | 'codebuddy' | 'claude-code'>('codebuddy-sdk');

  // 事件监听器清理函数
  let unlistenAgentEvent: (() => void) | null = null;
  let unlistenErrorEvent: (() => void) | null = null;

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

      console.log('[Agent Store] 启动 Agent...');
      const result = await invoke('start_agent');
      console.log('[Agent Store] Agent 启动成功:', result);

      status.value = 'running';
    } catch (e) {
      status.value = 'error';
      error.value = e instanceof Error ? e.message : String(e);
      console.error('[Agent Store] Agent 启动失败:', error.value);
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

      console.log('[Agent Store] 停止 Agent...');
      const result = await invoke('stop_agent');
      console.log('[Agent Store] Agent 停止成功:', result);

      status.value = 'idle';
      skills.value = [];
      console.log('[Agent Store] Agent 已停止');
    } catch (e) {
      status.value = 'error';
      error.value = e instanceof Error ? e.message : String(e);
      console.error('[Agent Store] Agent 停止失败:', error.value);
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

      console.log('[Agent Store] 获取 Skill 列表...');
      const result = await invoke('get_skills');
      skills.value = result as Skill[];

      console.log('[Agent Store] 获取到 ' + skills.value.length + ' 个 Skill');
      return skills.value;
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      console.error('[Agent Store] 获取 Skill 列表失败:', error.value);
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

      console.log('[Agent Store] 刷新 Skill 列表...');
      const result = await invoke('get_skills');
      skills.value = result as Skill[];

      console.log('[Agent Store] 刷新 Skill 列表成功，共 ' + skills.value.length + ' 个 Skill');
      return skills.value;
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e);
      console.error('[Agent Store] 刷新 Skill 列表失败:', error.value);
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

  /**
   * 初始化事件监听
   */
  function setupEventListeners() {
    console.log('[Agent Store] 设置事件监听器');

    // 监听 Agent 事件
    unlistenAgentEvent = useAgentEvent((event) => {
      console.log('[Agent Store] 收到 Agent 事件:', event);

      switch (event.type) {
        case 'execution_start':
          console.log('[Agent Store] Skill 执行开始:', event);
          // 可以在这里更新 UI 状态
          break;

        case 'execution_complete':
          console.log('[Agent Store] Skill 执行完成:', event);
          // 可以在这里更新 UI 状态
          break;

        default:
          console.log('[Agent Store] 收到未处理的 Agent 事件类型:', event.type);
      }
    });

    // 监听错误事件
    unlistenErrorEvent = useErrorEvent((errorEvent) => {
      console.error('[Agent Store] 收到错误事件:', errorEvent);

      // 更新错误状态
      error.value = `${errorEvent.code}: ${errorEvent.message}`;

      // 如果有建议，显示给用户
      if (errorEvent.suggestion) {
        console.log('[Agent Store] 建议解决方案:', errorEvent.suggestion);
      }
    });
  }

  /**
   * 清理事件监听
   */
  function cleanupEventListeners() {
    console.log('[Agent Store] 清理事件监听器');

    if (unlistenAgentEvent) {
      unlistenAgentEvent();
      unlistenAgentEvent = null;
    }

    if (unlistenErrorEvent) {
      unlistenErrorEvent();
      unlistenErrorEvent = null;
    }
  }

  return {
    status,
    skills,
    loading,
    error,
    agentType,
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
    setupEventListeners,
    cleanupEventListeners,
  };

  // 在 store 初始化时自动设置事件监听
  onMounted(() => {
    console.log('[Agent Store] 组件已挂载，设置事件监听器');
    setupEventListeners();
  });

  // 在 store 销毁时清理事件监听
  onUnmounted(() => {
    console.log('[Agent Store] 组件已卸载，清理事件监听器');
    cleanupEventListeners();
  });
});
