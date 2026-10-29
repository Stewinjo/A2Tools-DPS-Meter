#!/bin/sh
# Print the shared Linux build packages as a single space-separated line
# (for cache-apt-pkgs-action and similar).
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
grep -vE '^[[:space:]]*(#|$)' "$root/tools/linux-build-deps.txt" | tr '\n' ' ' | sed 's/[[:space:]]*$/\n/'
