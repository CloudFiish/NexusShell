// src/composables/useCommunicationDiagnostics.ts

/**
 * CodeBuddy 通信诊断工具
 *
 * 提供前端与 CodeBuddy SDK 通信状态的检查和诊断功能
 */

import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen as tauriListen } from '@tauri-apps/api/event'
import { useAgentStore } from '@/stores/agent'
import { useSessionStore } from '@/stores/session'

export interface DiagnosticsResult {
  success: boolean
  stage: string
  message: string
  details?: any
  timestamp: string
}

export interface CommunicationStatus {
  // Agent 状态
  agentRunning: boolean
  agentStatus: string
  agentError: string | null

  // 事件系统状态
  eventListenerActive: boolean
  lastEventReceived: string | null
  eventCount: number

  // 会话状态
  activeSessions: number
  totalSessions: number

  // 连接质量
  latency: number | null
  lastPingTime: string | null
}

// 诊断结果存储
const diagnosticsResults = ref<DiagnosticsResult[]>([])
const isRunningDiagnostics = ref(false)

// 通信状态
const communicationStatus = ref<CommunicationStatus>({
  agentRunning: false,
  agentStatus: 'unknown',
  agentError: null,
  eventListenerActive: false,
  lastEventReceived: null,
  eventCount: 0,
  activeSessions: 0,
  totalSessions: 0,
  latency: null,
  lastPingTime: null
})

// 事件监听状态
let eventUnlistener: (() => void) | null = null
let eventCheckTimer: number | null = null

