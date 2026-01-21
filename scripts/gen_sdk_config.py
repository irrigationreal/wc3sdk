#!/usr/bin/env python3
import argparse
import hashlib
import json
from pathlib import Path


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()


def main() -> None:
    ap = argparse.ArgumentParser(description="Generate SDK config from pdb-dump outputs")
    ap.add_argument("--dump-dir", required=True, help="Directory containing pdb_info.json/symbols.json/types_udt.json")
    ap.add_argument("--exe", required=True, help="Path to Warcraft III.exe")
    ap.add_argument("--out", default="local/sdk-config.json", help="Output config path")
    args = ap.parse_args()

    dump_dir = Path(args.dump_dir)
    pdb_info_path = dump_dir / "pdb_info.json"
    symbols_path = dump_dir / "symbols.json"
    types_udt_path = dump_dir / "types_udt.json"
    types_enum_path = dump_dir / "types_enum.json"

    if not pdb_info_path.exists():
        raise SystemExit(f"missing {pdb_info_path}")
    if not symbols_path.exists():
        raise SystemExit(f"missing {symbols_path}")
    if not types_udt_path.exists():
        raise SystemExit(f"missing {types_udt_path}")

    pdb_info = json.loads(pdb_info_path.read_text())
    symbols = json.loads(symbols_path.read_text())
    types = json.loads(types_udt_path.read_text())
    exe_path = Path(args.exe)
    exe_sha = sha256_file(exe_path)

    machine = pdb_info.get("machine_type")
    pointer_size = 8 if machine == "Amd64" else 4

    config = {
        "target": {
            "exe_sha256": exe_sha,
            "pdb_guid": pdb_info.get("pdb_guid"),
            "pdb_age": pdb_info.get("pdb_age"),
            "machine": machine,
        },
        "symbols_path": str(symbols_path),
        "types_udt_path": str(types_udt_path),
        "types_enum_path": str(types_enum_path if types_enum_path.exists() else ""),
        "pointer_size": pointer_size,
        "roots": [],
    }

    def has_symbol(name: str) -> bool:
        return any(s.get("name") == name for s in symbols)

    def has_type(name: str) -> bool:
        return any(t.get("name") == name for t in types)

    # Prefer the global game singleton if present.
    if has_symbol("CGameWar3::s_pGameWar3") and has_type("CGameWar3"):
        config["roots"].append(
            {
                "name": "game",
                "symbol": "CGameWar3::s_pGameWar3",
                "type_name": "CGameWar3",
                "symbol_is_ptr": True,
            }
        )

    out_path = Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(config, indent=2))
    print(f"Wrote {out_path}")


if __name__ == "__main__":
    main()
