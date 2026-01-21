# AGENTS.md — wc3-cli

This repository is a reverse‑engineering + systems project to expose **Warcraft III** runtime state and controls through a **local CLI** by injecting code into the game process (initially: **Warcraft III under Wine on Linux**).

The goal of this file is to make work reproducible and to keep changes safe, legal, and maintainable.

## Mission

- Provide a CLI that can **observe** and **control** *anything* in the running game: units, buildings, UI/menus, camera, resources, triggers, etc.
- Support **live interaction with the game’s scripting layer** (e.g., JASS/Lua depending on target build) when feasible.
- Start with a practical first milestone:
  - Launch WC3 under Wine
  - Inject our in‑process module (DLL) reliably
  - Automate UI enough to start a single‑player game
  - Read minimal world state (player, selected unit)
  - Issue one safe command (e.g., move a unit)

## Non‑goals (for now)

- Online/Battle.net multiplayer interaction or automation.
- Anything that meaningfully enables cheating in competitive environments.
- Redistribution of Blizzard binaries/assets or derived disassembly/decompilation outputs.

## Guardrails (legal, ethics, safety)

- **No copyrighted artifacts in git**: do not commit Warcraft III executables, DLLs, MPQs/CASC data, maps, scripts, memory dumps, decompiled output, or large byte signatures that reconstruct code.
- **User supplies the game**: the repo should assume the user has a legal copy and points the tooling at their local install path.
- **Offline‑only default**: default behavior should target single‑player/offline. If any feature might apply online, it must be explicitly gated behind opt‑in flags and documented with warnings.
- **No anti‑cheat bypass work**: do not implement, document, or suggest bypasses. Avoid interacting with protected processes.
- **Minimize attack surface**: injected code should not open network listeners by default. Prefer local IPC only (named pipes / Unix sockets) and require explicit flags for any networking.

## Targeting & versioning (critical for RE projects)

Warcraft III changes across patches/builds. Every RE claim must be tied to a specific target.

When adding offsets, structs, or hooks:

- Always record the target’s **SHA‑256**, file version, and build metadata.
- Never “just hardcode an address” without:
  - a version guard (hash/version check), and
  - a validation step (sanity checks that fail closed).
- Prefer **signature/pattern scanning** or **export/RTTI/vtable discovery** over raw offsets when practical.
- Keep everything **versioned**. Expect multiple target profiles.

## Recommended repo layout (create as needed)

Keep proprietary inputs out of the repo and keep RE outputs human‑reviewable.

- `crates/` (or `src/`): product code
  - `injector/`: host-side injector (runs on Linux, launches/injects into Wine process)
  - `shim/`: in-process Windows module (DLL; runs inside wc3.exe under Wine)
  - `proxy-mss32/`: proxy DLL for `mss32.dll`-based injection (loads shim + forwards to original)
  - `protocol/`: shared IPC schema + message types
  - `cli/`: end-user CLI (talks to injector/shim)
  - `sdk/`: data-driven SDK (loads PDB/EXE-derived JSON to resolve symbols/layouts at runtime)
- `docs/`
  - `docs/targets/`: one file per supported WC3 build (hash/version + notes)
  - `docs/re/`: reverse-engineering notes (structures, systems, call flows)
  - `docs/runbooks/`: step-by-step “how to reproduce” guides (Wine prefix, launch, inject)
- `scripts/`: helper scripts (hashing, launching with Wine env, log collection)
- `local/`: **ignored**. User’s local WC3 install pointers, logs, captures, scratch data.

Add or keep a `.gitignore` entry for `local/`, `*.gpr` (Ghidra projects), memory dumps, and any game binaries.

## Documentation hygiene (keep the project navigable)

This repo should stay easy to pick up after weeks/months away. Treat docs as a first-class artifact of the RE process.

- **On big changes, update `AGENTS.md`**: if you introduce a new major workflow, crate, injection method, protocol/IPC
  shape, safety/guardrail policy, or target-selection strategy, reflect it here so the repo’s “operating rules” stay true.
- **Otherwise, prefer `docs/` for details**: keep long procedures and deep RE notes in `docs/runbooks/` and `docs/re/`.
  `AGENTS.md` should remain the concise index + policy surface.
