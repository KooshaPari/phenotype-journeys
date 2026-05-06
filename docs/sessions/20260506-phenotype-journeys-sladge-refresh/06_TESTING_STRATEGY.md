# Testing Strategy

## Required

- `git diff --check`
- `rg -n "sladge|AI Slop" README.md`

## Cargo Gates

- `cargo test --workspace --offline`
- `cargo clippy --workspace --offline -- -D warnings`
- `cargo fmt --all --check`

Any failures outside README/session docs should be recorded as existing blockers.
