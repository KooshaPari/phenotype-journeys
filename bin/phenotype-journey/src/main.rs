use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use phenotype_journey_core::{
    assertions::{run_on_manifest, AssertionReport, ViolationKind},
    manifest_schema, validate_manifest, verify_manifest, Manifest, Step, StepAssertions,
    VerifyMode,
};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "phenotype-journey", version, about = "Phenotype journey harness")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Record a VHS tape and emit a journey manifest stub.
    Record {
        #[arg(long)]
        tape: PathBuf,
        /// Output directory for recording + keyframes.
        #[arg(long, default_value = "./out")]
        out: PathBuf,
    },
    /// Run Claude describe+judge verification on an existing manifest.
    Verify {
        manifest: PathBuf,
        /// Force live Anthropic API (requires ANTHROPIC_API_KEY and `live` feature at build time).
        #[arg(long)]
        live: bool,
    },
    /// Validate a manifest against the canonical JSONSchema.
    Validate { manifest: PathBuf },
    /// Sync journey artefacts from a recording dir into a docs public dir.
    Sync {
        #[arg(long)]
        from: PathBuf,
        #[arg(long)]
        to: PathBuf,
    },
    /// Emit the canonical JSONSchema for manifests.
    Schema,
    /// Run ground-truth assertions against keyframes (OCR + sentinel checks).
    ///
    /// Reads `<tape>.intents.yaml` (sibling to the manifest's tape) to pull
    /// per-step assertions, OCR's the matching keyframes, and prints a
    /// per-step report. With `--strict`, exits non-zero on any violation.
    Assert {
        manifest: PathBuf,
        /// Artefacts root containing `keyframes/<journey>/frame-*.png`.
        /// Defaults to the manifest's parent-of-parent (i.e. `apps/cli-journeys/`).
        #[arg(long)]
        artefacts: Option<PathBuf>,
        /// Optional explicit path to the intents YAML.
        /// Default: `<artefacts>/tapes/<journey>.intents.yaml`.
        #[arg(long)]
        intents: Option<PathBuf>,
        /// Exit non-zero on any violation.
        #[arg(long)]
        strict: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Cmd::Record { tape, out } => cmd_record(tape, out),
        Cmd::Verify { manifest, live } => cmd_verify(manifest, live),
        Cmd::Validate { manifest } => cmd_validate(manifest),
        Cmd::Sync { from, to } => cmd_sync(from, to),
        Cmd::Schema => cmd_schema(),
        Cmd::Assert {
            manifest,
            artefacts,
            intents,
            strict,
        } => cmd_assert(manifest, artefacts, intents, strict),
    }
}

fn cmd_record(tape: PathBuf, out: PathBuf) -> Result<()> {
    std::fs::create_dir_all(&out).with_context(|| format!("create {}", out.display()))?;
    let status = std::process::Command::new("vhs")
        .arg(&tape)
        .arg("--output")
        .arg(out.join(
            tape.file_stem()
                .map(|s| s.to_string_lossy().to_string() + ".mp4")
                .unwrap_or_else(|| "out.mp4".into()),
        ))
        .status()
        .with_context(|| "failed to invoke `vhs` — install charmbracelet/vhs")?;
    anyhow::ensure!(status.success(), "vhs exited non-zero");
    println!("recorded {} -> {}", tape.display(), out.display());
    Ok(())
}

fn cmd_verify(path: PathBuf, live: bool) -> Result<()> {
    let mode = if live { VerifyMode::Live } else { VerifyMode::Mock };
    let v = verify_manifest(&path, mode).with_context(|| format!("verify {}", path.display()))?;
    println!("{}", serde_json::to_string_pretty(&v)?);
    Ok(())
}

fn cmd_validate(path: PathBuf) -> Result<()> {
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("read {}", path.display()))?;
    let value: serde_json::Value = serde_json::from_str(&raw)?;
    validate_manifest(&value)?;
    println!("ok: {}", path.display());
    Ok(())
}

fn cmd_sync(from: PathBuf, to: PathBuf) -> Result<()> {
    anyhow::ensure!(from.is_dir(), "--from must be a directory: {}", from.display());
    std::fs::create_dir_all(&to)?;
    let mut count = 0usize;
    for entry in walk(&from)? {
        let rel = entry.strip_prefix(&from).unwrap();
        let dst = to.join(rel);
        if entry.is_dir() {
            std::fs::create_dir_all(&dst)?;
        } else {
            if let Some(p) = dst.parent() {
                std::fs::create_dir_all(p)?;
            }
            std::fs::copy(&entry, &dst)?;
            count += 1;
        }
    }
    println!("synced {} files -> {}", count, to.display());
    Ok(())
}

