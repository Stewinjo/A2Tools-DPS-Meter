#!/bin/sh
# The private build (desktop, without online) must not link the upload stack.
set -eu
cd src-tauri
tree=$(cargo tree --no-default-features --features desktop -e no-dev --prefix none)
for crate in reqwest tokio-tungstenite discord-rich-presence; do
  if printf '%s\n' "$tree" | grep -q "^${crate} "; then
    echo "desktop build links ${crate}"
    exit 1
  fi
done