- **Write RE notes continuously**: any new understanding (structures, call flows, invariants, signatures) should land in
  `docs/re/` with “what/how/validate/enables”. Update existing notes rather than scattering context across threads.
- **Record target identities**: when you add offsets/sigs/hooks for a build, update/add the corresponding file in
  `docs/targets/` (hash/version/env) so we can reproduce and validate later.

## Git workflow

- **Commit changes** when work is complete and tests pass.
- Include a `Co-authored-by` trailer using the latest requester info from
  `.codex-forum/requester.json`.

## Engineering approach (architecture)

High-level shape:

1. **CLI (host)**: user-facing commands (e.g., `wc3 unit list`, `wc3 unit move --id ...`).
2. **Injector (host)**: finds/launches `wc3.exe` under Wine and injects the shim DLL.
3. **Shim DLL (in-process)**: reads game state, performs actions, exposes a stable API over IPC.
4. **IPC protocol**: versioned request/response + events (telemetry, game ticks, unit updates).

Guidelines:

- Keep the **shim minimal**: do the least in-process; push parsing/formatting to the host when possible.
- Fail closed: if the target build is unknown or validations fail, **refuse to act** with a clear error.
- Prefer an **idempotent command model** (requests with correlation IDs) and structured errors.

## Reverse engineering workflow conventions

When you discover something:

- Write it down in `docs/re/` as:
  - “What it is”
  - “How we found it” (strings, xrefs, call stacks)
  - “How to validate it” (runtime checks / invariants)
  - “What it enables” (commands/features)
- If you need to refer to disassembly, do it in *summaries*, not pasted blocks of proprietary code.
- Keep signatures short and robust (mask volatile bytes). Avoid huge byte patterns.

If the work requires dynamic validation:

- Prefer safe read-only probes first (introspection, logging).
- Add “dry-run” modes for commands that would change state.
- Make state writes explicit and auditable (log what was attempted).

### Local RE skill pack

- Ghidra CLI workflow skill (persistent instance + MCP bridge) lives at
  `skills/ghidra-binary-reversing/`. Use it for headless imports, keeping Ghidra warm, and scripted decompilation.

### Ghidra MCP workflow (pyghidra-mcp + mcporter)

Use the local skill `skills/ghidra-binary-reversing/SKILL.md` for the full tool list. Repo-specific defaults:

- **Ghidra project path (gitignored):** `local/ghidra-projects/`
- **Target binary (staged, gitignored):** `local/wc3-install/Warcraft III.exe`

Bootstrap steps (headless, HTTP transport):

1. Stage the WC3 binary (already required for RE):
   - `scripts/stage_wc3_install.sh`
2. Start the MCP server:
   - `uvx pyghidra-mcp --transport streamable-http --project-path local/ghidra-projects`
3. Discover tools:
   - `npx mcporter list --http-url http://127.0.0.1:8000/mcp`
4. Import the WC3 binary (example; confirm tool name/schema via `mcporter list`):
   - `npx mcporter call --http-url http://127.0.0.1:8000/mcp pyghidra_mcp.import_program \
     '{"binary_path":"local/wc3-install/Warcraft III.exe","project_name":"wc3","program_name":"warcraft_iii"}'`
5. Inspect program info + symbols:
   - `npx mcporter call --http-url http://127.0.0.1:8000/mcp pyghidra_mcp.list_project_program_info \
     '{"project_name":"wc3"}'`
   - `npx mcporter call --http-url http://127.0.0.1:8000/mcp pyghidra_mcp.search_functions_by_name \
     '{"project_name":"wc3","query":"*"}'`

Notes:
- Keep Ghidra projects, caches, and any analysis artifacts under `local/` only.
- Never commit decompiled output or large byte signatures; summarize findings in `docs/re/`.

## Local target binaries (user-supplied; never commit)

For this workspace we assume the user has a legal install mounted at:

- `/mnt/storage/Warcraft 3/`

Do **not** commit anything from that folder. If you need a stable, reproducible path for tooling, copy only the
needed binaries into `local/` (which is gitignored) and work from there.

Recommended staging layout (gitignored):

