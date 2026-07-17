# Borrowed assets / patterns

This directory holds patterns and source references copied (not forked) from
sibling repositories so the `phenotype-journeys` Remotion pipeline can reuse
established techniques rather than reinventing them.

## dinoforge / `KooshaPari/dino`

**Source repo:** `/Users/kooshapari/CodeProjects/Phenotype/repos/dino`
**Commit probed:** HEAD at 2026-04-19 (read-only inspection)
**License:** see upstream `LICENSE`

### Inspection outcome (updated 2026-04-19 by agent a9335787)

The dino repo DOES contain a Remotion project at `scripts/video/` (missed in
the initial scan because it's nested under a tooling subdirectory). Four
React/TSX components (`CalloutBox`, `CaptionBar`, `FeatureScene`, `TitleCard`)
were borrowed verbatim into `./dino-components/` with a provenance header on
each file. License and attribution preserved.

### What was borrowable

1. **Frame-numbered screenshot naming convention** — dino uses
   `NN_before_fN_<epoch>.png` / `NN_after_fN_<label>_<epoch>.png` to pair
   state-before / state-after frames. phenotype-journeys already applies a
   similar `frame-NNN.png` convention, so no code was copied — the alignment
   is documented here for consistency.
2. **Capture-driver pattern** — `dino/capture_ui_states.py` drives the
   game to a specific UI state before snapping. The equivalent in this repo
   is `apps/streamlit/journeys/scripts/record-all.sh`, which the Remotion
   compositions already consume.

### What was **not** borrowable

- No Remotion compositions (dino has no Remotion).
- No TTS wiring (dino has no audio pipeline).
- No annotate-screenshot component.

### Outcome for sibling agent af29499a

The Remotion enrichment pipeline should therefore be built on top of the
existing VitePress `KeyframeLightbox.vue` annotation model (bbox + label +
optional note) rather than importing a dino component. The new
`<Shot>` Vue component (see `npm/journey-viewer/src/Shot.vue`) exposes the
same annotation schema so Remotion compositions can reuse identical JSON.

### Annotation schema (shared contract)

```ts
interface Annotation {
  bbox: [x: number, y: number, w: number, h: number]
  label: string
  color?: string | null
  style?: 'solid' | 'dashed'
  note?: string | null
}
```

This is the canonical shape across KeyframeGallery, KeyframeLightbox, Shot,
the annotation registry (`data/shot-annotations.yaml`), and the Remotion
compositions. Do not diverge.
