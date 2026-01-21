use anyhow::{Context, Result};
use clap::Parser;
use goblin::pe::PE;
use pdb::{FallibleIterator, PDB};
use serde::Serialize;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(about = "Dump PDB symbols/types into JSON for analysis", version)]
struct Args {
    /// Path to PDB file
    #[arg(long)]
    pdb: PathBuf,

    /// Path to matching PE/EXE (optional, enables RVA/file-offset mapping)
    #[arg(long)]
    exe: Option<PathBuf>,

    /// Output directory (default: local/pdb-dumps/<pdb basename>)
    #[arg(long)]
    out: Option<PathBuf>,

    /// Limit number of symbols written (0 = no limit)
    #[arg(long, default_value_t = 0)]
    max_symbols: usize,

    /// Limit number of UDTs written (0 = no limit)
    #[arg(long, default_value_t = 0)]
    max_types: usize,
}

#[derive(Serialize)]
struct PdbInfoOut {
    pdb_path: String,
    pdb_signature: u32,
    pdb_age: u32,
    pdb_guid: String,
    pdb_version: String,
    dbi_age: Option<u32>,
    machine_type: Option<String>,
    streams: Vec<String>,
}

#[derive(Serialize)]
struct PeInfoOut {
    image_base: u64,
    entry: u32,
    sections: Vec<PeSectionOut>,
}

#[derive(Serialize)]
struct PeSectionOut {
    name: String,
    virtual_address: u32,
    virtual_size: u32,
    raw_data_ptr: u32,
    raw_data_size: u32,
    characteristics: u32,
}

#[derive(Serialize)]
struct SymbolOut {
    name: String,
    kind: String,
    section: u16,
    section_offset: u32,
    rva: Option<u32>,
    file_offset: Option<u32>,
    is_code: Option<bool>,
    is_function: Option<bool>,
    type_index: Option<String>,
}

#[derive(Serialize)]
struct FieldOut {
    name: String,
    offset: u64,
    type_name: String,
    type_index: String,
    kind: String,
}

#[derive(Serialize)]
struct UdtOut {
    name: String,
    kind: String,
    size: u64,
    fields: Vec<FieldOut>,
    base_classes: Vec<FieldOut>,
}

#[derive(Serialize)]
struct EnumValueOut {
    name: String,
    value: String,
}

#[derive(Serialize)]
struct EnumOut {
    name: String,
    underlying_type: String,
    values: Vec<EnumValueOut>,
}

#[derive(Serialize)]
struct SummaryOut {
    public_symbols: usize,
    data_symbols: usize,
    other_symbols: usize,
    udt_count: usize,
    enum_count: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let out_dir = args.out.unwrap_or_else(|| {
        let base = args
            .pdb
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("pdb");
        PathBuf::from("local/pdb-dumps").join(base)
    });
    fs::create_dir_all(&out_dir).context("create output directory")?;

    let pe_info = if let Some(exe_path) = args.exe.as_ref() {
        Some(load_pe_info(exe_path).context("parse PE file")?)
    } else {
        None
    };

    let mut pdb = open_pdb(&args.pdb)?;

    let pdb_info = pdb.pdb_information().context("read PDB information")?;
    let mut streams = Vec::new();
    if let Ok(names) = pdb_info.stream_names() {
        for name in names.iter() {
            streams.push(name.name.to_string().into_owned());
        }
    }
    streams.sort();

    let dbi = pdb.debug_information().context("read DBI stream")?;
    let machine_type = dbi.machine_type().ok().map(|m| format!("{m:?}"));

    let pdb_info_out = PdbInfoOut {
        pdb_path: args.pdb.display().to_string(),
        pdb_signature: pdb_info.signature,
        pdb_age: pdb_info.age,
        pdb_guid: pdb_info.guid.to_string(),
        pdb_version: format!("{:?}", pdb_info.version),
        dbi_age: dbi.age(),
        machine_type,
        streams,
    };
    write_json(&out_dir.join("pdb_info.json"), &pdb_info_out)?;

