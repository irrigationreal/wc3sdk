# Locate your WC3 executable (runbook)

We do **not** ship Warcraft III binaries. The first step for any RE work is identifying the exact `*.exe` you’re targeting.

## Why this matters

Offsets, signatures, and structs are **build-specific**. We need a stable identity (hash/version) for the target exe.

## Quick search

From the repo root:

```bash
./scripts/find_wc3_exe.sh "/mnt/storage"
```

If you already know the install folder, point it directly:

```bash
./scripts/find_wc3_exe.sh "/path/to/Warcraft 3"
```

## Next steps after you find it

1. Hash it:
   - `./scripts/hash_file.sh "/path/to/wc3.exe"`
2. Create a target file in `docs/targets/`:
   - copy `docs/targets/template.md`
   - fill in hash/version/environment notes

