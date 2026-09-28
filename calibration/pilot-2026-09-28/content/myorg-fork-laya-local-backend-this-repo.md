# MyOrg fork: `laya-local` backend (this repo)

## MyOrg fork: `laya-local` backend (this repo)

This is MyOrg's internal fork of `jev-seo`. It adds a third Jev backend, `laya-local`, that routes semantic scoring through a self-hosted bridge (`laya-bridge/`) over [Laya](https://github.com/NandhaKishorM/laya) (Apache-2.0) instead of the paid TypeSafe API — no API key, no cloud spend, everything stays on the local machine. `TYPESAFE_API_KEY` and `--no-jev` still work exactly as before and remain the fallback when `LAYA_LOCAL_URL` is unset.

Two terminals, no accounts, no keys:

```bash
# Terminal 1 — start the bridge (installs laya-bridge on first run,
# loads Laya's multilingual checkpoint; the first request after a cold
# start pays the model-load latency)
scripts/run-laya-bridge.sh

# Terminal 2 — audit anything with the bridge as the Jev backend
cargo build --release
LAYA_LOCAL_URL=http://127.0.0.1:8791 ./target/release/jev-seo audit <path> --json
```

Env vars: `LAYA_HOST` / `LAYA_PORT` (default `127.0.0.1:8791`), `LAYA_BRIDGE_LOG_LEVEL` (default `INFO`), `LAYA_LOCAL_API_KEY` (optional, only if you put the bridge behind auth). The bridge always forces the `multilingual` checkpoint — Laya's English checkpoint can hallucinate confidently on non-English/non-Latin content ("script blindness"), so this fork never lets a caller pick English instead.

Known limitation: the injection-risk pre-filter on `laya-bridge` is not yet calibrated on MyOrg data and can false-positive on ordinary content (seen at `noul` 0.95-0.98 on ordinary test fixtures), which makes Jev back off to "unsure" and fall through to rules-only checks. Tracked separately in the calibration follow-up issue; the audit still completes and produces a full report, just without the semantic upgrade on affected pages.

One command proves the pitch:

```console
$ jev-seo explain R19
R19 | content | Medium | advisory (warning only) | effort 2 (about a day)
  title: AI slop markers
  fix:   Rewrite flagged boilerplate in plain words.
```

```
$ jev-seo audit docs/ --min-pass 40 && echo gate_ok
gate_ok
```

---