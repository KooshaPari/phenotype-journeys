<template>
  <div class="recording-embed">
    <figure class="recording-figure">
      <video
        v-if="useMP4"
        class="recording-video"
        controls
        :autoplay="autoplay"
        muted
        loop
        loading="lazy"
      >
        <source :src="mp4Path" type="video/mp4" />
        <img :src="gifPath" :alt="caption || tape" loading="lazy" />
      </video>
      <img
        v-else
        class="recording-image"
        :src="gifPath"
        :alt="caption || tape"
        loading="lazy"
      />
      <figcaption v-if="caption" class="recording-caption">
        {{ caption }}
      </figcaption>
    </figure>

    <details class="keyframes-section" :open="keyframesData.length > 0">
      <summary class="keyframes-title">
        Keyframes ({{ keyframesData.length }} ΓÇö VLM-friendly, click to expand)
      </summary>
      <div v-if="keyframesData.length > 0" class="keyframes-gallery-host">
        <!--
          Delegate rendering to KeyframeGallery so thumbnails share the same
          scroll/collapse caption treatment, Intent/Blind dual labels, and
          lightbox (arrow nav, ESC, zoom, copy-JSON) that JourneyViewer uses.
        -->
        <KeyframeGallery
          :keyframes="keyframesData"
          :journey-id="tape"
          :title="caption || tape"
        />
      </div>
      <p v-else-if="manifestMissing" class="keyframes-empty">
        No verified manifest yet ΓÇö this journey ran but hasn't been
        blackbox-verified. The raw keyframes are in
        <a :href="keyframesRepoUrl" target="_blank" rel="noopener">the repo</a>.
      </p>
      <p v-else class="keyframes-empty">Loading manifestΓÇª</p>
    </details>

    <div class="recording-links">
      <a :href="mp4Path" download class="recording-link">Download MP4</a>
      <a :href="gifPath" download class="recording-link">Download GIF</a>
      <a :href="keyframesRepoUrl" target="_blank" rel="noopener" class="recording-link">
        Keyframes (repo)
      </a>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { withBase, useData } from 'vitepress'
import KeyframeGallery from './KeyframeGallery.vue'

interface Annotation {
  bbox: [number, number, number, number]
  label: string
  color?: string | null
  style?: 'solid' | 'dashed'
  note?: string | null
  kind?: 'region' | 'pointer' | 'highlight'
}

interface KeyframeData {
  path: string
  caption: string
  blind_description?: string | null
  annotations?: Annotation[] | null
}

const props = withDefaults(
  defineProps<{
    tape: string
    caption?: string
    autoplay?: boolean
    // Max keyframes to render in the gallery. Default 12 keeps the VLM
    // context budget under ~30k tokens per journey.
    maxKeyframes?: number
    // Which journey family this tape belongs to. Defaults to cli; override
    // to "streamlit" or "gui" for those recordings.
    kind?: 'cli' | 'streamlit' | 'gui'
  }>(),
  {
    autoplay: false,
    maxKeyframes: 12,
    kind: 'cli'
  }
)

// Search roots in order when probing for a manifest / recording. If a
// journey was misfiled (e.g. a streamlit tape named like a cli one), this
// still resolves rather than surfacing the "No verified manifest" fallback.
const SEARCH_ROOTS = ['cli-journeys', 'streamlit-journeys', 'gui-journeys']
const primaryRoot = computed(() => {
  switch (props.kind) {
    case 'streamlit': return 'streamlit-journeys'
    case 'gui': return 'gui-journeys'
    default: return 'cli-journeys'
  }
})
const orderedRoots = computed(() => {
  const p = primaryRoot.value
  return [p, ...SEARCH_ROOTS.filter((r) => r !== p)]
})

const { site } = useData()
const keyframesData = ref<KeyframeData[]>([])
const manifestMissing = ref(false)

// base-aware URL helpers. VitePress's withBase() handles the leading
// `/hwLedger/` prefix in production so assets resolve correctly.
// Streamlit + GUI recordings live under <root>/recordings/<tape>/<tape>.{mp4,gif}
// whereas CLI tapes are flat at <root>/recordings/<tape>.{mp4,gif}. Resolve
// per-kind.
function recPath(ext: 'mp4' | 'gif'): string {
  const root = primaryRoot.value
  if (root === 'cli-journeys') {
    return withBase(`/${root}/recordings/${props.tape}.${ext}`)
  }
  return withBase(`/${root}/recordings/${props.tape}/${props.tape}.${ext}`)
}
const gifPath = computed(() => recPath('gif'))
const mp4Path = computed(() => recPath('mp4'))
// No pre-built ZIPs exist ΓÇö link to the keyframes directory listing instead.
const keyframesRepoUrl = computed(() => {
  const dir = primaryRoot.value === 'cli-journeys'
    ? `apps/cli-journeys/keyframes/${props.tape}`
    : primaryRoot.value === 'streamlit-journeys'
      ? `apps/streamlit/journeys/recordings/${props.tape}`
      : `apps/macos/HwLedgerUITests/journeys/${props.tape}`
  return `https://github.com/KooshaPari/hwLedger/tree/main/${dir}`
})

