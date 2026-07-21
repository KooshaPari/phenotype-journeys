# Cutting a phenotype-journeys release

Operator runbook for tagging `v0.1.0` (T1) and publishing Rust crates to
crates.io (T2). Do **not** run these until `main` has the release-ready
CHANGELOG section and local quality gates are green.

T0 honesty still applies until T2 completes: end-user install remains
**path/git only** — do not advertise `cargo install phenotype-journey` /
`cargo add phenotype-journey-core` until crates.io publish succeeds.

## Preconditions

```bash
cd /path/to/phenotype-journeys   # worktree or canonical after merge
git checkout main
git pull --rebase origin main
cargo test --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
grep -A2 '## \[0.1.0\]' CHANGELOG.md   # section must exist with a date
```

Confirm root `Cargo.toml` `workspace.package.version = "0.1.0"` matches the
CHANGELOG heading.

## Publish order (crates.io)

Publish in dependency order. Do **not** publish `klipdot-capture` in this
release (`publish = false` — local KlipDot daemon, not part of the journey
harness crates.io surface).

| Order | Crate | Notes |
|------:|-------|-------|
| 1 | `phenotype-journey-core` | No workspace path deps |
| 2 | `phenotype-journeys-observability` | No workspace path deps |
| 3 | `phenotype-journey` | Depends on (1) and (2) via version+path |

## T1 — git tag + GitHub Release (no crates.io yet)

Exact commands (annotated tag preferred):

```bash
git tag -a v0.1.0 -m "phenotype-journeys v0.1.0"
git push origin v0.1.0

gh release create v0.1.0 \
  --title "v0.1.0" \
  --notes-file <(sed -n '/## \[0.1.0\]/,/^## \[/p' CHANGELOG.md | sed '$d')
```

Alternate notes from the tag:

```bash
gh release create v0.1.0 --title "v0.1.0" --notes-from-tag
```

Publishing the GitHub Release triggers
[`.github/workflows/release-attestation.yml`](../../.github/workflows/release-attestation.yml)
(SLSA build attestation). GitHub Actions billing may block the run — treat
attestation as best-effort; do not block crates.io publish on Actions green.

Do **not** run `cargo publish` in T1.

## T2 — crates.io publish

After the tag exists on the release commit:

```bash
cargo login          # once per machine / token

# Pre-T2 local check (path deps): CLI builds without needing crates.io yet
cargo build -p phenotype-journey --locked

# 1) library (types + verify)
cargo publish -p phenotype-journey-core --dry-run --locked
cargo publish -p phenotype-journey-core --locked

# 2) observability adapter
cargo publish -p phenotype-journeys-observability --dry-run --locked
cargo publish -p phenotype-journeys-observability --locked

# 3) CLI — dry-run only works AFTER 1+2 are indexed on crates.io.
# Before that, expect: "no matching package named `phenotype-journey-core`".
# Wait a few seconds / retry if the index lags.
cargo publish -p phenotype-journey --dry-run --locked
cargo publish -p phenotype-journey --locked
```

After a successful publish:

1. Prefer `cargo install phenotype-journey --locked` / `cargo add phenotype-journey-core` in README Install.
2. Update Work State: crates.io = published; keep git tag / Release as done.
3. npm packages remain on the separate GitHub Packages path (`npm/PUBLISHING.md`).

## Rollback / mistakes

- Wrong tag before push: `git tag -d v0.1.0` (local only).
- Wrong tag already pushed: do **not** force-delete without explicit operator
  approval; prefer a patch tag (`v0.1.1`) and a forward CHANGELOG entry.
- Never `cargo publish` a version that does not match the git tag / CHANGELOG.
- crates.io versions are immutable — yank only for severe breakage, then ship
  a patch.

## Related

- [CHANGELOG.md](../../CHANGELOG.md)
- [docs/slsa.md](../slsa.md) — attestation / provenance
- [README.md](../../README.md) — install honesty until crates.io lands
- [npm/PUBLISHING.md](../../npm/PUBLISHING.md) — npm / GitHub Packages
