// src/composables/useEvent.ts

/**
 * Tauri 事件监听 composable
 *
 * 提供统一的接口来监听来自 Tauri 后端的事件
 */

import { ref, onUnmounted, type Ref } from 'vue';
import { listen as tauriListen, emit as tauriEmit } from '@tauri-apps/api/event';

export type TauriEventListener = (payload: any) => void;

export interface UseEventReturn {
  /**
   * 监听事件
   * @param eventName 事件名称
   * @param callback 回调函数
   * @returns 取消监听的函数
   */
  listen: (eventName: string, callback: TauriEventListener) => () => void;

  /**
   * 一次性事件监听
   * @param eventName 事件名称
   * @param callback 回调函数
   * @returns 取消监听的函数
   */
  once: (eventName: string, callback: TauriEventListener) => () => void;

  /**
   * 发送事件到后端
   * @param eventName 事件名称
   * @param payload 事件数据
   */
  emit: (eventName: string, payload?: any) => Promise<void>;

  /**
   * 移除所有事件监听器
   */
  removeAllListeners: () => void;
}

export function useEvent(): UseEventReturn {
  // 存储所有的事件监听器
  const listeners: Map<string, Promise<() => void>> = new Map();

  /**
   * 监听 Tauri 事件
   */
  function listen(eventName: string, callback: TauriEventListener): () => void {
    const unlisten = tauriListen(eventName, (event) => {
      console.log(`[useEvent] 收到事件: ${eventName}`, event.payload);
      callback(event.payload);
    });

    listeners.set(`${eventName}-${Date.now()}`, unlisten);

    // 返回取消监听的函数
    return () => {
      unlisten.then((fn) => fn());
      listeners.delete(`${eventName}-${Date.now()}`);
    };
  }

  /**
   * 一次性事件监听
   */
  function once(eventName: string, callback: TauriEventListener): () => void {
    let unlisten: (() => void) | null = null;

    const wrappedCallback: TauriEventListener = (payload) => {
      callback(payload);
      // 执行后立即取消监听
      if (unlisten) {
        unlisten();
        unlisten = null;
      }
    };

    const unlistenPromise = tauriListen(eventName, (event) => {
      console.log(`[useEvent] 收到一次性事件: ${eventName}`, event.payload);
      wrappedCallback(event.payload);
    });

    unlistenPromise.then((fn) => {
      unlisten = () => {
        fn();
        listeners.delete(`${eventName}-once`);
      };
    });

    listeners.set(`${eventName}-once`, unlistenPromise);

    // 返回取消监听的函数
    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }

  /**
   * 发送事件到后端
   */
  async function emit(eventName: string, payload?: any): Promise<void> {
    console.log(`[useEvent] 发送事件: ${eventName}`, payload);
    await tauriEmit(eventName, payload);
  }

  /**
   * 移除所有事件监听器
   */
  function removeAllListeners(): void {
    console.log(`[useEvent] 移除所有监听器 (共 ${listeners.size} 个)`);

    listeners.forEach((unlistenPromise) => {
      unlistenPromise.then((fn) => fn());
    });

    listeners.clear();
  }

  // 组件卸载时自动清理
  onUnmounted(() => {
    removeAllListeners();
  });

  return {
    listen,
    once,
    emit,
    removeAllListeners,
  };
}

/**
 * 监听 Agent 事件的便捷函数
 */
export function useAgentEvent(
  callback: (event: {
    type: string;
    event_id: number;
    session_id?: string;
    [key: string]: any;
  }) => void
) {
  const { listen } = useEvent();

  return listen('agent-event', (payload) => {
    console.log('[useAgentEvent] Agent 事件:', payload);
    callback(payload);
  });
}

/**
 * 监听 Session 更新事件的便捷函数
 */
export function useSessionEvent(
  callback: (event: {
    session_id: string;
    [key: string]: any;
  }) => void
) {
  const { listen } = useEvent();

  return listen('session-updated', (payload) => {
    console.log('[useSessionEvent] Session 更新:', payload);
    callback(payload);
  });
}

/**
 * 监听 Agent 状态变化事件的便捷函数
 */
export function useAgentStatusEvent(
  callback: (status: 'idle' | 'starting' | 'running' | 'stopping' | 'error') => void
) {
  const { listen } = useEvent();

  return listen('agent-status-changed', (payload) => {
    console.log('[useAgentStatusEvent] Agent 状态变化:', payload);
    callback(payload.status);
  });
}

/**
 * 监听错误的便捷函数
 */
export function useErrorEvent(
  callback: (error: {
    code: string;
    message: string;
    suggestion?: string;
    session_id?: string;
  }) => void
) {
  const { listen } = useEvent();

  return listen('error', (payload) => {
    console.error('[useErrorEvent] 错误事件:', payload);
    callback(payload);
  });
}
