// src/composables/useAgent.ts

import { ref, computed } from 'vue';
import { useAgentStore } from '@/stores/agent';
import { useSessionStore } from '@/stores/session';
import type { Session } from '@/stores/session';

export interface ExecuteSkillOptions {
  input: string;
}

export interface UseAgentReturn {
  isReady: computed<boolean>;
  isLoading: computed<boolean>;
  error: computed<string | null>;
  startAgent: () => Promise<void>;
  stopAgent: () => Promise<void>;
  getSkills: () => Promise<any[]>;
  executeSkill: (skillName: string, options: ExecuteSkillOptions) => Promise<string>;
  cancelSession: (sessionId: string) => Promise<void>;
  getSessions: () => Promise<Session[]>;
  getSession: (sessionId: string) => Promise<Session | null>;
}

export function useAgent(): UseAgentReturn {
  const agentStore = useAgentStore();
  const sessionStore = useSessionStore();
  
  const isReady = computed(() => agentStore.isRunning);
  const isLoading = computed(() => agentStore.loading);
  const error = computed(() => agentStore.error);

  async function startAgent(): Promise<void> {
    return agentStore.startAgent();
  }

  async function stopAgent(): Promise<void> {
    return agentStore.stopAgent();
  }

  async function getSkills(): Promise<any[]> {
    return agentStore.getSkills();
  }

  async function executeSkill(skillName: string, options: ExecuteSkillOptions): Promise<string> {
    if (!agentStore.isRunning) {
      throw new Error('Agent 未运行，无法执行 Skill');
    }

    try {
      const sessionId = await window.__TAURI__.invoke('execute_skill', {
        skillName,
        input: options.input
      });
      
      console.log(`Skill ${skillName} 执行已启动，会话 ID: ${sessionId}`);
      return sessionId as string;
    } catch (e) {
      console.error(`执行 Skill ${skillName} 失败:`, e);
      throw e;
    }
  }

  async function cancelSession(sessionId: string): Promise<void> {
    try {
      await window.__TAURI__.invoke('cancel_session', { sessionId });
      console.log(`会话 ${sessionId} 已取消`);
    } catch (e) {
      console.error(`取消会话 ${sessionId} 失败:`, e);
      throw e;
    }
  }

  async function getSessions(): Promise<Session[]> {
    try {
      const sessions = await window.__TAURI__.invoke('get_sessions');
      return sessions as Session[];
    } catch (e) {
      console.error('获取会话列表失败:', e);
      throw e;
    }
  }

  async function getSession(sessionId: string): Promise<Session | null> {
    try {
      const session = await window.__TAURI__.invoke('get_session', { sessionId });
      return session as Session | null;
    } catch (e) {
      console.error(`获取会话 ${sessionId} 失败:`, e);
      throw e;
    }
  }

  return {
    isReady,
    isLoading,
    error,
    startAgent,
    stopAgent,
    getSkills,
    executeSkill,
    cancelSession,
    getSessions,
    getSession,
  };
}