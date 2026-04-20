# @phenotype/journey-playwright

Playwright helper that scripts a web page interaction and emits a Phenotype-conformant journey manifest ready for `phenotype-journey verify`.

## Install

```bash
bun add -d @phenotype/journey-playwright @playwright/test playwright
```

## Use

```ts
import { record } from "@phenotype/journey-playwright";
import { chromium } from "playwright";

const browser = await chromium.launch();
const page = await browser.newPage();

await record({
  id: "checkout-flow",
  intent: "Complete a checkout from empty cart",
  outDir: "./journeys",
  page,
  steps: [
    { intent: "Load landing", action: (p) => p.goto("https://example.com") },
    { intent: "Add item", action: (p) => p.getByRole("button", { name: "Add" }).click() },
    { intent: "Confirm",   action: (p) => p.getByRole("button", { name: "Checkout" }).click() }
  ]
});

await browser.close();
```

Then:

```bash
phenotype-journey verify ./journeys/manifests/checkout-flow/manifest.json
```
