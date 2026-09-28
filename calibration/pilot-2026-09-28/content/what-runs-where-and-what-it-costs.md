# What runs where, and what it costs

## What runs where, and what it costs

<img src="docs/assets/pipeline.svg" alt="jev-seo pipeline: local machine, DuckDuckGo free path, TypeSafe Jev capped, paid backends opt-in only" width="880">

| Part | Where it runs | Cost |
|---|---|---|
| 58-rule audit, local HTML/PDF/MD, CSV, pair matrix | Your machine | Free |
| Live crawl (robots, redirects, orphans, timing) | Your binary → target host | Free |
| Rank history, snapshots, `--diff` | Local SQLite `.jev-seo.db` | Free |
| Search Console pulls (`gsc`) | Google API with your OAuth client | Free |
| SERP autocomplete and scrape | DuckDuckGo | Free |
| Page/site/keyword/brief Jev judgments | TypeSafe System One | Capped by `--jev-budget` (default $0.25/run) |
| Tavily search or extract | Tavily (`TAVILY_API_KEY`) | Opt-in only; free path never calls it |
| DataForSEO live SERP | `DATAFORSEO_USERNAME` + `DATAFORSEO_PASSWORD` | Opt-in only via `query --provider dfs`; free fallback on error |
| Jina / Firecrawl body upgrades | Respective APIs | Opt-in; `--max-credits 0` parks spend |
| DataForSEO | Not required | Non-goal on the free path |

Keys come from the environment: `TYPESAFE_API_KEY` for Jev, `TAVILY_*` / `JINA_API_KEY` / `FIRECRAWL_*` for paid upgrades, `GOOGLE_CLIENT_ID` + `GOOGLE_CLIENT_SECRET` for `gsc`. Missing optional keys degrade to local-only sections; they do not fail the run.

---