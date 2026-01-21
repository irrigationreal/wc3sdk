use anyhow::Result;

#[derive(Clone, Copy, Debug)]
pub enum Abi {
    Cdecl,
    Stdcall,
    Thiscall,
    Fastcall,
}

#[derive(Clone, Debug)]
pub enum Value {
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Ptr(u64),
}

#[derive(Clone, Debug)]
pub struct CallSpec {
    pub address: u64,
    pub abi: Abi,
    pub ret: ReturnType,
    pub args: Vec<Value>,
}

#[derive(Clone, Debug)]
pub enum CallResult {
    Void,
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Ptr(u64),
}

#[derive(Clone, Copy, Debug)]
pub enum ReturnType {
    Void,
    U32,
    U64,
    F32,
    F64,
    Ptr,
}

pub trait CallInvoker {
    fn call(&self, spec: &CallSpec) -> Result<CallResult>;
}
