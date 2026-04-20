use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use phenotype_journey_core::{
    manifest_schema, validate_manifest, verify_manifest, VerifyMode,
};
use std::path::PathBuf;

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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Cmd::Record { tape, out } => cmd_record(tape, out),
        Cmd::Verify { manifest, live } => cmd_verify(manifest, live),
        Cmd::Validate { manifest } => cmd_validate(manifest),
        Cmd::Sync { from, to } => cmd_sync(from, to),
        Cmd::Schema => cmd_schema(),
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
