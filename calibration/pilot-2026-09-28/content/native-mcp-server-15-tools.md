# Native MCP Server (15 Tools)

## Native MCP Server (15 Tools)

`jev-seo mcp` speaks stdio JSON-RPC 2.0. It answers `initialize`, stays silent on notifications, reports parse errors, and sets `isError` on tool failures. File tools use a guarded reader (content extensions only, no dot-files, no URLs) plus a Jev safety classifier for secret-looking targets. Remote fetches refuse private hosts, resolved DNS, and redirect landings.

| MCP Tool | Arguments | Purpose |
|---|---|---|
| `seo_cite_check` | `target: string`, `query: string`, `answer?: string` | Citation verdict via own answer, sampling, or engine |
| `seo_gap` | `site: string`, `limit?: int` | Gap queue: impressions × weak positions + citation history |
| `seo_keywords` | `query: string` | Autocomplete variants and intent class |
| `seo_serp_inspect` | `query: string`, `limit?: int` | Live top titles, snippets, URLs |
| `seo_audit` | `path: string` | Local Markdown/HTML audit, orphans, AI slop |
| `seo_geo` | `target: string`, `query: string` | Citation likelihood 1-10 and answer presence |
| `seo_schema` | `target: string` | JSON-LD structure and deprecation rules |
| `seo_robots` | `domain: string` | Live robots.txt for major LLM crawlers |
| `seo_brief` | `topic: string`, `limit?: int` | Markdown brief with H2 outline |
| `seo_sitemap` | `target: string` | Sitemap protocol, HTTPS, hreflang |
| `seo_crawl` | `url: string`, `max_pages?: int` | Broken links, redirects, orphans |
| `seo_llms` | `domain: string` | llms.txt and AI crawler permissions |
| `seo_extract` | `urls: string[]`, `query: string` | URL to markdown (key-gated paid path) |
| `seo_explain` | `id: string` | Rule card for `R19` / `RULE-R19` |
| `seo_report` | `path: string`, `baseline?: string`, `baseline_label?: string` | Audit JSON diff: score, rules, actions, pairs, drift |

---