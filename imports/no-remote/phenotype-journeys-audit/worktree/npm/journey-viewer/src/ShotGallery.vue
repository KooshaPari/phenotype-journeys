<template>
  <section class="shot-gallery" :aria-label="title || 'Screenshot gallery'">
    <h4 v-if="title" class="shot-gallery-title">{{ title }}</h4>

    <div v-if="shots.length" class="shot-gallery-body">
      <!-- Hero image -->
      <figure class="shot-hero">
        <button
          ref="heroBtnEl"
          type="button"
          class="shot-hero-btn"
          :aria-label="`Expand screenshot: ${activeShot.caption || activeShot.src}`"
          @click="openLightbox"
        >
          <img
            :src="activeShot.src"
            :alt="activeShot.caption || 'screenshot'"
            class="shot-hero-img"
            loading="lazy"
          />
          <span class="shot-hero-zoom" aria-hidden="true">Γñó Click to expand</span>
        </button>
        <figcaption v-if="activeShot.caption || activeShot.timestamp" class="shot-hero-caption">
          <span v-if="activeShot.timestamp" class="shot-hero-ts">{{ activeShot.timestamp }}</span>
          <span v-if="activeShot.caption" class="shot-hero-text">{{ activeShot.caption }}</span>
        </figcaption>
        <div v-if="activeShot.ocr" class="shot-hero-ocr" aria-label="OCR text">
          <span class="shot-hero-ocr-label">OCR</span>
          <code class="shot-hero-ocr-body">{{ activeShot.ocr }}</code>
        </div>
      </figure>

      <!-- Thumbnail strip -->
      <div
        v-if="shots.length > 1"
        class="shot-thumbs"
        role="tablist"
        :aria-label="`${shots.length} screenshots`"
      >
        <button
          v-for="(shot, i) in shots"
          :key="i"
          ref="thumbEls"
          type="button"
          role="tab"
          :aria-selected="i === activeIndex"
          :aria-label="`Screenshot ${i + 1}: ${shot.caption || shot.src}`"
          :class="['shot-thumb', { 'shot-thumb-active': i === activeIndex }]"
          @click="selectThumb(i)"
        >
          <img
            :src="shot.src"
            :alt="shot.caption || `screenshot ${i + 1}`"
            class="shot-thumb-img"
            loading="lazy"
          />
          <span class="shot-thumb-num">{{ i + 1 }}</span>
        </button>
      </div>
    </div>
    <div v-else class="shot-gallery-empty">No screenshots.</div>

    <KeyframeLightbox
      v-if="lightboxOpen"
      :open="lightboxOpen"
      :frames="lightboxFrames"
      :index="activeIndex"
      :journey-id="journeyId || ''"
      @update:index="activeIndex = $event"
      @close="closeLightbox"
    />
  </section>
</template>

<script setup lang="ts">
import { computed, nextTick, ref } from 'vue'
import KeyframeLightbox from './KeyframeLightbox.vue'

interface Shot {
  src: string
  caption?: string
  ocr?: string
  timestamp?: string
}

const props = withDefaults(
  defineProps<{
    shots: Shot[]
    title?: string
    journeyId?: string
  }>(),
  { shots: () => [], title: '', journeyId: '' },
)

const activeIndex = ref(0)
const lightboxOpen = ref(false)
const heroBtnEl = ref<HTMLElement | null>(null)
const thumbEls = ref<HTMLElement[]>([])

const activeShot = computed<Shot>(() => props.shots[activeIndex.value] || { src: '' })

// KeyframeLightbox expects `{ path, caption, annotations }` frames. Map shots accordingly.
const lightboxFrames = computed(() =>
  props.shots.map((s) => ({
    path: s.src,
    caption: [s.timestamp, s.caption].filter(Boolean).join(' ΓÇö '),
    annotations: [],
  })),
)

function selectThumb(i: number) {
  activeIndex.value = i
}

function openLightbox() {
  lightboxOpen.value = true
}

async function closeLightbox() {
  lightboxOpen.value = false
  await nextTick()
  heroBtnEl.value?.focus()
}
</script>

