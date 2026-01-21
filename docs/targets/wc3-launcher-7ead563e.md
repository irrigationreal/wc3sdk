# Target: Warcraft III Launcher.exe (7ead563e…)

## Identity

- File name: `Warcraft III Launcher.exe`
- SHA-256: `7ead563e6d4334b031333608ba5d989e52114264055941bc550de948adfbf918`
- File type: PE32 (x86 / 32-bit)
- PE header timestamp: `Thu Apr 12 15:37:40 2018` (from `objdump -x`)
- File version: (not captured yet)

## Location (local only)

- Example path: `/mnt/storage/Warcraft 3/Warcraft III Launcher.exe`

## Notes

This may be a convenience launcher/wrapper. We may still need to target the actual game executable for
hooks/state/control.

### Imported DLLs (FYI)

From `objdump -p`, this binary imports (unique):

- `ADVAPI32.dll`
- `GDI32.dll`
- `KERNEL32.dll`
- `MSIMG32.dll`
- `OLEAUT32.dll`
- `RPCRT4.dll`
- `SHELL32.dll`
- `USER32.dll`
- `VERSION.dll`
- `WINHTTP.dll`
- `WININET.dll`
- `WLDAP32.dll`
- `WS2_32.dll`
- `ole32.dll`
