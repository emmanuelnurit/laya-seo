# Architecture

## Architecture

```
jev-seo
├── src
│   ├── main.rs         CLI entrypoint and command dispatch
│   ├── engine.rs       TypeSafe Jev client, fan-out, spend counters
│   ├── policy.rs       Confidence thresholds, GEO weights, injection gate
│   ├── manifest.rs     run.json + ledger.json, citation gate, budgets
│   ├── narrative.rs    narrative.json load, ID refusal, number checks
│   ├── paths.rs        Guarded file reader, domain match, SSRF block
│   ├── serp.rs         DuckDuckGo HTML and suggest scraper
│   ├── audit.rs        Batch directory and file on-page auditor
│   ├── rules.rs        Rule registry R01-R58, scoring, explain, CSV
│   ├── actions.rs      Ranked actions and action-tracker CSV
│   ├── schema.rs       JSON-LD Schema.org validator
│   ├── robots.rs       Robots.txt and AI crawler analyzer
│   ├── brief.rs        SERP-driven content brief generator
│   ├── rank.rs         Local SQLite rank drift tracker
│   ├── sitemap.rs      XML sitemap and hreflang auditor
│   ├── crawl.rs        Live-site BFS crawler with dedup and timing
│   ├── fetch.rs        Direct / Jina / Firecrawl bodies under credit budget
│   ├── gsc.rs          Search Console device-flow client
│   ├── llms.rs         llms.txt and AI crawler readiness scorer
│   ├── mcp.rs          Stdio JSON-RPC 2.0 MCP server (15 tools)
│   └── tests.rs        Unit and integration harness (132 tests)
├── references/narrative.md
├── examples/           Real crawl, audit, and narrative fixtures
├── docs/               PRD, design, architecture, EVAL protocol
├── Cargo.toml
└── README.md
```

---