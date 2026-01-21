#![cfg_attr(windows, allow(non_snake_case))]

#[cfg(windows)]
mod win {
    use core::ffi::c_void;

    #[path = "sdk_bridge.rs"]
    pub mod sdk_bridge;

    type BOOL = i32;
    type DWORD = u32;
    type HINSTANCE = *mut c_void;
    type LPVOID = *mut c_void;

    const DLL_PROCESS_ATTACH: DWORD = 1;
    const TRUE: BOOL = 1;

    extern "system" {
        fn OutputDebugStringA(lpOutputString: *const i8);
    }

    fn debug(msg: &'static [u8]) {
        // msg must be NUL-terminated.
        unsafe { OutputDebugStringA(msg.as_ptr().cast::<i8>()) };
    }

    #[no_mangle]
    pub unsafe extern "system" fn DllMain(_hinst: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> BOOL {
        if reason == DLL_PROCESS_ATTACH {
            debug(b"wc3-shim: DLL_PROCESS_ATTACH (scaffold)\0");
        }
        TRUE
    }
}

// Allow `cargo check` on non-Windows hosts without requiring any Windows toolchain.
#[cfg(not(windows))]
pub fn shim_not_built_on_this_platform() {
    // Intentionally empty.
}
