#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
vendor_dir="${root_dir}/resources/vendor"

mkdir -p "${vendor_dir}"

clone_or_update() {
  local url="$1"
  local name="$2"
  local dest="${vendor_dir}/${name}"

  if [[ -d "${dest}/.git" ]]; then
    echo "Updating ${name}..."
    git -C "${dest}" pull --ff-only
    return
  fi

  echo "Cloning ${name}..."
  git clone --depth 1 "${url}" "${dest}"
}

clone_or_update "https://github.com/TinkerWorX/warcraftIII.git" "warcraftIII"
clone_or_update "https://github.com/Still4/War3Trainer.git" "War3Trainer"

echo "Done. Local repos are in ${vendor_dir} (ignored by git)."

