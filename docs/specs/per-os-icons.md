# Per-OS Icon Spec — phenotype-journeys

**Status:** PROPOSED (no assets generated yet)
**Date:** 2026-06-04
**Author:** L1 worker (apps registry audit)

## Why

phenotype-journeys ships a Rust core + a `journey-viewer` web SPA + a
`phenotype-journey` CLI. The web viewer needs favicon + social card;
the CLI is headless (no desktop icon needed). This spec is short.

## Base vector

Hand-authored SVG at `assets/brand/logo.svg` — a winding "journey path"
(a sinuous trail with milestone dots and an arrowhead terminus) over a
rounded app tile.

## Variants

| Surface     | File                                          | Size          | Notes |
|-------------|-----------------------------------------------|---------------|-------|
| Web viewer  | `npm/journey-viewer/public/favicon.ico`       | 16/32/48      | Browser tab |
| Web viewer  | `npm/journey-viewer/public/logo.svg`          | vector        | Navbar |
| Web viewer  | `npm/journey-viewer/public/apple-touch-icon.png` | 180x180    | iOS home-screen if PWA-installed |
| Social      | `assets/brand/social-512.png`                 | 512x512       | OG/Twitter card |
| Docs        | `docs/public/favicon.ico`                     | 16/32/48      | (if docs site exists) |
| CLI         | (none — headless)                             | —             | n/a |
| Crates      | (none — cargo doesn't display logos)          | —             | n/a |

## Material language

Phenotype keycap palette (teal #7ebab5 + midnight #090a0c). Journey =
sinuous trail, NOT a flat glyph or generic chevron.

## AI-DD + renderers

- AI-CODED (hand-authored SVG).
- Raster: resvg → ImageMagick → Pillow fallback.
- Favicon ICO: ImageMagick or Pillow.

## Out of scope

- Per-OS desktop icon variants (CLI is headless; web viewer runs in browser).
- Windows .ico for the CLI installer (separate infra decision; track in a follow-up if needed).

## Open questions

- Does `journey-viewer` ship a Tauri/Electron shell in the future, or stay browser-only? (decides whether to expand to per-OS desktop icons later.)
