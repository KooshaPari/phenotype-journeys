<template>
  <div class="journey-viewer">
    <template v-if="manifest">
      <h2 class="journey-title">{{ manifest.title }}</h2>
      <div class="journey-intent">
        {{ manifest.intent || 'Journey demonstration' }}
      </div>

      <div v-if="recordingUrl || recordingRichUrl" class="journey-section journey-recording">
        <div v-if="recordingRichUrl" class="journey-recording-toggle" role="tablist">
          <button
            type="button"
            role="tab"
            :aria-selected="recordingMode === 'rich'"
            :class="['journey-recording-tab', recordingMode === 'rich' && 'active']"
            @click="recordingMode = 'rich'"
          >Rich</button>
          <button
            type="button"
            role="tab"
            :aria-selected="recordingMode === 'raw'"
            :class="['journey-recording-tab', recordingMode === 'raw' && 'active']"
            @click="recordingMode = 'raw'"
            :disabled="!recordingUrl"
          >Raw</button>
        </div>
        <video
          class="journey-recording-video"
          :src="activeRecordingUrl"
          :poster="recordingPoster || undefined"
          controls
          preload="metadata"
          playsinline
        >
          <template v-if="recordingGifUrl">
            Sorry, your browser doesn't support embedded video.
            <a :href="recordingGifUrl">Watch the animated preview instead.</a>
          </template>
        </video>
        <div class="journey-recording-meta">
          <a v-if="recordingRichUrl" class="journey-recording-link" :href="recordingRichUrl" target="_blank" rel="noopener">Open Rich MP4 Γåù</a>
          <a v-if="recordingUrl" class="journey-recording-link" :href="recordingUrl" target="_blank" rel="noopener">Open Raw MP4 Γåù</a>
          <a v-if="recordingGifUrl" class="journey-recording-link" :href="recordingGifUrl" target="_blank" rel="noopener">Open GIF Γåù</a>
        </div>
      </div>

      <div v-if="enrichedKeyframes.length" class="journey-section">
        <KeyframeGallery
          :keyframes="enrichedKeyframes"
          :title="manifest.title"
          :journey-id="manifest.id || manifest.title || ''"
        />
      </div>

      <div class="journey-section">
        <div
          :class="['journey-status', manifest.pass ? 'pass' : 'fail']"
        >
          <div class="journey-status-badge">
            {{ manifest.pass ? 'PASS' : 'FAIL' }}
          </div>
          <span>{{ manifest.pass ? 'Journey completed successfully' : 'Journey encountered issues' }}</span>
        </div>
      </div>

      <div v-if="manifest.steps && manifest.steps.length > 0">
        <table class="journey-steps">
          <thead>
            <tr>
              <th>Step</th>
              <th>Slug</th>
              <th>Intent</th>
              <th>Screenshot</th>
              <th>Verification</th>
              <th>Score</th>
            </tr>
          </thead>
          <tbody>
            <JourneyStep
              v-for="(step, idx) in manifest.steps"
              :key="idx"
              :step="step"
              :index="idx"
            />
          </tbody>
        </table>
      </div>

      <div v-if="manifest.error" style="margin-top: 20px; padding: 12px; background-color: rgba(239, 68, 68, 0.1); border-left: 4px solid #ef4444; border-radius: 4px;">
        <div style="color: #ef4444; font-weight: 600; margin-bottom: 6px;">Error</div>
        <code style="color: #d1d5db; font-size: 12px; white-space: pre-wrap; overflow-x: auto;">{{ manifest.error }}</code>
      </div>
    </template>

    <template v-else>
      <div style="padding: 20px; text-align: center; color: var(--vp-c-text-3);">
        <p>Journey not yet recorded.</p>
        <p style="font-size: 13px; margin-top: 12px;">Run the journey recorder to capture interactions:</p>
        <code style="display: inline-block; background-color: var(--vp-c-bg-mute); padding: 8px 12px; border-radius: 4px; margin-top: 12px; font-size: 12px;">./apps/macos/HwLedgerUITests/scripts/run-journeys.sh</code>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import KeyframeGallery from './KeyframeGallery.vue'
import JourneyStep from './JourneyStep.vue'