    if let Some(pe) = &pe_info {
        write_json(&out_dir.join("pe_info.json"), pe)?;
    }

    let address_map = pdb.address_map().context("build address map")?;
    let (symbols, symbol_counts) = dump_symbols(
        &mut pdb,
        &address_map,
        pe_info.as_ref(),
        args.max_symbols,
    )?;
    write_json(&out_dir.join("symbols.json"), &symbols)?;

    let (udts, enums, mut summary) = dump_types(&mut pdb, args.max_types)?;
    summary.public_symbols = symbol_counts.public_symbols;
    summary.data_symbols = symbol_counts.data_symbols;
    summary.other_symbols = symbol_counts.other_symbols;
    write_json(&out_dir.join("types_udt.json"), &udts)?;
    write_json(&out_dir.join("types_enum.json"), &enums)?;
    write_json(&out_dir.join("summary.json"), &summary)?;

    let mut report = BufWriter::new(File::create(out_dir.join("report.txt"))?);
    writeln!(report, "PDB: {}", args.pdb.display())?;
    if let Some(exe) = args.exe {
        writeln!(report, "EXE: {}", exe.display())?;
    }
    writeln!(report, "Public symbols: {}", summary.public_symbols)?;
    writeln!(report, "Data symbols: {}", summary.data_symbols)?;
    writeln!(report, "Other symbols: {}", summary.other_symbols)?;
    writeln!(report, "UDTs: {}", summary.udt_count)?;
    writeln!(report, "Enums: {}", summary.enum_count)?;
    Ok(())
}

fn open_pdb(path: &Path) -> Result<PDB<'static, File>> {
    let file = File::open(path).with_context(|| format!("open PDB at {}", path.display()))?;
    let pdb = PDB::open(file).context("parse PDB")?;
    Ok(pdb)
}

fn load_pe_info(path: &Path) -> Result<PeInfoOut> {
    let buf = fs::read(path).with_context(|| format!("read PE at {}", path.display()))?;
    let pe = PE::parse(&buf).context("parse PE")?;
    let mut sections = Vec::new();
    for sec in &pe.sections {
        let name = sec
            .name()
            .unwrap_or("")
            .trim_end_matches('\0')
            .to_string();
        sections.push(PeSectionOut {
            name,
            virtual_address: sec.virtual_address,
            virtual_size: sec.virtual_size,
            raw_data_ptr: sec.pointer_to_raw_data,
            raw_data_size: sec.size_of_raw_data,
            characteristics: sec.characteristics,
        });
    }

    Ok(PeInfoOut {
        image_base: pe.image_base,
        entry: pe.entry,
        sections,
    })
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let file = File::create(path).with_context(|| format!("create {}", path.display()))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value).context("write JSON")?;
    Ok(())
}

fn rva_to_file_offset(pe: &PeInfoOut, rva: u32) -> Option<u32> {
    for sec in &pe.sections {
        let span = sec
            .virtual_size
            .max(sec.raw_data_size)
            .max(1);
        if rva >= sec.virtual_address && rva < sec.virtual_address.saturating_add(span) {
            let delta = rva.saturating_sub(sec.virtual_address);
            return Some(sec.raw_data_ptr.saturating_add(delta));
        }
    }
    None
}