const useMP4 = computed(() => {
  return typeof window !== 'undefined' && 'videoWidth' in document.createElement('video')
})

function normaliseFramePath(raw: string | undefined, index: number): string {
  if (!raw) {
    return withBase(
      `/cli-journeys/keyframes/${props.tape}/frame-${String(index + 1).padStart(3, '0')}.png`
    )
  }
  // Manifests sometimes store paths like "step-000-ΓÇªpng" (WP25 UI harness
  // format) or "frame-001.png" (WP26 CLI format); both are under the tape
  // directory. If the path is absolute (starts with /) or already fully
  // qualified, pass through withBase unchanged.
  if (raw.startsWith('http')) {
    return raw
  }
  if (raw.startsWith('/')) {
    return withBase(raw)
  }
  return withBase(`/cli-journeys/keyframes/${props.tape}/${raw}`)
}

// VitePress dev server returns index.html (200 OK) for missing paths, so
// `response.ok` is not sufficient ΓÇö we must also reject HTML responses.
function isJsonResponse(response: Response): boolean {
  const ct = response.headers.get('content-type') || ''
  return ct.includes('application/json') || ct.includes('json')
}

onMounted(async () => {
  try {
    const tryFetch = async (path: string): Promise<Response | null> => {
      const res = await fetch(withBase(path))
      if (!res.ok) return null
      if (!isJsonResponse(res)) return null
      return res
    }

    let response = await tryFetch(
      `/cli-journeys/manifests/${props.tape}/manifest.verified.json`
    )
    if (!response) {
      response = await tryFetch(
        `/cli-journeys/manifests/${props.tape}/manifest.json`
      )
    }
    if (response) {
      const manifest = await response.json()
      if (manifest.steps && Array.isArray(manifest.steps)) {
        keyframesData.value = manifest.steps
          .slice(0, props.maxKeyframes)
          .map((step: any, index: number) => ({
            path: normaliseFramePath(step.screenshot_path, step.index ?? index),
            caption: step.intent || step.slug || `Step ${step.index ?? index + 1}`,
            blind_description: step.blind_description ?? null,
            annotations: step.annotations ?? null,
          }))
      }
    } else {
      manifestMissing.value = true
    }
  } catch (error) {
    manifestMissing.value = true
    // eslint-disable-next-line no-console
    console.warn(`Could not load manifest for ${props.tape}:`, error)
  }
})
</script>

<style scoped>
.recording-embed {
  margin: 24px 0;
  padding: 16px;
  border: 1px solid var(--vp-divider);
  border-radius: 8px;
  background-color: var(--vp-c-bg-soft);
}

.recording-figure {
  margin: 0 0 20px 0;
  padding: 0;
  text-align: center;
}

.recording-video,
.recording-image {
  max-width: 100%;
  height: auto;
  border-radius: 6px;
  display: block;
  margin: 0 auto;
  background-color: var(--vp-c-bg-mute);
}

.recording-caption {
  margin-top: 8px;
  font-size: 13px;
  color: var(--vp-c-text-2);
  font-style: italic;
}

.keyframes-section {
  margin: 20px 0;
  padding: 12px;
  border: 1px solid var(--vp-divider);
  border-radius: 6px;
  background-color: var(--vp-c-bg-mute);
}

.keyframes-title {
  cursor: pointer;
  font-weight: 600;
  color: var(--vp-c-text-1);
  padding: 4px 0;
  user-select: none;
}

.keyframes-title:hover {
  color: var(--color-accent);
}

.keyframes-gallery-host {
  margin-top: 8px;
}

.keyframes-empty {
  margin-top: 12px;
  font-size: 13px;
  color: var(--vp-c-text-3);
}

.recording-links {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  margin-top: 16px;
}

.recording-link {
  display: inline-block;
  padding: 6px 12px;
  font-size: 13px;
  border: 1px solid var(--vp-divider);
  border-radius: 4px;
  background-color: var(--vp-c-bg-mute);
  color: var(--color-accent);
  text-decoration: none;
  transition: all 0.3s ease;
}

.recording-link:hover {
  background-color: var(--color-accent);
  color: white;
  border-color: var(--color-accent);
}

@media (prefers-color-scheme: dark) {
  .recording-embed {
    background-color: rgba(255, 255, 255, 0.05);
  }

  .keyframes-section {
    background-color: rgba(255, 255, 255, 0.02);
  }
}
</style>
