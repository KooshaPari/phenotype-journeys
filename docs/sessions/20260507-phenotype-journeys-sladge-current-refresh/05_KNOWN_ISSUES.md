# Known Issues

## Superseded Branch

Older prepared evidence at `46bfc8c` on
`docs/phenotype-journeys-sladge-current2` diverged from the active local branch
and is superseded by this current-head refresh.

## Validation Blockers

`cargo fmt --all --check` reports pre-existing Rust formatting drift in
`bin/phenotype-journey/src/main.rs`,
`crates/phenotype-journey-core/src/agreement.rs`,
`crates/phenotype-journey-core/src/assertions.rs`,
`crates/phenotype-journey-core/src/pipeline.rs`,
`crates/phenotype-journey-core/src/verify/mod.rs`,
`crates/phenotype-journey-core/src/vision.rs`, and
`crates/phenotype-journey-core/tests/regression_traceability.rs`.

`cargo clippy --workspace --offline -- -D warnings` and
`cargo test --workspace --offline` both stop before source checks because
`fancy-regex v0.13.0` is not cached for offline validation.

Direct `task lint` and `task test` attempt to update the crates.io index and
fail DNS resolution for `index.crates.io` in the restricted network
environment.
