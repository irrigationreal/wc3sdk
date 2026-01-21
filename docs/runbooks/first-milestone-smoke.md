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

## Exit criteria

- WC3 launches reliably under Wine
- The shim is loaded in-process and prints a recognizable “I’m alive” signal

