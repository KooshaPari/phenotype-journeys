# Publishing `@phenotype/*` npm packages

These packages publish to the **GitHub Packages** npm registry
(`https://npm.pkg.github.com`) — not the public `npmjs.com` registry.

## Automated publishing (preferred)

`.github/workflows/publish-npm.yml` publishes `@phenotype/journey-viewer` and
`@phenotype/playwright-record` to GitHub Packages on every GitHub Release
(`release: published`) and via `workflow_dispatch`. It runs on
`ubuntu-latest` only (macOS/Windows runners are billed) and authenticates
with the default `${{ secrets.GITHUB_TOKEN }}` — no extra secrets required.

To cut a release:

1. Bump `version` in `npm/journey-viewer/package.json` and/or
   `npm/playwright-record/package.json`.
2. Tag and push (`git tag v0.1.x && git push --tags`).
3. Create a GitHub Release on that tag — the workflow publishes both packages.

To publish a single package manually, use **Actions → Publish npm packages
to GitHub Packages → Run workflow** and pick the package from the dropdown.

## Prerequisites

A GitHub personal access token (classic) with the following scopes:

- `write:packages` — publish
- `read:packages` — pull on consumer side
- `repo` — link to the source repo

Export as `GITHUB_TOKEN` (or set in a local `.npmrc`).

## Publish

```bash
# authenticate to GitHub Packages
cat > ~/.npmrc-ghp <<EOF
//npm.pkg.github.com/:_authToken=${GITHUB_TOKEN}
@phenotype:registry=https://npm.pkg.github.com
EOF

# publish journey-viewer
cd npm/journey-viewer
npm --userconfig=$HOME/.npmrc-ghp publish --access restricted

# publish journey-playwright
cd ../journey-playwright
npm --userconfig=$HOME/.npmrc-ghp publish --access restricted
```

## Fallback: tarball

If you cannot publish to the registry (token missing scopes, etc.), run:

```bash
cd npm/journey-viewer && npm pack --pack-destination ../dist
cd ../journey-playwright && npm pack --pack-destination ../dist
```

Consumers can then depend on the tarball directly:

```json
{
  "dependencies": {
    "@phenotype/journey-viewer": "file:../vendor/phenotype-journeys/phenotype-journey-viewer-0.1.0.tgz"
  }
}
```

Current packed tarballs live in `npm/dist/`.

## Consuming (GitHub Packages)

Create `.npmrc` in the consumer repo:

```
@phenotype:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=${GITHUB_TOKEN}
```

Then:

```bash
bun add @phenotype/journey-viewer
```
