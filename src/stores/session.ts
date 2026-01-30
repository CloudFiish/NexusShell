// src/stores/session.ts

import { defineStore } from 'pinia';
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useAgentEvent, useErrorEvent } from '@/composables/useEvent';

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

export interface ContentBlock {
  type: 'text' | 'thinking' | 'tooluse' | 'toolresult';
  text?: string;
  thinking?: string;
  signature?: string;
  id?: string;
  name?: string;
  input?: any;
  tool_use_id?: string;
  content?: string;
  is_error?: boolean;
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
  prompt?: string; // New field
  render_mode?: RenderMode;
  data_chunks: DataChunk[];
  content_blocks: ContentBlock[]; // New field
  model?: string; // New field
  duration_ms?: number; // New field
  total_cost_usd?: number | null; // New field
  num_turns?: number; // New field
  progress?: ProgressInfo;
  error?: ErrorInfo;
  summary?: string;
  success: boolean;
}

export const useSessionStore = defineStore('session', () => {
  const sessionsMap = ref<Map<string, Session>>(new Map());
  const activeSessionId = ref<string | null>(null);

  // 事件监听器清理函数
  let unlistenAgentEvent: (() => void) | null = null;
  let unlistenErrorEvent: (() => void) | null = null;

  // 数据批处理配置
  const BATCH_SIZE = 100; // 每批最多 100 个数据块
  const BATCH_DELAY = 100; // 批处理延迟 100ms
  const MAX_DATA_CHUNKS = 10000; // 每个会话最多保留 10000 个数据块
  const MAX_MEMORY_MB = 50; // 每个会话最多使用 50MB 内存

  // 批处理缓冲区
  const batchBuffers = new Map<string, DataChunk[]>();
  const batchTimers = new Map<string, number>();

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
    if (!session.content_blocks) {
      session.content_blocks = [];
    }
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

    // Initialize content_blocks if not exists
    if (!session.content_blocks) {
      session.content_blocks = [];
    }

    // Convert chunk.data to ContentBlock
    let block: ContentBlock | null = null;
    if (typeof chunk.data === 'string') {
      block = { type: 'text', text: chunk.data };
    } else if (typeof chunk.data === 'object' && chunk.data !== null) {
      const data = chunk.data as any;
      if (data.type === 'thinking') {
        block = { type: 'thinking', thinking: data.thinking, signature: data.signature };
      } else if (data.id && data.name && data.input) {
        block = { type: 'tooluse', ...data };
      } else if (data.tool_use_id) {
        block = { type: 'toolresult', ...data };
      } else {
        // Default to text representation of json
        block = { type: 'text', text: JSON.stringify(data) };
      }
    }

    if (block) {
      session.content_blocks.push(block);
    }

    // 检查内存使用
    const memoryUsage = estimateSessionMemory(session);
    if (memoryUsage > MAX_MEMORY_MB) {
      console.warn(`Session ${sessionId} 内存使用过高 (${memoryUsage.toFixed(2)}MB), 开始清理旧数据`);
      trimOldData(session);
    }

    // 检查数据块数量
    if (session.data_chunks.length >= MAX_DATA_CHUNKS) {
      console.warn(`Session ${sessionId} 数据块过多 (${session.data_chunks.length}), 开始清理旧数据`);
      trimOldData(session);
    }

    // 添加到批处理缓冲区
    if (!batchBuffers.has(sessionId)) {
      batchBuffers.set(sessionId, []);
    }
    const buffer = batchBuffers.get(sessionId)!;
    buffer.push(chunk);

    // 如果缓冲区达到批处理大小,立即刷新
    if (buffer.length >= BATCH_SIZE) {
      flushBatch(sessionId);
      return;
    }

    // 设置定时器,延迟刷新
    if (batchTimers.has(sessionId)) {
      clearTimeout(batchTimers.get(sessionId)!);
    }

    const timerId = setTimeout(() => {
      flushBatch(sessionId);
    }, BATCH_DELAY);

    batchTimers.set(sessionId, timerId);
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

    // 在完成前刷新所有批处理缓冲区
    flushBatch(sessionId);

    session.summary = summary;
    session.success = success;
    session.status = success ? 'completed' : 'error';

    if (!session.completed_at) {
      session.completed_at = new Date().toISOString();
    }

    console.log(`[Session Store] Session ${sessionId} 执行完成:`, {
      success,
      summary,
      status: session.status
    });
  }

  /**
   * 刷新批处理缓冲区
   */
  function flushBatch(sessionId: string) {
    const session = sessionsMap.value.get(sessionId);
    if (!session) {
      return;
    }

    const buffer = batchBuffers.get(sessionId);
    if (!buffer || buffer.length === 0) {
      return;
    }

    // 将缓冲区的数据块添加到会话
    session.data_chunks.push(...buffer);

    // 清空缓冲区
    buffer.length = 0;

    // 清除定时器
    if (batchTimers.has(sessionId)) {
      clearTimeout(batchTimers.get(sessionId)!);
      batchTimers.delete(sessionId);
    }

    console.debug(`[Session Store] Session ${sessionId} 批处理已刷新: ${session.data_chunks.length} 个数据块`);
  }

  /**
   * 估算会话内存使用量 (MB)
   */
  function estimateSessionMemory(session: Session): number {
    // 估算数据块的总大小
    const chunksSize = session.data_chunks.reduce((total, chunk) => {
      return total + chunk.size;
    }, 0);

    // 转换为 MB (1 MB = 1024 * 1024 bytes)
    return chunksSize / (1024 * 1024);
  }

  /**
   * 清理旧数据
   */
  function trimOldData(session: Session) {
    // 保留最近的 70% 数据
    const keepCount = Math.floor(session.data_chunks.length * 0.7);
    const removeCount = session.data_chunks.length - keepCount;

    if (removeCount > 0) {
      session.data_chunks = session.data_chunks.slice(removeCount);
      console.log(`[Session Store] Session ${session.id} 已清理 ${removeCount} 个旧数据块, 保留 ${keepCount} 个`);
    }
  }

  /**
   * 获取会话的内存使用统计
   */
  function getSessionMemoryStats(sessionId: string): {
    chunksCount: number;
    memoryUsageMB: number;
    isNearLimit: boolean;
  } | null {
    const session = sessionsMap.value.get(sessionId);
    if (!session) {
      return null;
    }

    const chunksCount = session.data_chunks.length;
    const memoryUsageMB = estimateSessionMemory(session);
    const isNearLimit =
      chunksCount >= MAX_DATA_CHUNKS * 0.9 ||
      memoryUsageMB >= MAX_MEMORY_MB * 0.9;

    return {
      chunksCount,
      memoryUsageMB,
      isNearLimit,
    };
  }

  /**
   * 清理会话的所有资源
   */
  function cleanupSession(sessionId: string) {
    // 刷新批处理缓冲区
    flushBatch(sessionId);

    // 清理批处理缓冲区
    const buffer = batchBuffers.get(sessionId);
    if (buffer) {
      buffer.length = 0;
      batchBuffers.delete(sessionId);
    }

    // 清理定时器
    const timer = batchTimers.get(sessionId);
    if (timer) {
      clearTimeout(timer);
      batchTimers.delete(sessionId);
    }

    console.log(`[Session Store] Session ${sessionId} 资源已清理`);
  }

  /**
   * 处理 Agent 事件 (来自 ACP 协议)
   */
  function handleAgentEvent(event: any) {
    const { type, session_id, ...rest } = event;

    if (!session_id) {
      console.warn('[Session Store] 事件缺少 session_id:', event);
      return;
    }

    switch (type) {
      case 'execution_start': {
        const { skill_name } = rest;
        console.log(`[Session Store] 执行开始: ${session_id} - ${skill_name}`);
        // 更新 session 状态为 running
        handleSessionUpdated(session_id, {
          status: 'running',
          started_at: new Date().toISOString()
        });
        break;
      }

      case 'data_chunk': {
        const { chunk_index, data, is_final } = rest;
        console.log(`[Session Store] 数据块: ${session_id} - chunk ${chunk_index}`);

        const chunk: DataChunk = {
          index: chunk_index || 0,
          data,
          is_final: is_final || false,
          received_at: new Date().toISOString(),
          size: JSON.stringify(data).length
        };

        handleDataChunk(session_id, chunk);
        break;
      }

      case 'progress': {
        const { current, total, message } = rest;
        console.log(`[Session Store] 进度: ${session_id} - ${current}/${total} - ${message}`);

        const progress: ProgressInfo = {
          current,
          total,
          message,
          percentage: total > 0 ? Math.round((current / total) * 100) : 0,
          updated_at: new Date().toISOString()
        };

        handleProgress(session_id, progress);
        break;
      }

      case 'execution_complete': {
        const { success, summary } = rest;
        console.log(`[Session Store] 执行完成: ${session_id} - ${success}`);
        handleExecutionComplete(session_id, summary || '', success);
        break;
      }

      default:
        console.log('[Session Store] 未处理的事件类型:', type);
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
      // 清理会话资源
      cleanupSession(id);
      // 删除会话
      sessionsMap.value.delete(id);
      if (activeSessionId.value === id) {
        const remaining = Array.from(sessionsMap.value.keys());
        activeSessionId.value = remaining.length > 0 ? remaining[0] : null;
      }
    }

    return toRemove.length;
  }

  function reset() {
    // 清理所有会话的资源
    for (const id of sessionsMap.value.keys()) {
      cleanupSession(id);
    }

    sessionsMap.value.clear();
    activeSessionId.value = null;
  }

  /**
   * 初始化事件监听
   */
  function setupEventListeners() {
    // 监听 Agent 事件
    unlistenAgentEvent = useAgentEvent((event) => {
      console.log('[Session Store] 收到 Agent 事件:', event);
      handleAgentEvent(event);
    });

    // 监听错误事件
    unlistenErrorEvent = useErrorEvent((errorEvent) => {
      console.error('[Session Store] 收到错误事件:', errorEvent);

      if (errorEvent.session_id) {
        const error: ErrorInfo = {
          code: errorEvent.code,
          message: errorEvent.message,
          suggestion: errorEvent.suggestion || '',
          occurred_at: new Date().toISOString()
        };

        handleError(errorEvent.session_id, error);
      }
    });

    console.log('[Session Store] 事件监听器已设置');
  }

  /**
   * 清理事件监听
   */
  function cleanupEventListeners() {
    if (unlistenAgentEvent) {
      unlistenAgentEvent();
      unlistenAgentEvent = null;
    }

    if (unlistenErrorEvent) {
      unlistenErrorEvent();
      unlistenErrorEvent = null;
    }

    console.log('[Session Store] 事件监听器已清理');
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

    getSessionMemoryStats,
    cleanupSession,

    cleanupOldSessions,

    reset,
    setupEventListeners,
    cleanupEventListeners,
  };

  // 在 store 初始化时自动设置事件监听
  onMounted(() => {
    setupEventListeners();
  });

  // 在 store 销毁时清理事件监听
  onUnmounted(() => {
    cleanupEventListeners();
  });
});
