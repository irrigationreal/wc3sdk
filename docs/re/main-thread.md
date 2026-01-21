# Main-thread execution strategy

Most WC3 engine calls are not thread-safe. The shim should schedule work on
**the game’s main thread**.

## Options to drive a main-thread executor

1) **Hook a per-frame/tick function**
   - Find a stable, frequently-called function using symbols + sig scanning.
   - Install a small detour that calls `executor.pump()` and then chains to the original.

2) **Hook the window procedure**
   - Locate the WC3 window handle.
   - Replace the WndProc with a shim proc that pumps the queue and then calls the original.

3) **Render-present hook** (DX9/11/12, if used)
   - Can be stable across builds but requires careful ABI handling.

## Safety rules

- Keep detours tiny and re-entrant-safe.
- Use a lock-free or minimal-lock queue to avoid deadlocks.
- If the queue grows large, drop or coalesce requests.
- Validate pointers and target build identity before executing any action.

## Recommended execution model

- **IPC thread** (or background thread) receives requests and enqueues actions.
- **Main-thread pump** runs every tick and executes queued actions.
- Results are captured into a response buffer and sent back via IPC.

## Shim scaffold

The shim includes a simple queue in `crates/shim/src/main_thread.rs` with:
- `enqueue(task)` to queue work from background threads
- `pump(max_tasks)` to execute on the game thread (call from your hook)

## Fail-closed behavior

If a build mismatch is detected or required symbols are missing:
- refuse to install hooks
- refuse to execute actions
- return clear errors to the host
