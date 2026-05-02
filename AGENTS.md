# phenotype-journeys ẀAGENTS.MD

Project Overview
Ruskable journey harness (Rust CLI + Vue components + Playwright helper) for the Phenotype ecosystem. Tests user-facing flows across Phenotype products.

## Stack
- Core: Rust CLI
- UI : Vue components
- Testing : Playwright
- Build : Cargo + Vite

# K Key Commands
- `cargo build --release`	
- `cargo test`
- `playwright test`

## Quality Gates
- `cargo check --workspace`	
- `playwright test`

# # Notes
Journeys are cross-product integration tests. Run against live dev stacks or mock environments. Snapshot tests use Playwright for screenshot diffing.