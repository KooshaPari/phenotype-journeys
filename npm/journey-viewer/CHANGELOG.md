# Changelog

All notable changes to `@phenotype/journey-viewer` are documented here. This
project follows [Semantic Versioning](https://semver.org/) at the package
level; minor bumps may carry breaking changes while the package remains on
`0.x`.

## 0.1.0 — first GitHub Packages cut (2026-07)

First coordinated registry version for the `npm/*` suite (aligned with the
Rust `v0.1.0` train). Pre-registry local bumps noted as 0.1.1 / 0.1.2 below
ship in this `0.1.0` tarball; those sections remain as development history.

## 0.1.2 — 2026-04-22 (pre-registry)

### BREAKING

- **`<Shot>` `align` prop is deprecated and no longer honours `left` / `right`.**
  The float-based `.shot-align-left` and `.shot-align-right` CSS rules have
  been removed. The prop is retained as a type-only field for backwards
  compatibility across minor versions, but any value other than `inline`
  collapses to `center`. Consumers that relied on side-by-side floats must
  migrate to `<ShotGallery>`; consumers that passed `align="right"` for a
  solitary shot should drop the prop (or wrap in a single-shot
  `<ShotGallery>` for explicit layout).

  Rationale: floated figures produced inconsistent reflow across VitePress
  themes and broke the layout audit. `<ShotGallery>` (added in 0.1.1) is
  now the canonical side-by-side primitive; solitary shots render cleanly
  inline without alignment hints.

### Changed

- `<Shot>` default `align` is now `center` (previously `right`).
- Removed `.shot-align-right` / `.shot-align-left` CSS rules and the
  `@media (max-width: 640px)` override that reset their floats.

## 0.1.1 — 2026-04

- Added `<ShotGallery>` gallery + lightbox component.

## 0.1.0 — initial

- Initial `<Shot>`, `<JourneyViewer>`, `<KeyframeLightbox>`, annotation
  registry.
