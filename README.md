# wc3-cli

`wc3-cli` is a reverse-engineering project that aims to expose **Warcraft III** as a controllable runtime from a **local command-line interface**.

This repo intentionally does **not** include any Warcraft III binaries/assets. You must supply a local install.

## What this will become

- A CLI to query and control the game (units, buildings, UI, camera, resources, triggers, etc.).
- An injected in-process module (DLL) that safely exposes game state/actions.
- A versioned protocol so we can support multiple WC3 builds without “mystery offsets”.

## First milestone

1. Launch WC3 under Wine
2. Inject our DLL (or use a proxy DLL load path)
3. Automate menus enough to start a single-player game
4. Read basic state (player / selected unit)
5. Issue one safe command (e.g., move)

## Repo map

- `AGENTS.md`: project operating rules (read this first)
- `crates/`: Rust workspace crates (`cli`, `injector`, `protocol`, `shim`, `proxy-mss32`)
- `docs/`: runbooks + reverse-engineering notes
- `scripts/`: helper scripts (hashing, local setup)
- `resources/`: pointers to external reference repos (not vendored here)
- `local/` (ignored): your machine’s install paths, logs, captures, scratch

## Quick start (scaffold only)

This scaffold is meant to compile without external Rust dependencies.

```bash
cargo build
./target/debug/wc3 --help
```

Next: follow `docs/runbooks/` once you’ve identified your target WC3 executable.
