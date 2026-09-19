#!/usr/bin/env bash
set -euo pipefail

mode=${1:?usage: check-release-contents.sh <native|web> <root> [executable]}
root=${2:?usage: check-release-contents.sh <native|web> <root> [executable]}

test -d "$root/assets"
test -f "$root/credits/CREDITS.md"
test -f "$root/credits/THIRD-PARTY-LICENSES.md"
test -f "$root/LICENSE"

case "$mode" in
    native)
        executable=${3:?native bundle needs an executable name}
        test -f "$root/$executable"
        ;;
    web)
        test -f "$root/index.html"
        find "$root" -maxdepth 1 -type f -name '*.wasm' -print -quit | grep -q .
        ;;
    *)
        echo "unknown release mode: $mode" >&2
        exit 2
        ;;
esac
