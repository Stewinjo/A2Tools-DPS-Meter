#!/bin/sh
# Write bundle.windows.wix.version in tauri.conf.json for the MSI bundler.
# Usage: stamp-wix-version.sh [fork-version]
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
ver=${1:-$(sh "$root/tools/fork-version.sh")}
msi=$(sh "$root/tools/msi-version.sh" "$ver")
conf="$root/src-tauri/tauri.conf.json"
node -e "
const fs = require('fs');
const path = process.argv[1];
const msi = process.argv[2];
const conf = JSON.parse(fs.readFileSync(path, 'utf8'));
conf.bundle = conf.bundle || {};
conf.bundle.windows = conf.bundle.windows || {};
conf.bundle.windows.wix = conf.bundle.windows.wix || {};
conf.bundle.windows.wix.version = msi;
fs.writeFileSync(path, JSON.stringify(conf, null, 2) + '\n');
console.log('wix.version=' + msi);
" "$conf" "$msi"
