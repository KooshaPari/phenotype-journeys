# Changelog

All notable changes to this project will be documented in this file.

## 🐛 Bug Fixes
- Fix(journey-viewer): pin lightbox toolbar so caption expansion never clips controls

- Toolbar now position:fixed at bottom of viewport with subtle upper shadow
  for scroll-affordance; always visible regardless of caption height.
- Caption scroll area clamped via min(var, calc(100vh - reserved)) so it
  cannot push content behind the toolbar.
- Image max-height driven by --kf-image-max (100vh minus toolbar + caption
  budget); shrinks proportionally when caption expands.
- Inner container reserves padding-bottom equal to toolbar height + gap.
- Mobile (<=480px) increases reserved toolbar height (wrap to 2 rows).

Verified layout budget at 1280x720, 1440x900, 390x844. (`b8b6c4d`)
- Fix(recording-embed): add kind prop + per-family URL resolution

cli-journeys flat at recordings/<tape>.mp4; streamlit/gui nested at
recordings/<tape>/<tape>.mp4 (or gui-journeys/<slug>/recording.mp4). Probe
the right layout so non-CLI recordings stop surfacing the 'No verified
manifest' fallback.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`4d0f29d`)
- Fix(assertions): char-boundary-safe OCR snippet slicing

Naive byte slicing in snippet() and find_snippet() panicked on multi-byte
characters (smart quotes, em-dashes) produced by tesseract. Replace with
char-count truncation and floor/ceil_char_boundary helpers.

