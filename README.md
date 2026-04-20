# phenotype-journeys

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
