// src/composables/useToast.ts

/**
 * Toast 通知 composable
 *
 * 提供全局错误和成功通知功能
 */

import { ref } from 'vue'

export type ToastType = 'success' | 'error' | 'warning' | 'info'

export interface Toast {
  id: number
  type: ToastType
  message: string
  duration?: number
  timestamp: number
}

const toasts = ref<Toast[]>([])
let toastIdCounter = 0

export function useToast() {
  /**
   * 添加 Toast
   */
  function addToast(
    message: string,
    type: ToastType = 'info',
    duration: number = 5000
  ): number {
    const id = toastIdCounter++
    const toast: Toast = {
      id,
      type,
      message,
      duration,
      timestamp: Date.now()
    }

    toasts.value.push(toast)

    // 自动移除
    if (duration > 0) {
      setTimeout(() => {
        removeToast(id)
      }, duration)
    }

    return id
  }

  /**
   * 移除 Toast
   */
  function removeToast(id: number): void {
    const index = toasts.value.findIndex(t => t.id === id)
    if (index !== -1) {
      toasts.value.splice(index, 1)
    }
  }

  /**
   * 清除所有 Toast
   */
  function clearAll(): void {
    toasts.value = []
  }

  /**
   * 显示成功消息
   */
  function success(message: string, duration?: number): number {
    return addToast(message, 'success', duration)
  }

  /**
   * 显示错误消息
   */
  function error(message: string, duration?: number): number {
    return addToast(message, 'error', duration)
  }

  /**
   * 显示警告消息
   */
  function warning(message: string, duration?: number): number {
    return addToast(message, 'warning', duration)
  }

  /**
   * 显示信息消息
   */
  function info(message: string, duration?: number): number {
    return addToast(message, 'info', duration)
  }

  return {
    toasts: toasts.value,
    addToast,
    removeToast,
    clearAll,
    success,
    error,
    warning,
    info
  }
}
