use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::atomic::Ordering;

/// Alias from docs.typesafe.ai; resolves to the active production model.
pub const MODEL: &str = "jev-latest";

/// Checkpoint pinned for every laya-local request. Never negotiable: Laya's
/// English checkpoint can hallucinate confidently on non-English/non-Latin
/// content (script blindness), so the multilingual checkpoint is the only
/// one this client ever asks laya-bridge for.
pub const LAYA_LOCAL_MODEL: &str = "multilingual";

pub struct JevClient {
    api_key: Option<String>,
    endpoint: String,
    model: String,
    /// Short label for logs/manifest; never sent on the wire.
    backend: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub intent: String,
    pub intent_confidence: f64,
    pub geo_score: u32,
    pub geo_confidence: f64,
    pub direct_answer: bool,
    pub direct_answer_p: f64,
    pub content_gap: String,
    pub gap_confidence: f64,
    /// Answers to command-specific questions, keyed by question id.
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl AnalysisResult {
    /// Gate on intent + geo only. Gap options often split probability across
    /// near-winners, which must not veto the headline score.
    pub fn confidence(&self) -> f64 {
        self.intent_confidence.min(self.geo_confidence)
    }
}

/// Single deduped page state: one text field only. The old `content` +
/// `page.text` duplication sent the same head markup twice and halved useful
/// context; callers build state through this and add only small extras.
pub fn page_state(
    query: &str,
    title: Option<String>,
    description: Option<String>,
    text: String,
    word_count: usize,
    opening: Option<String>,
) -> serde_json::Value {
    json!({
        "query": query,
        "page": {
            "title": title,
            "description": description,
            "text": text,
            "word_count": word_count,
            "opening": opening
        }
    })
}

impl JevClient {
    /// Picks a backend, laya-local first: `LAYA_LOCAL_URL` (a self-hosted
    /// laya-bridge, see repo `laya-bridge/`) takes priority when set, so a
    /// site with both configured never spends against the paid TypeSafe API
    /// by accident. `TYPESAFE_API_KEY` / `--no-jev` behave exactly as before
    /// when `LAYA_LOCAL_URL` is unset — the cloud path is the fallback, not
    /// removed.
    pub fn new() -> Option<Self> {
        if let Some(base) = local_backend_base() {
            let endpoint = format!("{}/v1/systemone", base.trim_end_matches('/'));
            let api_key = std::env::var("LAYA_LOCAL_API_KEY")
                .ok()
                .filter(|k| !k.trim().is_empty());
            return Some(Self {
                api_key,
                endpoint,
                model: LAYA_LOCAL_MODEL.to_string(),
                backend: "laya-local",
            });
        }
        let key = std::env::var("TYPESAFE_API_KEY").ok()?;
        if key.trim().is_empty() {
            return None;
        }
        Some(Self {
            api_key: Some(key),
            endpoint: "https://api.typesafe.ai/v1/systemone".to_string(),
            model: MODEL.to_string(),
            backend: "typesafe-cloud",
        })
    }

    /// Pre-execution safety classifier for agent-driven file/URL tools.
    /// True = target looks like a secret, credential, or system path, block it.
    /// None = the check itself failed (transport error, bad schema): callers
    /// must fail closed, because an unreachable guard is not a clean bill.
    pub fn safety_block(&self, tool: &str, target: &str) -> Option<bool> {
        let payload = json!({
            "model": self.model,
            "state": { "tool": tool, "target": target },
            "questions": {
                "unsafe_target": {
                    "type": "noul",
                    "instructions": "Does `target` name a secret, credential, private key, token, password, system directory, or dot-file that the tool must not read?",
                }
            }
        });
        let body: serde_json::Value = match self
            .post(payload)
            .and_then(|(r, est)| {
                r.into_json()
                    .map_err(anyhow::Error::from)
                    .map(|b| (b, est))
            })
        {
            Ok((b, est)) => {
                record_usage(&b, est);
                b
            }
            Err(_) => return None,
        };
        body.get("answers")
            .and_then(|a| a.get("unsafe_target"))
            .and_then(|u| u.get("noul"))
            .and_then(|n| n.as_f64())
            .map(|p| p >= 0.7)
    }

