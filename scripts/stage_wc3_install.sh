#!/usr/bin/env bash
set -euo pipefail

# Stages user-supplied Warcraft III binaries into gitignored local/ for reproducible tooling.
#
# Default source assumes a local mount (do not commit anything from it):
#   /mnt/storage/Warcraft 3/
#
# Destination is always gitignored:
#   local/wc3-install/
#
# Usage:
#   scripts/stage_wc3_install.sh
#   WC3_INSTALL_DIR="/path/to/WC3" scripts/stage_wc3_install.sh

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC_DIR="${WC3_INSTALL_DIR:-/mnt/storage/Warcraft 3}"
DST_DIR="${ROOT_DIR}/local/wc3-install"

EXE_NAME="Warcraft III.exe"
MSS_NAME="Mss32.dll"

mkdir -p "${DST_DIR}"

if [[ ! -f "${SRC_DIR}/${EXE_NAME}" ]]; then
  echo "error: missing '${SRC_DIR}/${EXE_NAME}'" >&2
  exit 1
fi

cp -a "${SRC_DIR}/${EXE_NAME}" "${DST_DIR}/${EXE_NAME}"
echo "staged: ${DST_DIR}/${EXE_NAME}"
sha256sum "${DST_DIR}/${EXE_NAME}"

if [[ -f "${SRC_DIR}/${MSS_NAME}" ]]; then
  cp -a "${SRC_DIR}/${MSS_NAME}" "${DST_DIR}/${MSS_NAME}"
  echo "staged: ${DST_DIR}/${MSS_NAME}"
  sha256sum "${DST_DIR}/${MSS_NAME}"
else
  echo "note: '${SRC_DIR}/${MSS_NAME}' not found; skipping"
fi

