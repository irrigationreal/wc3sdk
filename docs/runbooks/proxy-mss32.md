# Proxy injection via `mss32.dll` (runbook)

This is a practical “first injection” approach for targets where the game imports `mss32.dll` (Miles Sound System).

**Why it’s useful:** `Warcraft III.exe` (see `docs/targets/wc3-a1950f17.md`) imports `mss32.dll`, and the install folder
already ships a `Mss32.dll`. That makes it a common candidate for a proxy DLL that loads first and then forwards to the
real one.

## Big warning

- Do this **offline only** (single-player).
- Always keep a backup copy of the original DLL so you can restore the install.
- The proxy must forward exports correctly or the game may crash at startup.

## Concept

1. Rename the original `Mss32.dll` to something like `Mss32.original.dll`.
2. Place our built DLL in the folder as `Mss32.dll` (or `mss32.dll`).
3. Our proxy DLL:
   - logs “loaded”
   - exports the *same* `mss32.dll` API and forwards it to `Mss32.original.dll`
   - (optionally) loads `wc3_shim.dll` / `wc3-shim.dll` so we can run our own code in-process

## Implementation in this repo

- Export enumeration / .def generation (from your local `Mss32.dll`):
  - `scripts/gen_proxy_def.py`
- Proxy DLL crate:
  - `crates/proxy-mss32/`

## Wine considerations

Sometimes Wine prefers its builtin stubs. If you see the proxy not loading:

- Try forcing native first:

```bash
export WINEDLLOVERRIDES="mss32=n,b"
```

- Use Wine DLL load logging to verify what is actually being loaded:

```bash
export WINEDEBUG=+loaddll
```

## Step 1: Generate a forwarding .def (optional but useful)

This produces a human-readable `.def` file you can inspect (kept under `local/`, ignored by git):

```bash
./scripts/gen_proxy_def.py \
  --dll "/mnt/storage/Warcraft 3/Mss32.dll" \
  --forward-module "Mss32.original" \
  --library "mss32.dll" \
  --out "$PWD/local/generated/mss32.def"
```

## Step 2: Build the proxy DLL (requires Windows cross toolchain)

The proxy DLL build auto-generates its own `.def` at build time from `WC3_MSS32_SOURCE_DLL`.

Typical prerequisites on Linux:

- Rust target: `i686-pc-windows-gnu`
- MinGW cross linker (`i686-w64-mingw32-gcc`), depending on distro/package name

Example build (after toolchain install):

```bash
export WC3_MSS32_SOURCE_DLL="/mnt/storage/Warcraft 3/Mss32.dll"
cargo build -p wc3-proxy-mss32 --target i686-pc-windows-gnu
```

## Step 3: Install into the WC3 directory (offline only)

From `/mnt/storage/Warcraft 3/`:

1. Rename the original:
   - `Mss32.dll` → `Mss32.original.dll`
2. Copy the built proxy DLL into place as `Mss32.dll`.
3. (Optional) also place `wc3_shim.dll` next to it (the proxy will try to load it).

## Step 4: Verify it loads

Use Wine DLL load logging:

```bash
export WINEDEBUG=+loaddll
export WINEDLLOVERRIDES="mss32=n,b"
```

Then launch WC3 and confirm you see:

- `wc3-proxy-mss32: DLL_PROCESS_ATTACH`