    fn post(&self, payload: serde_json::Value) -> Result<(ureq::Response, u64)> {
        let body = payload.to_string();
        // Reserve before dispatch: parallel calls see each other's estimates
        // and cannot jointly overshoot the cap. Released on transport failure,
        // settled against real usage by the caller.
        let est_tokens = (body.len() / 3) as u64;
        if !crate::manifest::reserve_jev_tokens(est_tokens) {
            return Err(anyhow::anyhow!(
                "Jev budget cap reached (${:.4}); raise --jev-budget to continue",
                crate::manifest::jev_budget_usd()
            ));
        }
        // Transient 429/5xx get two more tries with backoff. Every attempt
        // counts as a request: the ledger bills attempts, not wishes.
        let waits = [500u64, 1500u64];
        for attempt in 0..=waits.len() {
            let mut req = ureq::post(&self.endpoint)
                .set("Content-Type", "application/json")
                .timeout(std::time::Duration::from_secs(12));
            // laya-local has no auth by default (trusted loopback); only send
            // a bearer token when the client was actually given one.
            if let Some(key) = &self.api_key {
                req = req.set("Authorization", &format!("Bearer {}", key));
            }
            let raw = req.send_string(&body);
            let retryable = matches!(&raw, Err(ureq::Error::Status(code, _)) if *code == 429 || (500..=599).contains(code));
            let resp = raw.context(format!("Failed to communicate with {} at {}", self.backend, self.endpoint));
            match resp {
                Ok(r) => {
                    crate::manifest::JEV_REQUESTS.fetch_add(1, Ordering::Relaxed);
                    return Ok((r, est_tokens));
                }
                Err(e) => {
                    crate::manifest::JEV_REQUESTS.fetch_add(1, Ordering::Relaxed);
                    crate::manifest::JEV_FAILED.fetch_add(1, Ordering::Relaxed);
                    if retryable {
                        if let Some(ms) = waits.get(attempt) {
                            std::thread::sleep(std::time::Duration::from_millis(*ms));
                            continue;
                        }
                    }
                    crate::manifest::release_jev_tokens(est_tokens);
                    return Err(e);
                }
            }
        }
        unreachable!("retry loop always returns")
    }
    /// Same as fanout_eval plus command-specific questions merged into the one
    /// request. Answers land in `extra` for code to consume.
    /// State is pre-filtered (skill: strip junk) then truncated.
    pub fn fanout_eval_with(
        &self,
        state: serde_json::Value,
        extra_questions: serde_json::Value,
    ) -> Result<AnalysisResult> {
        let state = truncate_state(prefilter_state(state));
        let input_chars = state.to_string().len();
        let mut questions = json!({
                "intent": {
                    "type": "choice",
                    "instructions": "Select the primary search intent.",
                    "criteria": {
                        "informational": "How-to, tutorial, explanation, documentation, research",
                        "commercial": "Product reviews, pricing comparisons, buying evaluation",
                        "transactional": "Immediate download, sign-up, purchase, command execution",
                        "navigational": "Specific brand, GitHub repo, or homepage search",
                        "insufficient_context": "Supplied evidence is too thin to choose safely"
                    }
                },
                "geo_score": {
                    "type": "score",
                    "instructions": "Rate citation likelihood for generative search engines (Perplexity, SearchGPT, Gemini).",
                    "criteria": [
                        "Very low: promotional fluff, lacks concrete documentation or technical specifics",
                        "Low: shallow overview, missing practical code examples or proof",
                        "Moderate: helpful technical details but lacks authoritative benchmark or structured layout",
                        "High: clear, structured, copy-pasteable commands and direct factual definitions",
                        "Exceptional: comprehensive authoritative reference, zero fluff, perfect citation density"
                    ]
                },
                "direct_answer": {
                    "type": "noul",
                    "instructions": "Does the content provide a direct, concise factual answer or code example in the opening section?",
                    "criteria": {
                        "true": "Content begins with a direct definition, quickstart command, or concise answer",
                        "false": "Content rambles, buries the solution, or lacks concrete code"
                    }
                },
                "content_gap": {
                    "type": "choice",
                    "instructions": "What is the primary weakness or missing angle?",
                    "criteria": {
                        "missing_statistics": "Lacks empirical benchmarks, numbers, or proof",
                        "generic_prose": "AI-slop, superficial fluff, empty buzzwords",
                        "no_step_by_step": "Missing practical reproduction steps or code blocks",
                        "outdated_examples": "Obsolete APIs or dead references",
                        "none": "Satisfies user query with high information density",
                        "insufficient_context": "Too little content to judge a weakness"
                    }
                }
        });
        if let Some(map) = questions.as_object_mut() {
            if let Some(extra) = extra_questions.as_object() {
                for (k, v) in extra {
                    map.insert(k.clone(), v.clone());
                }
            }
        }
        let payload = json!({
            "model": self.model,
            "state": state,
            "questions": questions
        });

        let started = std::time::Instant::now();
        let (resp, est) = self.post(payload)?;
        let latency_ms = started.elapsed().as_millis() as u64;

        let body: serde_json::Value = resp.into_json()?;
        record_usage(&body, est);
        let answers = body.get("answers").context("Invalid Jev response schema")?;
        let ts_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        // Structured log line per bridge call (backend, latency, resolved model,
        // per-question confidence): the only way to diagnose scoring drift after
        // the fact, so every backend writes it, not only laya-local.
        crate::manifest::append_eval_log(serde_json::json!({
            "ts_ms": ts_ms,
            "qver": crate::policy::QUESTION_VERSION,
            "backend": self.backend,
            "model": self.model,
            "resolved_model": body.get("model").and_then(|m| m.as_str()),
            "latency_ms": latency_ms,
            "input_chars": input_chars,
            "answers": answers
        }));

        let intent_obj = &answers["intent"];
        let intent = intent_obj["choice"]
            .as_str()
            .context("Jev response missing answers.intent.choice")?
            .to_string();
        let intent_confidence = intent_obj["confidence"]
            .as_f64()
            .context("Jev response missing answers.intent.confidence")?;

        let geo_obj = &answers["geo_score"];
        let geo_val = geo_obj["score"]
            .as_f64()
            .context("Jev response missing answers.geo_score.score")?;
        // Score decisiveness comes from the side-of-midpoint probability mass,
        // not the confidence scalar: a 4-1 split on one side is decisive even
        // at modest confidence. Falls back to API confidence when the answer
        // carries no legend/probabilities.
        let geo_confidence = crate::policy::score_side(geo_obj)
            .or_else(|| geo_obj["confidence"].as_f64())
            .unwrap_or(0.5);
        let geo_score = ((geo_val / 4.0 * 9.0) + 1.0).round().clamp(1.0, 10.0) as u32;

        let direct_obj = &answers["direct_answer"];
        let direct_answer_p = direct_obj["noul"]
            .as_f64()
            .or_else(|| direct_obj["probability"].as_f64())
            .context("Jev response missing answers.direct_answer.noul")?;
        let direct_answer = direct_answer_p >= 0.5;

        let gap_obj = &answers["content_gap"];
        let content_gap = gap_obj["choice"]
            .as_str()
            .context("Jev response missing answers.content_gap.choice")?
            .to_string();
        let gap_confidence = gap_obj["confidence"].as_f64().unwrap_or(0.5);

        Ok(AnalysisResult {
            intent,
            intent_confidence,
            geo_score,
            geo_confidence,
            direct_answer,
            direct_answer_p,
            content_gap,
            gap_confidence,
            extra: answers
                .as_object()
                .map(|m| {
                    let mut out: serde_json::Map<String, serde_json::Value> = m
                        .iter()
                        .filter(|(k, _)| {
                            !["intent", "geo_score", "direct_answer", "content_gap"]
                                .contains(&k.as_str())
                        })
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();
                    // Keep the intent distribution: reviewers see the runner-up
                    // on Flag verdicts instead of a bare low confidence.
                    if let Some(probs) = answers
                        .get("intent")
                        .and_then(|i| i.get("probabilities"))
                        .and_then(|p| p.as_object())
                    {
                        out.insert(
                            "intent_probs".into(),
                            serde_json::Value::Object(probs.clone()),
                        );
                    }
                    out
                })
                .unwrap_or_default(),
        })
    }