Surfaced by running the assert gate against hwLedger tapes where OCR text
contained U+2018 LEFT SINGLE QUOTATION MARK (3 bytes).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`c70d17e`)
## ✨ Features
- Feat(journey-viewer): keyframes_root manifest field for non-keyframes layouts (v0.1.3) (#4)

Bug: streamlit-journey manifests put frame-*.png inside recordings/<id>/,
not in a sibling keyframes/<id>/ dir. Viewer hardcoded the CLI-journeys
path prefix so all streamlit frames 404'd and keyframe gallery rendered
broken-image thumbs (user screenshot on hwLedger docs-site/journeys/
streamlit-fleet).

Fix: add optional keyframes_root manifest field; defaults to keyframes/<id>
(CLI layout unchanged). Streamlit manifests can set keyframes_root:
recordings/<id> to resolve frames from their real location.

Minor version bump 0.1.2 -> 0.1.3 (additive field, backwards compatible).

Co-authored-by: Forge <forge@phenotype.dev> (`f6ee613`)
- Feat(journey-viewer)!: deprecate Shot align prop, drop float CSS (0.1.2) (#2)

BREAKING: removes .shot-align-right / .shot-align-left CSS and stops
honouring align="left|right" on <Shot>. `align` is retained as a type-only
no-op for backwards compat; only `inline` and `center` paths remain, with
`center` now the default. Side-by-side layouts must migrate to <ShotGallery>
(introduced in 0.1.1).

Rationale: floated figures produced inconsistent reflow across VitePress
themes and blocked the docs-site layout audit from declaring Shot
deprecation complete. The docs-site migration (mech/gallery-batch +
fix/docs-layout-gallery) already converted all multi-shot layouts to
<ShotGallery>; the two solitary stragglers (secrets.md, cli-ingest-error.md)
can now drop align without visual regression.

- Bumps @phenotype/journey-viewer 0.1.1 -> 0.1.2
- Adds CHANGELOG.md with breaking-change entry
- Regenerates phenotype-journey-viewer-0.1.2.tgz for vendored consumers

Co-authored-by: Forge <forge@phenotype.dev>
Co-authored-by: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`cc78cf9`)
- Feat(journey-viewer): ShotGallery gallery-lightbox component v0.1.1 (#1)

Hero + thumbnail strip layout, flexbox/grid (zero floats), monospace for
code-like captions. Reuses KeyframeLightbox. Replaces adjacent Shot-with-
alternating-align that caused column-wrap regressions in hwLedger docs-site.

Consumed by hwLedger fix/docs-layout-gallery (commit 092fb92) which replaces
22 Shots with 12 ShotGalleries in visual-walkthrough-plan-deepseek.md and
5 Shots with 1 ShotGallery in quickstart Section 2.

Co-authored-by: Forge <forge@phenotype.dev> (`4acb288`)
- Feat(npm): @phenotype/playwright-record plugin

Batch 3 of the user-story-as-test framework (ADR 0034). Parses
@user-story YAML frontmatter from a Playwright spec's JSDoc block
and emits a manifest.verified.json with journey_id, persona,
given/when/then, traces_to, and per-step screenshot + ARIA
snapshot pointers.

- npm/playwright-record/src/frontmatter.ts: YAML extractor +
  schema validator with kebab-case journey_id enforcement and
  blind_judge enum checks.
- npm/playwright-record/src/recorder.ts: Recorder class that
  captures screenshots, ARIA snapshots, and assembles a
  schema-conformant manifest at
  target/user-stories/<journey_id>.manifest.json.
- npm/playwright-record/src/index.ts: Playwright test extension
  exposing a recorder fixture that auto-initialises from
  testInfo.file and finalises on teardown.
- 14/14 vitest tests green (11 frontmatter + 3 recorder).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`dd1ff8a`)
- Feat(journey-viewer): StructuralPane for Tier 0 structural-capture

New StructuralPane.vue renders the sibling `<frame>.structural.json`
produced by the three per-family walkers (macOS accessibility tree,
Streamlit ARIA + HTML, CLI terminal grid). Family-discriminated
rendering:
  - macos     -> JSON-tree for the a11y hierarchy
  - streamlit -> url/title meta + ARIA literal block
  - cli       -> reconstructed rows×cols terminal buffer w/ cursor marker

Wired into KeyframeLightbox.vue via a new "Structural: on/off"
toolbar button (disabled when the frame has no sibling). Toggle
state persists via localStorage (`phenotype-journey:structural-on`);
pane renders as a right-rail overlay (clamp(320px, 42vw, 560px))
that collapses below the image on narrow viewports.

types.ts: Step gains `structural_path?: string | null`; new
StructuralMacos / StructuralStreamlit / StructuralCli types + a
discriminated StructuralSnapshot union.

index.ts: re-exports StructuralPane.

Tarball refreshed in-place via `npm pack`:
  18,771 bytes -> 21,243 bytes  (+2,472 / +13%).

Consumers (hwLedger docs-site vendor/phenotype-journeys/…-0.1.0.tgz)
pick this up via `bun install`; build is green.

Traces to: Tier 0 structural-capture (viewer surface). (`de870dc`)
- Feat(journey-core+viewer): pluggable SigLIP/Sentence-Transformer agreement scorer

Replace the single-backend Jaccard-token scorer with a pluggable
`AgreementScorer` trait + three implementations selected by
`AgreementBackend` (Jaccard / SentenceTransformer / SigLip / Auto).

- phenotype-journey-core::agreement now exposes:
  - trait AgreementScorer { score(intent, blind, image) -> AgreementReport; name() }
  - JaccardScorer (text-text, legacy; Green >=0.60 / Yellow / Red)
  - SentenceTransformerScorer (text-text cosine via `python -c` subprocess
    driving `sentence-transformers`; Green >=0.75 / Yellow / Red)
  - SigLipScorer (image-text softmax via `transformers` + `torch`;
    Green >=0.30 / Yellow / Red)
  - AgreementBackend::Auto probes the Python env once per run and picks
    SigLip > SentenceTransformer > Jaccard based on module availability.
  - `backend_from_env()` resolves `PHENOTYPE_AGREEMENT_BACKEND`.
  - `AgreementReport` now carries `raw_score`, `backend`, `backend_model`
    so the viewer can tag the chip with the scorer used.
- Legacy `agreement::score(intent, blind)` preserved for backward compat
  (delegates to Jaccard).
- `pipeline::project` resolves the backend once per verify pass, looks up
  the keyframe path per step, and dispatches through the trait.
- Viewer: `Agreement` interface picks up `backend` / `raw_score`; chip
  tooltip reads "Agreement: GREEN -- SigLIP 0.42 (78%). Click for diff."
  Helper `backendLabel()` formats Jaccard / Sentence / SigLIP +
  `jaccard-fallback:*` variants.

Fallback policy: when the configured backend's Python deps are missing
or an image is required but absent, the scorer transparently falls back
to Jaccard and stamps `backend = "jaccard-fallback:<reason>"` so the UI
can surface the degradation.

Tests (agreement module, all pass):
- test_jaccard_still_works_as_fallback
- test_sentence_transformer_falls_back_to_jaccard_when_unavailable
- test_siglip_falls_back_when_no_image
- test_auto_backend_picks_something
- backend_parse_flag_accepts_all_variants
- report_roundtrips_through_serde
- matching_intent_and_blind_scores_green / divergent_intent_scores_red

Full workspace: `cargo test --workspace --lib -- --test-threads=1` ->
27/27 pass. Parallel execution triggers a pre-existing env-var race in
pipeline::tests around PHENOTYPE_JOURNEY_OCR_CMD; unrelated here.

Traces to: FR-UX-VERIFY-003

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`d54b14f`)
- Feat(journey-core+viewer): intent↔blind agreement scoring + chip/diff UI

