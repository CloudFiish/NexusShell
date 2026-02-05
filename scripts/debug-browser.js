// 🔧 简化版调试工具
// 复制以下代码到浏览器控制台

(function() {
  console.group('🔍 NexusShell 简化诊断');
  
  // 1. 检查 window 上的 Store
  console.log('1. 检查 window Store:');
  console.log('   window.agentStore:', window.agentStore);
  console.log('   window.sessionStore:', window.sessionStore);
  
  // 2. 尝试获取 Pinia 实例
  console.log('\n2. 查找 Pinia 实例:');
  const app = document.querySelector('#app')?.__vue_app__;
  console.log('   Vue App:', app);
  console.log('   Pinia:', app?.config?.globalProperties?.$pinia);
  
  // 3. 如果找到了，尝试获取 Store
  let sessionStore = null;
  
  if (window.sessionStore) {
    sessionStore = window.sessionStore;
    console.log('   ✅ 从 window.sessionStore 获取');
  } else if (app?.config?.globalProperties?.$pinia) {
    const pinia = app.config.globalProperties.$pinia;
    sessionStore = pinia.state.value.session;
    console.log('   ✅ 从 Vue App 获取');
  } else {
    console.log('   ❌ 未找到 sessionStore');
  }
  
  // 4. 显示 Session 信息
  console.log('\n3. Session 信息:');
  if (sessionStore) {
    console.log('   Store 对象:', sessionStore);
    console.log('   Sessions 数组:', sessionStore.sessions?.value || sessionStore.sessions);
    console.log('   Active ID:', sessionStore.activeSessionId?.value || sessionStore.activeSessionId);
  }
  
  // 5. 查找 Vue 组件实例
  console.log('\n4. 查找 Vue 组件:');
  const vm = app?.$parent || app;
  console.log('   Vue 实例:', vm);
  
  if (vm && vm.$store) {
    console.log('   $store:', vm.$store);
  }
  
  console.groupEnd();
  
  // 返回 sessionStore 供后续使用
  window.__debug_sessionStore = sessionStore;
  console.log('\n💡 提示: 已保存到 window.__debug_sessionStore');
})();
