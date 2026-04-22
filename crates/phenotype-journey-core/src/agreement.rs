//! Intent ↔ blind-description agreement scoring.
//!
//! Scores how well a human-authored `intent` overlaps a VLM-produced
//! `blind_description`. The score is a Jaccard-style overlap on stemmed,
//! stop-word-filtered tokens; three buckets are surfaced:
//!
//! * **Green** — overlap ≥ 0.6: intent and blind description agree.
//! * **Yellow** — 0.3 ≤ overlap < 0.6: partial agreement; worth reviewing.
//! * **Red** — overlap < 0.3: the blind judge saw something very different.
//!
//! The report also exposes the tokenised intent / blind sets plus the two
//! diff sets ("missing in blind", "extras in blind") so the viewer can
//! render a remediation hint ("re-record this step OR rewrite intent.yaml").

use rust_stemmers::{Algorithm, Stemmer};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Agreement bucket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Agreement {
    Green,
    Yellow,
    Red,
}

/// Full agreement report for a (intent, blind) pair.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct AgreementReport {
    pub status: Agreement,
    pub overlap: f64,
    pub intent_tokens: Vec<String>,
    pub blind_tokens: Vec<String>,
    pub missing_in_blind: Vec<String>,
    pub extras_in_blind: Vec<String>,
}

/// English stop-words pruned before stemming. Deliberately small — we keep the
/// CLI/UI vocabulary (plan, run, open, show, …) in the intent signal.
const STOPWORDS: &[&str] = &[
    "a", "an", "the", "and", "or", "but", "if", "then", "else", "of", "for",
    "to", "in", "on", "at", "by", "with", "from", "as", "is", "are", "was",
    "were", "be", "been", "being", "it", "its", "this", "that", "these",
    "those", "i", "you", "he", "she", "we", "they", "them", "his", "her",
    "their", "our", "your", "my", "me", "him", "us", "do", "does", "did",
    "have", "has", "had", "will", "would", "should", "could", "can", "may",
    "might", "must", "so", "than", "when", "while", "where", "who", "what",
    "which", "some", "any", "all", "no", "not", "out", "up", "down", "into",
    "over", "under", "again", "about", "after", "before", "just", "also",
    "only", "very", "too", "there", "here", "s", "t",
];

fn is_stopword(w: &str) -> bool {
    w.len() <= 1 || STOPWORDS.contains(&w)
}

/// Tokenise a caption into lowercase alphanumeric words, drop stopwords,
/// stem with the Porter2/Snowball English stemmer, de-duplicate.
pub fn tokenise(text: &str) -> Vec<String> {
    let stemmer = Stemmer::create(Algorithm::English);
    let mut out: BTreeSet<String> = BTreeSet::new();
    let mut current = String::new();
    for ch in text.chars() {
        if ch.is_alphanumeric() {
            for c in ch.to_lowercase() {
                current.push(c);
            }
        } else {
            if !current.is_empty() {
                push_token(&stemmer, &current, &mut out);
                current.clear();
            }
        }
    }
    if !current.is_empty() {
        push_token(&stemmer, &current, &mut out);
    }
    out.into_iter().collect()
}

fn push_token(stemmer: &Stemmer, word: &str, out: &mut BTreeSet<String>) {
    if is_stopword(word) {
        return;
    }
    let stem = stemmer.stem(word).to_string();
    if stem.is_empty() || is_stopword(&stem) {
        return;
    }
    out.insert(stem);
}

/// Score an (intent, blind) pair.
pub fn score(intent: &str, blind: &str) -> AgreementReport {
    let intent_tokens = tokenise(intent);
    let blind_tokens = tokenise(blind);

    let intent_set: BTreeSet<&String> = intent_tokens.iter().collect();
    let blind_set: BTreeSet<&String> = blind_tokens.iter().collect();

    let overlap = if intent_set.is_empty() && blind_set.is_empty() {
        // Both empty → vacuously green.
        1.0
    } else if intent_set.is_empty() || blind_set.is_empty() {
        0.0
    } else {
        let inter = intent_set.intersection(&blind_set).count() as f64;
        let union = intent_set.union(&blind_set).count() as f64;
        inter / union
    };

    let status = if overlap >= 0.6 {
        Agreement::Green
    } else if overlap >= 0.3 {
        Agreement::Yellow
    } else {
        Agreement::Red
    };

    let missing_in_blind: Vec<String> = intent_set
        .difference(&blind_set)
        .map(|s| (*s).clone())
        .collect();
    let extras_in_blind: Vec<String> = blind_set
        .difference(&intent_set)
        .map(|s| (*s).clone())
        .collect();

    AgreementReport {
        status,
        overlap,
        intent_tokens,
        blind_tokens,
        missing_in_blind,
        extras_in_blind,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Traces to: FR-UX-VERIFY-003
    #[test]
    fn matching_intent_and_blind_scores_green() {
        let intent = "Show the plan command help options available";
        let blind = "Terminal shows plan command help options available";
        let r = score(intent, blind);
        assert!(
            r.overlap >= 0.6,
            "expected green overlap, got {:.3} (missing={:?}, extras={:?})",
            r.overlap,
            r.missing_in_blind,
            r.extras_in_blind
        );
        assert_eq!(r.status, Agreement::Green);
    }

    /// Traces to: FR-UX-VERIFY-003
    #[test]
    fn divergent_intent_and_blind_scores_red() {
        let intent = "Show the plan command help text with all available options";
        let blind = "A photograph of a cat sitting on a windowsill bathed in sunlight.";
        let r = score(intent, blind);
        assert!(
            r.overlap < 0.3,
            "expected red overlap, got {:.3}",
            r.overlap
        );
        assert_eq!(r.status, Agreement::Red);
        assert!(!r.missing_in_blind.is_empty());
        assert!(!r.extras_in_blind.is_empty());
    }
}