Add `phenotype-journey-core::agreement` module (Jaccard overlap on
stopword-stripped, Porter-stemmed tokens via rust-stemmers) with
Green ≥0.6 / Yellow 0.3–0.6 / Red <0.3 buckets. AgreementReport
carries overlap, tokens, missing_in_blind, extras_in_blind.

`verify` bakes a per-step `agreement` payload into
manifest.verified.json so downstream consumers (viewer,
hwledger-traceability gate, hwledger-agreement-report) need no
second-pass scoring.

KeyframeLightbox + KeyframeGallery render a 🟢/🟡/🔴 "{overlap}%
overlap" chip next to the Intent label; clicking expands a diff
panel ("Missing in Blind" / "Extras in Blind") with the remediation
hint "re-record this step OR rewrite intent.yaml for frame N".
JourneyViewer plumbs `agreement` through both the top-level
keyframes[] path and the derived-from-steps[] path.

Fixes a pre-existing regression test that omitted
StepAssertions.must_contain_regex. Vendored tarball refreshed.

Traces to: FR-UX-VERIFY-003

Note: the JourneyViewer.vue also carries a pre-existing Rich/Raw
video toggle (Mode-2 WIP) that was already uncommitted on `main`;
those hunks land with this commit because they are not entangled
with the agreement plumbing.

Commit is unsigned: the local SSH signing key
(/Users/kooshapari/.ssh/id-git) is passphrase-locked and no agent
has it loaded; the environment cannot complete the attestation
non-interactively.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`081f4de`)
- Feat(journey-viewer): unify keyframe rendering + lightbox zoom + author glyphs

RecordingEmbed previously rendered its own <img> thumbnail grid which
bypassed the scroll/collapse caption treatment, Intent/Blind dual
labels, and lightbox with arrow nav + ESC + annotations toggle that
JourneyViewer already uses via KeyframeGallery. The raw <img> had no
click-to-expand and no lightbox at all.

Fixes:

1. RecordingEmbed.vue now delegates to <KeyframeGallery>. The manifest
   loader maps steps[] to the shape KeyframeGallery expects
   ({ path, caption, blind_description, annotations }) so thumbnails
   get the full treatment (hover-darken, expand pill, lightbox).
   (330 -> 312 LOC; -18, gallery logic moves out of this component.)

2. KeyframeLightbox gains click-to-zoom: clicking the image toggles
   between "fit" mode (viewBox scales to viewport, default) and
   "actual" mode (1:1 pixel, scrollbars for panning). Cursor becomes
   zoom-in / zoom-out to match. Keyboard: +/= zoom in, -/_ zoom out,
   0 reset to fit. A "Zoom: fit/1:1" button lives in the toolbar.

