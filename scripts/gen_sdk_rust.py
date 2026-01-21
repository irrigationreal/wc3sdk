#!/usr/bin/env python3
import argparse
import json
import re
from pathlib import Path

IDENT_RE = re.compile(r"[^A-Za-z0-9_]")
RUST_KEYWORDS = {
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "async",
    "await",
    "dyn",
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "try",
    "typeof",
    "unsized",
    "virtual",
    "yield",
}


def to_ident(name: str) -> str:
    name = name.replace("::", "__")
    name = IDENT_RE.sub("_", name)
    if not name:
        return "_"
    if name[0].isdigit():
        name = "_" + name
    if name in RUST_KEYWORDS:
        name = name + "_"
    return name


def unique_ident(base: str, counts: dict) -> str:
    if base not in counts:
        counts[base] = 0
        return base
    counts[base] += 1
    return f"{base}__{counts[base]}"


def load_filter(path: Path | None) -> dict:
    if path is None:
        default = Path("scripts/sdk_gen_filter.json")
        if default.exists():
            return json.loads(default.read_text())
        return {}
    return json.loads(path.read_text())


def match_any(name: str, patterns: list[str]) -> bool:
    for pat in patterns:
        if re.search(pat, name):
            return True
    return False


def should_include(name: str, include: list[str], exclude: list[str]) -> bool:
    if include and not match_any(name, include):
        return False
    if exclude and match_any(name, exclude):
        return False
    return True


def gen_types(types_path: Path, filt: dict) -> tuple[str, int]:
    types = json.loads(types_path.read_text())
    out = []
    out.append("// Generated opaque structs + field offsets. Do not edit by hand.\n")
    out.append("#![allow(non_camel_case_types)]\n")
    out.append("#![allow(non_upper_case_globals)]\n")
    out.append("#![allow(dead_code)]\n")

    seen = {}
    type_cfg = filt.get("types", {})
    include = type_cfg.get("include", [])
    exclude = type_cfg.get("exclude", [])
    min_size = int(type_cfg.get("min_size", 0))
    max_types = int(type_cfg.get("max_types", 0))
    emitted = 0
    for t in types:
        name = t.get("name", "")
        size = int(t.get("size", 0))
        if not name:
            continue
        if size < min_size:
            continue
        if not should_include(name, include, exclude):
            continue
        ident = unique_ident(to_ident(name), seen)
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
        emitted += 1
        if max_types and emitted >= max_types:
            break

    return "".join(out), emitted


def gen_symbols(symbols_path: Path, filt: dict) -> tuple[str, int]:
    symbols = json.loads(symbols_path.read_text())
    out = []
    out.append("// Generated symbols (name + RVA). Do not edit by hand.\n")
    out.append("#![allow(non_upper_case_globals)]\n")
    out.append("#![allow(dead_code)]\n")
    seen = {}
    sym_cfg = filt.get("symbols", {})
    include = sym_cfg.get("include", [])
    exclude = sym_cfg.get("exclude", [])
    kinds = set(sym_cfg.get("kinds", []))
    max_symbols = int(sym_cfg.get("max_symbols", 0))
    emitted = 0
    for s in symbols:
        name = s.get("name", "")
        rva = s.get("rva")
        if not name or rva is None:
            continue
        if kinds and s.get("kind") not in kinds:
            continue
        if not should_include(name, include, exclude):
            continue
        ident = unique_ident(to_ident(name), seen)
        out.append(f"pub const {ident}__RVA: u32 = {int(rva)};\n")
        out.append(f"pub const {ident}__NAME: &str = \"{name}\";\n")
        emitted += 1
        if max_symbols and emitted >= max_symbols:
            break
    return "".join(out), emitted


def main() -> None:
    ap = argparse.ArgumentParser(description="Generate Rust SDK bindings from PDB dumps")
    ap.add_argument("--types", required=True, help="types_udt.json")
    ap.add_argument("--symbols", required=True, help="symbols.json")
    ap.add_argument("--out-dir", default="crates/sdk/sdk-gen", help="output dir")
    ap.add_argument("--filter", default=None, help="filter json path (optional)")
    args = ap.parse_args()

    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    filt = load_filter(Path(args.filter) if args.filter else None)
    types_rs, type_count = gen_types(Path(args.types), filt)
    symbols_rs, sym_count = gen_symbols(Path(args.symbols), filt)

    # If filters were too strict, fall back to exclude-only for symbols.
    if sym_count == 0 and filt.get("symbols", {}).get("include"):
        fallback = dict(filt)
        fallback["symbols"] = dict(filt.get("symbols", {}))
        fallback["symbols"]["include"] = []
        symbols_rs, sym_count = gen_symbols(Path(args.symbols), fallback)

    (out_dir / "types.rs").write_text(types_rs)
    (out_dir / "symbols.rs").write_text(symbols_rs)
    print(f"Wrote {out_dir / 'types.rs'}")
    print(f"Wrote {out_dir / 'symbols.rs'}")


if __name__ == "__main__":
    main()
