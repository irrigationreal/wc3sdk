# First milestone smoke test (runbook)

This is the minimal “does the pipeline work at all?” checklist.

## Preconditions

- You have a local WC3 executable (not committed to git)
- You can run it under Wine

## Steps

1. Identify your target exe path and compute a hash:
   - `scripts/hash_file.sh "/path/to/wc3.exe"`
2. Create or select a Wine prefix:
   - see `docs/runbooks/wine-setup.md`
3. Launch WC3 in windowed mode (recommended) and verify it starts.
4. Build the shim for Windows (once toolchain is set up).
5. Load the shim into the process (proxy DLL or injection; method TBD per target).
6. Confirm we see a log/handshake signal (initially: debug output / log file).
7. Generate SDK config (local, gitignored) and set env for snapshotting:
   - `scripts/gen_sdk_config.py --dump-dir "local/pdb-dumps/Warcraft III" --exe "/mnt/storage/Warcraft III-Debug/Warcraft III.exe"`
   - `export WC3_SDK_CONFIG=local/sdk-config.json`
8. Use a main-thread hook or manual pump to execute queued actions (see `docs/re/main-thread.md`).

## Exit criteria

- WC3 launches reliably under Wine
- The shim is loaded in-process and prints a recognizable “I’m alive” signal
- SDK config is generated and ready for snapshot reads
