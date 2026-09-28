# FAQ

## FAQ

**What is jev-seo in one sentence?** A free, open-source Rust SEO and GEO CLI plus 15-tool MCP that audits, crawls, scores AI citation readiness, and gates CI without a monthly subscription.

**What does it cost?** Nothing for rules, crawls, and reports. Semantic Jev calls cost fractions of a cent under a hard per-run USD cap.

**Is there a free Semrush or Ahrefs alternative?** jev-seo is built as that alternative: local binary, DuckDuckGo search path, MIT license, no DataForSEO requirement on the free tier.

**Does it need API keys?** Not for deterministic work. Jev scoring wants `TYPESAFE_API_KEY`. Without it, those sections print local-only output. Paid search and extract backends stay parked until you pass their flags and keys.

**How is this different from a web dashboard?** It is a binary. You script it, gate CI on it, and agents call it over MCP. No seat to renew.

**What is a GEO score?** Citation likelihood 1-10 for answer engines, from a five-dimension composite with published weights in `src/policy.rs`.

**What is AEO / answer engine optimization here?** GEO score, `llms.txt` readiness, AI crawler permissions, and briefs that open with a direct answer. Rules that measure those stay labeled heuristics.

**Which pages does a crawl check?** Up to `--max-pages` (default 50), seeded from `/sitemap.xml` when present, robots.txt honored.

**Can CI fail on it?** Yes. `audit --min-pass` exits nonzero under your floor, and `--fail-on blocking` (the default) fails only on deterministic rules. `crawl --diff` reports drift against the last SQLite snapshot.

**Do I need paid APIs?** No. Paid backends are opt-in per flag. Free DuckDuckGo paths never call them.

**When do I get explain, report, and action trackers from crates.io?** Shipped in v0.1.2. `cargo install jev-seo`.

**Is my data sent anywhere?** Rank history and snapshots stay in `.jev-seo.db`. Audits run on your filesystem. Only the commands you point at remote hosts make network requests.

**Can an agent drive it?** Yes. `jev-seo mcp` exposes 15 tools over stdio JSON-RPC 2.0. One config block in your MCP client is enough.

**What license?** MIT. Source and binaries are free to use, modify, and redistribute.

---