<style scoped>
.shot-gallery {
  margin: 18px 0 22px 0;
  padding: 14px;
  border: 1px solid var(--vp-divider);
  border-radius: 10px;
  background: var(--vp-c-bg-soft);
  /* Critical: no floats, no clears. Pure flow. */
  display: block;
  clear: both;
}

.shot-gallery-title {
  margin: 0 0 12px 0;
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0.04em;
  text-transform: uppercase;
  color: var(--vp-c-text-2);
}

.shot-gallery-body {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

/* Hero */
.shot-hero {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin: 0;
}

.shot-hero-btn {
  all: unset;
  display: block;
  width: 100%;
  cursor: zoom-in;
  position: relative;
  background: var(--vp-c-bg-mute);
  border: 1px solid var(--vp-divider);
  border-radius: 8px;
  overflow: hidden;
}

.shot-hero-btn:focus-visible {
  outline: 2px solid var(--color-accent, #89b4fa);
  outline-offset: 2px;
}

.shot-hero-img {
  display: block;
  width: 100%;
  height: auto;
  max-height: 70vh;
  object-fit: contain;
}

.shot-hero-zoom {
  position: absolute;
  right: 10px;
  bottom: 10px;
  padding: 4px 10px;
  border-radius: 999px;
  background: rgba(17, 17, 27, 0.85);
  color: #cdd6f4;
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 11px;
  letter-spacing: 0.02em;
  opacity: 0;
  transition: opacity 120ms ease;
  pointer-events: none;
}

.shot-hero-btn:hover .shot-hero-zoom,
.shot-hero-btn:focus-visible .shot-hero-zoom {
  opacity: 1;
}

.shot-hero-caption {
  display: flex;
  align-items: baseline;
  gap: 8px;
  padding: 0 4px;
  font-size: 13px;
  color: var(--vp-c-text-2);
  line-height: 1.4;
}

.shot-hero-ts {
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 11px;
  color: var(--vp-c-text-3);
  padding: 1px 6px;
  border-radius: 4px;
  background: var(--vp-c-bg-mute);
  flex-shrink: 0;
}

.shot-hero-text {
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 12px;
}

.shot-hero-ocr {
  margin-top: 4px;
  padding: 8px 10px;
  border: 1px dashed var(--vp-divider);
  border-radius: 6px;
  background: var(--vp-c-bg-mute);
  display: flex;
  gap: 8px;
  align-items: flex-start;
}

.shot-hero-ocr-label {
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  color: var(--vp-c-text-3);
  padding: 2px 6px;
  border-radius: 4px;
  background: var(--vp-c-bg-soft);
  flex-shrink: 0;
}

.shot-hero-ocr-body {
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 11px;
  line-height: 1.45;
  color: var(--vp-c-text-2);
  white-space: pre-wrap;
  word-break: break-word;
  background: transparent;
  padding: 0;
}

/* Thumbnails */
.shot-thumbs {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(96px, 1fr));
  gap: 8px;
}

.shot-thumb {
  all: unset;
  position: relative;
  display: block;
  cursor: pointer;
  border: 2px solid transparent;
  border-radius: 6px;
  overflow: hidden;
  background: var(--vp-c-bg-mute);
  aspect-ratio: 16 / 9;
  transition: border-color 120ms ease, transform 120ms ease;
}

.shot-thumb:hover {
  border-color: var(--vp-c-text-3);
  transform: translateY(-1px);
}

.shot-thumb:focus-visible {
  outline: 2px solid var(--color-accent, #89b4fa);
  outline-offset: 2px;
}

.shot-thumb-active {
  border-color: var(--color-accent, #89b4fa);
}

.shot-thumb-img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.shot-thumb-num {
  position: absolute;
  top: 4px;
  left: 4px;
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace;
  font-size: 10px;
  font-weight: 600;
  color: #cdd6f4;
  background: rgba(17, 17, 27, 0.8);
  padding: 1px 5px;
  border-radius: 3px;
}

.shot-gallery-empty {
  padding: 20px;
  text-align: center;
  color: var(--vp-c-text-3);
  font-style: italic;
}

@media (max-width: 640px) {
  .shot-hero-img { max-height: 50vh; }
  .shot-thumbs { grid-template-columns: repeat(auto-fill, minmax(72px, 1fr)); }
}
</style>