3. Intent / Blind attribution glyphs: the chips now carry a ✍︎ glyph
   (Intent = human authoring from intents.yaml) and ◉ glyph
   (Blind = VLM blind evaluator) with tooltip-on-hover that explains
   the provenance. Applied to both the gallery thumbnail captions and
   the lightbox caption pane for consistency.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`652a458`)
- Feat(shot-annotations): add v3 expected_text_index for OCR assertion

Extends the <Shot> annotation registry with an `expected_text_index`
section keyed by journey_id/frame_index. Each entry lists lowercase
tokens that MUST all appear (case-insensitive substring) in tesseract's
OCR of the baked keyframe. Consumed by `hwledger-shot-linter` to turn
the docs-site gate from caption-token heuristic into a registry-backed
ground-truth assertion.

Schema doc header updated to v3 with examples.

Existing v2 bare-list annotations remain valid (backward compatible).

Traces to: Deliverable B (annotation-registry extension) in the
shot-audit brief. (`7e348d3`)
- Feat(journey-viewer): baked-annotation toggle for gallery + lightbox

When a frame's `<frame>.annotated.png` sibling exists (pre-rendered by
hwledger-journey-render's annotate step), the gallery and lightbox now
prefer that baked image and suppress the live SVG overlay to avoid
double-rendering bboxes.

- New persistent toggle button in the lightbox toolbar: "Annotations
  baked: on/off" (localStorage key `phenotype-journey:annotations-baked-on`,
  default on).
- Gallery thumbnails respect the same key (no separate toggle in the grid
  to keep the UI quiet; the lightbox toggle drives both surfaces).
- Graceful fallback: if the baked PNG 404s at runtime (missing in dev,
  incomplete bake), both components transparently fall back to the raw
  image + live SVG overlay for that frame only.

Bumps vendored `phenotype-journey-viewer-0.1.0.tgz` accordingly.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`9c5d50a`)
- Feat(journey-viewer): add <Shot> inline screenshot component + annotation registry

Adds a new Vue 3 <Shot> component for inline game-walkthrough-wiki-style
screenshots with optional bbox annotations and click-to-expand lightbox
reusing KeyframeLightbox. Sizes: inline|small|medium|large. Alignments:
left|right|center|inline.

Also:
- data/shot-annotations.yaml: central annotation registry indexed by
  (journey_id, frame_index); Shot component consumes it when inline
  annotations are absent.
- remotion/borrowed/PROVENANCE.md: source provenance for patterns copied
  from dino (documents inspection outcome and shared annotation schema).
- Repacks phenotype-journey-viewer-0.1.0.tgz so vendored consumers
  (hwLedger docs-site) pick up the new export.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`70b7229`)
- Feat(viewer): embed recording video (+gif fallback) at top of JourneyViewer

Prior shape rendered only KeyframeGallery + steps table, omitting the MP4/GIF
that verify produces. New block reads manifest.recording (mp4 path) and
recording_gif, renders <video controls> with first keyframe as poster and GIF
as <noscript> fallback. Adds 'Open MP4 ↗' / 'Open GIF ↗' raw-asset links below.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`38c6198`)
- Feat(lightbox): scrollable + collapsible caption with Show-more toggle

Matches the KeyframeGallery caption UX shipped earlier. Default caption
scrolls within 6rem; 'Show more ▾' expands to 20rem; aria-expanded wired.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`f5766ce`)
- Feat(assertions): add must_contain_regex for OCR-mangled output

Tesseract consistently mangles certain glyph pairs (e.g. `__EXIT_0__`
reads as `EXIT_O.`, `__` collapses to `_`, digit `0` reads as letter `O`
or `@`). Literal `must_contain` substrings can't express that tolerance
without losing precision, so add a parallel `must_contain_regex: Vec<String>`
field on `StepAssertions` that is compiled via `regex::Regex` and
fail-fasts on bad patterns (new `InvalidRegex` violation kind).