fn dump_symbols(
    pdb: &mut PDB<'static, File>,
    address_map: &pdb::AddressMap<'_>,
    pe: Option<&PeInfoOut>,
    max_symbols: usize,
) -> Result<(Vec<SymbolOut>, SymbolCounts)> {
    let symbol_table = pdb.global_symbols().context("read symbol table")?;
    let mut iter = symbol_table.iter();
    let mut out = Vec::new();
    let mut counts = SymbolCounts::default();

    while let Some(symbol) = iter.next()? {
        let data = match symbol.parse() {
            Ok(data) => data,
            Err(pdb::Error::UnimplementedSymbolKind(_)) => continue,
            Err(err) => return Err(err.into()),
        };

        let (name, kind, section, section_offset, rva, is_code, is_function, type_index) =
            match data {
                pdb::SymbolData::Public(p) => {
                    counts.public_symbols += 1;
                    (
                        p.name.to_string().into_owned(),
                        "public".to_string(),
                        p.offset.section,
                        p.offset.offset,
                        p.offset.to_rva(address_map).map(|r| r.0),
                        Some(p.code),
                        Some(p.function),
                        None,
                    )
                }
                pdb::SymbolData::Data(d) => {
                    counts.data_symbols += 1;
                    (
                        d.name.to_string().into_owned(),
                        if d.global { "global" } else { "local" }.to_string(),
                        d.offset.section,
                        d.offset.offset,
                        d.offset.to_rva(address_map).map(|r| r.0),
                        None,
                        None,
                        Some(format!("{}", d.type_index)),
                    )
                }
                _ => {
                    counts.other_symbols += 1;
                    continue;
                }
            };

        let file_offset = rva.and_then(|r| pe.and_then(|pe| rva_to_file_offset(pe, r)));

        out.push(SymbolOut {
            name,
            kind,
            section,
            section_offset,
            rva,
            file_offset,
            is_code,
            is_function,
            type_index,
        });

        if max_symbols != 0 && out.len() >= max_symbols {
            break;
        }
    }

    Ok((out, counts))
}

fn dump_types(
    pdb: &mut PDB<'static, File>,
    max_types: usize,
) -> Result<(Vec<UdtOut>, Vec<EnumOut>, SummaryOut)> {
    let type_info = pdb.type_information().context("read type information")?;
    let mut type_finder = type_info.finder();
    let mut iter = type_info.iter();

    let mut udts = Vec::new();
    let mut enums = Vec::new();

    while let Some(typ) = iter.next()? {
        type_finder.update(&iter);
        let data = match typ.parse() {
            Ok(data) => data,
            Err(pdb::Error::UnimplementedTypeKind(_)) => continue,
            Err(err) => return Err(err.into()),
        };

        match data {
            pdb::TypeData::Class(class) => {
                if max_types != 0 && udts.len() >= max_types {
                    continue;
                }

                let (fields, bases) = if let Some(fields) = class.fields {
                    collect_fields(&type_finder, fields)
                        .unwrap_or_else(|_| (Vec::new(), Vec::new()))
                } else {
                    (Vec::new(), Vec::new())
                };

                udts.push(UdtOut {
                    name: class.name.to_string().into_owned(),
                    kind: format!("{:?}", class.kind),
                    size: class.size,
                    fields,
                    base_classes: bases,
                });
            }
            pdb::TypeData::Union(union) => {
                if max_types != 0 && udts.len() >= max_types {
                    continue;
                }

                let (fields, _bases) = collect_fields(&type_finder, union.fields)
                    .unwrap_or_else(|_| (Vec::new(), Vec::new()));
                udts.push(UdtOut {
                    name: union.name.to_string().into_owned(),
                    kind: "Union".to_string(),
                    size: union.size,
                    fields,
                    base_classes: Vec::new(),
                });
            }
            pdb::TypeData::Enumeration(en) => {
                if max_types != 0 && enums.len() >= max_types {
                    continue;
                }

                let mut values = Vec::new();
                let field_data = match type_finder.find(en.fields)?.parse() {
                    Ok(data) => data,
                    Err(pdb::Error::UnimplementedTypeKind(_)) => continue,
                    Err(err) => return Err(err.into()),
                };
                if let pdb::TypeData::FieldList(list) = field_data {
                    for item in list.fields {
                        if let pdb::TypeData::Enumerate(val) = item {
                            values.push(EnumValueOut {
                                name: val.name.to_string().into_owned(),
                                value: format!("{:?}", val.value),
                            });
                        }
                    }
                }

                enums.push(EnumOut {
                    name: en.name.to_string().into_owned(),
                    underlying_type: resolve_type_name(&type_finder, en.underlying_type),
                    values,
                });
            }
            _ => {}
        }
    }

    let summary = SummaryOut {
        public_symbols: 0,
        data_symbols: 0,
        other_symbols: 0,
        udt_count: udts.len(),
        enum_count: enums.len(),
    };

    Ok((udts, enums, summary))
}