interface Annotation {
  bbox: [number, number, number, number]
  label: string
  color?: string | null
  style?: 'solid' | 'dashed'
  note?: string | null
  kind?: 'region' | 'pointer' | 'highlight'
}

interface AgreementReport {
  status: 'green' | 'yellow' | 'red'
  overlap: number
  intent_tokens?: string[]
  blind_tokens?: string[]
  missing_in_blind: string[]
  extras_in_blind: string[]
}

interface Manifest {
  id?: string
  title: string
  intent: string
  pass: boolean
  /** legacy bool OR (new) relative mp4 path like `recordings/<id>.mp4` */
  recording: boolean | string
  recording_gif?: string
  /** Path to the enriched MP4 (rendered by Remotion). When present, the
   *  viewer shows a Rich/Raw toggle defaulting to Rich. */
  recording_rich?: string
  /**
   * Optional override: path (relative to the journey root) where per-step
   * `screenshot_path` frames live. Defaults to `keyframes/<id>/` (CLI layout);
   * streamlit-journey manifests override to `recordings/<id>/` because
   * their frames are co-located with the recording.
   */
  keyframes_root?: string
  keyframes?: Array<{ path: string; caption: string; blind_description?: string | null; annotations?: Annotation[] | null; agreement?: AgreementReport | null }>
  steps?: Array<{
    index?: number
    slug: string
    intent: string
    screenshot?: string
    screenshot_path?: string
    description?: string
    blind_description?: string
    judge_score?: number
    annotations?: Annotation[] | null
    agreement?: AgreementReport | null
  }>
  error?: string
}

const props = defineProps<{
  manifest?: string | object
}>()

const manifest = ref<Manifest | null>(null)

/**
 * Merge `steps[].annotations` onto `keyframes[i]` by position (best-effort).
 * Legacy hwLedger manifests have separate `keyframes` + `steps` arrays ΓÇö
 * we pair them by caption frame-number or by array index fallback.
 */
/**
 * Synthesize keyframes from either a top-level `keyframes[]` array (legacy
 * hwLedger manifests) or from `steps[]` (canonical phenotype-journeys format
 * that only tracks `screenshot_path` per step).
 */
