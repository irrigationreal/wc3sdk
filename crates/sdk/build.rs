use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let gen_path = out_dir.join("generated.rs");

    let maybe_dir = env::var("WC3_SDK_GEN_DIR").ok().map(PathBuf::from);
    let content = if let Some(dir) = maybe_dir {
        let types_path = dir.join("types.rs");
        let symbols_path = dir.join("symbols.rs");
        if types_path.exists() && symbols_path.exists() {
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
        }
    } else {
        stub_content(Path::new("<unset>"))
    };

    fs::write(&gen_path, content).expect("write generated.rs");
    println!("cargo:rerun-if-env-changed=WC3_SDK_GEN_DIR");
}

fn stub_content(dir: &Path) -> String {
    format!(
        "// No generated bindings found.\n// Set WC3_SDK_GEN_DIR to a directory containing\n// types.rs and symbols.rs to embed generated bindings.\n// Current: {}\n\n\
         pub mod types {{}}\n\n\
         pub mod symbols {{}}\n",
        dir.display()
    )
}
