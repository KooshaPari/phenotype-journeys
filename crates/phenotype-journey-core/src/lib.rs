//! # phenotype-journey-core
//!
//! Shared types and verification logic for Phenotype journey manifests.
//!
//! A *journey* is a recorded (CLI tape, UI test, or Playwright trace) sequence
//! of intents that prove a user-facing capability works. Manifests describe
//! the recording plus per-step intent/screenshot pairs, and are optionally
//! verified by a Claude-describe + Claude-judge loop.

pub mod assertions;
pub mod verify;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Optional ground-truth assertions for a step.
///
/// When populated, the `phenotype-journey assert` subcommand OCRs the matching
/// keyframe and applies these constraints as hard gates. Tapes without
/// assertions still record and verify as before, but the verifier emits a
/// warning per journey that carries zero assertions.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct StepAssertions {
    /// Substrings that MUST appear in the OCR'd frame text.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub must_contain: Vec<String>,
    /// Substrings that MUST NOT appear in the OCR'd frame text.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub must_not_contain: Vec<String>,
    /// If set, the LAST keyframe of the journey must contain `__EXIT_<N>__`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_exit: Option<i32>,
    /// If true, OCR must succeed for this step; defaults to true whenever any
    /// contain/not_contain assertion is set.
    #[serde(default)]
    pub ocr_required: bool,
}

impl StepAssertions {
    pub fn is_empty(&self) -> bool {
        self.must_contain.is_empty()
            && self.must_not_contain.is_empty()
            && self.expected_exit.is_none()
    }
}

/// A single step (keyframe) in a journey.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Step {
    pub index: u32,
    pub slug: String,
    pub intent: String,
    pub screenshot_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub judge_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assertions: Option<StepAssertions>,
}

/// Verification payload added by `phenotype-journey verify`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Verification {
    pub overall_score: f64,
    pub describe_confidence: f64,
    pub judge_confidence: f64,
    pub all_intents_passed: bool,
    /// Either `"mock"` or `"api"` (live Anthropic call).
    pub mode: String,
    /// RFC-3339 timestamp.
    pub timestamp: String,
    /// Ground-truth assertion violations (empty when none).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assertion_violations: Vec<assertions::Violation>,
}

/// Top-level manifest persisted as `manifest.json` / `manifest.verified.json`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Manifest {
    pub id: String,
    pub intent: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recording: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recording_gif: Option<String>,
    #[serde(default)]
    pub keyframe_count: u32,
    #[serde(default)]
    pub passed: bool,
    #[serde(default)]
    pub steps: Vec<Step>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<Verification>,
}

/// Mode for `verify_manifest`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyMode {
    /// Deterministic, offline: produces a canned high-confidence verification.
    Mock,
    /// Calls the live Anthropic API (requires the `live` feature + `ANTHROPIC_API_KEY`).
    Live,
}

#[derive(Debug, Error)]
pub enum JourneyError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("schema validation failed: {0}")]
    Schema(String),
    #[error("verify backend error: {0}")]
    Backend(String),
    #[error("not configured for live mode (enable `live` feature and set ANTHROPIC_API_KEY)")]
    LiveUnavailable,
    #[error("ocr backend error: {0}")]
    Ocr(String),
}

/// Produce the canonical JSONSchema for [`Manifest`].
pub fn manifest_schema() -> serde_json::Value {
    let schema = schemars::schema_for!(Manifest);
    serde_json::to_value(schema).expect("schema serialisation cannot fail")
}

/// Validate a manifest JSON blob against the canonical schema.
pub fn validate_manifest(value: &serde_json::Value) -> Result<(), JourneyError> {
    let schema = manifest_schema();
    let compiled = jsonschema::JSONSchema::compile(&schema)
        .map_err(|e| JourneyError::Schema(e.to_string()))?;
    if let Err(errors) = compiled.validate(value) {
        let msgs: Vec<String> = errors.map(|e| e.to_string()).collect();
        return Err(JourneyError::Schema(msgs.join("; ")));
    }
    Ok(())
}

/// Verify a manifest on disk and return the populated [`Verification`].
///
/// In [`VerifyMode::Mock`] the returned verification is deterministic and
/// does not require network access — this mirrors the behaviour of the
/// original `verify-manifests.sh` script when `ANTHROPIC_API_KEY` is unset.
pub fn verify_manifest(
    path: impl AsRef<std::path::Path>,
    mode: VerifyMode,
) -> Result<Verification, JourneyError> {
    let raw = std::fs::read_to_string(path.as_ref())?;
    let manifest: Manifest = serde_json::from_str(&raw)?;
    verify::run(&manifest, mode)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest() -> Manifest {
        Manifest {
            id: "first-plan".into(),
            intent: "Run your first plan".into(),
            recording: Some("recordings/first-plan.mp4".into()),
            recording_gif: Some("recordings/first-plan.gif".into()),
            keyframe_count: 2,
            passed: true,
            steps: vec![
                Step {
                    index: 0,
                    slug: "frame-0".into(),
                    intent: "Open CLI".into(),
                    screenshot_path: "frame-001.png".into(),
                    description: None,
                    judge_score: None,
                    assertions: None,
                },
                Step {
                    index: 1,
                    slug: "frame-1".into(),
                    intent: "See output".into(),
                    screenshot_path: "frame-002.png".into(),
                    description: None,
                    judge_score: None,
                    assertions: None,
                },
            ],
            verification: None,
        }
    }

    #[test]
    fn mock_mode_returns_all_passed() {
        let m = sample_manifest();
        let v = verify::run(&m, VerifyMode::Mock).expect("mock must succeed");
        assert_eq!(v.mode, "mock");
        assert!(v.all_intents_passed);
        assert!(v.overall_score > 0.0 && v.overall_score <= 1.0);
        assert!(!v.timestamp.is_empty());
    }

    #[test]
    fn schema_roundtrip_validates() {
        let m = sample_manifest();
        let value = serde_json::to_value(&m).unwrap();
        validate_manifest(&value).expect("sample manifest must validate");
    }

    #[test]
    fn assertions_roundtrip() {
        let a = StepAssertions {
            must_contain: vec!["hello".into()],
            must_not_contain: vec!["error:".into()],
            expected_exit: Some(0),
            ocr_required: true,
        };
        let s = serde_json::to_string(&a).unwrap();
        let back: StepAssertions = serde_json::from_str(&s).unwrap();
        assert_eq!(a, back);
    }
}
