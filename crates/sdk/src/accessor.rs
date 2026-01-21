use crate::layout::{FieldRecord, LayoutMap};
use crate::memory::{Endian, MemoryReader, MemoryView};
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct FieldAccess<'a> {
    pub field: &'a FieldRecord,
    pub address: u64,
}

#[derive(Clone, Debug)]
pub enum FieldValue {
    U64(u64),
    I64(i64),
    F64(f64),
    Ptr(u64),
    Bytes(Vec<u8>),
}

pub struct Accessor<'a, R: MemoryReader> {
    pub layout: &'a LayoutMap,
    pub memory: MemoryView<'a, R>,
}

impl<'a, R: MemoryReader> Accessor<'a, R> {
    pub fn new(layout: &'a LayoutMap, memory: MemoryView<'a, R>) -> Self {
        Self { layout, memory }
    }

    pub fn field_by_name(
        &self,
        type_name: &str,
        base: u64,
        field_name: &str,
    ) -> Result<FieldAccess<'_>> {
        let udt = self
            .layout
            .get(type_name)
            .ok_or_else(|| anyhow::anyhow!("type not found: {type_name}"))?;
        let field = udt
            .fields
            .iter()
            .find(|f| f.name == field_name)
            .ok_or_else(|| anyhow::anyhow!("field not found: {field_name}"))?;

        Ok(FieldAccess {
            field,
            address: base.wrapping_add(field.offset),
        })
    }

    pub fn read_value(&self, access: &FieldAccess<'_>) -> Result<FieldValue> {
        let (base, is_ptr) = strip_ptr_qualifiers(&access.field.type_name);
        if is_ptr {
            let ptr = self.memory.read_ptr(access.address)?;
            return Ok(FieldValue::Ptr(ptr));
        }

        let size = primitive_size(base, self.memory.pointer_size)
            .ok_or_else(|| anyhow::anyhow!("unknown primitive size: {base}"))?;
        match size {
            1 => Ok(FieldValue::U64(self.memory.reader.read_u8(access.address)? as u64)),
            2 => Ok(FieldValue::U64(
                self.memory.reader.read_u16(access.address, Endian::Little)? as u64,
            )),
            4 => Ok(FieldValue::U64(
                self.memory.reader.read_u32(access.address, Endian::Little)? as u64,
            )),
            8 => Ok(FieldValue::U64(
                self.memory.reader.read_u64(access.address, Endian::Little)?,
            )),
            other => Ok(FieldValue::Bytes(
                self.memory.reader.read_bytes(access.address, other)?,
            )),
        }
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
