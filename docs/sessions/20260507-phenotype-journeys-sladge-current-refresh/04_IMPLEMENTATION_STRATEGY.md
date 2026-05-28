# Implementation Strategy

## Approach

Use a fresh current-head worktree instead of reusing stale prepared evidence.
Keep the downstream change limited to README badge evidence and session
documentation.

## Boundaries

- Do not modify canonical phenotype-journeys during the refresh.
- Do not reuse stale `docs/phenotype-journeys-sladge-current2` evidence.
- Do not apply broad formatter or dependency changes unless validation proves
  they are necessary for this scoped update.
