#!/bin/sh
# Windows and Linux builds share one codebase. Only src/platform/ may know
# which OS it is on: it picks one implementation and everything else calls
# platform::. A cfg or a Windows API call anywhere else fails this check.
set -eu

leaks=$(grep -rnE --include='*.rs' \
  'cfg\((not\()?(windows|unix|target_os|target_family)|windows::|std::os::windows' \
  src-tauri/src | grep -v '^src-tauri/src/platform/' || true)

if [ -n "$leaks" ]; then
  echo "OS-specific code outside src-tauri/src/platform/:"
  echo "$leaks"
  exit 1
fi
