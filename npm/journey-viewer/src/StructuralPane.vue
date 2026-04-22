<template>
  <div class="kf-structural-pane" role="region" aria-label="Structural snapshot">
    <div class="kf-structural-header">
      <span class="kf-structural-title">
        <span class="kf-structural-glyph" aria-hidden="true">◫</span>
        Structural — {{ familyLabel }}
      </span>
      <span class="kf-structural-path" :title="path">{{ shortPath }}</span>
    </div>

    <div v-if="loading" class="kf-structural-muted">loading…</div>
    <div v-else-if="error" class="kf-structural-error">error: {{ error }}</div>

    <!-- macOS/Streamlit: JSON tree for walker output; Streamlit also shows the ARIA literal. -->
    <template v-else-if="snapshot">
      <!-- CLI: render a terminal grid with cells + cursor highlight. -->
      <pre v-if="snapshot.family === 'cli'" class="kf-term-buffer" :aria-label="`${snapshot.rows}×${snapshot.cols} terminal buffer`">{{ termText }}</pre>

      <!-- Streamlit: show aria tree first (highest-value), html collapsed. -->
      <template v-else-if="snapshot.family === 'streamlit'">
        <div class="kf-structural-meta">
          <div><strong>url</strong> <code>{{ snapshot.url }}</code></div>
          <div><strong>title</strong> <code>{{ snapshot.title }}</code></div>
        </div>
        <pre class="kf-aria" aria-label="ARIA snapshot">{{ snapshot.aria }}</pre>
      </template>

      <!-- macOS: JSON-tree layout. -->
      <template v-else-if="snapshot.family === 'macos'">
        <pre class="kf-json-tree" aria-label="Accessibility tree">{{ prettyJson }}</pre>
      </template>
    </template>

    <div v-else class="kf-structural-muted">no structural sibling</div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { StructuralSnapshot, StructuralCli } from './types'

const props = defineProps<{
  /** Absolute or site-root URL to the `.structural.json` sibling. */
  path: string | null | undefined
}>()

const loading = ref(false)
const error = ref<string | null>(null)
const snapshot = ref<StructuralSnapshot | null>(null)

const shortPath = computed(() => {
  if (!props.path) return ''
  const p = props.path
  return p.length > 48 ? '…' + p.slice(-46) : p
})

const familyLabel = computed(() => {
  const f = snapshot.value?.family
  if (f === 'macos') return 'macOS accessibility tree'
  if (f === 'streamlit') return 'Streamlit ARIA + HTML'
  if (f === 'cli') return 'CLI terminal grid'
  return '—'
})

const prettyJson = computed(() =>
  snapshot.value ? JSON.stringify(snapshot.value, null, 2) : '',
)

/** Render the CLI cells into a rows×cols monospace grid. */
const termText = computed(() => {
  const s = snapshot.value as StructuralCli | null
  if (!s || s.family !== 'cli') return ''
  const buf: string[][] = Array.from({ length: s.rows }, () =>
    Array(s.cols).fill(' '),
  )
  for (const c of s.cells) {
    if (c.row < s.rows && c.col < s.cols) {
      buf[c.row][c.col] = c.ch.length > 0 ? c.ch[0] : ' '
    }
  }
  // Overlay cursor as `█`-style marker if visible and in-bounds.
  if (s.cursor.visible && s.cursor.row < s.rows && s.cursor.col < s.cols) {
    buf[s.cursor.row][s.cursor.col] = '▉'
  }
  return buf.map((row) => row.join('').trimEnd()).join('\n')
})

async function load(p: string) {
  loading.value = true
  error.value = null
  snapshot.value = null
  try {
    const res = await fetch(p)
    if (!res.ok) throw new Error(`HTTP ${res.status}`)
    snapshot.value = (await res.json()) as StructuralSnapshot
  } catch (e) {
    error.value = (e as Error).message
  } finally {
    loading.value = false
  }
}

watch(
  () => props.path,
  (p) => {
    if (p) load(p)
    else {
      snapshot.value = null
      error.value = null
    }
  },
  { immediate: true },
)
</script>

<style scoped>
.kf-structural-pane {
  background: rgba(17, 17, 27, 0.82);
  border: 1px solid rgba(205, 214, 244, 0.14);
  border-radius: 8px;
  padding: 10px 12px;
  color: #cdd6f4;
  max-width: 44vw;
  max-height: 80vh;
  overflow: auto;
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 12px;
  line-height: 1.45;
}
.kf-structural-header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 8px;
  gap: 10px;
}
.kf-structural-title {
  font-weight: 600;
  font-size: 11px;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  color: #89b4fa;
}
.kf-structural-glyph { margin-right: 4px; }
.kf-structural-path {
  font-size: 10px;
  color: #a6adc8;
}
.kf-structural-muted { color: #6c7086; font-style: italic; }
.kf-structural-error {
  color: #f38ba8;
  background: rgba(243, 139, 168, 0.08);
  border: 1px dashed rgba(243, 139, 168, 0.32);
  padding: 6px 8px;
  border-radius: 4px;
}
.kf-structural-meta {
  display: flex;
  flex-direction: column;
  gap: 2px;
  margin-bottom: 6px;
  font-size: 11px;
}
.kf-structural-meta code {
  color: #a6e3a1;
  word-break: break-all;
}
.kf-term-buffer,
.kf-json-tree,
.kf-aria {
  background: rgba(11, 11, 17, 0.72);
  border: 1px solid rgba(205, 214, 244, 0.08);
  border-radius: 4px;
  padding: 8px 10px;
  white-space: pre;
  overflow-x: auto;
}
</style>