export function useCommunicationDiagnostics() {
  const agentStore = useAgentStore()
  const sessionStore = useSessionStore()

  /**
   * 添加诊断结果
   */
  function addResult(success: boolean, stage: string, message: string, details?: any) {
    const result: DiagnosticsResult = {
      success,
      stage,
      message,
      details,
      timestamp: new Date().toISOString()
    }
    diagnosticsResults.value.push(result)
    console.log(`[Diagnostics] ${success ? '✅' : '❌'} [${stage}] ${message}`, details || '')
    return result
  }

  /**
   * 清除诊断结果
   */
  function clearResults() {
    diagnosticsResults.value = []
  }

  /**
   * 检查 Agent 状态
   */
  async function checkAgentStatus(): Promise<DiagnosticsResult> {
    try {
      const isRunning = await invoke('is_agent_running')
      communicationStatus.value.agentRunning = isRunning as boolean
      communicationStatus.value.agentStatus = agentStore.status
      communicationStatus.value.agentError = agentStore.error

      if (isRunning) {
        return addResult(true, 'Agent', 'Agent 正在运行', { status: agentStore.status })
      } else {
        return addResult(false, 'Agent', 'Agent 未运行', { status: agentStore.status })
      }
    } catch (error) {
      return addResult(false, 'Agent', '检查 Agent 状态失败', { error: String(error) })
    }
  }

  /**
   * 检查事件系统
   */
  async function checkEventSystem(): Promise<DiagnosticsResult> {
    try {
      // 设置事件监听（使用直接的 Tauri listen API）
      if (!eventUnlistener) {
        console.log('[Diagnostics] 正在设置事件监听器...')
        eventUnlistener = await tauriListen('agent-event', (event: any) => {
          console.log('[Diagnostics] 收到事件:', event.payload)
          communicationStatus.value.eventCount++
          communicationStatus.value.lastEventReceived = new Date().toISOString()
          communicationStatus.value.eventListenerActive = true
        })
        console.log('[Diagnostics] 事件监听器已设置:', eventUnlistener)
      } else {
        console.log('[Diagnostics] 事件监听器已存在')
      }

      // 等待一段时间检查是否收到事件
      await new Promise(resolve => setTimeout(resolve, 100))

      if (communicationStatus.value.eventListenerActive) {
        return addResult(true, 'EventSystem', '事件监听器已激活', {
          eventCount: communicationStatus.value.eventCount
        })
      } else {
        return addResult(true, 'EventSystem', '事件监听器已设置，等待事件中...', {
          note: '这是正常的，如果没有正在执行的 Skill，就不会收到事件',
          listenerActive: !!eventUnlistener
        })
      }
    } catch (error) {
      console.error('[Diagnostics] 事件系统检查失败:', error)
      return addResult(false, 'EventSystem', '事件系统检查失败', { error: String(error) })
    }
  }

  /**
   * 检查会话系统
   */
  async function checkSessionSystem(): Promise<DiagnosticsResult> {
    try {
      const sessions = sessionStore.getAllSessions()
      communicationStatus.value.totalSessions = sessions.length
      communicationStatus.value.activeSessions = sessions.filter(
        s => s.status === 'running' || s.status === 'pending'
      ).length

      return addResult(true, 'SessionSystem', '会话系统正常', {
        totalSessions: communicationStatus.value.totalSessions,
        activeSessions: communicationStatus.value.activeSessions
      })
    } catch (error) {
      return addResult(false, 'SessionSystem', '会话系统检查失败', { error: String(error) })
    }
  }

  /**
   * 测试端到端通信
   */
  async function testEndToEndCommunication(): Promise<DiagnosticsResult> {
    const startTime = Date.now()

    try {
      // 1. 确保 Agent 正在运行
      if (!communicationStatus.value.agentRunning) {
        return addResult(false, 'EndToEnd', 'Agent 未运行，无法测试端到端通信')
      }

      // 2. 记录测试开始
      addResult(true, 'EndToEnd', '开始端到端通信测试', {
        startTime: new Date().toISOString()
      })

      // 3. 执行一个简单的 Skill
      const testSessionId = await invoke('execute_skill', {
        skillName: 'assistant',
        input: 'Hello, this is a communication test. Please respond with "TEST_OK".'
      })

      addResult(true, 'EndToEnd', 'Skill 执行请求已发送', { sessionId: testSessionId })

      // 4. 等待事件（最多等待 30 秒）
      const initialEventCount = communicationStatus.value.eventCount
      let waitTime = 0
      const maxWaitTime = 30000 // 30 秒（API 调用可能需要较长时间）

      addResult(true, 'EndToEnd', '等待事件中...', {
        initialEventCount,
        maxWaitTime: `${maxWaitTime}ms`
      })

      while (waitTime < maxWaitTime) {
        await new Promise(resolve => setTimeout(resolve, 1000))
        waitTime += 1000

        if (communicationStatus.value.eventCount > initialEventCount) {
          break
        }
      }

      const latency = Date.now() - startTime
      communicationStatus.value.latency = latency
      communicationStatus.value.lastPingTime = new Date().toISOString()

      if (communicationStatus.value.eventCount > initialEventCount) {
        return addResult(true, 'EndToEnd', '端到端通信测试成功', {
          latency: `${latency}ms`,
          eventsReceived: communicationStatus.value.eventCount - initialEventCount,
          waited: `${waitTime}ms`
        })
      } else {
        return addResult(false, 'EndToEnd', '端到端通信测试失败：未收到事件', {
          waited: `${waitTime}ms`,
          initialEventCount,
          finalEventCount: communicationStatus.value.eventCount,
          note: '可能是 API 限制、网络问题或事件监听未正确设置'
        })
      }

    } catch (error) {
      return addResult(false, 'EndToEnd', '端到端通信测试失败', { error: String(error) })
    }
  }

  /**
   * 运行完整诊断
   */
  async function runFullDiagnostics(): Promise<DiagnosticsResult[]> {
    isRunningDiagnostics.value = true
    clearResults()

    try {
      // 1. 检查 Agent 状态
      await checkAgentStatus()

      // 2. 检查事件系统
      await checkEventSystem()

      // 3. 检查会话系统
      await checkSessionSystem()

      // 4. 如果 Agent 正在运行，测试端到端通信
      if (communicationStatus.value.agentRunning) {
        await testEndToEndCommunication()
      }

      return diagnosticsResults.value

    } finally {
      isRunningDiagnostics.value = false
    }
  }

  /**
   * 启动 Agent 并等待就绪
   */
  async function startAgentAndWait(): Promise<DiagnosticsResult> {
    try {
      if (communicationStatus.value.agentRunning) {
        return addResult(true, 'StartAgent', 'Agent 已经在运行')
      }

      addResult(true, 'StartAgent', '正在启动 Agent...')

      await invoke('start_agent')

      // 等待 Agent 启动（最多 5 秒）
      let waitTime = 0
      const maxWaitTime = 5000

      while (waitTime < maxWaitTime) {
        await new Promise(resolve => setTimeout(resolve, 500))
        waitTime += 500

        const isRunning = await invoke('is_agent_running')
        if (isRunning) {
          communicationStatus.value.agentRunning = true
          return addResult(true, 'StartAgent', 'Agent 启动成功', {
            startupTime: `${waitTime}ms`
          })
        }
      }

      return addResult(false, 'StartAgent', 'Agent 启动超时')

    } catch (error) {
      return addResult(false, 'StartAgent', 'Agent 启动失败', { error: String(error) })
    }
  }

  /**
   * 获取通信状态摘要
   */
  function getStatusSummary(): string {
    const parts: string[] = []

    if (communicationStatus.value.agentRunning) {
      parts.push('✅ Agent 运行中')
    } else {
      parts.push('❌ Agent 未运行')
    }

    if (communicationStatus.value.eventListenerActive) {
      parts.push(`✅ 事件监听正常 (${communicationStatus.value.eventCount} 个事件)`)
    } else {
      parts.push('⏳ 等待事件...')
    }

    if (communicationStatus.value.latency) {
      parts.push(`📡 延迟 ${communicationStatus.value.latency}ms`)
    }

    return parts.join(' | ')
  }

  /**
   * 清理资源
   */
  function cleanup() {
    if (eventUnlistener) {
      eventUnlistener()
      eventUnlistener = null
    }
    if (eventCheckTimer) {
      clearInterval(eventCheckTimer)
      eventCheckTimer = null
    }
  }

  return {
    // 状态
    diagnosticsResults: computed(() => diagnosticsResults.value),
    isRunningDiagnostics: computed(() => isRunningDiagnostics.value),
    communicationStatus: computed(() => communicationStatus.value),

    // 方法
    checkAgentStatus,
    checkEventSystem,
    checkSessionSystem,
    testEndToEndCommunication,
    runFullDiagnostics,
    startAgentAndWait,
    getStatusSummary,
    clearResults,
    cleanup
  }
}
