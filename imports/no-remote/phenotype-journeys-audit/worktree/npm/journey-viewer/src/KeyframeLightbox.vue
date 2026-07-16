<template>
  <Teleport to="body">
    <Transition name="kf-lightbox">
      <div
        v-if="open"
        ref="overlayEl"
        class="kf-lightbox-overlay"
        role="dialog"
        aria-modal="true"
        :aria-label="`Keyframe ${index + 1} of ${frames.length}`"
        tabindex="-1"
        @click.self="close"
        @keydown="onKey"
      >
        <div
          class="kf-lightbox-inner"
          :class="{ 'caption-expanded': captionExpanded }"
          @click.stop
        >
          <div
            class="kf-image-wrap"
            :class="[`zoom-${zoomMode}`]"
            :style="zoomMode === 'actual' ? { overflow: 'auto', maxWidth: '90vw' } : undefined"
          >
            <img
              ref="imgEl"
              class="kf-image"
              :class="[`zoom-${zoomMode}`]"
              :src="displayedFramePath"
              :alt="currentFrame.caption"
              :title="zoomMode === 'fit' ? 'Click to zoom to 1:1 (or press +)' : 'Click to fit (or press 0)'"
              @load="onImgLoad"
              @error="onImgError"
              @click="toggleZoom"
            />
            <svg
              v-if="showAnnotations && !annotationsBakedOn && annotations.length && natW && natH"
              class="kf-annot-svg"
              :viewBox="`0 0 ${natW} ${natH}`"
              preserveAspectRatio="xMidYMid meet"
            >
              <g
                v-for="(a, i) in annotations"
                :key="i"
                :class="['kf-annot-g', { active: hoverIdx === i }]"
                @mouseenter="hoverIdx = i"
                @mouseleave="hoverIdx = null"
              >
                <rect
                  :x="a.bbox[0]"
                  :y="a.bbox[1]"
                  :width="a.bbox[2]"
                  :height="a.bbox[3]"
                  :stroke="paletteFor(a, i)"
                  :stroke-dasharray="(a.style === 'dashed') ? '6 4' : undefined"
                  fill="none"
                  stroke-width="2"
                  rx="2"
                />
                <title v-if="a.note">{{ a.note }}</title>
              </g>
            </svg>
            <div
              v-if="showAnnotations && !annotationsBakedOn && annotations.length"
              class="kf-label-layer"
              :style="{ aspectRatio: `${natW} / ${natH}` }"
            >
              <div
                v-for="(a, i) in annotations"
                :key="i"
                :class="['kf-label-pill', { active: hoverIdx === i }]"
                :style="labelStyle(a)"
                :title="a.note || ''"
                @mouseenter="hoverIdx = i"
                @mouseleave="hoverIdx = null"
              >
                <span class="kf-label-dot" :style="{ background: paletteFor(a, i) }" />
                {{ a.label }}
              </div>
            </div>
          </div>

          <div
            class="kf-caption"
            :class="{ 'is-expanded': captionExpanded }"
            tabindex="0"
          >
            <div class="kf-caption-scroll">
              <div class="kf-caption-row">
                <span
                  class="kf-caption-label kf-caption-label-intent"
                  title="Author: human ΓÇö what the step is meant to demonstrate (from intents.yaml)"
                >
                  <span class="kf-caption-label-glyph" aria-hidden="true">Γ£ì∩╕Ä</span>
                  Intent
                </span>
                <button
                  v-if="agreement"
                  class="kf-agreement-chip"
                  :class="['kf-agreement-' + agreement.status]"
                  :aria-expanded="agreementOpen"
                  :title="agreementTooltip(agreement)"
                  @click="agreementOpen = !agreementOpen"
                >
                  <span aria-hidden="true">{{ agreementGlyph(agreement.status) }}</span>
                  <span>{{ agreementPct(agreement) }} overlap</span>
                </button>
                <span class="kf-caption-text">{{ currentFrame.caption || 'ΓÇö' }}</span>
              </div>
              <div class="kf-caption-row kf-caption-row-blind">
                <span
                  class="kf-caption-label kf-caption-label-blind"
                  title="Author: VLM blind evaluator ΓÇö what the judge independently saw in the frame (no caption context)"
                >
                  <span class="kf-caption-label-glyph" aria-hidden="true">Γùë</span>
                  Blind
                </span>
                <span class="kf-caption-text kf-caption-text-blind">{{ currentFrame.blind_description || 'ΓÇö' }}</span>
              </div>
              <div
                v-if="agreement && agreementOpen"
                class="kf-agreement-panel"
                :class="['kf-agreement-panel-' + agreement.status]"
                role="region"
                aria-label="Intent / blind agreement diff"
              >
                <div class="kf-agreement-section">
                  <div class="kf-agreement-remedy">
                    Remediation: re-record this step OR rewrite intent.yaml for frame {{ index + 1 }}
                  </div>
                  <div class="kf-agreement-title">Missing in Blind</div>
                  <div v-if="agreement.missing_in_blind.length" class="kf-agreement-tokens">
                    <span
                      v-for="t in agreement.missing_in_blind"
                      :key="'m-' + t"
                      class="kf-agreement-token kf-agreement-token-missing"
                    >{{ t }}</span>
                  </div>
                  <div v-else class="kf-agreement-empty">ΓÇö none ΓÇö</div>
                </div>
                <div class="kf-agreement-section">
                  <div class="kf-agreement-remedy">
                    Remediation: re-record this step OR rewrite intent.yaml for frame {{ index + 1 }}
                  </div>
                  <div class="kf-agreement-title">Extras in Blind</div>
                  <div v-if="agreement.extras_in_blind.length" class="kf-agreement-tokens">
                    <span
                      v-for="t in agreement.extras_in_blind"
                      :key="'e-' + t"
                      class="kf-agreement-token kf-agreement-token-extra"
                    >{{ t }}</span>
                  </div>
                  <div v-else class="kf-agreement-empty">ΓÇö none ΓÇö</div>
                </div>
              </div>
            </div>
            <div class="kf-caption-footer">
              <span class="kf-meta">frame {{ index + 1 }} / {{ frames.length }} ┬╖ {{ journeyId }}</span>
              <button
                class="kf-caption-toggle"
                :aria-expanded="captionExpanded"
                @click="captionExpanded = !captionExpanded"
              >
                {{ captionExpanded ? 'Show less Γû┤' : 'Show more Γû╛' }}
              </button>
            </div>
          </div>

          <button
            class="kf-nav kf-prev"
            @click="prev"
            :disabled="index === 0"
            aria-label="Previous frame"
          >ΓÇ╣</button>
          <button
            class="kf-nav kf-next"
            @click="next"
            :disabled="index === frames.length - 1"
            aria-label="Next frame"
          >ΓÇ║</button>

          <div class="kf-toolbar">
            <button
              class="kf-btn"
              @click="toggleZoom"
              :aria-pressed="zoomMode === 'actual'"
              :title="zoomMode === 'fit' ? 'Zoom to 1:1 (+)' : 'Fit to window (0)'"
            >
              Zoom: {{ zoomMode === 'fit' ? 'fit' : '1:1' }}
            </button>
            <button class="kf-btn" @click="toggleAnnotations" :aria-pressed="showAnnotations">
              Annotations: {{ showAnnotations ? 'on' : 'off' }}
            </button>
            <button
              v-if="hasBakedFrame"
              class="kf-btn"
              @click="toggleBaked"
              :aria-pressed="annotationsBakedOn"
              :title="annotationsBakedOn ? 'Showing pre-baked keyframe with bbox overlays rendered in' : 'Showing raw keyframe with live SVG overlay'"
            >
              Annotations baked: {{ annotationsBakedOn ? 'on' : 'off' }}
            </button>
            <button
              class="kf-btn"
              :disabled="!annotations.length"
              @click="copyAnnotationsJson"
              :title="'Copy annotations for frame ' + (index + 1) + ' as JSON'"
            >
              {{ copyLabel }}
            </button>
            <button
              class="kf-btn"
              :disabled="!hasStructural"
              :aria-pressed="showStructural"
              :title="hasStructural ? 'Toggle structural snapshot (a11y tree / ARIA / terminal buffer)' : 'No structural sibling available for this frame'"
              @click="toggleStructural"
            >
              Structural: {{ showStructural ? 'on' : 'off' }}
            </button>
            <button class="kf-btn kf-close" @click="close" aria-label="Close">Γ£ò</button>
          </div>

          <div v-if="showStructural && hasStructural" class="kf-structural-rail">
            <StructuralPane :path="currentFrame.structural_path ?? null" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue'
