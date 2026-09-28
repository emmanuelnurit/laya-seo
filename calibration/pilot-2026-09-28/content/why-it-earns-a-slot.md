# Why it earns a slot

## Why it earns a slot

Paid suites price a seat. This prices a compile. The table maps what you get to why a FOSS dev or agent loop cares.

| What you get | Why it matters |
|---|---|
| One binary, zero services | `cargo install jev-seo` or a release tarball. No Electron, no SaaS login, no browser farm. |
| 58-rule local + live checks | R01-R58 across crawl, on-page, content, links, structured data, AI access, performance, security, canonical. Crawl and audit share one registry. |
| Jev as typed oracle, not chat | Intent, GEO composite, page quality as `Choice` / `Score` / `Noul` with confidence gates. Weak answers print `[verify]` or drop. No free-form generation. |
| Spend you can prove | `--jev-budget` hard-caps USD before each POST. `--no-jev` forces rules-only. `ledger.json` records requests and tokens when you pass `--manifest`. |
| Agent-native MCP | 15 tools on stdio JSON-RPC 2.0. One config block wires Claude Code, Gemini CLI, Antigravity. Guarded file reads and SSRF refuse on remote hops. |
| CI gate you can paste | `audit --min-pass 40` exits nonzero under the floor. Workflow lives in `.github/workflows/seo-gate.yml`. |
| Reports from one run | HTML, PDF, Markdown, findings CSV, action-tracker CSV, cannibalization pair CSV, frozen `run.json` + `ledger.json`. |
| Local rank history | SQLite drift in `.jev-seo.db`. `crawl --diff` compares to the last snapshot. |
| Free search path | DuckDuckGo HTML and suggest. Tavily, DataForSEO, Jina, Firecrawl, extract stay parked until you opt in with a key. |

**Start here:** [Example crawl](examples/jevseo-site/crawl.json) · [Example audit HTML](examples/local-docs/report.html) · [Narrative contract](references/narrative.md) · [Eval protocol](docs/EVAL.md) · [Agent skill](SKILL.md) · [PRD](docs/PRD.md)

---