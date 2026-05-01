# phenotype-journeys — Claude Code Instructions

## Project Overview
- **Name**: phenotype-journeys
- **Description**: Reusable journey harness (Rust CLI + Vue components + Playwright helper) for the Phenotype ecosystem
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-journeys`
- **Language Stack**: Rust, Vue, TypeScript, Python
- **Status**: Active development

## AgilePlus Mandate
All work MUST be tracked in AgilePlus:
- Reference: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`
- CLI: `cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus && agileplus <command>`
- No code without corresponding AgilePlus spec.

## Stack & Commands
```bash
# Rust build
cargo build --workspace

# Rust tests
cargo test --workspace

# Frontend (if applicable)
bun install && bun run dev

# Quality gate
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

## Quality Checks
From this repository root:
- `cargo build --workspace` — compile
- `cargo test --workspace` — tests
- `cargo clippy --workspace -- -D warnings` — lint

## Git & Branch Discipline
- Feature branches: `worktrees/<topic>/`
- Canonical: `main`
- Never commit directly to `main`

## References
- Parent workspace: `/Users/kooshapari/CodeProjects/Phenotype/repos/CLAUDE.md`
- AgilePlus: `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus`
