<template>
  <div class="json-node" :style="{ marginLeft: level * 20 + 'px' }">
    <div 
      v-if="isObject || isArray" 
      class="node-header"
      @click="toggleExpanded"
    >
      <span class="expand-icon">{{ expanded ? '▼' : '▶' }}</span>
      <span class="node-key">{{ keyName }}:</span>
      <span class="node-type">{{ isArray ? `[${data.length}]` : `{${Object.keys(data).length}}` }}</span>
    </div>
    <div v-else class="node-item">
      <span class="node-key">{{ keyName }}:</span>
      <span :class="['node-value', valueTypeClass]">{{ displayValue }}</span>
    </div>
    
    <div v-if="(isObject || isArray) && expanded" class="node-children">
      <JsonTreeNode
        v-for="(value, key) in data"
        :key="key"
        :data="value"
        :keyName="isArray ? null : key"
        :expanded="false"
        :level="level + 1"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';

interface Props {
  data: any;
  keyName?: string | null;
  expanded?: boolean;
  level: number;
}

const props = withDefaults(defineProps<Props>(), {
  keyName: null,
  expanded: false,
});

const isExpanded = ref(props.expanded);
const isArray = computed(() => Array.isArray(props.data));
const isObject = computed(() => typeof props.data === 'object' && props.data !== null && !isArray.value);
const isPrimitive = computed(() => !isArray.value && !isObject.value);

const valueTypeClass = computed(() => {
  if (props.data === null) return 'null-value';
  if (typeof props.data === 'string') return 'string-value';
  if (typeof props.data === 'number') return 'number-value';
  if (typeof props.data === 'boolean') return 'boolean-value';
  return '';
});

const displayValue = computed(() => {
  if (props.data === null) return 'null';
  if (typeof props.data === 'string') return `"${props.data}"`;
  if (typeof props.data === 'boolean') return props.data.toString();
  return String(props.data);
});

const expanded = computed(() => isExpanded.value);

function toggleExpanded() {
  if (isObject.value || isArray.value) {
    isExpanded.value = !isExpanded.value;
  }
}
</script>

<style scoped>
.json-node {
  margin: 2px 0;
}

.node-header {
  cursor: pointer;
  padding: 2px 0;
  display: flex;
  align-items: center;
}

.node-header:hover {
  background-color: rgba(255, 255, 255, 0.05);
  border-radius: 2px;
}

.node-item {
  display: flex;
  padding: 2px 0;
}

.node-key {
  color: #9cdcfe;
  margin-right: 8px;
}

.node-value {
  color: #d4d4d4;
}

.string-value {
  color: #ce9178;
}

.number-value {
  color: #b5cea8;
}

.boolean-value {
  color: #569cd6;
}

.null-value {
  color: #569cd6;
  font-style: italic;
}

.node-type {
  color: #808080;
  margin-left: 8px;
  font-size: 0.9em;
}

.expand-icon {
  margin-right: 5px;
  font-size: 0.8em;
  width: 12px;
  display: inline-block;
}

.node-children {
  border-left: 1px dashed #454545;
  padding-left: 10px;
}
</style>