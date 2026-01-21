#![cfg(windows)]

use anyhow::{anyhow, Result};
use sdk::call::{CallInvoker, CallResult, CallSpec, ReturnType, Value};

use crate::main_thread;

pub struct DirectCallInvoker;

pub struct MainThreadCallInvoker;

impl CallInvoker for DirectCallInvoker {
    fn call(&self, spec: &CallSpec) -> Result<CallResult> {
        call_on_current_thread(spec)
    }
}

impl CallInvoker for MainThreadCallInvoker {
    fn call(&self, spec: &CallSpec) -> Result<CallResult> {
        if main_thread::is_main_thread() {
            return call_on_current_thread(spec);
        }

        let spec = spec.clone();
        main_thread::run_on_main(move || call_on_current_thread(&spec))
    }
}

fn call_on_current_thread(spec: &CallSpec) -> Result<CallResult> {
    if !spec.args.iter().all(|v| matches!(v, Value::U32(_) | Value::U64(_) | Value::Ptr(_))) {
        return Err(anyhow!("float arguments are not supported yet"));
    }

    #[cfg(target_arch = "x86_64")]
    {
        let args = encode_int_args(&spec.args)?;
        let raw = unsafe { call_x64_int(spec.address, &args)? };
        return decode_return(spec.ret, raw);
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = spec;
        return Err(anyhow!("call invoker not implemented for this architecture"));
    }
}

fn encode_int_args(args: &[Value]) -> Result<Vec<u64>> {
    if args.len() > 4 {
        return Err(anyhow!("only up to 4 arguments are supported for now"));
    }
    Ok(args
        .iter()
        .map(|v| match v {
            Value::U32(v) => *v as u64,
            Value::U64(v) => *v,
            Value::Ptr(v) => *v,
            Value::F32(_) | Value::F64(_) => 0,
        })
        .collect())
}

fn decode_return(ret: ReturnType, raw: u64) -> Result<CallResult> {
    Ok(match ret {
        ReturnType::Void => CallResult::Void,
        ReturnType::U32 => CallResult::U32(raw as u32),
        ReturnType::U64 => CallResult::U64(raw),
        ReturnType::Ptr => CallResult::Ptr(raw),
        ReturnType::F32 | ReturnType::F64 => {
            return Err(anyhow!("float return values are not supported yet"))
        }
    })
}

#[cfg(target_arch = "x86_64")]
unsafe fn call_x64_int(address: u64, args: &[u64]) -> Result<u64> {
    use core::arch::asm;

    let rcx = *args.get(0).unwrap_or(&0);
    let rdx = *args.get(1).unwrap_or(&0);
    let r8 = *args.get(2).unwrap_or(&0);
    let r9 = *args.get(3).unwrap_or(&0);

    let mut out: u64;
    asm!(
        "sub rsp, 0x20",
        "call rax",
        "add rsp, 0x20",
        in("rcx") rcx,
        in("rdx") rdx,
        in("r8") r8,
        in("r9") r9,
        in("rax") address,
        lateout("rax") out,
        clobber_abi("win64"),
    );

    Ok(out)
}
