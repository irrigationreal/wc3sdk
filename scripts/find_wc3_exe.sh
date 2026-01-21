#!/usr/bin/env bash
set -euo pipefail

search_root="${1:-/mnt/storage}"

if [[ ! -d "${search_root}" ]]; then
  echo "Not a directory: ${search_root}" >&2
  exit 2
fi

echo "Searching for Warcraft III executables under: ${search_root}"
echo

find "${search_root}" -type f \( \
  -iname "war3.exe" -o \
  -iname "warcraft iii.exe" -o \
  -iname "*warcraft*iii*.exe" -o \
  -iname "*frozen*throne*.exe" -o \
  -iname "*war3*.exe" \
\) 2>/dev/null | sort -u

