// src/composables/useAgent.ts

import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useAgentStore } from '@/stores/agent';
import { useSessionStore } from '@/stores/session';
import type { Session } from '@/stores/session';

export interface ExecuteSkillOptions {
  input: string;
}

export interface SendInputOptions {
  sessionId: string;
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
  sendInputToSession: (options: SendInputOptions) => Promise<void>;
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
      const sessionId = await invoke('execute_skill', {
        skillName,
        input: options.input
      });
      
      console.log(`[useAgent] Skill ${skillName} 执行已启动，后端返回的会话 ID: ${sessionId}`);

      // 手动确保 Session 在 Store 中存在，避免竞态条件
      const tempSession: Session = {
          id: sessionId as string,
          skill_name: skillName,
          status: 'pending',
          input: options.input,
          created_at: new Date().toISOString(),
          data_chunks: [],
          content_blocks: [
            {
              type: 'user',
              text: options.input,
              timestamp: new Date().toISOString()
            }
          ],
          success: false
      };
      console.log(`[useAgent] 创建前端 session:`, tempSession.id);
      sessionStore.handleSessionCreated(tempSession);
      
      return sessionId as string;
    } catch (e) {
      console.error(`执行 Skill ${skillName} 失败:`, e);
      throw e;
    }
  }

  async function cancelSession(sessionId: string): Promise<void> {
    try {
      await invoke('cancel_session', { sessionId });
      console.log(`会话 ${sessionId} 已取消`);
    } catch (e) {
      console.error(`取消会话 ${sessionId} 失败:`, e);
      throw e;
    }
  }

  async function getSessions(): Promise<Session[]> {
    try {
      const sessions = await invoke('get_sessions');
      return sessions as Session[];
    } catch (e) {
      console.error('获取会话列表失败:', e);
      throw e;
    }
  }

  async function getSession(sessionId: string): Promise<Session | null> {
    try {
      const session = await invoke('get_session', { sessionId });
      return session as Session | null;
    } catch (e) {
      console.error(`获取会话 ${sessionId} 失败:`, e);
      throw e;
    }
  }

  async function sendInputToSession(options: SendInputOptions): Promise<void> {
    if (!agentStore.isRunning) {
      throw new Error('Agent 未运行，无法发送输入');
    }

    const { sessionId, input } = options;
    
    try {
      console.log(`[useAgent] 向会话 ${sessionId} 发送输入:`, input);

      // 获取当前会话
      const session = sessionStore.getSession(sessionId);
      if (!session) {
        throw new Error(`会话 ${sessionId} 不存在`);
      }

      // 将用户输入添加到 content_blocks
      const userBlock = {
        type: 'user' as const,
        text: input,
        timestamp: new Date().toISOString()
      };
      
      session.content_blocks = [...session.content_blocks, userBlock];
      session.status = 'running'; // 更新状态为运行中
      
      // 触发响应式更新
      sessionStore.sessionsMap = new Map(sessionStore.sessionsMap);

      // 调用后端 send_session_input 命令
      // 后端会使用相同的 skill 执行新的输入，并返回新的 session_id
      try {
        const newSessionId = await invoke('send_session_input', {
          sessionId,
          input
        });
        console.log(`[useAgent] 新会话已创建: ${newSessionId}`);
      } catch (e) {
        console.error('[useAgent] send_session_input 失败:', e);
        throw e;
      }

      console.log(`[useAgent] 输入已发送到会话 ${sessionId}`);
    } catch (e) {
      console.error(`向会话 ${sessionId} 发送输入失败:`, e);
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
    sendInputToSession,
    cancelSession,
    getSessions,
    getSession,
  };
}