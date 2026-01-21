# Data-driven SDK (PDB/EXE-backed)

This repo now includes a data-driven SDK (`crates/sdk/`) that loads:

- `symbols.json` (PDB public/data symbols + RVAs)
- `types_udt.json` (UDT layouts and field offsets)
- `types_enum.json` (enums)

These artifacts come from the local `pdb-dump` tool and should live under `local/` (gitignored).

## Goals

- Avoid hardcoding offsets in Rust.
- Keep layout drift manageable by loading offsets at runtime.
- Fail closed if the target build does not match the expected PDB/EXE.

## Runtime shape

- **SymbolMap** resolves symbol names to RVAs/VAs.
- **LayoutMap** provides field offsets and sizes.
- **SnapshotEngine** can read structured state for a declared root (symbol + type name).

## Required per-target config

You should store target info + root definitions in a JSON file referenced by host/shim.
A minimal example:

```json
{
  "target": {
    "exe_sha256": "<sha256>",
    "pdb_guid": "<guid>",
    "pdb_age": 2,
    "machine": "Amd64"
  },
  "symbols_path": "local/pdb-dumps/Warcraft III/symbols.json",
  "types_udt_path": "local/pdb-dumps/Warcraft III/types_udt.json",
  "types_enum_path": "local/pdb-dumps/Warcraft III/types_enum.json",
  "pointer_size": 8,
  "roots": [
    {
      "name": "game_state",
      "symbol": "<GLOBAL_SYMBOL_NAME>",
      "type_name": "<C++_TYPE_NAME>",
      "symbol_is_ptr": true
    }
  ]
}
```

Generate a template config from local dumps:

```
scripts/gen_sdk_config.py \
  --dump-dir "local/pdb-dumps/Warcraft III" \
  --exe "/mnt/storage/Warcraft III-Debug/Warcraft III.exe"
```

Find candidate root symbols/types:

```
scripts/find_sdk_roots.py \
  --symbols "local/pdb-dumps/Warcraft III/symbols.json" \
  --types "local/pdb-dumps/Warcraft III/types_udt.json"
```

Generate Rust bindings (opaque structs + field offsets + symbol RVAs):

```
scripts/gen_sdk_rust.py \
  --types "local/pdb-dumps/Warcraft III/types_udt.json" \
  --symbols "local/pdb-dumps/Warcraft III/symbols.json" \
  --out-dir "crates/sdk/sdk-gen"
```

Then build with (default loads `crates/sdk/sdk-gen`):

```
cargo build -p sdk
```

Override the generated directory:

```
WC3_SDK_GEN_DIR=/path/to/sdk-gen cargo build -p sdk
```

Notes:
- Identifiers are normalized to be valid Rust names.
- Collisions are disambiguated with `__N` suffixes.

## Using the SnapshotEngine

The snapshot engine reads a root object (symbol + type) and recursively reads fields.
It treats pointer fields as addresses and can optionally dereference them to nested objects
when the pointed-to type is known in the layout map.

## Field accessor (utility)

`sdk::Accessor` provides a direct way to read a named field from a type at a base address.
This is useful for targeted reads without building a full snapshot tree.

## Shim integration (current scaffold)

The shim includes a `sdk_bridge` that can load `WC3_SDK_CONFIG` and build snapshots
in-process. It expects:

- `WC3_SDK_CONFIG` to point at a JSON config (see above)
- matching `symbols.json` + `types_udt.json` paths in that config

This is wired for future IPC exposure but does not open any network listeners.

## Calling methods & main-thread scheduling (design notes)

Calling engine methods is target-specific and should be done **only** after validating:
- correct build identity (hash + PDB GUID/age)
- pointer sanity (module range, vtable range, alignment)

Recommended approach:
- **Background thread** handles IPC and queues “main-thread” actions.
- **Main-thread executor** is driven by a hook into a safe per-frame/tick function or by
  a window-proc hook that runs on the game thread.
- Actions are executed on the main thread, results marshaled back to the IPC layer.

This avoids race conditions and keeps dangerous calls on the thread WC3 expects.

See also: `docs/re/main-thread.md`.

## Next steps

- Identify a stable root symbol in the PDB (global) that points to the main game state.
- Add a root entry in the SDK config.
- Implement shim-side `MemoryReader` and wire snapshots into IPC responses.
- Add invariants to validate pointer ranges and vtable sanity before trusting data.
