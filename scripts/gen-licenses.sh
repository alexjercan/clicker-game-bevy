#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

if ! cargo about --version >/dev/null 2>&1; then
    echo "error: cargo-about is required" >&2
    exit 1
fi

cargo about generate --locked --no-default-features --fail about.hbs -o credits/THIRD-PARTY-LICENSES.md