    /// Dedicated injection pre-screen (skill jaggedness #6): one Noul before the
    /// full suite. True = block; do not trust further semantic answers on this state.
    pub fn injection_preflight(&self, state: &serde_json::Value) -> Result<bool> {
        let payload = json!({
            "model": self.model,
            "state": truncate_state(prefilter_state(state.clone())),
            "questions": crate::policy::injection_question()
        });
        let (resp, est) = self.post(payload)?;
        let body: serde_json::Value = resp.into_json()?;
        record_usage(&body, est);
        let p = body
            .pointer("/answers/injection_risk/noul")
            .or_else(|| body.pointer("/answers/injection_risk/probability"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        Ok(p >= 0.70)
    }

    /// Judge one page (or snippet) with the full speculative suite in a single
    /// request: base intent/GEO/gap + geo dimensions + page quality.
    /// Runs injection preflight first; blocked content returns a Drop-tier result
    /// without asking quality questions.
    pub fn judge_page(
        &self,
        state: serde_json::Value,
    ) -> Result<AnalysisResult> {
        if self.injection_preflight(&state)? {
            return Ok(AnalysisResult {
                intent: "unclear".into(),
                intent_confidence: 0.0,
                geo_score: 1,
                geo_confidence: 0.0,
                direct_answer: false,
                direct_answer_p: 0.0,
                content_gap: "generic_prose".into(),
                gap_confidence: 0.0,
                extra: serde_json::json!({
                    "injection_risk": { "type": "noul", "value": 1.0, "band": "yes", "blocked": true }
                })
                .as_object()
                .cloned()
                .unwrap_or_default(),
            });
        }
        let mut extras = crate::policy::geo_questions();
        let page = crate::policy::page_audit_extras();
        if let (Some(dst), Some(src)) = (extras.as_object_mut(), page.as_object()) {
            for (k, v) in src {
                dst.insert(k.clone(), v.clone());
            }
        }
        // Conditional inclusion: never ask about fields the state does not
        // have. A missing title cannot fit, a missing description cannot match.
        let has_title = state
            .pointer("/page/title")
            .and_then(|t| t.as_str())
            .is_some_and(|t| !t.trim().is_empty());
        let has_desc = state
            .pointer("/page/description")
            .and_then(|d| d.as_str())
            .is_some_and(|d| !d.trim().is_empty());
        if let Some(dst) = extras.as_object_mut() {
            if !has_title {
                dst.remove("title_fit");
            }
            if !has_desc {
                dst.remove("meta_fit");
                dst.remove("meta_verdict");
            }
            // Policy pages (privacy, terms, contact) get content and trust
            // judgments they can never pass: expertise signals do not belong
            // on legal boilerplate, and asking wastes money.
            let query = state.get("query").and_then(|q| q.as_str()).unwrap_or("");
            let stem = query.rsplit(['/', '\\']).next().unwrap_or(query);
            let stem = stem.split('.').next().unwrap_or(stem).to_ascii_lowercase();
            if ["privacy", "terms", "terms-of-service", "contact", "legal", "cookies", "cookie-policy", "disclaimer"]
                .contains(&stem.as_str())
            {
                for id in ["page_helpfulness", "page_trust", "page_specificity", "geo_density", "geo_statistics"] {
                    dst.remove(id);
                }
            }
        }
        self.fanout_eval_with(state, extras)
    }

    /// Site/homepage judgment: base + GEO dims + value prop / entity / model.
    pub fn judge_site(&self, state: serde_json::Value) -> Result<AnalysisResult> {
        if self.injection_preflight(&state)? {
            return Ok(AnalysisResult {
                intent: "unclear".into(),
                intent_confidence: 0.0,
                geo_score: 1,
                geo_confidence: 0.0,
                direct_answer: false,
                direct_answer_p: 0.0,
                content_gap: "generic_prose".into(),
                gap_confidence: 0.0,
                extra: serde_json::json!({
                    "injection_risk": { "type": "noul", "value": 1.0, "band": "yes", "blocked": true }
                })
                .as_object()
                .cloned()
                .unwrap_or_default(),
            });
        }
        let mut extras = crate::policy::geo_questions();
        let site = crate::policy::site_extras();
        if let (Some(dst), Some(src)) = (extras.as_object_mut(), site.as_object()) {
            for (k, v) in src {
                dst.insert(k.clone(), v.clone());
            }
        }
        self.fanout_eval_with(state, extras)
    }
}

/// Base URL of a self-hosted laya-bridge from `LAYA_LOCAL_URL`
/// (e.g. `http://127.0.0.1:8000`), or `None` when unset/blank so callers fall
/// back to the TypeSafe cloud path.
fn local_backend_base() -> Option<String> {
    let v = std::env::var("LAYA_LOCAL_URL").ok()?;
    let v = v.trim();
    if v.is_empty() {
        None
    } else {
        Some(v.to_string())
    }
}

/// Drop noise fields before truncate so Jev sees decisive evidence only.
fn prefilter_state(value: serde_json::Value) -> serde_json::Value {
    const DROP_KEYS: &[&str] = &[
        "raw_html",
        "html",
        "scripts",
        "styles",
        "css",
        "inline_script",
        "dom_snapshot",
        "history",
        "chat_transcript",
        "entire_repo",
        "conversation",
    ];
    match value {
        serde_json::Value::Object(map) => serde_json::Value::Object(
            map.into_iter()
                .filter(|(k, v)| {
                    let kl = k.to_ascii_lowercase();
                    if DROP_KEYS.iter().any(|d| kl == *d) {
                        return false;
                    }
                    // Drop empty containers and pure-whitespace strings.
                    match v {
                        serde_json::Value::Array(a) => !a.is_empty(),
                        serde_json::Value::Object(o) => !o.is_empty(),
                        serde_json::Value::String(s) => !s.trim().is_empty(),
                        _ => true,
                    }
                })
                .map(|(k, v)| (k, prefilter_state(v)))
                .collect(),
        ),
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.into_iter().map(prefilter_state).collect())
        }
        other => other,
    }
}

/// Fold API usage into the run ledger (input tokens drive cost at list price).
/// Settles the pre-dispatch reservation: missing usage blocks charge the
/// estimate instead of vanishing to zero.
/// Also pins the resolved model version: thresholds couple to one model's
/// distribution, so the ledger records which build actually served.
pub static JEV_MODEL: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub fn jev_model() -> &'static str {
    JEV_MODEL.get().map(String::as_str).unwrap_or("unknown")
}

fn record_usage(body: &serde_json::Value, est_tokens: u64) {
    let actual = body
        .get("usage")
        .and_then(|u| u.as_object())
        .and_then(|u| u.get("input_tokens"))
        .and_then(|t| t.as_u64());
    crate::manifest::settle_jev_tokens(est_tokens, actual);
    if let Some(m) = body.get("model").and_then(|m| m.as_str()) {
        let _ = JEV_MODEL.set(m.to_string());
    }
}

/// Cap every string in the state so oversized pages never 400 the API.
fn truncate_state(value: serde_json::Value) -> serde_json::Value {
    const LIMIT: usize = 8000;
    match value {
        serde_json::Value::String(s) => {
            if s.len() > LIMIT {
                let cut = s.floor_char_boundary(LIMIT);
                serde_json::Value::String(format!("{}…[truncated]", &s[..cut]))
            } else {
                serde_json::Value::String(s)
            }
        }
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.into_iter().map(truncate_state).collect())
        }
        serde_json::Value::Object(map) => serde_json::Value::Object(
            map.into_iter().map(|(k, v)| (k, truncate_state(v))).collect(),
        ),
        other => other,
    }
}
