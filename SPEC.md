# phenotype-journeys — SPEC

## 1. Scope

Project-agnostic **user journey annotation + rendering system** for the Phenotype
org. Captures a user-facing flow (CLI tape, UI test, Playwright trace, or hand
keyframes), emits a canonical manifest, annotates inline screenshots with
callouts and OCR assertions, and renders the result inside VitePress docs via a
Vue viewer. Replaces duplicated per-repo journey code in `hwLedger`,
`AgilePlus`, and `thegent`.

## 2. Surface

**Rust core** (`crates/phenotype-journey-core`)
- `lib.rs` re-exports `agreement`, `assertions`, `pipeline`, `vision`, `verify/`.
- Serde types + `schemars` export of `schema/manifest.schema.json`.
- Verify loop: `mock` (offline, no key) and `live` (`--features live`,
  Anthropic API) for `describe` + `judge` passes.
- `assertions.rs` — tesseract-backed hard gates (`must_contain`,
  `must_not_contain`, `expected_exit` via `__EXIT_<N>__` sentinel).

**Rust CLI** (`bin/phenotype-journey`): `record | verify | validate | sync | assert`.

**npm/ — three sub-packages**
- `journey-viewer` (Vue 3): `JourneyViewer` + `RecordingEmbed` for VitePress.
- `journey-playwright` (TS): scripts a web page, emits a conformant manifest.
- `playwright-record` (TS): record-to-manifest helper.

**Remotion/** — `borrowed/` holds upstream PROVENANCE; the Remotion renderer for
inline `<Shot>` callouts and `JourneyRich` bubble anchoring (position = `auto |
top-left | top-right | bottom-left | bottom-right | center | custom`).

**Data** (`data/shot-annotations.yaml` v3) — journey_id → frame_index →
`annotations[]` (bbox, label, color, style, kind, position) plus new
`expected_text[]` for OCR-assert.

## 3. Key invariants

- Manifest is the single source of truth; `schema/manifest.schema.json` is
  canonical — CI `validate` is a hard gate.
- `frame_index` is 1-based; matches `frame-NNN.png`; bbox in natural pixel
  space, pre-scale.
- Bare-list frame entries (legacy) are tolerated as `expected_text: []`.
- Tesseract is **required** when `assert` is invoked — no silent skip.
- `position` must vary across consecutive frames so callouts don't stack.
- Verify mode is fail-loud: `mock` vs `live` is explicit, never inferred.

## 4. Top gaps

- **No workspace publish automation**: tarballs in `journey-viewer/` are
  hand-dropped; need `bun publish` CI on tag.
- **`remotion/borrowed/` is provenance-only** — no first-party Remotion
  compositions wired into the VitePress viewer yet.
- **No `FUNCTIONAL_REQUIREMENTS.md` ↔ manifest traceability autograder**;
  the contract exists in `docs/journey-traceability.md` but enforcement is
  manual.
- **Playwright packages share lockfile root** — no per-package publish
  isolation, and `playwright-record` lacks a README consumer story.
- **No image-diff renderer** for keyframe regression; `assertions.rs` gates
  text only.
- **`data/shot-annotations.yaml` has no linter in CI** (`tools/shot-linter`
  referenced in the header is not present in the workspace tree).
