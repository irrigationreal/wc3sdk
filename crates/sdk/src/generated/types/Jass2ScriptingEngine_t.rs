// Generated types. Do not edit by hand.
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub mod Compile {
    pub mod __l2 {
        #[repr(C)]
        pub struct errctx_t {
            pub _opaque: [u8; 16],
        }
        pub const errctx_t__SIZE: usize = 16;
        pub const errctx_t__NAME: &str = "Jass2ScriptingEngine_t::Compile::__l2::errctx_t";
        pub const errctx_t__user_data__OFFSET: usize = 0;
        pub const errctx_t__error_handler__OFFSET: usize = 8;

    }

}

pub mod Load {
    pub mod __l2 {
        #[repr(C)]
        pub struct ctx_t {
            pub _opaque: [u8; 24],
        }
        pub const ctx_t__SIZE: usize = 24;
        pub const ctx_t__NAME: &str = "Jass2ScriptingEngine_t::Load::__l2::ctx_t";
        pub const ctx_t__globalCB__OFFSET: usize = 0;
        pub const ctx_t__handleCB__OFFSET: usize = 8;
        pub const ctx_t__opaqueData__OFFSET: usize = 16;

    }

}

