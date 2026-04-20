<template>
  <div class="journey-viewer">
    <template v-if="data">
      <h2 class="journey-title">{{ data.title || data.id }}</h2>
      <div class="journey-intent">{{ data.intent || 'Journey demonstration' }}</div>

      <div class="journey-section">
        <div :class="['journey-status', passed ? 'pass' : 'fail']">
          <div class="journey-status-badge">{{ passed ? 'PASS' : 'FAIL' }}</div>
          <span>{{ passed ? 'Journey completed successfully' : 'Journey encountered issues' }}</span>
        </div>
      </div>

      <div v-if="data.verification" class="journey-section verification">
        <div>overall_score: <b>{{ data.verification.overall_score }}</b></div>
        <div>describe_confidence: {{ data.verification.describe_confidence }}</div>
        <div>judge_confidence: {{ data.verification.judge_confidence }}</div>
        <div>mode: {{ data.verification.mode }} @ {{ data.verification.timestamp }}</div>
      </div>

      <div v-if="data.steps && data.steps.length > 0">
        <table class="journey-steps">
          <thead>
            <tr><th>#</th><th>Slug</th><th>Intent</th><th>Screenshot</th><th>Score</th></tr>
          </thead>
          <tbody>
            <tr v-for="(step, idx) in data.steps" :key="idx">
              <td>{{ step.index }}</td>
              <td><code>{{ step.slug }}</code></td>
              <td>{{ step.intent }}</td>
              <td><code>{{ step.screenshot_path }}</code></td>
              <td>{{ step.judge_score ?? '—' }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div v-if="data.error" class="journey-error">
        <div class="journey-error-title">Error</div>
        <code>{{ data.error }}</code>
      </div>
    </template>

    <template v-else>
      <div class="journey-empty">
        <p>Journey not yet recorded.</p>
        <p class="hint">Run <code>phenotype-journey record --tape &lt;path&gt;</code> to capture.</p>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import type { Manifest } from './types'

const props = defineProps<{ manifest?: string | Manifest }>()
const data = ref<Manifest | null>(null)

const passed = computed(() => {
  if (!data.value) return false
  return data.value.passed ?? data.value.pass ?? false
})

onMounted(async () => {
  if (!props.manifest) return
  if (typeof props.manifest === 'object') {
    data.value = props.manifest as Manifest
    return
  }
  try {
    const res = await fetch(props.manifest)
    data.value = await res.json()
  } catch (e) {
    console.error('Failed to load journey manifest:', e)
    data.value = null
  }
})
</script>

<style scoped>
.journey-viewer { border: 1px solid var(--vp-divider, #e5e7eb); border-radius: 8px; padding: 24px; margin: 24px 0; background: var(--vp-c-bg-soft, #fafafa); }
.journey-title { font-size: 24px; font-weight: 600; margin: 0 0 12px 0; }
.journey-intent { font-size: 14px; color: var(--vp-c-text-2, #555); margin-bottom: 20px; padding-bottom: 12px; border-bottom: 1px solid var(--vp-divider, #e5e7eb); }
.journey-section { margin: 20px 0; }
.journey-status { display: flex; gap: 12px; padding: 12px; border-radius: 6px; align-items: center; }
.journey-status.pass { background: rgba(16,185,129,0.1); border-left: 4px solid #10b981; }
.journey-status.fail { background: rgba(239,68,68,0.1); border-left: 4px solid #ef4444; }
.journey-status-badge { font-weight: 600; font-size: 12px; text-transform: uppercase; letter-spacing: 0.5px; }
.journey-status.pass .journey-status-badge { color: #10b981; }
.journey-status.fail .journey-status-badge { color: #ef4444; }
.verification { font-size: 13px; font-family: ui-monospace, monospace; }
.journey-steps { width: 100%; border-collapse: collapse; font-size: 13px; }
.journey-steps th, .journey-steps td { text-align: left; padding: 6px 8px; border-bottom: 1px solid var(--vp-divider, #e5e7eb); }
.journey-error { margin-top: 20px; padding: 12px; background: rgba(239,68,68,0.1); border-left: 4px solid #ef4444; border-radius: 4px; }
.journey-error-title { color: #ef4444; font-weight: 600; margin-bottom: 6px; }
.journey-empty { padding: 20px; text-align: center; color: var(--vp-c-text-3, #777); }
.hint { font-size: 13px; margin-top: 12px; }
</style>