function resolveAsset(rel: string | undefined | null): string {
  if (!rel) return ''
  if (/^https?:|^\//.test(rel)) return rel
  return manifestUrlBase.value ? `${manifestUrlBase.value}/${rel}` : rel
}

const recordingUrl = computed(() => {
  const m = manifest.value
  if (!m) return ''
  return typeof m.recording === 'string' ? resolveAsset(m.recording) : ''
})
const recordingRichUrl = computed(() => resolveAsset(manifest.value?.recording_rich))
const recordingGifUrl = computed(() => resolveAsset(manifest.value?.recording_gif))

type RecordingMode = 'rich' | 'raw'
const recordingMode = ref<RecordingMode>('rich')
const activeRecordingUrl = computed(() => {
  if (recordingMode.value === 'rich' && recordingRichUrl.value) return recordingRichUrl.value
  return recordingUrl.value || recordingRichUrl.value
})
const recordingPoster = computed(() => {
  const m = manifest.value
  if (!m) return ''
  // Prefer the first keyframe as poster; fall back to GIF.
  const firstKf = (m.keyframes && m.keyframes[0]?.path)
    || (m.steps && m.steps.find((s) => s.screenshot_path || s.screenshot))
  if (!firstKf) return recordingGifUrl.value
  if (typeof firstKf === 'string') return resolveAsset(firstKf)
  const step = firstKf as any
  const id = m.id || m.title || ''
  return resolveAsset(`${m.keyframes_root || `keyframes/${id}`}/${step.screenshot_path || step.screenshot}`)
})

const enrichedKeyframes = computed(() => {
  const m = manifest.value
  if (!m) return []
  const steps = m.steps || []
  if (m.keyframes && m.keyframes.length) {
    return m.keyframes.map((kf, i) => {
      const step = steps[i]
      return {
        ...kf,
        annotations: kf.annotations ?? step?.annotations ?? null,
        blind_description: kf.blind_description ?? step?.blind_description ?? null,
        agreement: kf.agreement ?? step?.agreement ?? null,
      }
    })
  }
  // Derive from steps[]: path = "<dir>/<screenshot_path>" relative to the
  // manifest URL's directory (public/cli-journeys/keyframes/<id>/).
  if (!manifestUrlBase.value) return []
  const id = m.id || m.title || ''
  return steps
    .filter((s) => s.screenshot_path || s.screenshot)
    .map((s) => ({
      path: `${manifestUrlBase.value}/${m.keyframes_root || `keyframes/${id}`}/${s.screenshot_path || s.screenshot}`,
      caption: s.intent,
      blind_description: s.blind_description ?? null,
      annotations: s.annotations ?? null,
      agreement: s.agreement ?? null,
    }))
})

/**
 * Base URL for resolving keyframe paths. For a manifest loaded from
 * `/cli-journeys/manifests/plan-deepseek/manifest.verified.json`, this
 * returns `/cli-journeys`.
 */
const manifestUrlBase = ref<string>('')

onMounted(async () => {
  if (!props.manifest) {
    return
  }

  if (typeof props.manifest === 'object') {
    manifest.value = props.manifest as Manifest
  } else if (typeof props.manifest === 'string') {
    // Derive base dir: `/cli-journeys/manifests/<id>/manifest.verified.json`
    // -> `/cli-journeys`.
    const url = props.manifest
    const m = url.match(/^(.*?)\/manifests\/[^/]+\/manifest(\.verified)?\.json$/)
    manifestUrlBase.value = m ? m[1] : url.replace(/\/[^/]+$/, '')
    try {
      const response = await fetch(url)
      manifest.value = await response.json()
    } catch (error) {
      console.error('Failed to load journey manifest:', error)
      manifest.value = null
    }
  }
})
</script>

<style scoped>
.journey-viewer {
  border: 1px solid var(--vp-divider);
  border-radius: 8px;
  padding: 24px;
  margin: 24px 0;
  background-color: var(--vp-c-bg-soft);
}

.journey-title {
  font-size: 24px;
  font-weight: 600;
  margin: 0 0 12px 0;
  color: var(--color-accent);
}

.journey-intent {
  font-size: 14px;
  color: var(--vp-c-text-2);
  margin-bottom: 20px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--vp-divider);
}

.journey-section {
  margin: 20px 0;
}

.journey-recording-video {
  width: 100%;
  max-height: 70vh;
  border-radius: 6px;
  background: #000;
  display: block;
}

.journey-recording-meta {
  display: flex;
  gap: 14px;
  margin-top: 6px;
  font-size: 12px;
}

.journey-recording-link {
  color: var(--color-accent, #89b4fa);
  text-decoration: none;
  border-bottom: 1px dotted currentColor;
}
.journey-recording-link:hover { border-bottom-style: solid; }

.journey-recording-toggle {
  display: inline-flex;
  gap: 2px;
  margin-bottom: 10px;
  border: 1px solid var(--vp-divider);
  border-radius: 6px;
  overflow: hidden;
  background: var(--vp-c-bg-mute);
}

.journey-recording-tab {
  padding: 6px 14px;
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0.4px;
  text-transform: uppercase;
  background: transparent;
  color: var(--vp-c-text-2);
  border: none;
  cursor: pointer;
  font-family: inherit;
  transition: background 120ms ease, color 120ms ease;
}
.journey-recording-tab:hover:not(:disabled) {
  background: var(--vp-c-bg-soft);
  color: var(--vp-c-text-1);
}
.journey-recording-tab:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
.journey-recording-tab.active {
  background: var(--color-accent, #89b4fa);
  color: #0a0a0f;
}

.journey-status {
  display: flex;
  gap: 12px;
  padding: 12px;
  border-radius: 6px;
  align-items: center;
}

.journey-status.pass {
  background-color: rgba(16, 185, 129, 0.1);
  border-left: 4px solid #10b981;
}

.journey-status.fail {
  background-color: rgba(239, 68, 68, 0.1);
  border-left: 4px solid #ef4444;
}

.journey-status-badge {
  font-weight: 600;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.journey-status.pass .journey-status-badge {
  color: #10b981;
}

.journey-status.fail .journey-status-badge {
  color: #ef4444;
}
</style>
