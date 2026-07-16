# Journey Traceability (operations annex)

Implements the [phenotype-infra journey-traceability standard](https://github.com/kooshapari/phenotype-infra/blob/main/docs/governance/journey-traceability-standard.md) for `phenotype-journeys`. The canonical project doc remains `docs/journey-traceability.md`; this annex extends it with concrete FR/NFR anchors and autograder gates for downstream consumers.

## Traceability Model

Every consumer-facing journey produced with the `phenotype-journeys` harness should be traceable across:

1. **FR/NFR** ΓÇö requirement ID from `docs/FUNCTIONAL_REQUIREMENTS.md`.
2. **Spec** ΓÇö manifest schema, validation contract, and non-regression constraints.
3. **Docs** ΓÇö operator/user documentation and rich media placeholders.
4. **Code** ΓÇö CLI, HTTP API, UI components (`ShotGallery`, `RecordingEmbed`, `JourneyViewer`), schema validator, OCR assertion, or recording harness.
5. **Tests/Gates** ΓÇö unit, integration, BDD, lint, and journey verification acting as autograders.
6. **Evidence** ΓÇö journey manifest, recording/keyframes, and evaluation verdict.

## Consumer-Facing and Operator-Facing Flows

| Flow | Requirement | Implementation surface | Autograder gates | Evidence status |
| --- | --- | --- | --- | --- |
| Consumer runs `phenotype-journey` CLI for record/verify/assert/sync | FR-PHENOTYPE_JOURNEYS-001, NFR-PHENOTYPE_JOURNEYS-USABILITY-001 | CLI command dispatch, subcommand handlers, flag parsing | CLI fixture tests, subcommand snapshot tests, BDD journey, eval verdict | Stubbed |
| Harness exposes HTTP/REST endpoints for journey automation | FR-PHENOTYPE_JOURNEYS-002, NFR-PHENOTYPE_JOURNEYS-OBSERVABILITY-001 | HTTP API routes, request/response schema, error mapper | API contract tests, schema validation, BDD journey, journey manifest | Stubbed |
| Authn/authz enforce least-privilege access to journey actions | FR-PHENOTYPE_JOURNEYS-003, NFR-PHENOTYPE_JOURNEYS-SECURITY-001 | auth handler, scope checks, audit log | auth contract tests, negative auth fixtures, journey eval | Stubbed |
| Docs surface renders `ShotGallery` and `RecordingEmbed` evidence | FR-PHENOTYPE_JOURNEYS-007, NFR-PHENOTYPE_JOURNEYS-UX-001 | UI components, doc embed layout, accessibility | component snapshot tests, a11y checks, screenshot journey | Stubbed |
| Manifest schema rejects invalid or drifted journey bundles | FR-PHENOTYPE_JOURNEYS-008, NFR-PHENOTYPE_JOURNEYS-CONTRACT-001 | schema validator, manifest loader, error reporting | schema negative fixtures, drift tests, eval verdict | Stubbed |
| Operator monitors harness health and journey evidence metrics | FR-PHENOTYPE_JOURNEYS-010, NFR-PHENOTYPE_JOURNEYS-OBSERVABILITY-001 | health/metric endpoint, observability hooks, dashboards | health contract tests, metric/log assertions, journey eval | Stubbed |

## Rich Media Stubs

<!-- RICH-MEDIA-STUB type="animated-gif" subject="phenotype-journey CLI record/verify/assert/sync" journey="phenotype-journey-cli-commands" status="TODO" -->
![phenotype-journey CLI commands ΓÇö record, verify, assert, sync output, and exit code evidence](../assets/rich-media/phenotype-journeys/phenotype-journey-cli-commands.gif)

*Expected capture: run record/verify/assert/sync against a fixture manifest, show output, exit code, and links to the resulting evidence bundle.*

<!-- RICH-MEDIA-STUB type="annotated-screenshot" subject="Manifest schema validation evidence" journey="manifest-schema-validation" status="TODO" -->
![phenotype-journey manifest schema ΓÇö valid bundle, invalid bundle, error report, and CI verdict](../assets/rich-media/phenotype-journeys/manifest-schema-validation.png)

*Expected capture: run the schema validator against a valid manifest and a deliberately invalid one, annotate error report fields, and link the CI verdict.*

<!-- RICH-MEDIA-STUB type="journey-eval" subject="ShotGallery and RecordingEmbed rendering verdict" journey="shot-gallery-recording-embed" status="TODO" -->
![phenotype-journey embed rendering ΓÇö ShotGallery, RecordingEmbed, accessibility state, and eval verdict](../assets/rich-media/phenotype-journeys/shot-gallery-recording-embed.png)

*Expected capture: render a docs page with `ShotGallery` and `RecordingEmbed`, annotate accessibility state, and attach a pass/fail verdict for FR-PHENOTYPE_JOURNEYS-007.*

<!-- RICH-MEDIA-STUB type="journey-eval" subject="Authn/authz least-privilege verdict" journey="authn-authz-least-privilege" status="TODO" -->
![phenotype-journey auth ΓÇö denied access, scoped access, audit log, and eval verdict](../assets/rich-media/phenotype-journeys/authn-authz-least-privilege.png)

*Expected capture: run negative auth fixtures (denied scope, missing token), run a scoped happy path, annotate audit log entries, and attach a pass/fail security verdict.*

## Journey Manifests

Journey manifests should live in `docs/journeys/manifests/` and include:

- FR/NFR IDs covered by the journey;
- CLI command, HTTP endpoint, or UI component used to reproduce the flow;
- fixture manifest, recording, or schema artifact required for replay;
- expected screenshots/GIFs/keyframes;
- tests and gates that must pass before the journey is accepted;
- eval verdict schema and pass/fail criteria.

## Autograder Gates

Minimum gates before marking a journey complete:

- CLI subcommand fixture tests for record/verify/assert/sync;
- HTTP API contract tests with schema validation;
- auth negative and scoped-positive fixture tests;
- UI component snapshot and a11y tests for `ShotGallery` and `RecordingEmbed`;
- schema negative and drift tests for manifest validation;
- health and observability assertions for harness monitoring;
- doc link validation for every referenced rich media asset;
- journey manifest validation via `phenotype-journey validate`;
- eval verdict linked to the FR/NFR IDs in the manifest.

## Status

- [x] Identify initial CLI/API/auth/UI/schema/observability flows from `docs/FUNCTIONAL_REQUIREMENTS.md`
- [x] Stub rich media embeds for expected screenshots/GIFs/evals
- [ ] Author manifests in `docs/journeys/manifests/`
- [ ] Record journey captures for each flow
- [ ] Run `phenotype-journey validate` in CI
