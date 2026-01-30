// src/main.ts

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import './style.css'

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.mount('#app')

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
