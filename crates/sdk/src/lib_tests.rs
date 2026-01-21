#[cfg(test)]
mod tests {
    use super::super::accessor::{Accessor, FieldValue};
    use super::super::layout::{FieldRecord, LayoutMap, UdtRecord};
    use super::super::memory::{MemoryView, MockMemory};
    #[test]
    fn read_primitive_field() {
        let udt = UdtRecord {
            name: "Test".to_string(),
            kind: "Struct".to_string(),
            size: 8,
            fields: vec![FieldRecord {
                name: "value".to_string(),
                offset: 0,
                type_name: "U32".to_string(),
                type_index: "0x0".to_string(),
                kind: "member".to_string(),
            }],
            base_classes: vec![],
        };

        let layout = LayoutMap::from_records(vec![udt]);

        let memory = MockMemory::new(0x1000, vec![0x34, 0x12, 0x00, 0x00]);
        let view = MemoryView::new(&memory, 4);
        let accessor = Accessor::new(&layout, view);

        let access = accessor.field_by_name("Test", 0x1000, "value").unwrap();
        let value = accessor.read_value(&access).unwrap();

        match value {
            FieldValue::U64(v) => assert_eq!(v, 0x1234),
            _ => panic!("unexpected value"),
        }
    }
}
