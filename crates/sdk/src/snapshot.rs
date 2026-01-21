use crate::config::RootSpec;
use crate::layout::LayoutMap;
use crate::memory::{Endian, MemoryReader, MemoryView};
use crate::symbols::SymbolMap;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SnapshotValue {
    U64(u64),
    I64(i64),
    F64(f64),
    Ptr(u64),
    Bytes(Vec<u8>),
    Unavailable(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotField {
    pub name: String,
    pub type_name: String,
    pub offset: u64,
    pub value: SnapshotValue,
    pub nested: Option<SnapshotObject>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotObject {
    pub type_name: String,
    pub address: u64,
    pub fields: Vec<SnapshotField>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotRoot {
    pub name: String,
    pub address: u64,
    pub object: SnapshotObject,
}

#[derive(Clone, Debug)]
pub struct SnapshotOptions {
    pub max_depth: usize,
    pub max_fields: usize,
    pub deref_pointers: bool,
}

impl Default for SnapshotOptions {
    fn default() -> Self {
        Self {
            max_depth: 2,
            max_fields: 128,
            deref_pointers: false,
        }
    }
}

pub struct SnapshotEngine<'a, R: MemoryReader> {
    pub layout: &'a LayoutMap,
    pub symbols: &'a SymbolMap,
    pub memory: MemoryView<'a, R>,
    pub module_base: u64,
    pub options: SnapshotOptions,
}

impl<'a, R: MemoryReader> SnapshotEngine<'a, R> {
    pub fn snapshot_root(&self, root: &RootSpec) -> Result<SnapshotRoot> {
        let symbol_va = self
            .symbols
            .resolve_va(self.module_base, &root.symbol)
            .ok_or_else(|| anyhow::anyhow!("symbol not found: {}", root.symbol))?;

        let address = if root.symbol_is_ptr {
            self.memory.read_ptr(symbol_va)?
        } else {
            symbol_va
        };

        if address == 0 {
            bail!("root address is null for {}", root.name);
        }

        let mut visited = HashSet::new();
        let object = self.snapshot_type_at(
            &root.type_name,
            address,
            0,
            &mut visited,
        )?;

        Ok(SnapshotRoot {
            name: root.name.clone(),
            address,
            object,
        })
    }

    fn snapshot_type_at(
        &self,
        type_name: &str,
        address: u64,
        depth: usize,
        visited: &mut HashSet<u64>,
    ) -> Result<SnapshotObject> {
        if depth > self.options.max_depth {
            return Ok(SnapshotObject {
                type_name: type_name.to_string(),
                address,
                fields: Vec::new(),
            });
        }

        if !visited.insert(address) {
            return Ok(SnapshotObject {
                type_name: type_name.to_string(),
                address,
                fields: Vec::new(),
            });
        }

        let udt = self
            .layout
            .get(type_name)
            .ok_or_else(|| anyhow::anyhow!("type not found: {type_name}"))?;

        let mut fields = Vec::new();
        for (idx, field) in udt.fields.iter().enumerate() {
            if self.options.max_fields != 0 && idx >= self.options.max_fields {
                break;
            }
            let field_addr = address.wrapping_add(field.offset);
            let (value, nested) = self.read_field(field.type_name.as_str(), field_addr, depth, visited)?;
            fields.push(SnapshotField {
                name: field.name.clone(),
                type_name: field.type_name.clone(),
                offset: field.offset,
                value,
                nested,
            });
        }

        Ok(SnapshotObject {
            type_name: udt.name.clone(),
            address,
            fields,
        })
    }

    fn read_field(
        &self,
        type_name: &str,
        address: u64,
        depth: usize,
        visited: &mut HashSet<u64>,
    ) -> Result<(SnapshotValue, Option<SnapshotObject>)> {
        let (base, is_ptr) = strip_ptr_qualifiers(type_name);
        if is_ptr {
            let ptr = self.memory.read_ptr(address)?;
            let value = SnapshotValue::Ptr(ptr);
            if self.options.deref_pointers && ptr != 0 {
                if self.layout.has(base) {
                    let nested = self.snapshot_type_at(base, ptr, depth + 1, visited).ok();
                    return Ok((value, nested));
                }
            }
            return Ok((value, None));
        }

        if let Some(size) = primitive_size(base, self.memory.pointer_size) {
            let value = read_primitive(self.memory.reader, address, size)?;
            return Ok((value, None));
        }

        if self.layout.has(base) {
            let nested = self.snapshot_type_at(base, address, depth + 1, visited)?;
            return Ok((SnapshotValue::Unavailable("inline-udt".to_string()), Some(nested)));
        }

        Ok((SnapshotValue::Unavailable("unknown-type".to_string()), None))
    }
}

fn read_primitive<R: MemoryReader>(reader: &R, address: u64, size: usize) -> Result<SnapshotValue> {
    match size {
        1 => Ok(SnapshotValue::U64(reader.read_u8(address)? as u64)),
        2 => Ok(SnapshotValue::U64(reader.read_u16(address, Endian::Little)? as u64)),
        4 => Ok(SnapshotValue::U64(reader.read_u32(address, Endian::Little)? as u64)),
        8 => Ok(SnapshotValue::U64(reader.read_u64(address, Endian::Little)?)),
        _ => Ok(SnapshotValue::Bytes(reader.read_bytes(address, size)?)),
    }
}

fn strip_ptr_qualifiers(name: &str) -> (&str, bool) {
    let mut base = name.trim();
    if let Some(stripped) = base.strip_prefix("const ") {
        base = stripped.trim();
    }
    if let Some(stripped) = base.strip_prefix("volatile ") {
        base = stripped.trim();
    }

    if base.ends_with('*') {
        let b = base.trim_end_matches('*').trim();
        return (b, true);
    }
    if base.ends_with('&') {
        let b = base.trim_end_matches('&').trim();
        return (b, true);
    }
    (base, false)
}

fn primitive_size(name: &str, pointer_size: u8) -> Option<usize> {
    let n = name.trim();
    let n = n.strip_prefix("const ").unwrap_or(n).trim();
    match n {
        "Bool8" | "Char" | "UChar" | "I8" | "U8" | "RChar" => Some(1),
        "Short" | "UShort" | "I16" | "U16" | "WChar" | "RChar16" => Some(2),
        "Long" | "ULong" | "I32" | "U32" | "F32" => Some(4),
        "Quad" | "UQuad" | "I64" | "U64" | "F64" => Some(8),
        "Ptr64" => Some(8),
        "Ptr32" => Some(4),
        "Int" | "UInt" => Some(4),
        "SizeT" | "SSIZE_T" => Some(pointer_size as usize),
        _ => None,
    }
}
