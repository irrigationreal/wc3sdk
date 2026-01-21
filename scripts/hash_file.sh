#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 /path/to/file" >&2
  exit 2
fi

file="$1"
if [[ ! -f "$file" ]]; then
  echo "Not a file: $file" >&2
  exit 2
fi

sha256sum "$file"

