# Mined patterns: KlipDot → phenotype-journeys

**Source:** [KooshaPari/KlipDot](https://github.com/KooshaPari/KlipDot) (read-only mine, 2026-05-31)  
**Status:** KlipDot is a legacy terminal-image interceptor; product path is superseded by browser/desktop automation. These patterns are **evidence-capture conventions only**.

## Why this doc exists

KlipDot already implemented the phenotype journey-traceability stub and a full **VHS** (`.tape`) demo library. This file captures what is worth reusing in **phenotype-journeys** without reviving KlipDot.

## Borrow: VHS demo layout

KlipDot structure (reference):

```
demos/
  demo-authentic-usage.tape
  demo-clipboard-workflow.tape
  demo-comprehensive-showcase.tape
  README.md          # vhs <tape> regeneration commands
```

**Adopt in consumers:**

1. One `.tape` per user-facing flow listed in `journeys/manifests/<spec-id>/manifest.json`.
2. README section: install VHS, regenerate GIF, commit keyframes + recording paths.
3. Prefer **authentic command output** (real binary) over mocked terminal text — KlipDot `demo-authentic-usage.tape` pattern.

## Borrow: journey traceability checklist

KlipDot `docs/operations/journey-traceability.md` mirrors our standard:

- [ ] Identify key user-facing flows
- [ ] Record VHS tapes for each flow
- [ ] Author manifests under `docs/journeys/manifests/` (or repo `journeys/manifests/`)
- [ ] Run `phenotype-journey verify` in CI

Link canonical schema: `schema/manifest.schema.json` in this repo.

## Borrow: CI quality-gate fallback

KlipDot `.github/workflows/quality-gate.yml` builds `phenotype-tooling` `quality-gate` if missing — same pattern as other Phenotype repos. No change required here; documented for parity.

## Do not borrow

- Clipboard interception daemon (platform-specific, superseded).
- Node `terminal-interceptor.js` legacy paths.
- HTTP image API for agents (use desktop/browser capture in KDesktopVirt / PlayCua instead).

## Related fork-lane repos

| Repo | Role |
|------|------|
| KommandLineAutomation | CLI recording harness — pair VHS keyframes with KLA session artifacts |
| KDesktopVirt | Full desktop evidence when flows leave the terminal |

## Full mine index

See `E:/scratch/mined-4-repos.md` (local scratch; not committed).
