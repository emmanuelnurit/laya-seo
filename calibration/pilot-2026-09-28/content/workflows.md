# Workflows

## Workflows

### 1. Batch directory and file audit

```bash
jev-seo audit content/posts/
jev-seo audit content/posts/ --html report.html --pdf report.pdf --md report.md
jev-seo audit content/posts/ --actions-csv actions.csv --pairs-csv pairs.csv --manifest out/
```

Walks every Markdown and HTML file. Flags title collisions, thin content under 300 words, missing canonicals, missing alt text, skip-level headings, AI boilerplate, em-dash density, orphan pages, and keyword stems fighting each other. `--pairs-csv` expands each multi-file stem into A×B rows with a word-count winner.

### 2. JSON-LD Schema.org validation

```bash
jev-seo schema content/guide.md
```

Checks SoftwareApplication, Article, Organization, Product. Flags deprecated types such as HowTo rich results that no longer qualify.

### 3. Robots.txt and AI crawler inspection

```bash
jev-seo robots example.com
```

Reads `robots.txt` for GPTBot, ClaudeBot, anthropic-ai, PerplexityBot, Google-Extended, Bytespider. Prints allow/disallow and readiness actions.

<details>
<summary><strong>More workflows</strong> — briefs, GEO scoring, CI gate, sitemap, AI-slop radar, explain, baselines, narrative (click to expand)</summary>

### 4. SERP-driven content brief

```bash
jev-seo brief "fast local sqlite tui" --markdown
```

Live DuckDuckGo competitors, word-count estimate, Jev fan-out for H2 outline plus a direct-answer block sized for answer engines.

### 5. GEO citation score

```bash
jev-seo geo README.md --query "agentic skills framework for coding agents"
```

Citation likelihood 1-10 from Jev `Score` and `Noul`. Five-dimension composite (structure, density, directness, statistics, freshness) with published weights and history delta. Low-confidence answers are withheld. For a broader view of how AI engines perceive your brand, see [Prefer](https://tryprefer.com/).

### 6. Confidence-gated scoring

Strong Jev scores print as facts. Shaky ones print `[verify]`. Unsure ones print a note and no number. Thresholds live in `src/policy.rs`. Act confidence is 0.80. Injection pre-screen runs before quality suites.

### 7. CI SEO gate

```bash
jev-seo audit docs/ --min-pass 40
jev-seo audit docs/ --no-jev --jev-budget 0
```

Two gates, different jobs. `--min-pass` exits nonzero under your score floor. `--fail-on` (default `blocking`) exits nonzero only on deterministic blocking findings: missing titles, broken links, invalid JSON-LD, missing canonicals. Judgment calls (slop markers, length windows, lab vitals) print as `warn` and never fail the build alone, so a rule Google quietly changes cannot become the flaky test everyone bypasses. `--fail-on all` restores fail-on-anything. `--no-jev` and `--jev-budget 0` keep CI offline for semantics. Workflow: `.github/workflows/seo-gate.yml`. The full blocking list lives in `src/rules.rs` (`gate()`); every action line, CSV row, and `explain` card carries its class.

### 8. XML sitemap and hreflang

```bash
jev-seo sitemap sitemap.xml
jev-seo sitemap https://example.com/sitemap.xml
```

Canonical HTTPS, parameter pollution, 50,000 URL limit, `hreflang` codes (rejects `en-UK` for `en-GB`, requires `x-default` when alternates exist).

### 9. Helpful Content and AI slop radar

Built into `audit`:

- Em-dash density over 2 per 500 words
- 17 AI boilerplate markers (delve, leverage, testament, foster, seamless, crucial, robust, landscape, and peers)
- Heading hierarchy skip-levels (H1 to H3 with no H2)
- Internal link graph for zero-inbound orphans
- Keyword cannibalization stems plus A×B conflict pairs

### 10. Explain a rule, then diff a baseline

```bash
jev-seo explain R19
jev-seo explain RULE-R42 --json

jev-seo audit docs/ --json > before.json
# ... fix content ...
jev-seo audit docs/ --json > after.json
jev-seo report after.json --baseline before.json
```

`explain` prints area, severity, effort band, title, fix for any stable rule id. `report` prints score delta, rules cleared, rules new, ranked actions.

### 11. Optional narrative beside the report

Drop `narrative.json` next to exports (or in `--manifest`). Required keys: `executive_summary`, `strengths`, `risks`, `plan`. Every `RULE-Rxx` you cite must exist or load fails. Numbers that match nothing and plan effort bands that fight the action table print as warnings. Missing file embeds an automatic evidence-only summary, labeled automatic. Spec: `references/narrative.md`. Shape: `examples/narrative.example.json`.

</details>

---