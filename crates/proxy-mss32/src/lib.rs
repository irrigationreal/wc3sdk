#![cfg_attr(windows, allow(non_snake_case))]

#[cfg(windows)]
mod win {
    use core::ffi::c_void;

    type BOOL = i32;
    type DWORD = u32;
    type HANDLE = *mut c_void;
    type HINSTANCE = *mut c_void;
    type LPVOID = *mut c_void;

    type LPTHREAD_START_ROUTINE = Option<unsafe extern "system" fn(LPVOID) -> DWORD>;

    const DLL_PROCESS_ATTACH: DWORD = 1;
    const TRUE: BOOL = 1;

    extern "system" {
        fn CreateThread(
            lpThreadAttributes: LPVOID,
            dwStackSize: usize,
            lpStartAddress: LPTHREAD_START_ROUTINE,
            lpParameter: LPVOID,
            dwCreationFlags: DWORD,
            lpThreadId: *mut DWORD,
        ) -> HANDLE;
        fn DisableThreadLibraryCalls(hLibModule: HINSTANCE) -> BOOL;
        fn LoadLibraryW(lpLibFileName: *const u16) -> HINSTANCE;
        fn OutputDebugStringA(lpOutputString: *const i8);
    }

    fn debug(msg: &'static [u8]) {
        // msg must be NUL-terminated.
        unsafe { OutputDebugStringA(msg.as_ptr().cast::<i8>()) };
    }

    unsafe extern "system" fn init_thread(_param: LPVOID) -> DWORD {
        debug(b"wc3-proxy-mss32: init thread start\0");

        // Best-effort: load the actual shim DLL if present next to the game.
        // Keep this optional and non-fatal so the game can still run with only forwarding.
        // Cargo/Rust will usually emit `wc3_shim.dll` (hyphens become underscores), but keep a fallback.
        const SHIM_UNDERSCORE: &[u16] = &[
            b'w' as u16,
            b'c' as u16,
            b'c' as u16,
            b'3' as u16,
            b'_' as u16,
            b's' as u16,
            b'h' as u16,
            b'i' as u16,
            b'm' as u16,
            b'.' as u16,
            b'd' as u16,
            b'l' as u16,
            b'l' as u16,
            0,
        ];
        const SHIM_HYPHEN: &[u16] = &[
            b'w' as u16,
            b'c' as u16,
            b'c' as u16,
            b'3' as u16,
            b'-' as u16,
            b's' as u16,
            b'h' as u16,
            b'i' as u16,
            b'm' as u16,
            b'.' as u16,
            b'd' as u16,
            b'l' as u16,
            b'l' as u16,
            0,
        ];

        let mut shim = LoadLibraryW(SHIM_UNDERSCORE.as_ptr());
        if shim.is_null() {
            debug(b"wc3-proxy-mss32: LoadLibraryW(wc3_shim.dll) failed, trying fallback\0");
            shim = LoadLibraryW(SHIM_HYPHEN.as_ptr());
        }

        if shim.is_null() {
            debug(b"wc3-proxy-mss32: shim load failed (wc3_shim.dll / wc3-shim.dll)\0");
        } else {
            debug(b"wc3-proxy-mss32: shim loaded\0");
        }

        0
    }

    #[no_mangle]
    pub unsafe extern "system" fn DllMain(hinst: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> BOOL {
        if reason == DLL_PROCESS_ATTACH {
            debug(b"wc3-proxy-mss32: DLL_PROCESS_ATTACH\0");
            let _ = DisableThreadLibraryCalls(hinst);
            let _ = CreateThread(core::ptr::null_mut(), 0, Some(init_thread), core::ptr::null_mut(), 0, core::ptr::null_mut());
        }
        TRUE
    }
}

// Allow `cargo build` on non-Windows hosts without requiring any Windows toolchain.
#[cfg(not(windows))]
pub fn proxy_not_built_on_this_platform() {}
