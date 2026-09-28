# Limits and non-goals

## Limits and non-goals

- Large live crawls sample at `--max-pages` (default 50). The report says so via completeness notes.
- Heuristic rules (AI slop markers, em-dash density) are editorial conventions, not Google ranking factors.
- Jev thresholds are confidence-gated, not human-label-tuned. Decisive answers print; others print `[verify]` or drop.
- This does not measure traffic, revenue, or long-term rankings. Search Console (`gsc`) reports first-party clicks and positions you already own.
- No Electron or cloud web dashboard.
- No forced DataForSEO or other paid API on the free path.
- No generative prose synthesis. Jev is an evaluation oracle, not a writer.
- No heavy local browser farm. Crawls use lightweight HTTP fetches.

---