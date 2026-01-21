// Generated types. Do not edit by hand.
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

#[repr(C)]
pub struct ErrorInfo {
    pub _opaque: [u8; 24],
}
pub const ErrorInfo__SIZE: usize = 24;
pub const ErrorInfo__NAME: &str = "Lua::ErrorInfo";
pub const ErrorInfo__file__OFFSET: usize = 0;
pub const ErrorInfo__msg__OFFSET: usize = 8;
pub const ErrorInfo__line__OFFSET: usize = 16;

#[repr(C)]
pub struct VmShared {
    pub _opaque: [u8; 40],
}
pub const VmShared__SIZE: usize = 40;
pub const VmShared__NAME: &str = "Lua::VmShared";
pub const VmShared__functions__OFFSET: usize = 0;

#[repr(C)]
pub struct NativeFunc {
    pub _opaque: [u8; 136],
}
pub const NativeFunc__SIZE: usize = 136;
pub const NativeFunc__NAME: &str = "Lua::NativeFunc";
pub const NativeFunc__handler__OFFSET: usize = 0;
pub const NativeFunc__name__OFFSET: usize = 8;
pub const NativeFunc__args__OFFSET: usize = 48;
pub const NativeFunc__argsCount__OFFSET: usize = 72;
pub const NativeFunc__result__OFFSET: usize = 80;
pub const NativeFunc__resultCount__OFFSET: usize = 128;

#[repr(C)]
pub struct Vm {
    pub _opaque: [u8; 64],
}
pub const Vm__SIZE: usize = 64;
pub const Vm__NAME: &str = "Lua::Vm";
pub const Vm__L__OFFSET: usize = 0;
pub const Vm__id__OFFSET: usize = 8;
pub const Vm__seed__OFFSET: usize = 12;
pub const Vm__current__OFFSET: usize = 16;
pub const Vm__threadWantYields__OFFSET: usize = 24;
pub const Vm__shared__OFFSET: usize = 56;

#[repr(C)]
pub struct FileInput {
    pub _opaque: [u8; 24],
}
pub const FileInput__SIZE: usize = 24;
pub const FileInput__NAME: &str = "Lua::FileInput";
pub const FileInput__name__OFFSET: usize = 0;
pub const FileInput__buffer__OFFSET: usize = 8;
pub const FileInput__length__OFFSET: usize = 16;

#[repr(C)]
pub struct ErrorInfo__1 {
    pub _opaque: [u8; 24],
}
pub const ErrorInfo__1__SIZE: usize = 24;
pub const ErrorInfo__1__NAME: &str = "Lua::ErrorInfo";
pub const ErrorInfo__1__file__OFFSET: usize = 0;
pub const ErrorInfo__1__msg__OFFSET: usize = 8;
pub const ErrorInfo__1__line__OFFSET: usize = 16;

#[repr(C)]
pub struct Allocator_RCString_ {
    pub _opaque: [u8; 368],
}
pub const Allocator_RCString___SIZE: usize = 368;
pub const Allocator_RCString___NAME: &str = "Lua::Allocator<RCString>";
pub const Allocator_RCString___index__OFFSET: usize = 0;
pub const Allocator_RCString___data__OFFSET: usize = 8;

#[repr(C)]
pub struct Allocator_unreal_ {
    pub _opaque: [u8; 64],
}
pub const Allocator_unreal___SIZE: usize = 64;
pub const Allocator_unreal___NAME: &str = "Lua::Allocator<unreal>";
pub const Allocator_unreal___index__OFFSET: usize = 0;
pub const Allocator_unreal___data__OFFSET: usize = 4;

#[repr(C)]
pub struct CodePair {
    pub _opaque: [u8; 8],
}
pub const CodePair__SIZE: usize = 8;
pub const CodePair__NAME: &str = "Lua::CodePair";
pub const CodePair__index__OFFSET: usize = 0;
pub const CodePair__ref___OFFSET: usize = 4;

pub mod Allocator_Lua {
    #[repr(C)]
    pub struct CodePair_ {
        pub _opaque: [u8; 124],
    }
    pub const CodePair___SIZE: usize = 124;
    pub const CodePair___NAME: &str = "Lua::Allocator<Lua::CodePair>";
    pub const CodePair___index__OFFSET: usize = 0;
    pub const CodePair___data__OFFSET: usize = 4;

}

pub mod Debug {
    #[repr(C)]
    pub struct LogHandle {
        pub _opaque: [u8; 16],
    }
    pub const LogHandle__SIZE: usize = 16;
    pub const LogHandle__NAME: &str = "Lua::Debug::LogHandle";
    pub const LogHandle__log__OFFSET: usize = 0;
    pub const LogHandle__ref_count__OFFSET: usize = 8;

}

pub mod Handle {
    #[repr(C)]
    pub struct ReferenceChangedContext {
        pub _opaque: [u8; 16],
    }
    pub const ReferenceChangedContext__SIZE: usize = 16;
    pub const ReferenceChangedContext__NAME: &str = "Lua::Handle::ReferenceChangedContext";
    pub const ReferenceChangedContext__handle__OFFSET: usize = 0;
    pub const ReferenceChangedContext__user_data__OFFSET: usize = 8;

}

pub mod NativeFunc__mod {
    #[repr(C)]
    pub struct ExprData {
        pub _opaque: [u8; 48],
    }
    pub const ExprData__SIZE: usize = 48;
    pub const ExprData__NAME: &str = "Lua::NativeFunc::ExprData";
    pub const ExprData__type___OFFSET: usize = 0;
    pub const ExprData__meta__OFFSET: usize = 8;

}

pub mod Persist {
    #[repr(C)]
    pub struct Reader {
        pub _opaque: [u8; 1040],
    }
    pub const Reader__SIZE: usize = 1040;
    pub const Reader__NAME: &str = "Lua::Persist::Reader";
    pub const Reader__image__OFFSET: usize = 0;
    pub const Reader__size__OFFSET: usize = 8;
    pub const Reader__buffer__OFFSET: usize = 16;

    #[repr(C)]
    pub struct Writer {
        pub _opaque: [u8; 16],
    }
    pub const Writer__SIZE: usize = 16;
    pub const Writer__NAME: &str = "Lua::Persist::Writer";
    pub const Writer__image__OFFSET: usize = 0;
    pub const Writer__size__OFFSET: usize = 8;

}

pub mod Typed {
    #[repr(C)]
    pub struct GlobalChangedContext {
        pub _opaque: [u8; 16],
    }
    pub const GlobalChangedContext__SIZE: usize = 16;
    pub const GlobalChangedContext__NAME: &str = "Lua::Typed::GlobalChangedContext";
    pub const GlobalChangedContext__handle__OFFSET: usize = 0;
    pub const GlobalChangedContext__user_data__OFFSET: usize = 8;

}

