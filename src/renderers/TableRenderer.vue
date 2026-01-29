<template>
  <div class="table-renderer">
    <div class="table-container" ref="containerRef" @scroll="handleScroll">
      <div class="virtual-spacer" :style="{ height: totalHeight + 'px' }"></div>
      <table class="data-table" :style="{ transform: `translateY(${offsetY}px)` }">
        <thead>
          <tr>
            <th v-for="(header, index) in headers" :key="index">{{ header }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="row in visibleRows" :key="row.index">
            <td v-for="(cell, cellIndex) in row.data" :key="cellIndex">{{ cell }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  data: any;
}

const props = defineProps<Props>();

// Extract headers and rows from the data
const headers = computed(() => {
  if (!props.data) return [];
  
  if (Array.isArray(props.data.headers)) {
    // If data has headers property
    return props.data.headers;
  } else if (Array.isArray(props.data) && props.data.length > 0) {
    // If data is an array of objects, extract keys as headers
    return Object.keys(props.data[0]);
  }
  return [];
});

const rows = computed(() => {
  if (!props.data) return [];
  
  if (Array.isArray(props.data.rows)) {
    // If data has rows property
    return props.data.rows;
  } else if (Array.isArray(props.data)) {
    // If data is an array of objects
    return props.data.map(item => Object.values(item));
  }
  return [];
});
</script>

<style scoped>
.table-renderer {
  height: 100%;
  overflow: auto;
  padding: 12px;
}

.table-container {
  overflow-x: auto;
}

.data-table {
  width: 100%;
  border-collapse: collapse;
  min-width: 400px;
  font-size: 14px;
}

.data-table th,
.data-table td {
  padding: 8px 12px;
  text-align: left;
  border: 1px solid #3a3a3a;
}

.data-table th {
  background-color: #2d2d2d;
  color: #cccccc;
  font-weight: 600;
  position: sticky;
  top: 0;
}

.data-table td {
  background-color: #1e1e1e;
  color: #d4d4d4;
}

.data-table tr:nth-child(even) {
  background-color: #252526;
}

.data-table tr:hover {
  background-color: #2a2d2e;
}

.data-table th {
  border-top: none;
}

.data-table tr:last-child td {
  border-bottom: none;
}

.data-table tr td:first-child,
.data-table tr th:first-child {
  border-left: none;
}

.data-table tr td:last-child,
.data-table tr th:last-child {
  border-right: none;
}
</style>