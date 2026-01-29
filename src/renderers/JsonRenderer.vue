<template>
  <div class="json-renderer">
    <div class="json-container">
      <div class="toolbar">
        <button @click="toggleExpandAll" class="btn btn-sm">
          {{ expanded ? '折叠全部' : '展开全部' }}
        </button>
        <button @click="copyToClipboard" class="btn btn-sm">
          复制
        </button>
      </div>
      <div class="json-content">
        <JsonTreeNode 
          :data="parsedData" 
          :expanded="expanded"
          :level="0"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import JsonTreeNode from './JsonTreeNode.vue';

interface Props {
  data: any;
}

const props = defineProps<Props>();
const expanded = ref(true);

// Parse the data to ensure it's a valid JSON object/array
const parsedData = computed(() => {
  if (typeof props.data === 'string') {
    try {
      return JSON.parse(props.data);
    } catch {
      return props.data;
    }
  }
  return props.data;
});

function toggleExpandAll() {
  expanded.value = !expanded.value;
}

function copyToClipboard() {
  navigator.clipboard.writeText(JSON.stringify(parsedData.value, null, 2));
}
</script>

<style scoped>
.json-renderer {
  height: 100%;
  overflow: auto;
  background-color: #1e1e1e;
  padding: 12px;
  border-radius: 4px;
}

.json-container {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.5;
  color: #d4d4d4;
}

.toolbar {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px solid #3a3a3a;
}

.btn {
  background-color: #2d2d2d;
  color: #d4d4d4;
  border: 1px solid #3a3a3a;
  padding: 4px 8px;
  border-radius: 3px;
  cursor: pointer;
  font-size: 12px;
}

.btn:hover {
  background-color: #3a3a3a;
}

.json-content {
  overflow: auto;
}
</style>