//! Confidence-threshold calibration (MYO-536).
//!
//! The article this product implements (auspia.ai) is explicit: never ship
//! Laya's documentation default confidence threshold — calibrate it on your
//! own labeled decisions. `policy.rs` centralizes every threshold this tool
//! gates on (`ACT`, `FLAG`, the 0.70 injection band); this module is how
//! those numbers get replaced with something backed by real labeled data
//! instead of a skill rule-of-thumb.
//!
//! Input: a CSV of `id,confidence,correct` rows — one row per past Jev
//! decision, `confidence` the score jev-seo reported, `correct` a human's
//! yes/no on whether that decision was actually right. Output: the
//! confidence threshold that maximizes F1 on that data, plus precision/
//! recall at that threshold and at the current default so the two are
//! directly comparable in one report.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct LabeledExample {
    pub id: String,
    pub confidence: f64,
    /// Ground truth: did a human confirm the Jev decision at this confidence
    /// was actually correct?
    pub correct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationResult {
    pub n: usize,
    pub positives: usize,
    pub recommended_threshold: f64,
    pub precision_at_threshold: f64,
    pub recall_at_threshold: f64,
    pub f1_at_threshold: f64,
    pub default_threshold: f64,
    pub precision_at_default: f64,
    pub recall_at_default: f64,
    pub f1_at_default: f64,
    /// ids of false positives/negatives at the recommended threshold, for a
    /// human to spot-check before trusting the number.
    pub misclassified_at_threshold: Vec<String>,
}

/// Parse `id,confidence,correct` CSV. Header row required; `id` is optional
/// (falls back to `row-N`); `correct`/`label` accepts true/false/1/0/yes/no.
pub fn parse_csv(raw: &str) -> Result<Vec<LabeledExample>> {
    let mut lines = raw.lines();
    let header = lines.next().context("empty CSV")?;
    let cols = crate::gsc::split_csv_line(header);
    let idx = |name: &str| cols.iter().position(|c| c.eq_ignore_ascii_case(name));
    let id_i = idx("id");
    let conf_i = idx("confidence").context("CSV must have a `confidence` column")?;
    let label_i = idx("correct")
        .or_else(|| idx("label"))
        .context("CSV must have a `correct` (or `label`) column")?;
    let mut out = Vec::new();
    for (n, line) in lines.enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let fields = crate::gsc::split_csv_line(line);
        let confidence: f64 = fields
            .get(conf_i)
            .context("missing confidence field")?
            .parse()
            .with_context(|| format!("row {}: bad confidence", n + 2))?;
        let raw_label = fields.get(label_i).context("missing label field")?.to_ascii_lowercase();
        let correct = matches!(raw_label.as_str(), "true" | "1" | "yes" | "correct");
        let id = id_i
            .and_then(|i| fields.get(i))
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("row-{}", n + 2));
        out.push(LabeledExample { id, confidence, correct });
    }
    if out.is_empty() {
        anyhow::bail!("CSV had a header but no data rows");
    }
    Ok(out)
}

fn precision_recall_f1(examples: &[LabeledExample], threshold: f64) -> (f64, f64, f64) {
    let mut tp = 0usize;
    let mut fp = 0usize;
    let mut fn_ = 0usize;
    for e in examples {
        let predicted_positive = e.confidence >= threshold;
        match (predicted_positive, e.correct) {
            (true, true) => tp += 1,
            (true, false) => fp += 1,
            (false, true) => fn_ += 1,
            (false, false) => {}
        }
    }
    let precision = if tp + fp == 0 { 0.0 } else { tp as f64 / (tp + fp) as f64 };
    let recall = if tp + fn_ == 0 { 0.0 } else { tp as f64 / (tp + fn_) as f64 };
    let f1 = if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    };
    (precision, recall, f1)
}

/// Sweep every observed confidence as a candidate threshold, keep the one
/// maximizing F1. Ties break toward the *higher* threshold: on a tie, fewer
/// false Acts on ambiguous data is the safer failure mode for a tool whose
/// output an editor may act on directly.
pub fn calibrate(examples: &[LabeledExample], default_threshold: f64) -> Result<CalibrationResult> {
    if examples.is_empty() {
        anyhow::bail!("no labeled examples to calibrate on");
    }
    let mut candidates: Vec<f64> = examples.iter().map(|e| e.confidence).collect();
    candidates.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    candidates.dedup();
    let mut best = (default_threshold, -1.0f64);
    for &t in &candidates {
        let (_, _, f1) = precision_recall_f1(examples, t);
        if f1 > best.1 || (f1 == best.1 && t > best.0) {
            best = (t, f1);
        }
    }
    let (p, r, f1) = precision_recall_f1(examples, best.0);
    let (dp, dr, df1) = precision_recall_f1(examples, default_threshold);
    let positives = examples.iter().filter(|e| e.correct).count();
    let misclassified_at_threshold: Vec<String> = examples
        .iter()
        .filter(|e| (e.confidence >= best.0) != e.correct)
        .map(|e| e.id.clone())
        .collect();
    Ok(CalibrationResult {
        n: examples.len(),
        positives,
        recommended_threshold: best.0,
        precision_at_threshold: p,
        recall_at_threshold: r,
        f1_at_threshold: f1,
        default_threshold,
        precision_at_default: dp,
        recall_at_default: dr,
        f1_at_default: df1,
        misclassified_at_threshold,
    })
}
