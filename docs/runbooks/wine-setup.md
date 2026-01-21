# Wine setup (runbook)

This project assumes **Linux host + Wine** for early milestones.

## Goals

- Keep the Wine environment reproducible
- Avoid cross-contamination between target builds
- Make debugging and logging easy

## Suggested approach

1. Use one Wine prefix per WC3 build (or at least per major patch line).
2. Prefer windowed mode and fixed settings (resolution/language) for deterministic UI automation.
3. Record:
   - Wine version
   - GPU driver + DXVK/vkd3d settings (if used)
   - WC3 executable hash + file version

## Minimal commands (example)

Create a prefix:

```bash
export WINEPREFIX="$PWD/local/wineprefixes/wc3"
wineboot -u
```

Run an executable:

```bash
export WINEPREFIX="$PWD/local/wineprefixes/wc3"
wine "/path/to/wc3.exe"
```

Notes:

- Keep anything under `local/` out of git.
- If you use `winetricks`, document exactly what you installed in a runbook.

