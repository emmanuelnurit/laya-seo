# Command Reference

## Command Reference

| Command | Description | Flags |
|---|---|---|
| `keywords <query>` | Autocomplete discovery and Jev intent classification | `--json` |
| `query <query>` | Live SERP scrape, Jev relevance rerank, gap analysis | `--limit <n>`, `--provider auto\|ddg\|tavily\|dfs`, `--depth`, `--topic`, `--json` |
| `audit <path>` | Directory or file audit under 58 rules; orphans, thin, cannibalization | `--target-query <query>`, `--json`, `--min-pass <pct>`, `--fail-on blocking\|all`, `--forbid R31,R36`, `--max-critical <n>`, `--html <path>`, `--pdf <path>`, `--md <path>`, `--csv <path>`, `--actions-csv <path>`, `--pairs-csv <path>`, `--rescore <path>`, `--manifest <dir>`, `--no-jev`, `--jev-budget <usd>`, `--user <u>`, `--password <p>`, `--header <name:value>` |
| `geo <target>` | GEO citation score 1-10, five-dimension composite, trend | `--query <query>`, `--json`, `--jev-budget <usd>` |
| `schema <target>` | JSON-LD structural and deprecation validator | `--json` |
| `robots <domain>` | Robots.txt and AI crawler permission auditor | `--json` |
| `brief <topic>` | SERP heading outline and direct-answer block | `--limit <n>`, `--provider auto\|ddg\|tavily\|dfs`, `--markdown`, `--json` |
| `link <path>` | Jev Choice internal-link suggestions per page, or `no_link` | `--limit <n>`, `--json` |
| `rank` | SQLite rank drift tracker (`.jev-seo.db`) | `--domain <domain>`, `--query <query>` |
| `sitemap <target>` | XML sitemap, 50k limit, HTTPS, hreflang | `--json` |
| `crawl <url>` | Live BFS crawl: health score, actions, redirects, orphans, timing | `--max-pages <n>`, `--fetch auto\|direct\|jina\|firecrawl`, `--max-credits <n>`, `--json`, `--diff`, `--csv <path>`, `--rescore <path>`, `--manifest <dir>`, `--no-jev`, `--jev-budget <usd>`, `--user <u>`, `--password <p>`, `--header <name:value>`, `--vitals` |
| `llms <domain>` | llms.txt and AI crawler readiness with actions | `--json` |
| `explain <id>` | Rule card for `R19` or `RULE-R19` | `--json` |
| `report <path>` | Diff two audit JSONs: score, rules, actions | `--baseline <path>`, `--actions-csv <path>`, `--json` |
| `doctor` | Version, API key, database, platform | `--json` |
| `gsc <auth\|sites\|query>` | Search Console: free first-party query data | `--site <url>`, `--code`, `--limit <n>`, `--json` |
| `mcp` | Stdio JSON-RPC 2.0 agent MCP server | (None) |

Real `--help` excerpt (`audit`):

```
Usage: jev-seo.exe audit [OPTIONS] <PATH>

Options:
      --json
      --min-pass <MIN_PASS>       Exit nonzero when pass rate falls below this percent (CI gate)
      --fail-on <FAIL_ON>         Which findings fail the build: blocking or all [default: blocking]
      --forbid <FORBID>           Comma-separated rule ids that always fail
      --max-critical <MAX_CRITICAL>  Fail when more than N Critical findings fire
      --html <PATH>               Write a single-file HTML report to this path
      --pdf <PATH>                Write a PDF report to this path
      --md <PATH>                 Write a Markdown report to this path
      --csv <PATH>                Write a CSV findings export to this path
      --actions-csv <PATH>        Write ranked action-tracker CSV (id, priority, effort, impact)
      --pairs-csv <PATH>          Write stem×URL conflict pairs CSV (cannibalization)
      --rescore <PATH>            Rebuild findings and actions from a saved audit JSON, no work
      --manifest <DIR>            Write run.json + ledger.json to this directory
      --no-jev                    Skip all Jev semantic calls (rules and local audit only)
      --jev-budget <USD>          Hard Jev spend cap in USD for this run [default: 0.25]
```

Optional `narrative.json` beside audit exports loads at report time: unknown action IDs fail closed; a missing file embeds an automatic evidence-only summary (`references/narrative.md`).

---