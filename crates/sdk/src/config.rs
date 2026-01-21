use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetInfo {
    /// SHA-256 of the EXE.
    pub exe_sha256: String,
    /// PDB GUID (string form).
    pub pdb_guid: String,
    /// PDB age (from PDB info stream).
    pub pdb_age: u32,
    /// Optional machine/arch hint (e.g., "Amd64").
    pub machine: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RootSpec {
    /// Friendly name for this root (e.g., "game_state").
    pub name: String,
    /// Symbol name that points to the root object or pointer.
    pub symbol: String,
    /// Type name for the root object.
    pub type_name: String,
    /// Whether the symbol is a pointer to the object (true) or the object itself (false).
    pub symbol_is_ptr: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SdkConfig {
    pub target: TargetInfo,
    /// Path to symbols.json (from pdb-dump).
    pub symbols_path: PathBuf,
    /// Path to types_udt.json (from pdb-dump).
    pub types_udt_path: PathBuf,
    /// Path to types_enum.json (from pdb-dump). Optional.
    pub types_enum_path: Option<PathBuf>,
    /// Pointer size in bytes (4 or 8).
    pub pointer_size: u8,
    /// Root objects that can be snapshot.
    pub roots: Vec<RootSpec>,
}

impl SdkConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let data = fs::read(path).with_context(|| format!("read SDK config {}", path.display()))?;
        let config: SdkConfig = serde_json::from_slice(&data).context("parse SDK config")?;
        Ok(config)
    }
}
