# CLAUDE.md — phenotype-journeys
#
# Tier-0 governance for agent context.
# See hierarchical chain:
#   ~/.claude/AGENTS.md  ←  repos/AGENTS.md  ←  repos/CLAUDE.md  ←  this

## Project

- **Name**: phenotype-journeys
- **Purpose**: Journey orchestration and state machine library (Rust)
- **Status**: Active development
- **Repository**: https://github.com/KooshaPari/phenotype-journeys

## Quick Commands

| Command | Description |
|---|---|
| `cargo check --workspace` | Fast compile check |
| `cargo build --workspace` | Build all crates |
| `cargo test --workspace` | Run all tests |
| `cargo clippy --workspace -- -D warnings` | Lint |
| `cargo fmt --check` | Format check |
| `cargo deny check` | License/advisory check |
| `cargo audit` | Security audit |
| `just ci` | Full CI sweep |

## Quality Gates

- All tests pass
- No clippy warnings (deny level)
- `cargo fmt --check` clean
- `cargo deny check` clean
- `cargo audit` clean (or documented exceptions)

## Governance Hierarchy

1. `~/.claude/AGENTS.md` — global agent instructions
2. `/repos/AGENTS.md` — Phenotype org governance
3. `/repos/CLAUDE.md` — Phenotype org project guidance
4. `AGENTS.md` — this repo's agent instructions
5. `docs/` — technical specifications and annexes
