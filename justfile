# Phenotype-org standard justfile — phenotype-journeys
# See https://github.com/KooshaPari/phenotype-tooling

default:
    @just --list --unsorted

# ── Build ───────────────────────────────────────────────────────────

# Build workspace (debug)
build:
    cargo build --workspace

# Build workspace (release)
release:
    cargo build --release --workspace

# Check workspace compiles (fast)
check:
    cargo check --workspace

# ── Test ────────────────────────────────────────────────────────────

# Run all tests
test:
    cargo test --workspace

# Run tests with output
test-verbose:
    cargo test --workspace -- --nocapture

# ── Quality ─────────────────────────────────────────────────────────

# Lint (clippy + fmt --check)
lint:
    cargo clippy --workspace -- -D warnings
    cargo fmt --check

# Format code
fmt:
    cargo fmt

# Security audits (cargo-deny + cargo-audit)
audit:
    cargo deny check
    cargo audit

# Find unused dependencies
unused:
    cargo machete

# Full local CI sweep (lint + test + audit)
ci: lint test audit

# ── Documentation ───────────────────────────────────────────────────

# Generate docs
docs:
    cargo doc --no-deps --workspace

# Generate and open docs
docs-open:
    cargo doc --no-deps --workspace --open

# ── Maintenance ─────────────────────────────────────────────────────

# Show outdated dependencies
outdated:
    cargo outdated -R

# Clean build artifacts
clean:
    cargo clean
