// src/stores/session.ts

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export type SessionStatus = 'pending' | 'running' | 'paused' | 'cancelled' | 'completed' | 'error';
export type RenderMode = 'log' | 'code' | 'table' | 'json' | 'chart' | 'file_tree' | 'markdown' | 'diff';

export interface DataChunk {
  index: number;
  data: unknown;
  is_final: boolean;
  received_at: string;
  size: number;
}

export interface ProgressInfo {
  current: number;
  total: number;
  message: string;
  percentage: number;
  updated_at: string;
}

export interface ErrorInfo {
  code: string;
  message: string;
  suggestion: string;
  occurred_at: string;
}

export interface Session {
  id: string;
  skill_name: string;
  skill_info?: {
    name: string;
    description: string;
    default_render: string;
    supported_renders: string[];
  };
  status: SessionStatus;
  created_at: string;
  started_at?: string;
  completed_at?: string;
  input: string;
  render_mode?: RenderMode;
  data_chunks: DataChunk[];
  progress?: ProgressInfo;
  error?: ErrorInfo;
  summary?: string;
  success: boolean;
}

export const useSessionStore = defineStore('session', () => {
  const sessionsMap = ref<Map<string, Session>>(new Map());
  const activeSessionId = ref<string | null>(null);

  const sessions = computed(() => Array.from(sessionsMap.value.values()));
  const activeSession = computed(() => {
    if (activeSessionId.value) {
      return sessionsMap.value.get(activeSessionId.value);
    }
    return null;
  });
  const activeSessions = computed(() =>
    sessions.value.filter(s => s.status === 'running' || s.status === 'pending')
  );
  const completedSessions = computed(() =>
    sessions.value.filter(s =>
      s.status === 'completed' || s.status === 'error' || s.status === 'cancelled'
    )
  );

  function handleSessionCreated(session: Session) {
    sessionsMap.value.set(session.id, session);
    console.log('Session created: ' + session.id);

    if (sessionsMap.value.size === 1) {
      setActiveSession(session.id);
    }
  }

  function handleSessionUpdated(sessionId: string, updates: Partial<Session>) {
    const session = sessionsMap.value.get(sessionId);
    if (!session) {
      console.warn('Session ' + sessionId + ' not found');
      return;
    }

    Object.assign(session, updates);
  }

  function handleDataChunk(sessionId: string, chunk: DataChunk) {
    const session = sessionsMap.value.get(sessionId);
    if (!session) {
      console.warn('Session ' + sessionId + ' not found for data chunk');
      return;
    }

    session.data_chunks.push(chunk);
  }

  function handleProgress(sessionId: string, progress: ProgressInfo) {
    const session = sessionsMap.value.get(sessionId);
    if (!session) {
      console.warn('Session ' + sessionId + ' not found for progress update');
      return;
    }

    session.progress = progress;
  }

  function handleError(sessionId: string, error: ErrorInfo) {
    const session = sessionsMap.value.get(sessionId);
    if (!session) {
      console.warn('Session ' + sessionId + ' not found for error');
      return;
    }

    session.error = error;
    session.status = 'error';
    if (!session.completed_at) {
      session.completed_at = error.occurred_at;
    }
  }

  function handleExecutionComplete(sessionId: string, summary: string, success: boolean) {
    const session = sessionsMap.value.get(sessionId);
    if (!session) {
      console.warn('Session ' + sessionId + ' not found for execution complete');
      return;
    }

    session.summary = summary;
    session.success = success;
    session.status = success ? 'completed' : 'error';

    if (!session.completed_at) {
      session.completed_at = new Date().toISOString();
    }
  }

  function setActiveSession(sessionId: string | null) {
    if (sessionId === null) {
      activeSessionId.value = null;
      return;
    }

    if (sessionsMap.value.has(sessionId)) {
      activeSessionId.value = sessionId;
    } else {
      console.warn('Session ' + sessionId + ' not found');
    }
  }

  function getSession(sessionId: string): Session | undefined {
    return sessionsMap.value.get(sessionId);
  }

  function getAllSessions(): Session[] {
    return sessions.value;
  }

  function getActiveSessions(): Session[] {
    return activeSessions.value;
  }

  function getCompletedSessions(): Session[] {
    return completedSessions.value;
  }

  function cleanupOldSessions(olderThanMs: number = 3600000): number {
    const now = Date.now();
    const toRemove: string[] = [];

    for (const [id, session] of sessionsMap.value.entries()) {
      if (session.completed_at) {
        const completedTime = new Date(session.completed_at).getTime();
        if (now - completedTime > olderThanMs) {
          toRemove.push(id);
        }
      }
    }

    for (const id of toRemove) {
      sessionsMap.value.delete(id);
      if (activeSessionId.value === id) {
        const remaining = Array.from(sessionsMap.value.keys());
        activeSessionId.value = remaining.length > 0 ? remaining[0] : null;
      }
    }

    return toRemove.length;
  }

  function reset() {
    sessionsMap.value.clear();
    activeSessionId.value = null;
  }

  return {
    sessionsMap,
    activeSessionId,

    sessions,
    activeSession,
    activeSessions,
    completedSessions,

    handleSessionCreated,
    handleSessionUpdated,
    handleDataChunk,
    handleProgress,
    handleError,
    handleExecutionComplete,

    setActiveSession,

    getSession,
    getAllSessions,
    getActiveSessions,
    getCompletedSessions,

    cleanupOldSessions,

    reset,
  };
});
