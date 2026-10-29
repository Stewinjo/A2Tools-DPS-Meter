#!/bin/sh
# Install the Ubuntu packages listed in tools/linux-build-deps.txt.
# Used by the Dockerfile (as root) and by Linux CI/package workflows (with sudo).
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
list="$root/tools/linux-build-deps.txt"
packages=$(grep -vE '^[[:space:]]*(#|$)' "$list" | tr '\n' ' ')
if [ -z "$packages" ]; then
  echo "no packages in $list" >&2
  exit 1
fi
if [ "$(id -u)" -eq 0 ]; then
  apt-get update
  # shellcheck disable=SC2086
  apt-get install -y --no-install-recommends $packages
else
  sudo apt-get update
  # shellcheck disable=SC2086
  sudo apt-get install -y --no-install-recommends $packages
fi
