use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let gen_path = out_dir.join("generated.rs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let default_dir = manifest_dir.join("sdk-gen");

    let dir = env::var("WC3_SDK_GEN_DIR")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| default_dir.clone());

    let types_path = dir.join("types.rs");
    let symbols_path = dir.join("symbols.rs");

    let content = if types_path.exists() && symbols_path.exists() {
        let types = fs::read_to_string(&types_path).unwrap_or_default();
        let symbols = fs::read_to_string(&symbols_path).unwrap_or_default();
        format!(
            "// Auto-generated at build time from {}\n\n\
             pub mod types {{\n{}\n}}\n\n\
             pub mod symbols {{\n{}\n}}\n",
            dir.display(),
            types,
            symbols
        )
    } else {
        stub_content(&dir)
    };

    fs::write(&gen_path, content).expect("write generated.rs");
    println!("cargo:rerun-if-env-changed=WC3_SDK_GEN_DIR");
    println!("cargo:rerun-if-changed={}", types_path.display());
    println!("cargo:rerun-if-changed={}", symbols_path.display());
}

fn stub_content(dir: &Path) -> String {
    format!(
        "// No generated bindings found.\n// Set WC3_SDK_GEN_DIR to a directory containing\n// types.rs and symbols.rs to embed generated bindings.\n// Current: {}\n\n\
         pub mod types {{}}\n\n\
         pub mod symbols {{}}\n",
        dir.display()
    )
}
