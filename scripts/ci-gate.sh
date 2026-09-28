#!/usr/bin/env bash
# Laya SEO CI gate (MYO-536): reuses jev-seo's existing gate machinery
# (cargo test, `audit --fail-on`/`--min-pass`) rather than inventing new
# thresholds, and adds the laya-bridge test suite for the Python half.
#
# Usage:
#   scripts/ci-gate.sh                 # rules-only gate, no Jev calls
#   LAYA_LOCAL_URL=http://127.0.0.1:8791 scripts/ci-gate.sh   # gate + Jev suite
#
# Exit code is the gate's: 0 = pass, nonzero = fail the build.
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "== cargo test (release) ==" >&2
# Unit tests are not hermetic against LAYA_LOCAL_URL/TYPESAFE_API_KEY -- a
# handful assume no Jev backend configured (keyless/offline paths). Run them
# in a clean env regardless of what the caller exported for the gate step
# below.
env -u LAYA_LOCAL_URL -u LAYA_LOCAL_API_KEY -u TYPESAFE_API_KEY cargo test --release

if [ -d laya-bridge/tests ]; then
    echo "== pytest laya-bridge ==" >&2
    ( cd laya-bridge && python3 -m pytest tests -q )
fi

echo "== jev-seo audit gate fixtures ==" >&2
no_jev_flag=()
if [ -z "${LAYA_LOCAL_URL:-}" ] && [ -z "${TYPESAFE_API_KEY:-}" ]; then
    echo "No LAYA_LOCAL_URL/TYPESAFE_API_KEY set: gating on deterministic rules only (--no-jev)." >&2
    no_jev_flag=(--no-jev)
fi

# Same blocking-rule contract as jev-seo upstream: 20 deterministic rules
# fail the build, judgment calls print as warnings only.
./target/release/jev-seo audit tests/fixtures/good --fail-on blocking --min-pass 80 "${no_jev_flag[@]}"

echo "CI gate: PASS" >&2
