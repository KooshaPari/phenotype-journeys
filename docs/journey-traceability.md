---
title: Journey Traceability
---

# Journey Traceability

`phenotype-journeys` is the shared harness for capturing, validating, and
publishing journey evidence across Phenotype projects.

This repo owns the reusable mechanics:

- manifest schema validation
- record / verify / assert / sync flows
- `ShotGallery` and `RecordingEmbed` UI components for docs surfaces
- OCR-backed assertions for ground-truth keyframes

## Canonical standard

The org-wide traceability contract lives in:

- [phenotype-infra journey-traceability standard](https://github.com/kooshapari/phenotype-infra/blob/main/docs/governance/journey-traceability-standard.md)

## Repo expectation

Any project that uses this harness should keep:

- a stable manifest under `journeys/manifests/<spec-id>/`
- keyframes for important states
- a replayable recording for the full flow
- a docs page or gallery that links the evidence bundle

## Consumer rule

Consumers should fail their quality gate when a user-facing spec is missing a
passing journey manifest. The `phenotype-journey validate` command is the
canonical check.
