#!/bin/sh
# Frontend integrity check shared by the pre-commit hook. CI runs the same
# two commands as separate steps.
set -eu
npm run build
npm test
