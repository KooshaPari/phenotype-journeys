# @phenotype/playwright-record

Playwright test wrapper that turns **Playwright specs into verified user-story
manifests**. Part of the Phenotype *user-story-as-test* framework (Batch 3,
ADR 0034).

Each spec declares its user story as a YAML frontmatter block inside a JSDoc
comment; the `recorder` fixture parses the block, captures screenshots +
ARIA snapshots on each `recorder.capture()` call, and emits a
`manifest.verified.json` at `target/user-stories/<journey_id>.manifest.json`
(parallel to the Rust proc-macro output from Batch 2).

## Install

```bash
bun add -d @phenotype/playwright-record @playwright/test
```

## Example 1 — Minimal journey

```ts
/**
 * @user-story
 * ---
 * journey_id: streamlit-hf-search
 * title: Streamlit — HuggingFace model search journey
 * persona: ML engineer picking a model for local inference
 * given: Streamlit server running, HF token cached
 * when:
 *   - navigate to /HF_Search
 *   - type "llama 3" in the query box
 *   - click first result card
 * then:
 *   - url contains /Planner
 *   - resolved-model chip shows "meta-llama/Llama-3-*"
 * traces_to: [FR-HF-001, FR-UI-001]
 * record: true
 * blind_judge: auto
 * family: streamlit
 * ---
 */
import { test, expect } from '@phenotype/playwright-record';

test('hf-search to planner handoff', async ({ page, recorder }) => {
  await page.goto('/HF_Search');
  await recorder.capture(page, 'landing', 'HF Search landing page rendered');

  await page.getByPlaceholder(/llama/i).fill('llama 3');
  await recorder.capture(page, 'typed', 'Query typed');

  await page.getByRole('button', { name: /Plan it/i }).first().click();
  await expect(page).toHaveURL(/\/Planner/);
  await recorder.capture(page, 'handoff', 'Planner opened');
});
```

Run with `bunx playwright test` — the fixture reads frontmatter from
`testInfo.file`, runs the test body, and writes the manifest on teardown.

## Example 2 — Custom output root

```ts
import { test as base } from '@phenotype/playwright-record';

export const test = base.extend({
  recorderOptions: [
    { outputRoot: './artefacts' },
    { option: true, scope: 'test' },
  ],
});
```

## Example 3 — Disabling capture per-story

Set `record: false` in the frontmatter to turn a spec into a pure
functional test — the manifest is still emitted (with 0 keyframes) so
traceability remains intact.

## Output layout

```
target/user-stories/
  <journey_id>.manifest.json         # primary manifest
  <journey_id>/
    manifest.verified.json           # co-located copy
    keyframes/frame-NNN.png
    aria/frame-NNN.aria.txt
```

## Manifest shape

See `schema/manifest.schema.json` in `phenotype-journeys`. Batch 3 extends
the base schema with frontmatter-sourced fields (`persona`, `given`, `when`,
`then`, `traces_to`, `family`, `blind_judge`) and a `verification` block
identifying the generator.

## Tests

```bash
bun install
bun run test
```