- `local/wc3-install/Warcraft III.exe`
- `local/wc3-install/Mss32.dll` (if needed for proxy export inspection)

Staging helper:

- `scripts/stage_wc3_install.sh` copies binaries into `local/wc3-install/` and prints SHA-256 so it can be recorded
  in `docs/targets/`.

## Wine-specific guidance

Assume Linux host + Wine:

- Make `WINEPREFIX` explicit in scripts and docs.
- Keep a dedicated prefix per target build to avoid cross-contamination.
- Prefer windowed mode and deterministic settings for automation (resolution, language, UI scale).
- Capture environment needed for reproduction: Wine version, DXVK/vkd3d settings, GPU driver.

## First injection: `mss32.dll` proxy workflow

For the current observed target (`docs/targets/wc3-a1950f17.md`), `Warcraft III.exe` imports `mss32.dll` and the install
ships `Mss32.dll`. This makes **proxy-DLL injection** via `mss32.dll` a practical first milestone path.

Repo support:

- Runbook: `docs/runbooks/proxy-mss32.md`
- Proxy DLL crate: `crates/proxy-mss32/` (builds a forwarding `mss32.dll` that can optionally load our shim)
- Export/forwarder generation helper: `scripts/gen_proxy_def.py` (uses `objdump -p` on your local DLL)

Workflow summary (offline only):

1. Build the proxy for Windows (32-bit): set `WC3_MSS32_SOURCE_DLL` to the user-local `Mss32.dll` path and build
   `wc3-proxy-mss32` for a 32-bit Windows target (typically `i686-pc-windows-gnu`).
2. Install into the WC3 folder: rename `Mss32.dll` → `Mss32.original.dll`, copy the proxy into place as `Mss32.dll`.
3. Optional: place `wc3_shim.dll` next to it; the proxy will try to `LoadLibraryW` it to run our in-process code.
4. Verify load order under Wine with `WINEDEBUG=+loaddll` and (if needed) `WINEDLLOVERRIDES="mss32=n,b"`.

Notes:

- The proxy must forward **all** exports correctly; otherwise WC3 may crash at startup.
- Do not commit generated `.def` files or local DLL copies; keep them under `local/` (ignored by git).

## Coding conventions (until the codebase says otherwise)

If starting fresh, prefer **Rust** for new components (CLI/injector/protocol/shim), because it supports:

- `cdylib` builds for DLLs,
- strong typing for protocol + memory layouts,
- safer containment of `unsafe` in small modules.

Rules:

- Keep `unsafe` isolated; document invariants at the boundary.
- Wrap OS APIs/FFI behind small modules; avoid leaking raw pointers across layers.
- Use structured logging; keep logs machine-parseable (JSON) when feasible.
- Treat all in-process reads as fallible; never assume pointers are valid.

If introducing C/C++ (sometimes unavoidable for Windows ABI edges):

- Keep it minimal and contained behind a narrow FFI surface.
- Prefer explicit calling conventions and packed-struct annotations with tests.

## Testing & validation expectations

We can’t fully automate “does WC3 behave correctly” in CI, but we can still test a lot:

- Unit tests for:
  - protocol encoding/decoding
  - address/sig scanning logic (with synthetic fixtures)
  - data structure parsing from byte buffers
- “Smoke runbook” for humans:
  - install target build (local)
  - launch under Wine
  - inject shim
  - run `wc3 status`
  - start a skirmish
  - read one unit + issue one command

If you add a new hook/feature, add at least one:

- validation check (runtime invariant), and
- documented reproduction step in `docs/runbooks/`.

## Security posture

- Default to localhost-only IPC (no `0.0.0.0` binds).
- Never execute arbitrary code from the game; do not eval untrusted input in the shim.
- Treat any “scripting bridge” as high-risk: keep it opt-in, sandboxed where possible, and logged.

## What to do first when making progress

If you’re unsure where to start, prioritize in this order:

1. Add scaffolding for CLI + injector + shim + protocol.
2. Make injection reliable and observable (logs, handshake).
3. Implement one read-only capability (`game status`, `players`, `selected unit`).
4. Implement one safe write capability (e.g., “select unit”, “issue move”).
5. Expand coverage (units list, world queries, UI automation).
