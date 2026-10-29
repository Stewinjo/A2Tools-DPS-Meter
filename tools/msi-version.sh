#!/bin/sh
# Map the fork version {upstream}-sj.{N} to a WiX-safe major.minor.patch.build.
# Tauri's MSI bundler only accepts a numeric pre-release/build id (≤ 65535);
# "sj.1" is rejected. Keep the GitHub tag as v{upstream}-sj.{N}.
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
ver=${1:-$(sh "$root/tools/fork-version.sh")}
msi=$(printf '%s' "$ver" | sed -E 's/^([0-9]+\.[0-9]+\.[0-9]+)-sj\.([0-9]+)$/\1.\2/')
if [ "$msi" = "$ver" ] || ! printf '%s' "$msi" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$'; then
  echo "cannot map fork version to MSI version (got $ver)" >&2
  exit 1
fi
printf '%s\n' "$msi"
