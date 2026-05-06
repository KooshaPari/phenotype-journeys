# Specifications

## Functional Contract

- `README.md` must include:
  `[![AI Slop Inside](https://sladge.net/badge.svg)](https://sladge.net)`.
- The badge must stay in the README badge block.
- The change must not touch source, workflow, `deny.toml`, fixtures, or
  generated artifacts.

## ARUs

- Assumption: `chore/trufflehog-20260502` is the current branch to reconcile.
- Risk: existing OCR fixture, lint, or fmt failures may still block full
  integration.
- Uncertainty: online Cargo validation may remain blocked by sandbox DNS.
