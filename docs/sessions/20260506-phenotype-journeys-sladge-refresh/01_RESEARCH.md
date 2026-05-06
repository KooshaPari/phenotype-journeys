# Research

## Current State

- Canonical `phenotype-journeys` is clean on `chore/trufflehog-20260502`.
- Older badge worktree `phenotype-journeys-wtrees/sladge-current` is
  `ahead 1, behind 1` and should not be reused.
- Current branch includes workflow concurrency changes absent from the older
  badge evidence.

## Decision

Refresh the README badge from the current canonical head in a new isolated
worktree, then re-run the same validation gates that previously exposed OCR,
clippy, and fmt drift.
