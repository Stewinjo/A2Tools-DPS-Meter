#!/bin/sh
# Read or check the fork version: {upstream}-sj.{N} in the three version files.
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cargo_ver=$(grep -m1 '^version' "$root/src-tauri/Cargo.toml" | cut -d'"' -f2)
pkg_ver=$(node -p "require('$root/package.json').version")
tauri_ver=$(node -p "JSON.parse(require('fs').readFileSync('$root/src-tauri/tauri.conf.json','utf8')).version")

if [ "$cargo_ver" != "$pkg_ver" ] || [ "$cargo_ver" != "$tauri_ver" ]; then
  echo "version mismatch: Cargo.toml=$cargo_ver package.json=$pkg_ver tauri.conf.json=$tauri_ver" >&2
  exit 1
fi

if ! printf '%s' "$cargo_ver" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+-sj\.[0-9]+$'; then
  echo "fork version must look like 2.0.86-sj.1 (got $cargo_ver)" >&2
  exit 1
fi

printf '%s\n' "$cargo_ver"