- crates/phenotype-journey-core/Cargo.toml: +regex = \"1\"
- StepAssertions: +must_contain_regex (serde-skipped when empty)
- run_on_manifest: evaluate regex patterns, emit MustContainRegex or
  InvalidRegex violations
- ViolationKind: +MustContainRegex, +InvalidRegex
- bin/phenotype-journey print_report: render both new kinds
- 2 unit tests: match + (non-match + invalid pattern)

Consumers (e.g. hwLedger strict-journey gate) can now anchor
assertions against glyph-fuzzy OCR output (`EXIT[_ ]?[0O]` matches
both the canonical `__EXIT_0__` and the OCR-rendered `EXIT_O.`).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`de1b514`)
- Feat(ocr): add Apple Vision OCR backend (PHENOTYPE_JOURNEY_OCR_BACKEND=vision)

Adds an opt-in Apple Vision VNRecognizeTextRequest OCR backend for journey
keyframe assertions, targeting Catppuccin Mocha + JetBrains Mono terminal
captures that Tesseract mis-reads without heavy ImageMagick preprocessing.

* New `vision` cargo feature on phenotype-journey-core and phenotype-journey
  (macOS only). Uses idiomatic objc2 + objc2-foundation + objc2-vision
  bindings — no hand-rolled msg_send!.
* Runtime selection via PHENOTYPE_JOURNEY_OCR_BACKEND env var:
  `tesseract` (default), `vision` (Vision), unknown → hard error.
* Backend fails loudly when `vision` selected but feature not compiled or
  host is not macOS (no silent fallback).
* Exit-sentinel checker now tolerates common OCR artefacts around
  consecutive underscores (both Tesseract and Vision mangle `__EXIT_N__`);
  the shell producer still emits the canonical sentinel, we only loosen
  the recogniser.

Verified on hwLedger plan-hf-resolve keyframes: 0 violations under Vision
where Tesseract produced 11. (`292f9db`)
- Feat(npm): sync journey-viewer package with hwLedger components

- Sync JourneyViewer, KeyframeLightbox, RecordingEmbed from hwLedger
  (richer implementations with annotations, video fallbacks, a11y)
- Add KeyframeGallery, JourneyStep, JudgeScore to the package so
  consumers can render the full hwLedger-style journey UI out of the box
- Add publishConfig.registry=https://npm.pkg.github.com to both npm
  packages (GitHub Packages, org-private distribution)
- Add repository metadata pointing at KooshaPari/phenotype-journeys
- Add npm/PUBLISHING.md documenting PAT scope + tarball fallback

Prepares @phenotype/journey-viewer@0.1.0 and
@phenotype/journey-playwright@0.1.0 for publish to npm.pkg.github.com
and unblocks the hwLedger consumer swap in MIGRATION.md.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`22f7a35`)
- Feat: check-verified subcommand + Step.blind_description + dual-label lightbox

- bin: new `phenotype-journey check-verified --root <dir>` scans manifest.json
  dirs for missing manifest.verified.json; strict, no escape hatch.
- core: Step gains blind_description field (serde opt-skip). pipeline.rs
  synthesize_blind_description deterministically fills it in mock mode
  when Claude-describe isn't populated.
- npm/journey-viewer: KeyframeLightbox renders dual Intent + Blind labels.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`f6886c0`)
- Feat(annotations): bbox annotations + Vue lightbox + tesseract auto-annotate

- Annotation / AnnotationKind / AnnotationStyle types on Step (serde + schemars).
- phenotype-journey annotate --provider tesseract emits line-level bboxes
  with confidence filter (--ocr-words flips to word-level); vlm provider
  stubbed with NotImplemented + TODO. --to stdout|manifest|yaml.
- pipeline.rs intents overlay copies YAML-authored annotations onto each
  step alongside intents + assertions (manifest-embedded wins).
- JSONSchema updated; TS types added.
- @phenotype/journey-viewer ships KeyframeLightbox.vue: Teleport modal,
  SVG bbox overlay with label pills, keyboard nav, focus trap, toggle
  (localStorage), copy-JSON-to-clipboard.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`8fe31df`)
