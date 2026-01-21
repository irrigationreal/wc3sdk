#!/usr/bin/env python3
import argparse
import json
import re
from dataclasses import dataclass, field
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


@dataclass
class Node:
    items: list = field(default_factory=list)
    children: dict = field(default_factory=dict)


def add_item(root: Node, parts: list[str], item):
    if len(parts) <= 1:
        root.items.append(item)
        return
    node = root
    for seg in parts[:-1]:
        node = node.children.setdefault(seg, Node())
    node.items.append(item)


def render_node(node: Node, kind: str, indent: int = 0) -> str:
    pad = " " * indent
    out = []

    # Emit items in this module.
    name_counts = {}
    item_names = set()
    for item in node.items:
        leaf = item["leaf"]
        ident = unique_ident(to_ident(leaf), name_counts)
        item_names.add(ident)
        full_name = item["name"]
        if kind == "types":
            size = int(item.get("size", 0))
            out.append(f"{pad}#[repr(C)]\n")
            out.append(f"{pad}pub struct {ident} {{\n{pad}    pub _opaque: [u8; {size}],\n{pad}}}\n")
            out.append(f"{pad}pub const {ident}__SIZE: usize = {size};\n")
            out.append(f"{pad}pub const {ident}__NAME: &str = \"{full_name}\";\n")
            for f in item.get("fields", []):
                fname = f.get("name", "")
                offset = int(f.get("offset", 0))
                if not fname:
                    continue
                fident = to_ident(fname)
                out.append(f"{pad}pub const {ident}__{fident}__OFFSET: usize = {offset};\n")
            out.append("\n")
        else:
            rva = int(item["rva"])
            out.append(
                f"{pad}pub const {ident}: crate::symbols::SymbolRef = "
                f"crate::symbols::SymbolRef::new(\"{full_name}\", {rva});\n\n"
            )

    # Emit submodules.
    mod_counts = {}
    for raw, child in sorted(node.children.items()):
        mod_ident = unique_ident(to_ident(raw), mod_counts)
        if mod_ident in item_names:
            mod_ident = unique_ident(f"{mod_ident}__mod", mod_counts)
        out.append(f"{pad}pub mod {mod_ident} {{\n")
        out.append(render_node(child, kind, indent + 4))
        out.append(f"{pad}}}\n\n")

    return "".join(out)


def gen_types(types_path: Path, filt: dict) -> tuple[Node, int]:
    types = json.loads(types_path.read_text())
    type_cfg = filt.get("types", {})
    include = type_cfg.get("include", [])
    exclude = type_cfg.get("exclude", [])
    min_size = int(type_cfg.get("min_size", 0))
    max_types = int(type_cfg.get("max_types", 0))

    root = Node()
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
        parts = name.split("::")
        add_item(root, parts, {"name": name, "leaf": parts[-1], "size": size, "fields": t.get("fields", [])})
        emitted += 1
        if max_types and emitted >= max_types:
            break
    return root, emitted


def gen_symbols(symbols_path: Path, filt: dict) -> tuple[Node, int]:
    symbols = json.loads(symbols_path.read_text())
    sym_cfg = filt.get("symbols", {})
    include = sym_cfg.get("include", [])
    exclude = sym_cfg.get("exclude", [])
    kinds = set(sym_cfg.get("kinds", []))
    max_symbols = int(sym_cfg.get("max_symbols", 0))

    root = Node()
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
        parts = name.split("::")
        add_item(root, parts, {"name": name, "leaf": parts[-1], "rva": rva})
        emitted += 1
        if max_symbols and emitted >= max_symbols:
            break
    return root, emitted


def write_module_tree(out_dir: Path, types_root: Node, symbols_root: Node):
    gen_dir = out_dir
    types_dir = gen_dir / "types"
    symbols_dir = gen_dir / "symbols"
    types_dir.mkdir(parents=True, exist_ok=True)
    symbols_dir.mkdir(parents=True, exist_ok=True)

    (gen_dir / "mod.rs").write_text("pub mod types;\npub mod symbols;\n")

    def write_modules(root: Node, kind: str, base_dir: Path):
        header = [
            f"// Generated {kind}. Do not edit by hand.\n",
            "#![allow(non_camel_case_types)]\n" if kind == "types" else "",
            "#![allow(non_upper_case_globals)]\n",
            "#![allow(dead_code)]\n\n",
        ]

        mod_counts = {}
        mod_lines = []

        # Global items
        if root.items:
            global_path = base_dir / "global.rs"
            global_body = "".join(header) + render_node(Node(items=root.items, children={}), kind, 0)
            global_path.write_text(global_body)
            mod_lines.append("pub mod global;\n")

        for raw, child in sorted(root.children.items()):
            mod_ident = unique_ident(to_ident(raw), mod_counts)
            file_path = base_dir / f"{mod_ident}.rs"
            body = "".join(header) + render_node(child, kind, 0)
            file_path.write_text(body)
            mod_lines.append(f"pub mod {mod_ident};\n")

        header = "// Generated modules. Do not edit by hand.\n#![allow(non_snake_case)]\n\n"
        body = "".join(mod_lines) if mod_lines else "// empty\n"
        (base_dir / "mod.rs").write_text(header + body)

    write_modules(types_root, "types", types_dir)
    write_modules(symbols_root, "symbols", symbols_dir)


def main() -> None:
    ap = argparse.ArgumentParser(description="Generate Rust SDK bindings from PDB dumps")
    ap.add_argument("--types", required=True, help="types_udt.json")
    ap.add_argument("--symbols", required=True, help="symbols.json")
    ap.add_argument("--out-dir", default="crates/sdk/src/generated", help="output dir")
    ap.add_argument("--filter", default=None, help="filter json path (optional)")
    args = ap.parse_args()

    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)

    filt = load_filter(Path(args.filter) if args.filter else None)
    types_root, type_count = gen_types(Path(args.types), filt)
    symbols_root, sym_count = gen_symbols(Path(args.symbols), filt)

    if sym_count == 0 and filt.get("symbols", {}).get("include"):
        fallback = dict(filt)
        fallback["symbols"] = dict(filt.get("symbols", {}))
        fallback["symbols"]["include"] = []
        symbols_root, sym_count = gen_symbols(Path(args.symbols), fallback)

    write_module_tree(out_dir, types_root, symbols_root)
    print(f"Wrote {out_dir / 'types'}")
    print(f"Wrote {out_dir / 'symbols'}")


if __name__ == "__main__":
    main()