fn cmd_schema() -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&manifest_schema())?);
    Ok(())
}

fn cmd_assert(
    manifest_path: PathBuf,
    artefacts: Option<PathBuf>,
    intents: Option<PathBuf>,
    strict: bool,
) -> Result<()> {
    let raw = std::fs::read_to_string(&manifest_path)
        .with_context(|| format!("read {}", manifest_path.display()))?;
    let mut manifest: Manifest = serde_json::from_str(&raw)?;

    // Artefacts root defaults to two levels up from manifest.json
    // (manifests/<id>/manifest.json -> <root>).
    let artefacts_root = match artefacts {
        Some(p) => p,
        None => manifest_path
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf())
            .ok_or_else(|| anyhow::anyhow!("could not derive artefacts root; pass --artefacts"))?,
    };

    // Intents YAML defaults to <artefacts>/tapes/<journey>.intents.yaml
    let intents_path = intents.unwrap_or_else(|| {
        artefacts_root
            .join("tapes")
            .join(format!("{}.intents.yaml", manifest.id))
    });

    if intents_path.exists() {
        overlay_assertions(&mut manifest, &intents_path)
            .with_context(|| format!("overlay {}", intents_path.display()))?;
    } else {
        eprintln!(
            "warn: no intents YAML at {} — continuing with manifest-embedded assertions only",
            intents_path.display()
        );
    }

    let report = run_on_manifest(&manifest, &artefacts_root)
        .with_context(|| format!("assert {}", manifest.id))?;

    print_report(&report);

    if report.no_assertions {
        eprintln!(
            "warn: journey '{}' has zero steps with assertions — consider adding must_contain / must_not_contain",
            report.journey_id
        );
    }

    if strict && !report.passed {
        anyhow::bail!(
            "{} assertion violation(s) in journey '{}'",
            report.violations.len(),
            report.journey_id
        );
    }
    Ok(())
}

fn print_report(report: &AssertionReport) {
    println!("journey: {}", report.journey_id);
    if report.violations.is_empty() {
        println!("  passed: 0 violations");
    } else {
        println!("  FAILED: {} violation(s)", report.violations.len());
        for v in &report.violations {
            let kind = match v.kind {
                ViolationKind::MustContain => "must_contain",
                ViolationKind::MustNotContain => "must_not_contain",
                ViolationKind::ExitCode => "exit_code",
            };
            println!(
                "    step {}: {} expected={:?} got=\"{}\"",
                v.step_index, kind, v.expected, v.got_snippet
            );
        }
    }
}

#[derive(Debug, Deserialize)]
struct IntentsFile {
    #[allow(dead_code)]
    #[serde(default)]
    journey: Option<String>,
    #[serde(default)]
    steps: Vec<IntentStep>,
}

#[derive(Debug, Deserialize)]
struct IntentStep {
    index: u32,
    #[serde(default)]
    #[allow(dead_code)]
    intent: Option<String>,
    #[serde(default)]
    assertions: Option<StepAssertions>,
}

/// Overlay per-step `assertions` from the intents YAML onto the manifest's
/// in-memory Steps. Non-destructive: manifest-embedded assertions win when
/// both define the same step.
fn overlay_assertions(manifest: &mut Manifest, intents_path: &Path) -> Result<()> {
    let raw = std::fs::read_to_string(intents_path)?;
    let parsed: IntentsFile = serde_yaml::from_str(&raw)
        .with_context(|| format!("parse yaml {}", intents_path.display()))?;
    for intent in parsed.steps {
        if let Some(a) = intent.assertions {
            if let Some(step) = find_step_mut(&mut manifest.steps, intent.index) {
                if step.assertions.is_none() {
                    step.assertions = Some(a);
                }
            }
        }
    }
    Ok(())
}

fn find_step_mut(steps: &mut [Step], index: u32) -> Option<&mut Step> {
    steps.iter_mut().find(|s| s.index == index)
}

fn walk(root: &std::path::Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_dir() {
                stack.push(p.clone());
            }
            out.push(p);
        }
    }
    Ok(out)
}
