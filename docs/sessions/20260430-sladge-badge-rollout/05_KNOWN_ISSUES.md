# Known Issues

- Canonical `phenotype-journeys` has unrelated local changes and is ahead/behind
  origin, so merge is deferred.
- `cargo fmt --check` fails on existing Rust formatting drift outside this
  README-only badge change.
- `cargo test --workspace` and `cargo clippy --workspace -- -D warnings` are
  blocked by crates.io DNS/network resolution while fetching `chrono`.
