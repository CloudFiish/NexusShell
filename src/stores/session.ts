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
  type: 'text' | 'thinking' | 'tooluse' | 'toolresult' | 'user';
  text?: string;
  thinking?: string;
  signature?: string;
  id?: string;
  name?: string;
  input?: any;
  tool_use_id?: string;
  content?: string;
  is_error?: boolean;
  timestamp?: string;
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
  console.log('[Session Store] defineStore 开始执行');
  
  const sessionsMap = ref<Map<string, Session>>(new Map());
  const activeSessionId = ref<string | null>(null);
  
  console.log('[Session Store] ref 已创建');

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

    // 使用 Object.assign 更新属性
    Object.assign(session, updates);

    // 如果是更新 content_blocks，确保创建新数组以触发响应式更新
    if (updates.content_blocks) {
      session.content_blocks = [...updates.content_blocks];
    }

    // 触发 Map 更新以确保响应式
    sessionsMap.value = new Map(sessionsMap.value);
  }

  function handleDataChunk(sessionId: string, chunk: DataChunk) {
    console.log(`[Session Store] handleDataChunk 被调用:`, { sessionId, chunk_index: chunk.index, data_type: typeof chunk.data });

    const session = sessionsMap.value.get(sessionId);
    if (!session) {
      console.warn('[Session Store] Session ' + sessionId + ' not found for data chunk');
      console.log('[Session Store] 当前所有 session IDs:', Array.from(sessionsMap.value.keys()));
      return;
    }

    console.log(`[Session Store] 找到 session:`, session.id, `当前 blocks 数量:`, session.content_blocks?.length || 0);

    // Initialize content_blocks if not exists
    if (!session.content_blocks) {
      session.content_blocks = [];
    }

    console.log(`[Session Store] 处理数据块:`, chunk.data, `类型:`, typeof chunk.data);

    // Convert chunk.data to ContentBlock
    let block: ContentBlock | null = null;
    
    // Try to parse string data as JSON first if it looks like an object
    if (typeof chunk.data === 'string') {
        const trimmed = chunk.data.trim();
        if ((trimmed.startsWith('{') && trimmed.endsWith('}')) || (trimmed.startsWith('[') && trimmed.endsWith(']'))) {
             try {
                 const parsed = JSON.parse(chunk.data);
                 if (typeof parsed === 'object' && parsed !== null) {
                     chunk.data = parsed; // Update chunk.data to object
                 }
             } catch (e) {
                 // Not valid JSON, treat as text
             }
        }
    }

    if (typeof chunk.data === 'string') {
      console.log(`[Session Store] 数据是字符串，创建 text block`);
      block = { type: 'text', text: chunk.data };
    } else if (typeof chunk.data === 'object' && chunk.data !== null) {
      const data = chunk.data as any;
      console.log(`[Session Store] 数据是对象:`, data);
      
      // Robust type detection
      if (data.type === 'thinking' || (data.thinking && data.signature)) {
        console.log(`[Session Store] 检测到 thinking block`);
        block = { type: 'thinking', thinking: data.thinking, signature: data.signature };
      } else if (data.type === 'tooluse' || (data.id && data.name && data.input)) {
        console.log(`[Session Store] 检测到 tooluse block`);
        block = { 
            type: 'tooluse', 
            id: data.id,
            name: data.name,
            input: data.input
        };
      } else if (data.type === 'toolresult' || data.tool_use_id) {
        console.log(`[Session Store] 检测到 toolresult block`);
        block = { 
            type: 'toolresult', 
            tool_use_id: data.tool_use_id,
            content: data.content,
            is_error: data.is_error
        };
      } else if (data.type === 'text' || data.text) {
        console.log(`[Session Store] 检测到 text block (from object)`);
        block = { type: 'text', text: data.text || JSON.stringify(data) };
      } else {
        console.log(`[Session Store] 默认转换为 text block`);
        // Default to text representation of json object
        block = { type: 'text', text: JSON.stringify(data, null, 2) };
      }
    }

    if (block) {
      // Avoid duplicate blocks if possible (optional optimization)
      // 创建新数组以触发 Vue 响应式更新
      session.content_blocks = [...session.content_blocks, block];
      console.log(`[Session Store] 生成 ContentBlock:`, block.type, `总块数:`, session.content_blocks.length);

      // 触发 Map 更新以确保响应式
      sessionsMap.value = new Map(sessionsMap.value);
    } else {
        console.warn(`[Session Store] 无法转换数据块:`, chunk.data);
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
    // 触发 Map 更新以确保响应式
    sessionsMap.value = new Map(sessionsMap.value);
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
    // 触发 Map 更新以确保响应式
    sessionsMap.value = new Map(sessionsMap.value);
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

    // 触发 Map 更新以确保响应式
    sessionsMap.value = new Map(sessionsMap.value);

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

    // 将缓冲区的数据块添加到会话（创建新数组以触发响应式更新）
    session.data_chunks = [...session.data_chunks, ...buffer];

    // 清空缓冲区
    buffer.length = 0;

    // 清除定时器
    if (batchTimers.has(sessionId)) {
      clearTimeout(batchTimers.get(sessionId)!);
      batchTimers.delete(sessionId);
    }

    // 触发 Map 更新以确保响应式
    sessionsMap.value = new Map(sessionsMap.value);

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

    console.log(`[Session Store] handleAgentEvent 被调用:`, { type, session_id });

    if (!session_id) {
      console.warn('[Session Store] 事件缺少 session_id:', event);
      return;
    }

    // 检查 session 是否存在
    const sessionExists = sessionsMap.value.has(session_id);
    console.log(`[Session Store] session_id ${session_id} 是否存在:`, sessionExists);
    if (!sessionExists) {
      console.log('[Session Store] 当前所有 session IDs:', Array.from(sessionsMap.value.keys()));
    }

    switch (type) {
      case 'execution_start': {
        const { skill_name } = rest;
        console.log(`🚀 [Session ${session_id}] 执行开始, Skill: ${skill_name}`);
        // 更新 session 状态为 running
        handleSessionUpdated(session_id, {
          status: 'running',
          started_at: new Date().toISOString()
        });
        break;
      }

      case 'data_chunk': {
        const { chunk_index, data, is_final } = rest;
        console.log(`[Session Store] 收到 data_chunk 事件:`, { session_id, chunk_index, is_final, data_type: typeof data });

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
        console.log(`⏳ [Session ${session_id}] 进度: ${Math.round((current / total) * 100)}% - ${message}`);

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
        console.log(`✅ [Session ${session_id}] 执行完成 (Success: ${success})`);
        if (summary) console.log('Summary:', summary);
        console.groupEnd();
        
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

  // Initialize event listeners immediately (BEFORE return to ensure it runs)
  console.log('[Session Store] 即将调用 setupEventListeners');
  setupEventListeners();
  console.log('[Session Store] Store 初始化完成，事件监听器已设置');

  // Clean up is generally not needed for a global store, but we can expose the method if needed
  // If HMR (Hot Module Replacement) causes issues, we might need to handle cleanup here.
  if (import.meta.hot) {
    import.meta.hot.dispose(() => {
      cleanupEventListeners();
    });
  }
  
  console.log('[Session Store] defineStore 执行完毕');
  
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
});
