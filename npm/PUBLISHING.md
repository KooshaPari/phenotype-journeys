# Publishing `@phenotype/*` npm packages

These packages publish to **GitHub Packages** (`https://npm.pkg.github.com`),
**not** the public `npmjs.com` registry.

| Package | Path | Version |
|---|---|---|
| `@phenotype/journey-viewer` | `npm/journey-viewer` | `0.1.0` |
| `@phenotype/journey-playwright` | `npm/journey-playwright` | `0.1.0` |
| `@phenotype/playwright-record` | `npm/playwright-record` | `0.1.0` |

**Current honesty:** none of these names exist on `npmjs.com`. Until a
successful GitHub Packages publish lands, consumers must use **path** or
**git** dependencies (see the root README Install section and each package
README). Do not assume `bun add @phenotype/...` resolves without a scoped
`.npmrc` pointing at GitHub Packages **and** a published version.

Out of scope for this cut: `@phenotype/doc-embeds` under `remotion/doc-embeds/`
(separate Remotion pipeline; same registry when published later).

## Prerequisites

A GitHub personal access token (classic) with:

| Scope | Why |
|---|---|
| `write:packages` | publish |
| `read:packages` | install on consumer side |
| `repo` | link package to this private/public source repo |

Fine-grained PATs need Packages write on `KooshaPari/phenotype-journeys`.

Export the token (either name works for the helper script):

```bash
export NODE_AUTH_TOKEN=ghp_...   # preferred for npm
# or
export GITHUB_TOKEN=ghp_...
```

The `gh` CLI token often **lacks** `write:packages` — verify scopes before
publishing (`gh auth status`). Do not publish with an underscoped token.

## Validate / dry-run (no registry write)

From the repo root (Node 18+; uses `npm` CLI):

```bash
node npm/publish.mjs
# equivalent:
node npm/publish.mjs --dry-run
```

Or per package:

```bash
cd npm/journey-viewer && npm run pack:check
cd ../journey-playwright && npm run pack:check
cd ../playwright-record && npm run pack:check
```

Task runner:

```bash
task npm:pack
```

## Publish to GitHub Packages

### Option A — helper script (recommended)

```bash
export NODE_AUTH_TOKEN=ghp_...   # write:packages + repo
node npm/publish.mjs --publish
```

### Option B — npm CLI (exact commands)

```bash
# one-time auth file (do not commit)
cat > ~/.npmrc-ghp <<'EOF'
@phenotype:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=${NODE_AUTH_TOKEN}
EOF
# expand token into the file if your shell does not expand inside single-quoted heredoc:
# printf '@phenotype:registry=https://npm.pkg.github.com\n//npm.pkg.github.com/:_authToken=%s\n' "$NODE_AUTH_TOKEN" > ~/.npmrc-ghp

cd npm/journey-viewer
npm --userconfig="$HOME/.npmrc-ghp" publish --access restricted

cd ../journey-playwright
npm --userconfig="$HOME/.npmrc-ghp" publish --access restricted

cd ../playwright-record
npm --userconfig="$HOME/.npmrc-ghp" publish --access restricted
```

### Option C — bun (publish still uses npm registry protocol)

```bash
# bun respects .npmrc; use the same ~/.npmrc-ghp or project .npmrc
cd npm/journey-viewer && bun publish --access restricted --registry https://npm.pkg.github.com
cd ../journey-playwright && bun publish --access restricted --registry https://npm.pkg.github.com
cd ../playwright-record && bun publish --access restricted --registry https://npm.pkg.github.com
```

Prefer **Option A or B** until bun’s GitHub Packages auth is verified in this
environment.

### Option D — GitHub Actions

Manual workflow: **Publish npm (GitHub Packages)**
(`.github/workflows/publish-npm.yml`), `workflow_dispatch` with
`dry_run=true` by default. Set `dry_run=false` only when ready to write.
Uses `permissions: packages: write` and `GITHUB_TOKEN`.

## Fallback: local tarball

If the registry is unreachable (billing, missing scopes, etc.):

```bash
node npm/publish.mjs --pack
# writes npm/dist/phenotype-*-0.1.0.tgz
```

Consumer `package.json`:

```json
{
  "dependencies": {
    "@phenotype/journey-viewer": "file:../vendor/phenotype-journeys/phenotype-journey-viewer-0.1.0.tgz"
  }
}
```

`npm/dist/` is gitignored; regenerate with `--pack` as needed.

## Consuming (after a successful publish)

Consumer repo `.npmrc`:

```
@phenotype:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=${NODE_AUTH_TOKEN}
```

Then:

```bash
bun add @phenotype/journey-viewer@0.1.0
bun add -d @phenotype/journey-playwright@0.1.0 @phenotype/playwright-record@0.1.0
# or
npm install @phenotype/journey-viewer@0.1.0 --registry=https://npm.pkg.github.com
```

Until publish succeeds, keep using path/git installs from the root README.

## Checklist before first publish

- [ ] `node npm/publish.mjs` exits 0 (version `0.1.0`, registry, `files`, `repository.directory`)
- [ ] Token has `write:packages` (not only `repo` / `workflow`)
- [ ] No conflicting version already on GitHub Packages for each name
- [ ] Prefer `dry_run` workflow run once before `dry_run=false`
