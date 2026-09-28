# Quickstart

## Quickstart

```bash
# Published crate (v0.1.2)
cargo install jev-seo

# Optional: TypeSafe key for Jev semantic scores
export TYPESAFE_API_KEY=your_key_here

# Competitive SERP radar
jev-seo query "offline expense tracker android"

# 58-rule audit on docs
jev-seo audit docs/

# Live crawl with health score and ranked actions
jev-seo crawl https://example.com --max-pages 10

# Exports agents and spreadsheets read
jev-seo audit docs/ --html out/report.html --actions-csv out/actions.csv --pairs-csv out/pairs.csv

# CI gate
jev-seo audit docs/ --min-pass 40

# Agent server
jev-seo mcp
```

*If jev-seo saved you a dashboard seat, star the repo so you can find it later.*

### Install

| Lane | Command |
|---|---|
| Cargo (published) | `cargo install jev-seo` |
| Build from source (What's new before release) | `git clone … && cargo install --path .` |
| Linux x86_64 / ARM64 | `jev-seo-<target>.tar.gz` from [Releases](https://github.com/AkashPriyadarshii/jev-seo/releases) |
| macOS Intel / ARM | Same Releases page, `tar.gz` with SHA256 |
| Windows x86_64 | Same Releases page, `.zip` |

MCP config for any client:

```json
{
  "mcpServers": {
    "jev-seo": { "command": "jev-seo", "args": ["mcp"] }
  }
}
```

---