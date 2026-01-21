#!/usr/bin/env python3
import argparse
import json
import re
from pathlib import Path

IDENT_RE = re.compile(r"[^A-Za-z0-9_]")


def to_ident(name: str) -> str:
    name = name.replace("::", "__")
    name = IDENT_RE.sub("_", name)
    if not name:
        return "_"
    if name[0].isdigit():
        name = "_" + name
    return name


def gen_types(types_path: Path) -> str:
    types = json.loads(types_path.read_text())
    out = []
    out.append("// Generated opaque structs + field offsets. Do not edit by hand.\n")
    out.append("#![allow(non_camel_case_types)]\n")
    out.append("#![allow(non_upper_case_globals)]\n")
    out.append("#![allow(dead_code)]\n")

    for t in types:
        name = t.get("name", "")
        size = int(t.get("size", 0))
        if not name:
            continue
        ident = to_ident(name)
        out.append(f"#[repr(C)]\npub struct {ident} {{\n    pub _opaque: [u8; {size}],\n}}\n")
        out.append(f"pub const {ident}__SIZE: usize = {size};\n")
        for f in t.get("fields", []):
            fname = f.get("name", "")
            offset = int(f.get("offset", 0))
            if not fname:
                continue
            fident = to_ident(fname)
            out.append(f"pub const {ident}__{fident}__OFFSET: usize = {offset};\n")
        out.append("\n")

    return "".join(out)


def gen_symbols(symbols_path: Path) -> str:
    symbols = json.loads(symbols_path.read_text())
    out = []
    out.append("// Generated symbols (name + RVA). Do not edit by hand.\n")
    out.append("#![allow(non_upper_case_globals)]\n")
    out.append("#![allow(dead_code)]\n")
    for s in symbols:
        name = s.get("name", "")
        rva = s.get("rva")
        if not name or rva is None:
            continue
        ident = to_ident(name)
        out.append(f"pub const {ident}__RVA: u32 = {int(rva)};\n")
        out.append(f"pub const {ident}__NAME: &str = \"{name}\";\n")
    return "".join(out)


def main() -> None:
    ap = argparse.ArgumentParser(description="Generate Rust SDK bindings from PDB dumps")
    ap.add_argument("--types", required=True, help="types_udt.json")
    ap.add_argument("--symbols", required=True, help="symbols.json")
    ap.add_argument("--out-dir", default="local/sdk-gen", help="output dir")
    args = ap.parse_args()

    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    types_rs = gen_types(Path(args.types))
    symbols_rs = gen_symbols(Path(args.symbols))

    (out_dir / "types.rs").write_text(types_rs)
    (out_dir / "symbols.rs").write_text(symbols_rs)
    print(f"Wrote {out_dir / 'types.rs'}")
    print(f"Wrote {out_dir / 'symbols.rs'}")


if __name__ == "__main__":
    main()
