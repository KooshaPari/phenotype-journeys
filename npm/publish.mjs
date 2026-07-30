#!/usr/bin/env node
/**
 * Operator helper for @phenotype/* packages under npm/.
 *
 * Usage:
 *   node npm/publish.mjs              # validate + pack dry-run (default)
 *   node npm/publish.mjs --dry-run    # same as default
 *   node npm/publish.mjs --pack       # write tarballs to npm/dist/
 *   node npm/publish.mjs --publish    # npm publish to GitHub Packages (requires token)
 *
 * Auth (publish only): set NODE_AUTH_TOKEN or GITHUB_TOKEN with write:packages.
 * Creates a temp userconfig so the machine ~/.npmrc is not mutated.
 */
import { spawnSync } from "node:child_process";
import {
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = __dirname;
const PACKAGES = [
  "journey-viewer",
  "journey-playwright",
  "playwright-record",
];
const REGISTRY = "https://npm.pkg.github.com";
const EXPECTED_VERSION = "0.1.0";

const args = new Set(process.argv.slice(2));
const doPublish = args.has("--publish");
const doPack = args.has("--pack");
// default and --dry-run: validate + npm pack --dry-run (no registry write)

function fail(msg) {
  console.error(`error: ${msg}`);
  process.exit(1);
}

function run(cmd, cmdArgs, opts = {}) {
  const r = spawnSync(cmd, cmdArgs, {
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
    ...opts,
  });
  if (r.status !== 0) {
    const err = (r.stderr || r.stdout || "").trim();
    fail(`${cmd} ${cmdArgs.join(" ")} failed:\n${err}`);
  }
  return (r.stdout || "").trim();
}

function validatePackage(dir) {
  const pkgPath = join(dir, "package.json");
  const pkg = JSON.parse(readFileSync(pkgPath, "utf8"));
  const name = pkg.name;
  if (!name?.startsWith("@phenotype/")) {
    fail(`${pkgPath}: name must be @phenotype/* (got ${name})`);
  }
  if (pkg.version !== EXPECTED_VERSION) {
    fail(
      `${pkgPath}: version must be ${EXPECTED_VERSION} for this cut (got ${pkg.version})`,
    );
  }
  if (pkg.publishConfig?.registry !== REGISTRY) {
    fail(`${pkgPath}: publishConfig.registry must be ${REGISTRY}`);
  }
  if (pkg.publishConfig?.access !== "restricted") {
    fail(`${pkgPath}: publishConfig.access must be "restricted"`);
  }
  if (!Array.isArray(pkg.files) || pkg.files.length === 0) {
    fail(`${pkgPath}: files[] must list published paths`);
  }
  if (!pkg.repository?.directory) {
    fail(`${pkgPath}: repository.directory is required`);
  }
  return pkg;
}

function writeUserconfig(token) {
  const dir = mkdtempSync(join(tmpdir(), "phenotype-npm-"));
  const path = join(dir, ".npmrc");
  writeFileSync(
    path,
    [
      `@phenotype:registry=${REGISTRY}`,
      `//npm.pkg.github.com/:_authToken=${token}`,
      "",
    ].join("\n"),
  );
  return { dir, path };
}

function main() {
  console.log(
    doPublish
      ? "mode: publish → GitHub Packages"
      : doPack
        ? "mode: pack → npm/dist/"
        : "mode: validate + pack dry-run",
  );

  const pkgs = [];
  for (const id of PACKAGES) {
    const dir = join(ROOT, id);
    const pkg = validatePackage(dir);
    pkgs.push({ id, dir, pkg });
    console.log(`ok  ${pkg.name}@${pkg.version}`);
  }

  if (doPack) {
    const dist = join(ROOT, "dist");
    mkdirSync(dist, { recursive: true });
    for (const { dir, pkg } of pkgs) {
      run("npm", ["pack", `--pack-destination=${dist}`], { cwd: dir });
      console.log(`packed ${pkg.name} → npm/dist/`);
    }
    return;
  }

  for (const { dir, pkg } of pkgs) {
    const out = run("npm", ["pack", "--dry-run"], { cwd: dir });
    console.log(`dry-run pack ${pkg.name}\n${out}\n`);
  }

  if (!doPublish) {
    console.log(
      "Done. To publish: export NODE_AUTH_TOKEN=<PAT with write:packages> && node npm/publish.mjs --publish",
    );
    return;
  }

  const token = process.env.NODE_AUTH_TOKEN || process.env.GITHUB_TOKEN;
  if (!token) {
    fail(
      "NODE_AUTH_TOKEN or GITHUB_TOKEN required for --publish (needs write:packages)",
    );
  }

  const { dir: cfgDir, path: userconfig } = writeUserconfig(token);
  try {
    for (const { dir, pkg } of pkgs) {
      console.log(`publishing ${pkg.name}@${pkg.version} …`);
      run(
        "npm",
        ["publish", "--access", "restricted", `--userconfig=${userconfig}`],
        { cwd: dir, env: { ...process.env, NODE_AUTH_TOKEN: token } },
      );
      console.log(`published ${pkg.name}@${pkg.version}`);
    }
  } finally {
    rmSync(cfgDir, { recursive: true, force: true });
  }
}

main();
