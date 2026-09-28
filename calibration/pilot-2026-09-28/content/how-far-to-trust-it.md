# How far to trust it

## How far to trust it

Measured 2026-09-22. Protocol and re-run commands: [`docs/EVAL.md`](docs/EVAL.md).

<img src="docs/assets/trust-strip.png" alt="Measured trust strip for jev-seo" width="880">

| Check | Result | Rerun |
|---|---|---|
| Cold start (`--help`) | 6ms | `time jev-seo --help` |
| Local audit, 4 files | ~0.2s | `jev-seo audit docs/ --json` |
| Live crawl, 6-10 pages | 3-11s wall (server-bound) | `jev-seo crawl <url> --max-pages 10` |
| Score repeatability, 3 crawls | Score 99 stable; findings ±2 from timing rules only | `jev-seo crawl <url> --json` ×3 |
| Test suite | 132 green, clippy `-D warnings` clean | `cargo test` |
| Budget guard | `--jev-budget 0` → zero Jev POSTs | `jev-seo audit docs/ --jev-budget 0` |
| Citation gate | Report text citing unknown `RULE-Rxx` refuses to write | Covered in unit tests |

Scores rank work. They do not predict rankings or traffic. Confidence gates print facts only when Jev is decisive; otherwise you get `[verify]` or a withheld number.

---