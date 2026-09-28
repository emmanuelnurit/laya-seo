//! Generic "4-step workflow" primitive (auspia.ai: Générer & raisonner →
//! Décider → Exécuter → Revoir), shared by every Laya SEO use case:
//!
//! 1. **Générer & raisonner** — one Jev call (`engine::JevClient`), already
//!    handled by each command's own state-building code; this module starts
//!    downstream of that call.
//! 2. **Décider** — `policy::gate` turns the raw confidence into a verdict
//!    (Act/Flag/Drop against the thresholds in `policy.rs`, themselves
//!    calibrated by `calibrate.rs` on labeled data, not left at the Laya doc
//!    defaults per MYO-536's mandate).
//! 3. **Exécuter** — `stage()` appends the decision to an append-only ledger.
//!    This is deliberately the *entire* execute step: jev-seo never renames,
//!    merges, or deletes a file on its own for any use case (intent, content
//!    decision, internal link, GEO score) — staging is the safe, reversible
//!    action a human then acts on.
//! 4. **Revoir** — `review_report()` turns the staged ledger into a markdown
//!    summary grouped by verdict, Drop/Flag rows first, so a human reviewer's
//!    attention goes where the model was least sure.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagedDecision {
    /// One of "intent-classify", "content-decide", "link", "geo".
    pub use_case: String,
    /// Query text, file path, or URL the decision is about.
    pub subject: String,
    /// The decided outcome (e.g. "commercial", "merge", a link target id).
    pub decision: String,
    pub confidence: f64,
    /// "act" | "flag" | "drop" — see `policy::Verdict`.
    pub verdict: String,
    /// Raw Jev evidence (probabilities, criteria matched) kept for audit and
    /// for building future calibration datasets from real staged runs.
    pub reasoning: serde_json::Value,
    pub ts_ms: u128,
}

pub fn verdict_str(v: crate::policy::Verdict) -> &'static str {
    match v {
        crate::policy::Verdict::Act => "act",
        crate::policy::Verdict::Flag => "flag",
        crate::policy::Verdict::Drop => "drop",
    }
}

pub fn make_decision(
    use_case: &str,
    subject: &str,
    decision: &str,
    confidence: f64,
    verdict: crate::policy::Verdict,
    reasoning: serde_json::Value,
) -> StagedDecision {
    let ts_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    StagedDecision {
        use_case: use_case.to_string(),
        subject: subject.to_string(),
        decision: decision.to_string(),
        confidence,
        verdict: verdict_str(verdict).to_string(),
        reasoning,
        ts_ms,
    }
}

/// Ledger path: beside the local DB (`JEV_SEO_DB`'s directory when set, same
/// override tests use), otherwise `~/.jev-seo/decisions.jsonl`.
fn ledger_path() -> Option<std::path::PathBuf> {
    crate::manifest::jev_home_dir().map(|d| d.join("decisions.jsonl"))
}

/// "Exécuter": append-only stage, best-effort like `manifest::append_eval_log`
/// -- a ledger write failure must never fail the surrounding audit/link/geo
/// command that produced the decision. Rotates past 5 MB, same cap and same
/// reasoning as `manifest::append_eval_log`: this ledger carries raw Jev
/// reasoning blobs per decision, so it grows the same way the eval log does.
pub fn stage(decision: &StagedDecision) -> Result<()> {
    const CAP_BYTES: u64 = 5 * 1024 * 1024;
    let Some(path) = ledger_path() else {
        return Ok(());
    };
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).ok();
    }
    if path.metadata().map(|m| m.len() > CAP_BYTES).unwrap_or(false) {
        let _ = std::fs::remove_file(&path);
    }
    use std::io::Write;
    let mut opts = std::fs::OpenOptions::new();
    opts.create(true).append(true);
    // Owner-only: this ledger carries staged page excerpts and raw Jev
    // reasoning (main.rs geo/intent/decide) on a workspace shared across
    // concurrent agents (CLAUDE.md) -- same posture as gsc.rs's token file.
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let mut f = opts
        .open(&path)
        .with_context(|| format!("cannot open {}", path.display()))?;
    writeln!(f, "{}", serde_json::to_string(decision)?)?;
    Ok(())
}

/// "Revoir": markdown summary grouped by verdict, Drop then Flag then Act, so
/// the rows needing a human decision sort above the ones the model already
/// cleared.
pub fn review_report(decisions: &[StagedDecision]) -> String {
    let mut out = String::new();
    out.push_str("# Revue des décisions — Laya SEO\n\n");
    if decisions.is_empty() {
        out.push_str("Aucune décision à revoir.\n");
        return out;
    }
    let mut by_verdict: std::collections::BTreeMap<&str, Vec<&StagedDecision>> =
        std::collections::BTreeMap::new();
    for d in decisions {
        by_verdict.entry(d.verdict.as_str()).or_default().push(d);
    }
    for verdict in ["drop", "flag", "act"] {
        let Some(rows) = by_verdict.get(verdict) else {
            continue;
        };
        let label = match verdict {
            "drop" => "A revoir en priorite (confiance insuffisante)",
            "flag" => "A verifier (confiance moyenne)",
            _ => "Decisions actees (confiance suffisante)",
        };
        out.push_str(&format!("## {} ({})\n\n", label, rows.len()));
        out.push_str("| Cas d'usage | Sujet | Decision | Confiance |\n|---|---|---|---|\n");
        for d in rows {
            out.push_str(&format!(
                "| {} | {} | {} | {:.2} |\n",
                d.use_case, d.subject, d.decision, d.confidence
            ));
        }
        out.push('\n');
    }
    out
}

/// Read back every staged decision (used by `review` output and by future
/// calibration passes that want real production decisions as a labeling
/// source, not just synthetic examples).
pub fn read_all() -> Result<Vec<StagedDecision>> {
    let Some(path) = ledger_path() else {
        return Ok(Vec::new());
    };
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return Ok(Vec::new());
    };
    Ok(raw
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect())
}
