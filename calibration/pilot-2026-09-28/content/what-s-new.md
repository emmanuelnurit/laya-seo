# What's new

## What's new

crates.io ships **v0.1.2** (26 September 2026).

| Feature on `master` | Use it |
|---|---|
| `explain R19` / `RULE-R19` | Rule card: area, severity, effort band, fix |
| `report --baseline` | Score delta, rules cleared/new between two audit JSONs |
| `crawl --vitals` | Keyless PageSpeed lab vitals with R51-R53 |
| `query --provider dfs` | Opt-in DataForSEO live SERP, free fallback |
| `audit --actions-csv` | Ranked action tracker (id, priority, effort, impact) |
| `audit --pairs-csv` | Cannibalization A×B pairs with word-count winner |
| `audit --manifest` | Frozen `run.json` + `ledger.json` beside exports |
| `--no-jev` / `--jev-budget` | Rules-only mode and hard USD spend cap |
| `--user` / `--password` / `--header` | Basic auth + custom headers on target-site fetches; staging behind login becomes auditable |
| R58 broken hreflang clusters | Alternates pointing at noindexed or unreciprocated targets (heuristic, advisory) |
| `narrative.json` | Owner narrative; unknown action IDs fail closed |
| MCP `seo_explain`, `seo_report` | Same explain and baseline over stdio (15 tools) |
| MCP `seo_cite_check` | Citation check that works on every client: answer, sampling, or engine path |
| MCP `seo_gap` + `gsc gap` | Search Console impressions × weak positions, joined with citation history |
| `geo` keyless score | Deterministic 0-100 GEO formula with zero keys; Jev upgrades it when keyed |
| `drift baseline/compare/history` | SQLite audit snapshots as deploy gates; `seo_report` takes `baseline_label` |
| `audit --digest`, `bundle` | Agent brief with verify lines; one-file canonical close with pairs + drift |
| Paid engine answers | `JEV_SEO_LLM_KEY` + model (+ optional URL) covers Perplexity, OpenRouter, Ollama |

Get the latest:

```bash
cargo install jev-seo
jev-seo explain R19
```

Build from source for the working tree:

```bash
git clone https://github.com/AkashPriyadarshii/jev-seo.git
cd jev-seo
cargo install --path .
```

---