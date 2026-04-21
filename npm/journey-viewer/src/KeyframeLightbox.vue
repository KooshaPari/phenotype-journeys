<template>
  <Teleport to="body">
    <Transition name="kf-lightbox">
      <div
        v-if="open"
        ref="overlayEl"
        class="kf-lightbox-overlay"
        role="dialog"
        aria-modal="true"
        tabindex="-1"
        @click.self="close"
        @keydown="onKey"
      >
        <div class="kf-lightbox-inner" @click.stop>
          <div class="kf-image-wrap">
            <img
              ref="imgEl"
              class="kf-image"
              :src="currentFrame.path"
              :alt="currentFrame.caption"
              @load="onImgLoad"
            />
            <svg
              v-if="showAnnotations && annotations.length && natW && natH"
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
                  :x="a.bbox[0]" :y="a.bbox[1]"
                  :width="a.bbox[2]" :height="a.bbox[3]"
                  :stroke="paletteFor(a, i)"
                  :stroke-dasharray="(a.style === 'dashed') ? '6 4' : undefined"
                  fill="none" stroke-width="2" rx="2"
                />
                <title v-if="a.note">{{ a.note }}</title>
              </g>
            </svg>
            <div
              v-if="showAnnotations && annotations.length"
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
          <div class="kf-caption">
            <div class="kf-caption-row">
              <span class="kf-caption-label kf-caption-label-intent">Intent</span>
              <span class="kf-caption-text">{{ currentFrame.caption || '—' }}</span>
            </div>
            <div class="kf-caption-row kf-caption-row-blind">
              <span class="kf-caption-label kf-caption-label-blind">Blind</span>
              <span class="kf-caption-text kf-caption-text-blind">{{ currentFrame.blind_description || '—' }}</span>
            </div>
            <div class="kf-meta">frame {{ index + 1 }} / {{ frames.length }} · {{ journeyId }}</div>
          </div>
          <button class="kf-nav kf-prev" @click="prev" :disabled="index === 0" aria-label="Previous">‹</button>
          <button class="kf-nav kf-next" @click="next" :disabled="index === frames.length - 1" aria-label="Next">›</button>
          <div class="kf-toolbar">
            <button class="kf-btn" @click="toggleAnnotations" :aria-pressed="showAnnotations">
              Annotations: {{ showAnnotations ? 'on' : 'off' }}
            </button>
            <button class="kf-btn" :disabled="!annotations.length" @click="copyAnnotationsJson">
              {{ copyLabel }}
            </button>
            <button class="kf-btn kf-close" @click="close">✕</button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue'
import type { Annotation } from './types'

interface Frame { path: string; caption: string; blind_description?: string | null; annotations?: Annotation[] | null }

const props = defineProps<{ open: boolean; frames: Frame[]; index: number; journeyId: string }>()
const emit = defineEmits<{ (e: 'update:index', v: number): void; (e: 'close'): void }>()

const PALETTE = ['#f38ba8','#a6e3a1','#f9e2af','#89b4fa','#cba6f7','#94e2d5','#fab387']
const overlayEl = ref<HTMLElement | null>(null)
const imgEl = ref<HTMLImageElement | null>(null)
const natW = ref(0); const natH = ref(0); const hoverIdx = ref<number | null>(null)
const copyLabel = ref('Copy JSON')
const STORAGE_KEY = 'phenotype-journey:annotations-on'
const showAnnotations = ref(true)
try { if (typeof localStorage !== 'undefined' && localStorage.getItem(STORAGE_KEY) === '0') showAnnotations.value = false } catch {}

