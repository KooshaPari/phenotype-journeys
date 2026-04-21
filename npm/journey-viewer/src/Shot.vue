<template>
  <figure
    :class="['shot', `shot-size-${resolvedSize}`, `shot-align-${align}`]"
    :style="wrapperStyle"
  >
    <button
      ref="btnEl"
      type="button"
      class="shot-btn"
      :aria-label="`Expand screenshot: ${caption || src}`"
      @click="openLightbox"
    >
      <span class="shot-img-wrap">
        <img
          ref="imgEl"
          :src="src"
          :alt="caption || 'screenshot'"
          class="shot-img"
          loading="lazy"
          @load="onLoad"
          @error="onError"
        />
        <svg
          v-if="effectiveAnnotations.length && natW && natH"
          class="shot-annot"
          :viewBox="`0 0 ${natW} ${natH}`"
          preserveAspectRatio="xMidYMid meet"
          aria-hidden="true"
        >
          <rect
            v-for="(a, i) in effectiveAnnotations"
            :key="i"
            :x="a.bbox[0]"
            :y="a.bbox[1]"
            :width="a.bbox[2]"
            :height="a.bbox[3]"
            :stroke="a.color || paletteColor(i)"
            :stroke-dasharray="a.style === 'dashed' ? '6 4' : undefined"
            stroke-width="2.5"
            fill="none"
            opacity="0.85"
            rx="2"
          />
        </svg>
        <span class="shot-zoom-pill" aria-hidden="true">⤢</span>
      </span>
    </button>
    <figcaption v-if="caption" class="shot-caption">{{ caption }}</figcaption>

    <KeyframeLightbox
      v-if="lightboxOpen"
      :open="lightboxOpen"
      :frames="singleFrame"
      :index="0"
      :journey-id="journeyId || ''"
      @update:index="() => {}"
      @close="closeLightbox"
    />
  </figure>
</template>

<script setup lang="ts">
import { computed, nextTick, ref } from 'vue'
import KeyframeLightbox from './KeyframeLightbox.vue'

interface Annotation {
  bbox: [number, number, number, number]
  label: string
  color?: string | null
  style?: 'solid' | 'dashed'
  note?: string | null
}

const props = withDefaults(
  defineProps<{
    src: string
    caption?: string
    annotations?: Annotation[] | null
    size?: 'inline' | 'small' | 'medium' | 'large'
    align?: 'left' | 'right' | 'center' | 'inline'
    journeyId?: string
    /** Optional frame index (1-based) for annotation registry lookup */
    frame?: number
    width?: string | number | null
  }>(),
  { size: 'medium', align: 'right', annotations: null, journeyId: '', frame: 0, width: null },
)

const SIZE_PX: Record<string, number> = {
  inline: 180,
  small: 240,
  medium: 420,
  large: 720,
}

const resolvedSize = computed(() => props.size)
const wrapperStyle = computed(() => {
  if (props.width) return { width: typeof props.width === 'number' ? `${props.width}px` : props.width }
  return { width: `${SIZE_PX[props.size] || SIZE_PX.medium}px` }
})

const PALETTE = ['#f38ba8', '#a6e3a1', '#f9e2af', '#89b4fa', '#cba6f7', '#94e2d5', '#fab387']
function paletteColor(i: number) { return PALETTE[i % PALETTE.length] }

const effectiveAnnotations = computed<Annotation[]>(() => {
  if (props.annotations && props.annotations.length) return props.annotations
  // Registry lookup would go here at build time; at runtime we simply honour the prop.
  return []
})

const imgEl = ref<HTMLImageElement | null>(null)
const btnEl = ref<HTMLElement | null>(null)
const natW = ref(0)
const natH = ref(0)
const lightboxOpen = ref(false)

function onLoad(ev: Event) {
  const img = ev.target as HTMLImageElement
  natW.value = img.naturalWidth
  natH.value = img.naturalHeight
}
function onError() {
  // Keep the img element visible; browser shows broken-image icon for debugging.
  // Build-time link checker is the right place to enforce.
}

const singleFrame = computed(() => [{
  path: props.src,
  caption: props.caption || '',
  annotations: effectiveAnnotations.value,
}])

function openLightbox() { lightboxOpen.value = true }
async function closeLightbox() {
  lightboxOpen.value = false
  await nextTick()
  btnEl.value?.focus()
}
</script>

<style scoped>
.shot {
  display: block;
  margin: 10px 14px 14px 14px;
  padding: 0;
  font-size: 12px;
  line-height: 1.35;
  border: 1px solid var(--vp-divider);
  border-radius: 6px;
  background: var(--vp-c-bg-soft);
  overflow: hidden;
  box-shadow: 0 2px 6px rgba(0,0,0,0.08);
}
.shot-align-right { float: right; clear: right; margin-left: 18px; }
.shot-align-left { float: left; clear: left; margin-right: 18px; }
.shot-align-center { float: none; margin-left: auto; margin-right: auto; }
.shot-align-inline {
  display: inline-block;
  vertical-align: middle;
  margin: 2px 6px;
}
.shot-size-inline { width: 180px; }
.shot-size-small { width: 240px; }
.shot-size-medium { width: 420px; }
.shot-size-large { width: 720px; max-width: 100%; }

.shot-btn {
  all: unset;
  display: block;
  cursor: zoom-in;
  width: 100%;
}
.shot-btn:focus-visible {
  outline: 2px solid var(--color-accent, #89b4fa);
  outline-offset: -2px;
}
.shot-img-wrap {
  position: relative;
  display: block;
  background: var(--vp-c-bg-mute);
}
.shot-img {
  display: block;
  width: 100%;
  height: auto;
  object-fit: contain;
}
.shot-annot {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}
.shot-zoom-pill {
  position: absolute;
  right: 6px;
  bottom: 6px;
  background: rgba(17,17,27,0.8);
  color: #cdd6f4;
  font-size: 11px;
  line-height: 1;
  padding: 4px 6px;
  border-radius: 4px;
  opacity: 0;
  transition: opacity 120ms ease;
  pointer-events: none;
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
}
.shot-btn:hover .shot-zoom-pill,
.shot-btn:focus-visible .shot-zoom-pill { opacity: 1; }

.shot-caption {
  padding: 6px 10px 8px 10px;
  color: var(--vp-c-text-2);
  border-top: 1px solid var(--vp-divider);
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 11px;
}

@media (max-width: 640px) {
  .shot-align-right, .shot-align-left {
    float: none;
    margin-left: auto;
    margin-right: auto;
  }
  .shot-size-medium, .shot-size-large { width: 100%; }
}
</style>
