<template>
  <div class="log-renderer">
    <div class="log-container" ref="logContainer">
      <div
        v-for="(chunk, index) in dataChunks"
        :key="index"
        class="log-entry"
      >
        <span class="log-timestamp">{{ formatTimestamp(chunk.received_at) }}</span>
        <span class="log-content">{{ formatLogContent(chunk.data) }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUpdated, nextTick } from 'vue';

interface Props {
  dataChunks: any[];
  autoScroll?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  autoScroll: true,
});

const logContainer = ref<HTMLElement>();

function formatTimestamp(timestamp: string): string {
  const date = new Date(timestamp);
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
  });
}

function formatLogContent(data: unknown): string {
  if (typeof data === 'string') {
    return data;
  }
  if (typeof data === 'object' && data !== null) {
    return JSON.stringify(data, null, 2);
  }
  return String(data);
}

function scrollToBottom() {
  if (props.autoScroll && logContainer.value) {
    nextTick(() => {
      logContainer.value?.scrollTo({
        top: logContainer.value.scrollHeight,
        behavior: 'smooth',
      });
    });
  }
}

onMounted(() => {
  scrollToBottom();
});

onUpdated(() => {
  scrollToBottom();
});
</script>

<style scoped>
.log-renderer {
  height: 100%;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.log-container {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  background-color: #1e1e1e;
  color: #d4d4d4;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.5;
  border-radius: 4px;
}

.log-entry {
  display: flex;
  gap: 8px;
  margin-bottom: 4px;
}

.log-timestamp {
  color: #858585;
  flex-shrink: 0;
  user-select: none;
}

.log-content {
  color: #d4d4d4;
  white-space: pre-wrap;
  word-break: break-word;
  flex: 1;
}

/* Scrollbar styles */
.log-container::-webkit-scrollbar {
  width: 8px;
}

.log-container::-webkit-scrollbar-track {
  background: #2d2d2d;
  border-radius: 4px;
}

.log-container::-webkit-scrollbar-thumb {
  background: #5a5a5a;
  border-radius: 4px;
}

.log-container::-webkit-scrollbar-thumb:hover {
  background: #6a6a6a;
}
</style>