import StructuralPane from './StructuralPane.vue'

interface Annotation {
  bbox: [number, number, number, number]
  label: string
  color?: string | null
  style?: 'solid' | 'dashed'
  note?: string | null
  kind?: 'region' | 'pointer' | 'highlight'
}

interface Agreement {
  status: 'green' | 'yellow' | 'red'
  overlap: number
  raw_score?: number
  backend?: string
  backend_model?: string | null
  intent_tokens?: string[]
  blind_tokens?: string[]
  missing_in_blind: string[]
  extras_in_blind: string[]
}

interface Frame {
  path: string
  caption: string
  blind_description?: string | null
  annotations?: Annotation[] | null
  agreement?: Agreement | null
  /** Tier 0 structural-capture sibling URL (optional). */
  structural_path?: string | null
}

const props = defineProps<{
  open: boolean
  frames: Frame[]
  index: number
  journeyId: string
}>()

const emit = defineEmits<{
  (e: 'update:index', v: number): void
  (e: 'close'): void
}>()

const PALETTE = [
  '#f38ba8', '#a6e3a1', '#f9e2af', '#89b4fa', '#cba6f7', '#94e2d5', '#fab387',
]

const overlayEl = ref<HTMLElement | null>(null)
const imgEl = ref<HTMLImageElement | null>(null)
const natW = ref(0)
const natH = ref(0)
const hoverIdx = ref<number | null>(null)
const copyLabel = ref('Copy JSON')

