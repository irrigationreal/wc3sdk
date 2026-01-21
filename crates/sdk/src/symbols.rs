use anyhow::{Context, Result};
use crate::call::{Abi, CallInvoker, CallResult, CallSpec, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
pub struct SymbolRef {
    pub name: &'static str,
    pub rva: u32,
}

impl SymbolRef {
    pub const fn new(name: &'static str, rva: u32) -> Self {
        Self { name, rva }
    }

    pub fn resolve_va(self, module_base: u64) -> u64 {
        module_base.wrapping_add(self.rva as u64)
    }

    pub fn call(
        self,
        invoker: &dyn CallInvoker,
        module_base: u64,
        abi: Abi,
        args: Vec<Value>,
    ) -> Result<CallResult> {
        let spec = CallSpec {
            address: self.resolve_va(module_base),
            abi,
            args,
        };
        invoker.call(&spec)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SymbolRecord {
    pub name: String,
    pub kind: String,
    pub section: u16,
    pub section_offset: u32,
    pub rva: Option<u32>,
    pub file_offset: Option<u32>,
    pub is_code: Option<bool>,
    pub is_function: Option<bool>,
    pub type_index: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SymbolMap {
    symbols: Vec<SymbolRecord>,
    by_name: HashMap<String, usize>,
}

impl SymbolMap {
    pub fn from_path(path: &Path) -> Result<Self> {
        let data = fs::read(path).with_context(|| format!("read symbols {}", path.display()))?;
        let symbols: Vec<SymbolRecord> =
            serde_json::from_slice(&data).context("parse symbols json")?;

        let mut by_name = HashMap::with_capacity(symbols.len());
        for (idx, sym) in symbols.iter().enumerate() {
            by_name.insert(sym.name.clone(), idx);
        }

        Ok(SymbolMap { symbols, by_name })
    }

    pub fn get(&self, name: &str) -> Option<&SymbolRecord> {
        self.by_name.get(name).and_then(|idx| self.symbols.get(*idx))
    }

    pub fn resolve_rva(&self, name: &str) -> Option<u32> {
        self.get(name).and_then(|s| s.rva)
    }

    pub fn resolve_va(&self, module_base: u64, name: &str) -> Option<u64> {
        self.resolve_rva(name)
            .map(|rva| module_base.wrapping_add(rva as u64))
    }

    pub fn search_substring(&self, needle: &str) -> Vec<&SymbolRecord> {
        if needle.is_empty() {
            return Vec::new();
        }
        let needle = needle.to_lowercase();
        self.symbols
            .iter()
            .filter(|s| s.name.to_lowercase().contains(&needle))
            .collect()
    }

    pub fn all(&self) -> &[SymbolRecord] {
        &self.symbols
    }
}
