#!/usr/bin/env python3
import argparse
import json
import re
from pathlib import Path

DEFAULT_PATTERNS = [
    r"Game", r"World", r"Map", r"Player", r"Unit", r"Selection", r"Camera", r"UI", r"Session", r"State",
]


def main() -> None:
    ap = argparse.ArgumentParser(description="Search symbols/types for potential root objects")
    ap.add_argument("--symbols", required=True)
    ap.add_argument("--types", required=True)
    ap.add_argument("--pattern", action="append", help="Regex pattern (can be repeated)")
    ap.add_argument("--limit", type=int, default=50)
    args = ap.parse_args()

    patterns = args.pattern or DEFAULT_PATTERNS
    regex = re.compile("|".join(f"({p})" for p in patterns), re.IGNORECASE)

    symbols_path = Path(args.symbols)
    types_path = Path(args.types)

    symbols = json.loads(symbols_path.read_text())
    types = json.loads(types_path.read_text())

    symbol_hits = [s for s in symbols if regex.search(s.get("name", ""))]
    type_hits = [t for t in types if regex.search(t.get("name", ""))]

    print("== Symbol hits ==")
    for s in symbol_hits[: args.limit]:
        print(f"{s['name']}  rva={s.get('rva')} kind={s.get('kind')}")

    print("\n== Type hits ==")
    for t in type_hits[: args.limit]:
        print(f"{t['name']}  size={t.get('size')} fields={len(t.get('fields', []))}")

    print("\nPatterns:", patterns)
    print(f"Total symbol hits: {len(symbol_hits)}; type hits: {len(type_hits)}")


if __name__ == "__main__":
    main()
