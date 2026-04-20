<template>
  <div class="recording-embed">
    <figure class="recording-figure">
      <video v-if="hasMP4" class="recording-video" controls :autoplay="autoplay" muted loop loading="lazy">
        <source :src="mp4Path" type="video/mp4" />
        <img :src="gifPath" :alt="caption || tape" loading="lazy" />
      </video>
      <img v-else class="recording-image" :src="gifPath" :alt="caption || tape" loading="lazy" />
      <figcaption v-if="caption" class="recording-caption">{{ caption }}</figcaption>
    </figure>

    <details class="keyframes-section" :open="keyframesData.length > 0">
      <summary class="keyframes-title">Keyframes ({{ keyframesData.length }})</summary>
      <div v-if="keyframesData.length > 0" class="keyframes-grid">
        <div v-for="(frame, idx) in keyframesData" :key="idx" class="keyframe-item">
          <img :src="frame.path" :alt="`${tape} keyframe ${idx + 1}: ${frame.alt}`" loading="lazy" />
          <p class="keyframe-caption">{{ idx + 1 }}. {{ frame.alt }}</p>
        </div>
      </div>
      <p v-else class="keyframes-empty">No manifest found at <code>{{ manifestUrl }}</code>.</p>
    </details>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

interface KeyframeData { path: string; alt: string }

const props = withDefaults(defineProps<{
  tape: string
  /** Base URL under which /recordings, /keyframes, /manifests live. */
  baseUrl?: string
  caption?: string
  autoplay?: boolean
  maxKeyframes?: number
}>(), { baseUrl: '/journeys', autoplay: false, maxKeyframes: 12 })

const keyframesData = ref<KeyframeData[]>([])
const hasMP4 = computed(() => typeof window !== 'undefined' && 'videoWidth' in document.createElement('video'))
const gifPath = computed(() => `${props.baseUrl}/recordings/${props.tape}.gif`)
const mp4Path = computed(() => `${props.baseUrl}/recordings/${props.tape}.mp4`)
const manifestUrl = computed(() => `${props.baseUrl}/manifests/${props.tape}/manifest.verified.json`)

function normalisePath(raw: string | undefined, index: number): string {
  if (!raw) return `${props.baseUrl}/keyframes/${props.tape}/frame-${String(index + 1).padStart(3, '0')}.png`
  if (raw.startsWith('http') || raw.startsWith('/')) return raw
  return `${props.baseUrl}/keyframes/${props.tape}/${raw}`
}

onMounted(async () => {
  const urls = [
    `${props.baseUrl}/manifests/${props.tape}/manifest.verified.json`,
    `${props.baseUrl}/manifests/${props.tape}/manifest.json`
  ]
  for (const url of urls) {
    try {
      const res = await fetch(url)
      if (!res.ok) continue
      const ct = res.headers.get('content-type') || ''
      if (!ct.includes('json')) continue
      const m = await res.json()
      if (Array.isArray(m.steps)) {
        keyframesData.value = m.steps.slice(0, props.maxKeyframes).map((s: any, i: number) => ({
          path: normalisePath(s.screenshot_path, s.index ?? i),
          alt: s.intent || s.slug || `Step ${s.index ?? i + 1}`
        }))
      }
      return
    } catch { /* try next */ }
  }
})
</script>

<style scoped>
.recording-embed { margin: 24px 0; padding: 16px; border: 1px solid var(--vp-divider, #e5e7eb); border-radius: 8px; background: var(--vp-c-bg-soft, #fafafa); }
.recording-figure { margin: 0 0 20px; text-align: center; }
.recording-video, .recording-image { max-width: 100%; height: auto; border-radius: 6px; display: block; margin: 0 auto; }
.recording-caption { margin-top: 8px; font-size: 13px; color: var(--vp-c-text-2, #555); font-style: italic; }
.keyframes-section { margin: 20px 0; padding: 12px; border: 1px solid var(--vp-divider, #e5e7eb); border-radius: 6px; }
.keyframes-title { cursor: pointer; font-weight: 600; }
.keyframes-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 12px; margin-top: 12px; }
.keyframe-item { border: 1px solid var(--vp-divider, #e5e7eb); border-radius: 4px; overflow: hidden; }
.keyframe-item img { display: block; width: 100%; height: auto; aspect-ratio: 16/9; object-fit: cover; }
.keyframe-caption { padding: 8px; margin: 0; font-size: 11px; color: var(--vp-c-text-2, #555); line-height: 1.3; }
.keyframes-empty { font-size: 13px; color: var(--vp-c-text-3, #777); }
</style>
