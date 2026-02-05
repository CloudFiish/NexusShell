// src/main.ts

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import './style.css'

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.mount('#app')

// 强制初始化 session store，确保事件监听器尽早设置
// 使用动态导入确保模块被加载
import('./stores/session').then(({ useSessionStore }) => {
  const sessionStore = useSessionStore()
  console.log('[main.ts] Session Store 强制初始化成功:', !!sessionStore)
}).catch(e => {
  console.error('[main.ts] Session Store 强制初始化失败:', e)
})

// 🔧 添加调试支持（仅在开发环境）
if (import.meta.env.DEV) {
  // 延迟初始化，确保 Store 已创建
  let initAttempts = 0;
  const maxAttempts = 10;

  const initDebugSupport = () => {
    initAttempts++;

    try {
      // 直接从 pinia 获取 store
      const agentStore = pinia.state.value.agent;
      const sessionStore = pinia.state.value.session;

      // 检查 store 是否存在
      if (!agentStore || !sessionStore) {
        if (initAttempts < maxAttempts) {
          console.log(`⏳ Store 尚未初始化，等待中... (${initAttempts}/${maxAttempts})`);
          setTimeout(initDebugSupport, 500);
          return;
        } else {
          console.error('❌ Store 初始化超时');
          return;
        }
      }

      // 添加到 window
      (window as any).agentStore = agentStore;
      (window as any).sessionStore = sessionStore;

      console.log('🔧 调试模式已启用');
      console.log('  访问 Store:');
      console.log('    window.agentStore - Agent Store');
      console.log('    window.sessionStore - Session Store');
      console.log('');
      console.log('  快速命令:');
      console.log('    debugStatus() - 查看完整状态');
      console.log('    debugAgent() - 查看 Agent 状态');
      console.log('    debugSessions() - 查看所有 Session');
      console.log('    debugActive() - 查看活跃 Session');

      // 添加便捷的调试函数
      (window as any).debugStatus = () => {
        console.group('🔍 NexusShell 状态');
        console.log('Agent:', (window as any).agentStore);
        console.log('Session:', (window as any).sessionStore);
        console.groupEnd();
      };

      (window as any).debugAgent = () => {
        const store = (window as any).agentStore;
        console.group('🔍 Agent 状态');
        console.log('完整 Store:', store);
        console.log('状态:', store?.status);
        console.log('是否运行:', store?.isRunning);
        console.log('加载中:', store?.loading);
        console.log('错误:', store?.error);
        console.log('Skills:', store?.skills);
        console.groupEnd();
      };

      (window as any).debugSessions = () => {
        const store = (window as any).sessionStore;
        console.group('🔍 所有 Session');
        console.log('完整 Store:', store);
        console.log('Sessions 属性:', store?.sessions);
        console.log('Sessions 数组:', store?.sessions?.value || []);
        console.log('总数:', store?.sessions?.value?.length || 0);
        console.table(store?.sessions?.value || []);
        console.groupEnd();
      };

      (window as any).debugActive = () => {
        const store = (window as any).sessionStore;
        const sessions = store?.sessions?.value || [];
        const active = sessions.find((s: any) => s.id === store.activeSessionId?.value);
        if (active) {
          console.group('🔍 活跃 Session');
          console.log('ID:', active.id);
          console.log('状态:', active.status);
          console.log('输入:', active.input);
          console.log('内容块数量:', active.content_blocks?.length || 0);
          console.log('内容块:', active.content_blocks);
          console.groupEnd();
        } else {
          console.warn('❌ 没有找到活跃的 Session');
          console.log('Active ID:', store?.activeSessionId?.value);
          console.log('所有 Session IDs:', sessions.map((s: any) => s.id));
        }
      };

    } catch (error) {
      console.warn('🔧 调试模式初始化失败:', error);
      console.log('提示: Store 可能还未初始化，请稍后再试');
      console.log('重试次数:', initAttempts, '/', maxAttempts);

      if (initAttempts < maxAttempts) {
        setTimeout(initDebugSupport, 500);
      }
    }
  };

  // 开始初始化
  setTimeout(initDebugSupport, 1000);
}

// 禁用默认右键菜单（仅在非输入区域）
document.addEventListener('contextmenu', (event) => {
  const target = event.target as HTMLElement;
  const isInput = target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable;
  // 允许选中的文本区域显示右键菜单（用于复制）
  const selection = window.getSelection();
  const hasSelection = selection && selection.toString().length > 0;

  if (!isInput && !hasSelection && import.meta.env.PROD) {
    event.preventDefault();
  }
});
