# Charter — phenotype-journeys

> **Boundary class:** sdk-domain  
> **Role:** test  
> **Lifecycle:** active  
> **Genesis template:** HexaKit `templates/genesis/` v1.0.0

## Mission

E2E journey harness and Rust CLI for the test role.

## Scope

### In scope

- Journey definitions, fixtures, CLI runner
- Integration with phenotype-python-sdk testing packages

### Out of scope

| Boundary | Owner repo |
|----------|------------|
| Unit test libraries | `phenotype-python-sdk testing-kit` |
| Static analysis | `KodeVibe` |

## Governance artifacts

| Artifact | Path |
|----------|------|
| Intent | [intent.md](intent.md) |
| Review | [review.md](review.md) |
| SOTA | [SOTA.md](SOTA.md) |
| OKF | [okf/manifest.okf.yaml](okf/manifest.okf.yaml) |

Authority: [phenotype-registry DOMAIN_ROLES](https://github.com/KooshaPari/phenotype-registry/blob/main/DOMAIN_ROLES.md)

## Decision rights

| Action | Authority |
|--------|-----------|
| Merge to `main` | KooshaPari + 1 reviewer |

## Changelog

| Date | Change |
|------|--------|
| 2026-06-17 | Genesis rollout Wave 5 |