- Feat(pipeline): add record / extract-keyframes / verify (batch) / sync subcommands

Ports the bash+python journey pipeline from hwLedger into Rust so the
`phenotype-journey` CLI owns the full record → extract → verify → sync flow
as a single, testable binary. The new core `pipeline` module exposes:

- `record_all`       replaces hwLedger/apps/cli-journeys/scripts/record-all.sh
- `extract_keyframes` replaces extract-keyframes.sh (stale-frame cleanup moved
                     into Rust so the race condition is fixed)
- `verify_all`       replaces verify-manifests.sh + mock-anthropic-server.py
                     (built-in mock responder, intents YAML overlay with
                     traces_to preservation, manifest-wins-on-conflict
                     assertion semantics, assertion_violations union in
                     verification.assertion_violations)
- `sync_artefacts`   generic sync with `--kind {cli|gui|streamlit|adrs|research|auto}`,
                     replaces all five docs-site/scripts/sync-*.sh

6 unit tests added covering the non-trivial contracts: stale-frame cleanup,
intents-overlay precedence (manifest wins), traces_to top-level preservation,
violation union into verification output, cli-journey sync shape, and auto
sniffing. Total core crate test count: 17 passed.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`775f98c`)
- Feat(assertions): add OCR-backed ground-truth assertion layer

Adds `phenotype_journey_core::assertions` + `phenotype-journey assert`
subcommand. Steps may carry optional `StepAssertions { must_contain,
must_not_contain, expected_exit, ocr_required }`. The assert layer OCRs
matching keyframes via `tesseract` (override with PHENOTYPE_JOURNEY_OCR_CMD)
and produces a `Violation` list with `MustContain`, `MustNotContain`, and
`ExitCode` kinds.

The layer fails loudly when tesseract is missing — no silent skip — and
warns when a journey carries zero assertions.

Includes regression test that pins the pre-fix traceability-report OCR
dump (error: unexpected argument) and proves the assertion layer catches
it.

Schema, README, and the manifest JSONSchema are updated to describe the
new fields and the canonical exit-code sentinel tape pattern
(`<cmd>; echo __EXIT_\$?__`).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`c570ff4`)
- Feat: initial phenotype-journeys package

Extract hwLedger's ad-hoc journey harness into a reusable Phenotype-org
package. Provides:

- phenotype-journey-core: Rust lib with Manifest/Step/Verification types,
  JSONSchema export, and a describe+judge verify loop (mock + live modes).
- phenotype-journey: CLI (record, verify, validate, sync, schema).
- @phenotype/journey-viewer: Vue 3 components (JourneyViewer, RecordingEmbed)
  ported from hwLedger docs-site and made project-agnostic.
- @phenotype/journey-playwright: Playwright -> manifest bridge.
- schema/manifest.schema.json: canonical JSONSchema.

Planned consumers: hwLedger, AgilePlus, thegent.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`f26ee66`)
## 🔨 Other
- Chore(governance): adopt CLAUDE.md + governance framework

Enable AgilePlus spec tracking, FR traceability, and standard project conventions. Wave-5 governance push.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`2eabaa2`)
- Test(smoke): seed minimal smoke test — proves harness works (`204e5aa`)
- Chore(ci): adopt phenotype-tooling quality-gate + fr-coverage (`ff0a048`)
- Chore(phenotype-journeys): prune 1 dead_code suppression, annotate 2 kept items (#3)

- Delete unused private `walk` helper in bin/phenotype-journey/src/main.rs
  (no callers; superseded by walkdir usage in overlay_assertions)
- Annotate IntentsFile.journey and IntentStep.intent with `// kept:` —
  these are serde-deserialized for YAML shape validation but not yet
  wired into the manifest overlay path

cargo build: clean in 2.3s.

Co-authored-by: Forge <forge@phenotype.dev>
Co-authored-by: Claude Opus 4.7 (1M context) <noreply@anthropic.com> (`1a2bcd3`)