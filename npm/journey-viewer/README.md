# @phenotype/journey-viewer

Vue 3 components for rendering Phenotype journey manifests inside VitePress docs.

## Install

**Not on public npmjs.com.** Prefer a path or git dependency until GitHub Packages
publish exists (see [`../PUBLISHING.md`](../PUBLISHING.md)):

```bash
# path (monorepo / sibling clone)
bun add ../phenotype-journeys/npm/journey-viewer

# or git
bun add github:KooshaPari/phenotype-journeys#main&path=npm/journey-viewer
```

Do not run `bun add @phenotype/journey-viewer` expecting the public registry.

## VitePress integration

Register the components in `.vitepress/theme/index.ts`:

```ts
import DefaultTheme from "vitepress/theme";
import { JourneyViewer, RecordingEmbed } from "@phenotype/journey-viewer";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("JourneyViewer", JourneyViewer);
    app.component("RecordingEmbed", RecordingEmbed);
  }
};
```

Then in any `.md` page:

```md
<RecordingEmbed tape="first-plan" base-url="/journeys" />
<JourneyViewer manifest="/journeys/manifests/first-plan/manifest.verified.json" />
```

Artefact layout expected under `base-url`:

```
<baseUrl>/recordings/<tape>.mp4
<baseUrl>/recordings/<tape>.gif
<baseUrl>/keyframes/<tape>/frame-001.png
<baseUrl>/manifests/<tape>/manifest.verified.json
```

Produce that layout with `phenotype-journey sync --from <artefact-dir> --to <docs-public-dir>`.
