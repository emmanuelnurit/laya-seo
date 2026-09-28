# Direct answer

## Direct answer

**What is jev-seo?** It is a free, open-source SEO and GEO command-line tool written in Rust. You install one binary from crates.io or a release archive. It audits local content and live sites under 58 stable rules, scores pages for AI answer-engine citation with TypeSafe AI Jev, stores rank history in local SQLite, gates CI on blocking rules, and exposes 15 MCP tools so coding agents can run the same checks. The free path uses DuckDuckGo and local files only. Optional paid backends stay off until you pass a flag and key.

**Who is it for?** Developers, indie hackers, technical writers, and autonomous coding agents (Claude Code, Gemini CLI, Cursor, Antigravity) who need SEO evidence without a $130/month dashboard seat.

**What does it cost?** Rules, crawls, reports, rank history: free. Jev semantic calls: fractions of a cent under `--jev-budget` (default $0.25 per run).

`jev-seo` is a single static Rust binary for developers who refuse a $130/month SEO seat. It audits local Markdown and HTML trees under 58 stable rules, crawls live hosts with robots.txt honored, scores pages for answer-engine citation with TypeSafe AI Jev typed primitives, tracks rank drift in a local SQLite file, and exposes the same surface as a 15-tool stdio MCP server. Free search data comes from DuckDuckGo. Semantic scores sit behind `--jev-budget` (default $0.25 per run). Paid backends never fire without an explicit flag and key.

---