const STORAGE_KEY = 'phenotype-journey:annotations-on'
const BAKED_KEY = 'phenotype-journey:annotations-baked-on'
const STRUCTURAL_KEY = 'phenotype-journey:structural-on'
const showAnnotations = ref(true)
const annotationsBakedOn = ref(true)
const showStructural = ref(false)
const bakedFrameMissing = ref<Record<number, boolean>>({})
try {
  const stored = localStorage.getItem(STORAGE_KEY)
  if (stored === '0') showAnnotations.value = false
  const bakedStored = localStorage.getItem(BAKED_KEY)
  if (bakedStored === '0') annotationsBakedOn.value = false
  const structStored = localStorage.getItem(STRUCTURAL_KEY)
  if (structStored === '1') showStructural.value = true
} catch {}
function toggleStructural() {
  showStructural.value = !showStructural.value
  try { localStorage.setItem(STRUCTURAL_KEY, showStructural.value ? '1' : '0') } catch {}
}

function bakedPathFor(p: string): string {
  // Replace trailing `.png`/`.jpg`/`.jpeg`/`.webp` with `.annotated.<ext>`.
  return p.replace(/\.(png|jpe?g|webp)(\?|#|$)/i, '.annotated.$1$2')
}
const hasBakedFrame = computed<boolean>(() => {
  const f = currentFrame.value
  if (!f || !f.annotations || f.annotations.length === 0) return false
  return !bakedFrameMissing.value[props.index]
})
const displayedFramePath = computed<string>(() => {
  const f = currentFrame.value
  if (!f || !f.path) return ''
  if (annotationsBakedOn.value && hasBakedFrame.value) {
    return bakedPathFor(f.path)
  }
  return f.path
})
function onImgError() {
  // If the baked variant fails to load (missing in dev or incomplete bake),
  // remember it for this frame index and fall back to the raw image.
  if (annotationsBakedOn.value && hasBakedFrame.value) {
    bakedFrameMissing.value = { ...bakedFrameMissing.value, [props.index]: true }
  }
}
function toggleBaked() {
  annotationsBakedOn.value = !annotationsBakedOn.value
  try { localStorage.setItem(BAKED_KEY, annotationsBakedOn.value ? '1' : '0') } catch {}
}

const captionExpanded = ref(false)

// Zoom state ΓÇö "fit" scales the image into the viewport (default);
// "actual" renders the image at its natural pixel size and lets the
// wrapper scroll for panning. Reset to "fit" on frame change and on open.
type ZoomMode = 'fit' | 'actual'
const zoomMode = ref<ZoomMode>('fit')
function toggleZoom() {
  zoomMode.value = zoomMode.value === 'fit' ? 'actual' : 'fit'
}

const currentFrame = computed<Frame>(() => props.frames[props.index] || { path: '', caption: '', blind_description: null, annotations: [], agreement: null, structural_path: null })
const hasStructural = computed<boolean>(() => !!currentFrame.value.structural_path)
const annotations = computed<Annotation[]>(() => currentFrame.value.annotations || [])
const agreement = computed<Agreement | null>(() => currentFrame.value.agreement || null)
const agreementOpen = ref(false)
function agreementGlyph(s: 'green'|'yellow'|'red'): string {
  return s === 'green' ? '≡ƒƒó' : s === 'yellow' ? '≡ƒƒí' : '≡ƒö┤'
}
function agreementPct(a: Agreement): string {
  return `${Math.round((a.overlap || 0) * 100)}%`
}
function backendLabel(b?: string): string {
  if (!b) return 'Jaccard'
  if (b.startsWith('jaccard-fallback')) return 'Jaccard (fallback)'
  if (b === 'siglip') return 'SigLIP'
  if (b === 'sentence-transformer') return 'Sentence'
  if (b === 'jaccard') return 'Jaccard'
  return b
}
function agreementTooltip(a: Agreement): string {
  const label = backendLabel(a.backend)
  const raw = (a.raw_score ?? a.overlap ?? 0).toFixed(2)
  return `Agreement: ${a.status.toUpperCase()} ΓÇö ${label} ${raw} (${agreementPct(a)}). Click for diff.`
}
watch(() => props.index, () => { agreementOpen.value = false })

function paletteFor(a: Annotation, i: number): string {
  return a.color || PALETTE[i % PALETTE.length]
}

function labelStyle(a: Annotation) {
  const [x, y, _w, h] = a.bbox
  // flip to bottom when top label would clip (y < 24 px in source units).
  const below = y < 24
  const top = below ? (y + h + 4) : (y - 22)
  return {
    left: (x / (natW.value || 1) * 100) + '%',
    top: (top / (natH.value || 1) * 100) + '%',
  }
}

function onImgLoad() {
  const el = imgEl.value
  if (!el) return
  natW.value = el.naturalWidth
  natH.value = el.naturalHeight
}

function next() {
  if (props.index < props.frames.length - 1) emit('update:index', props.index + 1)
}
function prev() {
  if (props.index > 0) emit('update:index', props.index - 1)
}
function close() { emit('close') }
function onKey(e: KeyboardEvent) {
  if (e.key === 'Escape') { e.preventDefault(); close() }
  else if (e.key === 'ArrowRight') { e.preventDefault(); next() }
  else if (e.key === 'ArrowLeft') { e.preventDefault(); prev() }
  else if (e.key === '+' || e.key === '=') { e.preventDefault(); zoomMode.value = 'actual' }
  else if (e.key === '-' || e.key === '_') { e.preventDefault(); zoomMode.value = 'fit' }
  else if (e.key === '0') { e.preventDefault(); zoomMode.value = 'fit' }
}

function toggleAnnotations() {
  showAnnotations.value = !showAnnotations.value
  try { localStorage.setItem(STORAGE_KEY, showAnnotations.value ? '1' : '0') } catch {}
}

async function copyAnnotationsJson() {
  const payload = {
    journey: props.journeyId,
    frame: props.index + 1,
    screenshot: currentFrame.value.path,
    annotations: annotations.value,
  }
  const text = JSON.stringify(payload, null, 2)
  try {
    await navigator.clipboard.writeText(text)
    copyLabel.value = 'Copied Γ£ô'
  } catch {
    // Fallback: select a textarea.
    const ta = document.createElement('textarea')
    ta.value = text
    document.body.appendChild(ta)
    ta.select()
    document.execCommand('copy')
    document.body.removeChild(ta)
    copyLabel.value = 'Copied Γ£ô'
  }
  setTimeout(() => { copyLabel.value = 'Copy JSON' }, 1200)
}

// Focus trap + return-focus
watch(() => props.open, async (isOpen) => {
  if (isOpen) {
    await nextTick()
    overlayEl.value?.focus()
    natW.value = 0; natH.value = 0
    zoomMode.value = 'fit'
  }
})

// Reset natural dims + zoom on frame change.
watch(() => props.index, () => {
  natW.value = 0; natH.value = 0
  zoomMode.value = 'fit'
})
</script>

<style scoped>
.kf-lightbox-overlay {
  position: fixed;
  inset: 0;
  background: rgba(10, 10, 14, 0.82);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  outline: none;
  /* CSS variables drive image + caption budget so toolbar stays visible. */
  --kf-toolbar-h: 56px;
  --kf-toolbar-gap: 16px;
  --kf-caption-max: 9rem;  /* ~6 lines collapsed */
  --kf-image-max: calc(100vh - var(--kf-toolbar-h) - var(--kf-toolbar-gap) - var(--kf-caption-max) - 48px);
}
.kf-lightbox-overlay:has(.kf-lightbox-inner.caption-expanded) {
  --kf-caption-max: 20rem;
}
.kf-lightbox-inner {
  position: relative;
  max-width: 92vw;
  /* Reserve bottom space for the fixed toolbar so content never hides behind it. */
  max-height: calc(100vh - var(--kf-toolbar-h) - var(--kf-toolbar-gap) * 2);
  display: flex;
  flex-direction: column;
  gap: 10px;
  align-items: center;
  padding-bottom: calc(var(--kf-toolbar-h) + var(--kf-toolbar-gap));
}
.kf-image-wrap {
  position: relative;
  max-width: 90vw;
  max-height: var(--kf-image-max);
  display: inline-block;
  flex: 0 1 auto;
  min-height: 0;
}
.kf-image {
  display: block;
  max-width: 90vw;
  max-height: var(--kf-image-max);
  width: auto;
  height: auto;
  border-radius: 6px;
  box-shadow: 0 24px 80px rgba(0,0,0,0.6);
  transition: max-width 150ms ease, max-height 180ms ease;
}
.kf-image.zoom-fit { cursor: zoom-in; }
.kf-image.zoom-actual {
  cursor: zoom-out;
  max-width: none;
  max-height: none;
  width: auto;
  height: auto;
}
.kf-image-wrap.zoom-actual {
  /* Allow panning via scrollbars when the natural image exceeds viewport. */
  overflow: auto;
  border-radius: 6px;
  scrollbar-width: thin;
}
.kf-annot-svg {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}
.kf-annot-g { pointer-events: auto; cursor: pointer; }
.kf-annot-g rect { transition: stroke-width 150ms ease, opacity 150ms ease; }
.kf-annot-g.active rect { stroke-width: 3.5; }

.kf-label-layer {
  position: absolute;
  inset: 0;
  pointer-events: none;
}
.kf-label-pill {
  position: absolute;
  transform: translate(0, 0);
  background: rgba(17, 17, 27, 0.88);
  color: #cdd6f4;
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 11px;
  padding: 3px 8px;
  border-radius: 999px;
  white-space: nowrap;
  pointer-events: auto;
  display: inline-flex;
  align-items: center;
  gap: 6px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.35);
  transition: transform 150ms ease, box-shadow 150ms ease;
}
.kf-label-pill.active {
  transform: translateY(-1px) scale(1.04);
  box-shadow: 0 4px 14px rgba(0,0,0,0.55);
}
.kf-label-dot {
  display: inline-block;
  width: 8px; height: 8px;
  border-radius: 50%;
}

.kf-caption {
  color: #cdd6f4;
  text-align: left;
  max-width: 80vw;
  font-size: 14px;
  line-height: 1.5;
  display: flex;
  flex-direction: column;
  gap: 6px;
  transition: max-height 180ms ease;
}
.kf-caption-scroll {
  display: flex;
  flex-direction: column;
  gap: 6px;
  /* Bounded so caption never pushes toolbar off-screen. */
  max-height: min(var(--kf-caption-max), calc(100vh - var(--kf-toolbar-h) - var(--kf-toolbar-gap) * 2 - 120px));
  overflow-y: auto;
  padding-right: 4px;
  scrollbar-width: thin;
  transition: max-height 180ms ease;
}
.kf-caption:focus-visible {
  outline: 2px solid #89b4fa;
  outline-offset: 4px;
  border-radius: 4px;
}
.kf-caption-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}
.kf-caption-toggle {
  background: rgba(137, 180, 250, 0.08);
  border: 1px solid rgba(137, 180, 250, 0.32);
  color: #89b4fa;
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 11px;
  padding: 3px 10px;
  border-radius: 4px;
  cursor: pointer;
  transition: background 120ms ease;
}
.kf-caption-toggle:hover { background: rgba(137, 180, 250, 0.18); }
.kf-caption-toggle:focus-visible { outline: 2px solid #89b4fa; outline-offset: 2px; }
.kf-caption-row {
  display: flex;
  gap: 10px;
  align-items: baseline;
}
.kf-caption-row-blind { opacity: 0.88; }
.kf-caption-label {
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  padding: 2px 8px;
  border-radius: 4px;
  flex-shrink: 0;
  cursor: help;
  display: inline-flex;
  align-items: center;
  gap: 5px;
}
.kf-caption-label-glyph {
  font-size: 11px;
  line-height: 1;
  opacity: 0.9;
}
.kf-caption-label-intent {
  color: #89b4fa;
  background: rgba(137, 180, 250, 0.14);
}
.kf-caption-label-blind {
  color: #a6adc8;
  background: rgba(205, 214, 244, 0.06);
  border: 1px dashed rgba(205, 214, 244, 0.22);
}
.kf-caption-text { flex: 1 1 auto; }

.kf-agreement-chip {
  all: unset;
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.04em;
  padding: 2px 8px;
  border-radius: 999px;
  cursor: pointer;
  flex-shrink: 0;
  border: 1px solid transparent;
  transition: background 120ms ease, transform 120ms ease;
}
.kf-agreement-chip:hover { transform: translateY(-1px); }
.kf-agreement-chip:focus-visible { outline: 2px solid #89b4fa; outline-offset: 2px; }
.kf-agreement-green {
  color: #a6e3a1; background: rgba(166,227,161,0.12); border-color: rgba(166,227,161,0.30);
}
.kf-agreement-yellow {
  color: #f9e2af; background: rgba(249,226,175,0.12); border-color: rgba(249,226,175,0.30);
}
.kf-agreement-red {
  color: #f38ba8; background: rgba(243,139,168,0.14); border-color: rgba(243,139,168,0.36);
}

.kf-agreement-panel {
  margin-top: 4px;
  padding: 10px 12px;
  border-radius: 6px;
  background: rgba(17,17,27,0.55);
  border: 1px solid rgba(205,214,244,0.14);
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 12px;
}
.kf-agreement-panel-red { border-color: rgba(243,139,168,0.45); }
.kf-agreement-panel-yellow { border-color: rgba(249,226,175,0.40); }
.kf-agreement-panel-green { border-color: rgba(166,227,161,0.36); }
.kf-agreement-section { display: flex; flex-direction: column; gap: 4px; }
.kf-agreement-title {
  font-size: 10px;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: #cdd6f4;
  opacity: 0.85;
}
.kf-agreement-remedy {
  font-size: 10px;
  color: #a6adc8;
  font-style: italic;
  margin-bottom: 2px;
}
.kf-agreement-tokens {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}
.kf-agreement-token {
  padding: 1px 6px;
  border-radius: 4px;
  font-size: 11px;
}
.kf-agreement-token-missing {
  color: #f38ba8;
  background: rgba(243,139,168,0.10);
  border: 1px dashed rgba(243,139,168,0.45);
}
.kf-agreement-token-extra {
  color: #89b4fa;
  background: rgba(137,180,250,0.10);
  border: 1px dashed rgba(137,180,250,0.40);
}
.kf-agreement-empty {
  color: #6c7086;
  font-style: italic;
  font-size: 11px;
}
@media (max-width: 720px) {
  .kf-agreement-panel { grid-template-columns: 1fr; }
}
.kf-caption-text-blind { color: #a6adc8; font-style: italic; }
.kf-meta {
  margin-top: 4px;
  color: #a6adc8;
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 12px;
}

.kf-structural-rail {
  position: fixed;
  right: 24px;
  top: 16px;
  bottom: calc(var(--kf-toolbar-h) + var(--kf-toolbar-gap) + 8px);
  width: clamp(320px, 42vw, 560px);
  z-index: 9;
  display: flex;
  flex-direction: column;
}
@media (max-width: 900px) {
  .kf-structural-rail { position: static; width: 100%; margin-top: 12px; }
}

.kf-nav {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  background: rgba(17, 17, 27, 0.75);
  color: #cdd6f4;
  border: 1px solid rgba(205,214,244,0.2);
  width: 44px;
  height: 44px;
  border-radius: 50%;
  font-size: 24px;
  line-height: 1;
  cursor: pointer;
  transition: background 150ms ease;
}
.kf-nav:hover:not(:disabled) { background: rgba(49, 50, 68, 0.95); }
.kf-nav:disabled { opacity: 0.35; cursor: not-allowed; }
.kf-prev { left: -56px; }
.kf-next { right: -56px; }
@media (max-width: 900px) {
  .kf-prev { left: 4px; }
  .kf-next { right: 4px; }
}

.kf-toolbar {
  position: fixed;
  left: 50%;
  bottom: 16px;
  transform: translateX(-50%);
  display: flex;
  gap: 8px;
  align-items: center;
  flex-wrap: wrap;
  justify-content: center;
  max-width: 96vw;
  padding: 10px 14px;
  background: rgba(17, 17, 27, 0.82);
  border: 1px solid rgba(205, 214, 244, 0.14);
  border-radius: 10px;
  backdrop-filter: blur(6px);
  /* Subtle shadow above toolbar to signal scroll-affordance over content. */
  box-shadow:
    0 -12px 24px -12px rgba(0, 0, 0, 0.55),
    0 8px 24px rgba(0, 0, 0, 0.45);
  z-index: 10;
}
@media (max-width: 480px) {
  .kf-toolbar {
    bottom: 8px;
    padding: 8px 10px;
    gap: 6px;
  }
  .kf-lightbox-overlay { --kf-toolbar-h: 96px; }
}
.kf-btn {
  background: rgba(17,17,27,0.75);
  color: #cdd6f4;
  border: 1px solid rgba(205,214,244,0.2);
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 12px;
  cursor: pointer;
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  transition: background 150ms ease;
}
.kf-btn:hover:not(:disabled) { background: rgba(49,50,68,0.95); }
.kf-btn:disabled { opacity: 0.45; cursor: not-allowed; }
.kf-close { font-size: 14px; padding: 6px 10px; }

/* open/close animation */
.kf-lightbox-enter-active, .kf-lightbox-leave-active {
  transition: opacity 150ms ease-out;
}
.kf-lightbox-enter-active .kf-lightbox-inner,
.kf-lightbox-leave-active .kf-lightbox-inner {
  transition: transform 150ms ease-out;
}
.kf-lightbox-enter-from, .kf-lightbox-leave-to { opacity: 0; }
.kf-lightbox-enter-from .kf-lightbox-inner,
.kf-lightbox-leave-to .kf-lightbox-inner {
  transform: scale(0.96);
}
</style>
