# Injection methods (notes)

Early milestones require loading our in-process shim into Warcraft III under Wine.

There are multiple viable approaches. The “best” one depends on the target build and how it loads DLLs.

## Option A: Proxy DLL (recommended first attempt)

If the game imports a common Windows DLL (examples in other games: `dinput8.dll`, `version.dll`, `winmm.dll`),
we can place a **native** DLL with that name next to the game exe and configure Wine to prefer it.

Pros:

- Simple and reliable once you pick a DLL the game actually loads
- No remote thread injection required

Cons:

- Requires finding a DLL name the game imports
- If you need to forward exports to the real DLL, you must implement a proxy/forwarder correctly

## Option B: Windows injector (run under Wine)

Build a small Windows injector tool that:

1. finds the `wc3.exe` process
2. calls `OpenProcess` + `VirtualAllocEx` + `WriteProcessMemory`
3. calls `CreateRemoteThread(LoadLibraryW)`

Pros:

- Doesn’t depend on proxy DLL search order
- Works even if the game imports no “convenient” DLLs

Cons:

- More moving parts and more failure modes
- Some environments may block remote thread injection

## Option C: Wine debugging facilities

In some cases, Wine tools/debuggers can help with early experiments (observability, module loads),
but we generally want to own a deterministic injection method for automation.

## Practical guidance

- Start with **read-only** instrumentation first (prove “loaded and alive”).
- Keep DllMain minimal; do work after loader lock is released.
- Always gate behavior on the target build identity (hash/version) and fail closed.

## Target-specific note (current local build)

For the observed local target `docs/targets/wc3-a1950f17.md`, `Warcraft III.exe` imports `mss32.dll` and the install
already includes `Mss32.dll`. That often makes **`mss32.dll` a strong proxy-DLL injection candidate** because the loader
will prefer the adjacent DLL (and it’s already part of the game’s dependency graph).

See `docs/runbooks/proxy-mss32.md` and `crates/proxy-mss32/` for the concrete proxy-forwarding approach.
