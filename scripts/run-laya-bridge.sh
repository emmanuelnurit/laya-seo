#!/usr/bin/env bash
# Install (if needed) and start laya-bridge for local jev-seo audits.
#
# Usage:
#   scripts/run-laya-bridge.sh
#   LAYA_PORT=9000 scripts/run-laya-bridge.sh
#
# Once running, point jev-seo at it:
#   LAYA_LOCAL_URL=http://127.0.0.1:${LAYA_PORT:-8791} jev-seo audit <path>
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if ! python3 -c "import laya_bridge" >/dev/null 2>&1; then
    echo "Installing laya-bridge (and its laya[serve] dependency)..." >&2
    pip install --user -e "$repo_root/laya-bridge"
fi

export LAYA_HOST="${LAYA_HOST:-127.0.0.1}"
export LAYA_PORT="${LAYA_PORT:-8791}"
export LAYA_BRIDGE_LOG_LEVEL="${LAYA_BRIDGE_LOG_LEVEL:-INFO}"

echo "Starting laya-bridge on http://${LAYA_HOST}:${LAYA_PORT} (multilingual checkpoint)..." >&2
exec "$HOME/.local/bin/laya-bridge"
