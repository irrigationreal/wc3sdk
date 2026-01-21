#![cfg(windows)]

use anyhow::{Context, Result};
use sdk::config::SdkConfig;
use sdk::layout::LayoutMap;
use sdk::memory::{DefaultMemory, MemoryView};
use sdk::snapshot::{SnapshotEngine, SnapshotOptions};
use sdk::symbols::SymbolMap;
use std::env;
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Clone)]
struct SdkState {
    config: SdkConfig,
    symbols: SymbolMap,
    layouts: LayoutMap,
}

static SDK_STATE: OnceLock<SdkState> = OnceLock::new();

fn config_path_from_env() -> Option<PathBuf> {
    env::var("WC3_SDK_CONFIG").ok().map(PathBuf::from)
}

fn init_state() -> Result<SdkState> {
    let path = config_path_from_env().ok_or_else(|| {
        anyhow::anyhow!("WC3_SDK_CONFIG not set; cannot load SDK config")
    })?;
    let config = SdkConfig::load(&path)?;
    let symbols = SymbolMap::from_path(&config.symbols_path)
        .with_context(|| format!("load symbols from {}", config.symbols_path.display()))?;
    let layouts = LayoutMap::from_udt_path(&config.types_udt_path)
        .with_context(|| format!("load layouts from {}", config.types_udt_path.display()))?;
    Ok(SdkState {
        config,
        symbols,
        layouts,
    })
}

pub fn ensure_loaded() -> Result<&'static SdkState> {
    SDK_STATE.get_or_try_init(init_state)
}

pub fn snapshot_root_json(root_name: &str) -> Result<String> {
    let state = ensure_loaded()?;
    let root = state
        .config
        .roots
        .iter()
        .find(|r| r.name == root_name)
        .ok_or_else(|| anyhow::anyhow!("root not found: {root_name}"))?;

    let module_base = unsafe { get_module_base() };
    let memory = DefaultMemory::new();
    let view = MemoryView::new(&memory, state.config.pointer_size);

    let engine = SnapshotEngine {
        layout: &state.layouts,
        symbols: &state.symbols,
        memory: view,
        module_base,
        options: SnapshotOptions::default(),
    };

    let snapshot = engine.snapshot_root(root)?;
    Ok(serde_json::to_string_pretty(&snapshot)?)
}

unsafe fn get_module_base() -> u64 {
    let hmodule = GetModuleHandleW(std::ptr::null());
    hmodule as u64
}

type HMODULE = *mut core::ffi::c_void;

extern "system" {
    fn GetModuleHandleW(lpModuleName: *const u16) -> HMODULE;
}
