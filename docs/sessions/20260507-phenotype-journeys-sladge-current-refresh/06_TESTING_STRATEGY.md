# Testing Strategy

## Planned Checks

- `git diff --check` passed.
- README badge search with `rg` passed.
- `task fmt` completed but would rewrite pre-existing Rust formatting drift, so
  those formatter outputs were restored as out of scope.
- `cargo fmt --all --check` reports pre-existing Rust formatting drift outside
  this README/session-doc change.
- `task lint` and `task test` are blocked by restricted-network crates.io DNS.
- `cargo clippy --workspace --offline -- -D warnings` and
  `cargo test --workspace --offline` are blocked by uncached
  `fancy-regex v0.13.0`.

## Scope

This is a README/session-doc governance refresh. Failures from unrelated
pre-existing source, missing cached dependencies, or sandbox limits are recorded
as blockers rather than broadened into this change.
