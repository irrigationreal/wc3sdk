use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldRecord {
    pub name: String,
    pub offset: u64,
    pub type_name: String,
    pub type_index: String,
    pub kind: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UdtRecord {
    pub name: String,
    pub kind: String,
    pub size: u64,
    pub fields: Vec<FieldRecord>,
    pub base_classes: Vec<FieldRecord>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumValueRecord {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnumRecord {
    pub name: String,
    pub underlying_type: String,
    pub values: Vec<EnumValueRecord>,
}

#[derive(Clone, Debug)]
pub struct LayoutMap {
    types: HashMap<String, UdtRecord>,
}

#[derive(Clone, Debug)]
pub struct EnumMap {
    enums: HashMap<String, EnumRecord>,
}

impl LayoutMap {
    pub fn from_udt_path(path: &Path) -> Result<Self> {
        let data = fs::read(path).with_context(|| format!("read UDTs {}", path.display()))?;
        let udts: Vec<UdtRecord> = serde_json::from_slice(&data).context("parse UDT json")?;
        let mut types = HashMap::with_capacity(udts.len());
        for udt in udts {
            types.insert(udt.name.clone(), udt);
        }
        Ok(LayoutMap { types })
    }

    pub fn from_records(records: Vec<UdtRecord>) -> Self {
        let mut types = HashMap::with_capacity(records.len());
        for udt in records {
            types.insert(udt.name.clone(), udt);
        }
        LayoutMap { types }
    }

    pub fn get(&self, name: &str) -> Option<&UdtRecord> {
        self.types.get(name)
    }

    pub fn has(&self, name: &str) -> bool {
        self.types.contains_key(name)
    }
}

impl EnumMap {
    pub fn from_enum_path(path: &Path) -> Result<Self> {
        let data = fs::read(path).with_context(|| format!("read enums {}", path.display()))?;
        let enums: Vec<EnumRecord> =
            serde_json::from_slice(&data).context("parse enum json")?;
        let mut map = HashMap::with_capacity(enums.len());
        for en in enums {
            map.insert(en.name.clone(), en);
        }
        Ok(EnumMap { enums: map })
    }

    pub fn get(&self, name: &str) -> Option<&EnumRecord> {
        self.enums.get(name)
    }
}