fn collect_fields(
    type_finder: &pdb::TypeFinder<'_>,
    start: pdb::TypeIndex,
) -> Result<(Vec<FieldOut>, Vec<FieldOut>)> {
    let mut fields = Vec::new();
    let mut bases = Vec::new();
    let mut next = Some(start);

    while let Some(idx) = next {
        let data = match type_finder.find(idx)?.parse() {
            Ok(data) => data,
            Err(pdb::Error::UnimplementedTypeKind(_)) => break,
            Err(err) => return Err(err.into()),
        };
        match data {
            pdb::TypeData::FieldList(list) => {
                for field in list.fields {
                    match field {
                        pdb::TypeData::Member(member) => {
                            fields.push(FieldOut {
                                name: member.name.to_string().into_owned(),
                                offset: member.offset,
                                type_name: resolve_type_name(type_finder, member.field_type),
                                type_index: format!("{}", member.field_type),
                                kind: "member".to_string(),
                            });
                        }
                        pdb::TypeData::StaticMember(member) => {
                            fields.push(FieldOut {
                                name: member.name.to_string().into_owned(),
                                offset: 0,
                                type_name: resolve_type_name(type_finder, member.field_type),
                                type_index: format!("{}", member.field_type),
                                kind: "static".to_string(),
                            });
                        }
                        pdb::TypeData::BaseClass(base) => {
                            bases.push(FieldOut {
                                name: format!("{}", base.base_class),
                                offset: base.offset as u64,
                                type_name: resolve_type_name(type_finder, base.base_class),
                                type_index: format!("{}", base.base_class),
                                kind: "base".to_string(),
                            });
                        }
                        _ => {}
                    }
                }
                next = list.continuation;
            }
            _ => break,
        }
    }

    Ok((fields, bases))
}

fn resolve_type_name(type_finder: &pdb::TypeFinder<'_>, idx: pdb::TypeIndex) -> String {
    let item = match type_finder.find(idx) {
        Ok(item) => item,
        Err(_) => return format!("{}", idx),
    };
    let data = match item.parse() {
        Ok(data) => data,
        Err(_) => return format!("{}", idx),
    };

    match data {
        pdb::TypeData::Primitive(p) => match p.indirection {
            Some(ind) => format!("{:?}{:?}", p.kind, ind),
            None => format!("{:?}", p.kind),
        },
        pdb::TypeData::Pointer(ptr) => {
            let mut name = resolve_type_name(type_finder, ptr.underlying_type);
            let suffix = if ptr.attributes.is_reference() { "&" } else { "*" };
            name.push_str(suffix);
            name
        }
        pdb::TypeData::Modifier(m) => {
            let mut name = resolve_type_name(type_finder, m.underlying_type);
            if m.constant {
                name = format!("const {}", name);
            }
            if m.volatile {
                name = format!("volatile {}", name);
            }
            name
        }
        pdb::TypeData::Array(arr) => {
            let mut name = resolve_type_name(type_finder, arr.element_type);
            name.push_str("[]");
            name
        }
        pdb::TypeData::Procedure(proc) => {
            let ret = proc
                .return_type
                .map(|r| resolve_type_name(type_finder, r))
                .unwrap_or_else(|| "void".to_string());
            format!("fn({} args) -> {}", proc.parameter_count, ret)
        }
        other => other
            .name()
            .map(|n| n.to_string().into_owned())
            .unwrap_or_else(|| format!("{:?}", other)),
    }
}

#[derive(Default)]
struct SymbolCounts {
    public_symbols: usize,
    data_symbols: usize,
    other_symbols: usize,
}
