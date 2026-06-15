<!-- AI-DD-META:START -->
<!-- This repository is planned, maintained, and managed by AI Agents only. -->
<!-- Slop issues are expected and intentionally present as part of an HITL-less -->
<!-- /minimized AI-DD metaproject of learning, refining, and building brute-force -->
<!-- training for both agents and the human operator. -->
![Downloads](https://img.shields.io/github/downloads/KooshaPari/phenotype-journeys/total?style=flat-square&label=downloads&color=blue)
![GitHub release](https://img.shields.io/github/v/release/KooshaPari/phenotype-journeys?style=flat-square&label=release)
![License](https://img.shields.io/github/license/KooshaPari/phenotype-journeys?style=flat-square)
![AI-Slop](https://img.shields.io/badge/AI--DD-Slop%20Expected-orange?style=flat-square)
![AI-Only-Maintained](https://img.shields.io/badge/Planned%20%26%20Maintained%20by-AI%20Agents%20Only-red?style=flat-square)
![HITL-less](https://img.shields.io/badge/HITL--less%20AI--DD-metaproject-yellow?style=flat-square)

> ⚠️ **AI-Agent-Only Repository**
>
> This repo is **planned, maintained, and managed exclusively by AI Agents**.
> Slop issues, rough edges, and AI artifacts are **expected and intentionally
> present** as part of an **HITL-less / minimized AI-DD** metaproject focused
> on learning, refining, and brute-force training both the agents and the
> human operator. Bug reports and contributions are still welcome, but please
> expect AI-generated code, comments, and documentation throughout.
<!-- AI-DD-META:END -->
# phenotype-journeys

[![License](https://img.shields.io/github/license/KooshaPari/phenotype-journeys)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/KooshaPari/phenotype-journeys/ci.yml?branch=main&label=build)](https://github.com/KooshaPari/phenotype-journeys/actions/workflows/ci.yml)
[![AI Slop Inside](https://sladge.net/badge.svg)](https://sladge.net)

Shared, project-agnostic **journey harness** for the Phenotype org: record a
user-facing flow (CLI tape, UI test, or Playwright trace), emit a canonical
manifest, and verify it with a Claude-describe + Claude-judge loop.

## Why this exists

Journey harness code kept accreting inside individual projects — VHS tapes +
manifest JSON in `hwLedger/apps/cli-journeys/`, XCUITest capture in
`hwLedger/apps/macos/HwLedgerUITests/`, a `JourneyViewer.vue` in the hwLedger
docs theme, a `hwledger-gui-recorder` crate. None of it was reusable. Every
new Phenotype project that wanted journeys had to re-derive the format and
the verify loop from scratch.

`phenotype-journeys` extracts those into one package:

- **Canonical manifest schema** (`schema/manifest.schema.json`) — source of
  truth for step/verification shape.
- **`phenotype-journey-core`** (Rust) — serde types, schema export, and the
  verify loop in both `mock` and `live` (Anthropic API) modes.
- **`phenotype-journey`** (Rust CLI) — `record`, `verify`, `validate`, `sync`.
- **`@phenotype/journey-viewer`** (Vue 3) — `JourneyViewer` +
  `RecordingEmbed` components for VitePress docs.
- **`@phenotype/journey-playwright`** (TypeScript) — script a web page and
  emit a conformant manifest.

## Traceability

Journey evidence is part of the repo contract, not just a consumer concern.
See [docs/journey-traceability.md](docs/journey-traceability.md) for the
shared standard and repo expectations.

## Planned consumers

- **hwLedger** — swap `apps/cli-journeys/scripts/verify-manifests.sh` +
  in-theme `JourneyViewer.vue` for this package. (See `MIGRATION.md` in the
  hwLedger repo.)
- **AgilePlus** — journeys for feature specs (one per tagged FR).
- **thegent** — journeys for plugin onboarding flows.

## Acceptance criteria

Consuming projects should fail their quality gate if a spec tagged as
user-facing does not have a corresponding passing journey manifest. Enforce
via CI:

```bash
phenotype-journey validate docs/journeys/manifests/<spec-id>/manifest.verified.json
```

## Quickstart

```bash
# 1. Record a CLI tape (wraps charmbracelet/vhs)
phenotype-journey record --tape tapes/first-plan.tape --out journeys/

# 2. Write a stub manifest (hand-authored or from journey-playwright)

# 3. Validate against the canonical schema
phenotype-journey validate journeys/manifests/first-plan/manifest.json

# 4. Verify (mock mode, offline, no API key needed)
phenotype-journey verify journeys/manifests/first-plan/manifest.json

# 5. Ship artefacts to the docs public dir
phenotype-journey sync --from journeys --to docs/public/journeys
```

Set `ANTHROPIC_API_KEY` and pass `--live` (requires building with
`--features live`) to call real Claude for describe + judge passes.

## Ground-truth assertions (`phenotype-journey assert`)

Claude-judge alone is soft. A tape can literally display
`error: unexpected argument` and still pass if the judge hallucinates. The
`assert` subcommand adds hard gates that fail the build when frame content is
wrong.

### Dependencies

- `tesseract` CLI — install with `brew install tesseract` (macOS) or
  `apt-get install tesseract-ocr` (Debian). **No silent skip**: if tesseract
  is missing the command exits non-zero with a clear message.
- Override OCR for tests or custom pipelines via
  `PHENOTYPE_JOURNEY_OCR_CMD="my-ocr {{PATH}}"` (the `{{PATH}}` token is
  replaced with the PNG path).

### Intents YAML schema extension

Each step may carry an `assertions` block:

```yaml
journey: traceability-report
steps:
  - index: 1
    intent: "Command typed: cargo run ..."
    assertions:
      must_contain: ["cargo run", "hwledger-traceability"]
      must_not_contain: ["error:", "unexpected argument"]
      ocr_required: true
  - index: 8
    intent: "Integer row-count returned"
    assertions:
      expected_exit: 0
```

- `must_contain` — every listed substring must appear in the OCR of that
  step's keyframe.
- `must_not_contain` — none of the listed substrings may appear.
- `expected_exit` — the LAST keyframe of the journey must include the
  sentinel `__EXIT_<N>__`.
- `ocr_required` — reserved for future "OCR must succeed" gating; default
  inferred from the presence of contain/not_contain lists.

### Exit-code sentinel (canonical tape pattern)

Wrap the final command in the tape so the sentinel lands in the recording:

```vhs
Type "hwledger plan --help; echo __EXIT_$?__"
Enter
Sleep 500ms
```

This produces a visible `__EXIT_0__` (or `__EXIT_N__`) in the last frame that
`phenotype-journey assert` can OCR and gate on.

### Usage

```bash
phenotype-journey assert apps/cli-journeys/manifests/plan-deepseek/manifest.json --strict
```

With `--strict`, exits non-zero when any assertion is violated. Without, the
report prints but the process exits 0. Journeys with zero assertions print a
loud warning so they cannot hide.

## How to verify a journey manifest

The `phenotype-journey verify` subcommand is the canonical CI gate for journey
manifests. The first-class surface takes `--manifest <path>` (alias
`--manifest-path`) and `--docs-root <path>` so it slots directly into phenodocs
PR #168 and any consumer following the
[journey-traceability standard](docs/journey-traceability.md):

```bash
phenotype-journey verify \
  --manifest docs/journeys/manifests/phenodocs-bootstrap.journey.json \
  --docs-root docs
```

The flag pair runs the Claude-describe + Claude-judge verify loop in mock
mode (offline, deterministic) and also runs the OCR-backed assertion engine
against `<docs-root>/keyframes/<id>/frame-###.png`, then emits a unified
JSON envelope. The process exits non-zero when `all_intents_passed` is false
OR any `must_contain` / `must_not_contain` / `expected_exit` violation trips
— making the gate a hard fail for the build. The legacy positional
`phenotype-journey verify <path>` form still works for backwards
compatibility, and `--live` switches to the real Anthropic API backend when
the `live` cargo feature is enabled and `ANTHROPIC_API_KEY` is set.

## Repo layout

```
phenotype-journeys/
  crates/phenotype-journey-core/   # Rust lib: types, schema, verify loop
  bin/phenotype-journey/           # Rust CLI
  npm/journey-viewer/              # Vue 3 components
  npm/journey-playwright/          # Playwright -> manifest bridge
  schema/manifest.schema.json      # Canonical JSONSchema
```

## License

Apache-2.0.

## Documentation

This repository includes the following cross-cutting documents:

- [`AGENTS.md`](AGENTS.md) — operating instructions for AI agents and human contributors
- [`docs/`](docs/) — design notes, ADRs, and supporting documentation (see [`docs/index.md`](docs/index.md))
