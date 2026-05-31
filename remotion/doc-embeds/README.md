# @phenotype/doc-embeds

Generic Remotion pipeline for producing rich annotated-screenshot and mp4/gif
doc embeds from **phenotype-journey manifests**.

Promoted from DINOForge `tools/doc-embeds` into the shared home here so every
org repo can use it as the fill engine for `RICH-MEDIA-STUB` slots.

## How it works

1. The phenotype-journey engine (`phenotype-journey-core`) runs a journey and
   emits a `manifest.json` per journey ID, containing:
   - `steps[].intent` — what the step intends
   - `steps[].description` — Claude's blind description of the screenshot
   - `steps[].screenshot_path` — relative path to the keyframe PNG
   - `steps[].annotations[]` — bbox highlight/cursor rects

2. `bin/from-journey.mjs` reads that manifest and writes an `annotations.embed.json`
   (an `EmbedSpec`) next to it.

3. `bin/render.mjs` stages assets into Remotion's `public/` dir and calls the
   Remotion CLI to render annotated **mp4 + gif**.

## One-line invocation

```bash
# From the root of ANY repo:
node <path/to>/remotion/doc-embeds/bin/from-journey.mjs \
  --manifest data/journeys/<id>/manifest.json \
  --render
```

Or install globally and use the package scripts:

```bash
cd remotion/doc-embeds && npm install

# Convert a manifest + render in one shot:
npm run from-journey -- --manifest ../../data/journeys/my-feature/manifest.json --render

# Render an existing annotations.json:
npm run render -- --annotations path/to/annotations.json --format both

# Open Remotion Studio to preview / tweak:
npm run studio
```

## RICH-MEDIA-STUB fill workflow

For each stub-box repo:

1. Run the journey engine to generate manifests under `data/journeys/<id>/`.
2. Point `from-journey.mjs` at each manifest with `--render`.
3. Drop the produced `<id>.mp4` / `<id>.gif` into `docs/embeds/` and reference
   them in VitePress via `@phenotype/journey-viewer`'s `<RecordingEmbed>` or
   `<KeyframeGallery>` components.

## Manifest-to-EmbedSpec mapping

| Manifest field | EmbedSpec field |
|---|---|
| `id` | `id`, output filename |
| `intent` | `title` |
| `passed` | `subtitle` ("journey: FAILED" if false) |
| `steps[].screenshot_path` | `scenes[].src` |
| `steps[].intent` | `scenes[].callouts[0].text` |
| `steps[].description` | `scenes[].callouts[0].subText` |
| `steps[].annotations[].bbox` | `scenes[].highlights[]` or `scenes[].cursors[]` |

## Requirements

- Node 20+
- `npm install` inside this package
- ffmpeg on PATH (used by Remotion for mp4 encoding)
- Chrome/Chromium headless shell (auto-detected from Playwright install, or set
  `DOC_EMBEDS_BROWSER=/path/to/chrome-headless-shell.exe`)

## Package structure

```
src/
  schema.ts          EmbedSpec + Scene types (project-agnostic input contract)
  DocEmbed.tsx       Remotion root composition
  components/
    SceneView.tsx    Per-scene renderer (stills + video + annotations)
    Callout.tsx      Animated callout box (spring-in)
    Highlight.tsx    Pulsing highlight rect + cursor ripple ring
bin/
  from-journey.mjs  Manifest -> EmbedSpec adapter (canonical entry point)
  render.mjs        Asset staging + Remotion CLI wrapper
  capture.mjs       Playwright Capture helper for E2E test integration
examples/
  journey-walkthrough/   Sample annotations.json showing the schema
```

## Capture helper (E2E integration)

In Playwright specs, use the `Capture` class to emit keyframes + an
`annotations.json` automatically:

```ts
import { Capture } from "@phenotype/doc-embeds/bin/capture.mjs";

const cap = new Capture({ id: "my-feature", captureDir: "docs/embeds/captures" });
await page.goto("/my-feature");
await cap.step(page, "Feature loads");
await page.getByRole("button", { name: "Activate" }).click();
await cap.step(page, "Feature activated", {
  highlight: { x: 100, y: 200, width: 300, height: 60, label: "Activate button" },
});
await cap.finish({ title: "My Feature", accent: "#34d399" });
// Then: node bin/render.mjs --annotations docs/embeds/captures/my-feature/annotations.json
```