const currentFrame = computed(() => props.frames[props.index] || { path: '', caption: '', annotations: [] })
const annotations = computed<Annotation[]>(() => currentFrame.value.annotations || [])
function paletteFor(a: Annotation, i: number): string { return a.color || PALETTE[i % PALETTE.length] }
function labelStyle(a: Annotation) {
  const [x, y, , h] = a.bbox
  const below = y < 24
  const top = below ? (y + h + 4) : (y - 22)
  return { left: (x / (natW.value || 1) * 100) + '%', top: (top / (natH.value || 1) * 100) + '%' }
}
function onImgLoad() { const el = imgEl.value; if (!el) return; natW.value = el.naturalWidth; natH.value = el.naturalHeight }
function next() { if (props.index < props.frames.length - 1) emit('update:index', props.index + 1) }
function prev() { if (props.index > 0) emit('update:index', props.index - 1) }
function close() { emit('close') }
function onKey(e: KeyboardEvent) {
  if (e.key === 'Escape') { e.preventDefault(); close() }
  else if (e.key === 'ArrowRight') { e.preventDefault(); next() }
  else if (e.key === 'ArrowLeft') { e.preventDefault(); prev() }
}
function toggleAnnotations() {
  showAnnotations.value = !showAnnotations.value
  try { localStorage.setItem(STORAGE_KEY, showAnnotations.value ? '1' : '0') } catch {}
}
async function copyAnnotationsJson() {
  const payload = { journey: props.journeyId, frame: props.index + 1, screenshot: currentFrame.value.path, annotations: annotations.value }
  const text = JSON.stringify(payload, null, 2)
  try { await navigator.clipboard.writeText(text); copyLabel.value = 'Copied ✓' }
  catch {
    const ta = document.createElement('textarea'); ta.value = text; document.body.appendChild(ta); ta.select(); document.execCommand('copy'); document.body.removeChild(ta)
    copyLabel.value = 'Copied ✓'
  }
  setTimeout(() => { copyLabel.value = 'Copy JSON' }, 1200)
}
watch(() => props.open, async (o) => { if (o) { await nextTick(); overlayEl.value?.focus(); natW.value = 0; natH.value = 0 } })
watch(() => props.index, () => { natW.value = 0; natH.value = 0 })
</script>

<style scoped>
.kf-lightbox-overlay { position: fixed; inset: 0; background: rgba(10,10,14,0.82); backdrop-filter: blur(4px); display: flex; align-items: center; justify-content: center; z-index: 9999; outline: none; }
.kf-lightbox-inner { position: relative; max-width: 92vw; max-height: 92vh; display: flex; flex-direction: column; gap: 10px; align-items: center; }
.kf-image-wrap { position: relative; max-width: 90vw; max-height: 80vh; display: inline-block; }
.kf-image { display: block; max-width: 90vw; max-height: 80vh; border-radius: 6px; box-shadow: 0 24px 80px rgba(0,0,0,0.6); }
.kf-annot-svg { position: absolute; inset: 0; width: 100%; height: 100%; pointer-events: none; }
.kf-annot-g { pointer-events: auto; cursor: pointer; }
.kf-annot-g rect { transition: stroke-width 150ms ease; }
.kf-annot-g.active rect { stroke-width: 3.5; }
.kf-label-layer { position: absolute; inset: 0; pointer-events: none; }
.kf-label-pill { position: absolute; background: rgba(17,17,27,0.88); color: #cdd6f4; font-family: ui-monospace,monospace; font-size: 11px; padding: 3px 8px; border-radius: 999px; white-space: nowrap; pointer-events: auto; display: inline-flex; align-items: center; gap: 6px; box-shadow: 0 2px 8px rgba(0,0,0,0.35); transition: transform 150ms ease; }
.kf-label-pill.active { transform: translateY(-1px) scale(1.04); }
.kf-label-dot { display: inline-block; width: 8px; height: 8px; border-radius: 50%; }
.kf-caption { color: #cdd6f4; text-align: center; max-width: 80vw; font-size: 14px; }
.kf-meta { margin-top: 4px; color: #a6adc8; font-family: ui-monospace,monospace; font-size: 12px; }
.kf-nav { position: absolute; top: 50%; transform: translateY(-50%); background: rgba(17,17,27,0.75); color: #cdd6f4; border: 1px solid rgba(205,214,244,0.2); width: 44px; height: 44px; border-radius: 50%; font-size: 24px; cursor: pointer; }
.kf-nav:disabled { opacity: 0.35; cursor: not-allowed; }
.kf-prev { left: -56px; } .kf-next { right: -56px; }
@media (max-width: 900px) { .kf-prev { left: 4px; } .kf-next { right: 4px; } }
.kf-toolbar { display: flex; gap: 8px; }
.kf-btn { background: rgba(17,17,27,0.75); color: #cdd6f4; border: 1px solid rgba(205,214,244,0.2); padding: 6px 12px; border-radius: 6px; font-size: 12px; cursor: pointer; font-family: ui-monospace,monospace; }
.kf-btn:disabled { opacity: 0.45; cursor: not-allowed; }
.kf-lightbox-enter-active, .kf-lightbox-leave-active { transition: opacity 150ms ease-out; }
.kf-lightbox-enter-active .kf-lightbox-inner, .kf-lightbox-leave-active .kf-lightbox-inner { transition: transform 150ms ease-out; }
.kf-lightbox-enter-from, .kf-lightbox-leave-to { opacity: 0; }
.kf-lightbox-enter-from .kf-lightbox-inner, .kf-lightbox-leave-to .kf-lightbox-inner { transform: scale(0.96); }
</style>
