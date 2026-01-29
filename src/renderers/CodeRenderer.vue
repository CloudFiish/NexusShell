<template>
  <div class="code-renderer">
    <pre class="code-container"><code ref="codeElement" :class="'language-' + language">{{ codeContent }}</code></pre>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { highlight, languages } from 'prismjs';
import 'prismjs/components/prism-javascript';
import 'prismjs/components/prism-typescript';
import 'prismjs/components/prism-python';
import 'prismjs/components/prism-rust';
import 'prismjs/components/prism-json';
import 'prismjs/components/prism-markup';
import 'prismjs/components/prism-css';
import 'prismjs/themes/prism.css';

interface Props {
  data: any;
  language?: string;
}

const props = withDefaults(defineProps<Props>(), {
  language: 'javascript',
});

const codeElement = ref<HTMLElement>();

const codeContent = computed(() => {
  if (typeof props.data === 'string') {
    return props.data;
  }
  if (typeof props.data === 'object') {
    return JSON.stringify(props.data, null, 2);
  }
  return String(props.data);
});

onMounted(() => {
  if (codeElement.value) {
    codeElement.value.innerHTML = highlight(
      codeContent.value,
      languages[props.language] || languages.javascript,
      props.language
    );
  }
});

watch(() => props.data, () => {
  if (codeElement.value) {
    codeElement.value.innerHTML = highlight(
      codeContent.value,
      languages[props.language] || languages.javascript,
      props.language
    );
  }
}, { immediate: true });

</script>

<style scoped>
.code-renderer {
  height: 100%;
  overflow: auto;
  background-color: #1e1e1e;
  padding: 12px;
  border-radius: 4px;
}

.code-container {
  margin: 0;
  padding: 0;
  background: none;
  border: none;
  overflow: visible;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 13px;
  line-height: 1.5;
  color: #d4d4d4;
  tab-size: 4;
}

/* Prism.js theme overrides */
.code-container :deep(.token.comment) {
  color: #6a9955;
}

.code-container :deep(.token.string) {
  color: #ce9178;
}

.code-container :deep(.token.number) {
  color: #b5cea8;
}

.code-container :deep(.token.keyword) {
  color: #569cd6;
}

.code-container :deep(.token.function) {
  color: #dcdcaa;
}

.code-container :deep(.token.punctuation) {
  color: #d4d4d4;
}

.code-container :deep(.token.operator) {
  color: #d4d4d4;
}

.code-container :deep(.token.property) {
  color: #9cdcfe;
}

.code-container :deep(.token.tag) {
  color: #569cd6;
}

.code-container :deep(.token.attr-name) {
  color: #9cdcfe;
}

.code-container :deep(.token.attr-value) {
  color: #ce9178;
}
</style>