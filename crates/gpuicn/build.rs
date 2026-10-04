//! Embeds the component registry into the binary so `gpuicn` works offline.

use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // In the repo the registry lives at the workspace root; a packaged crate
    // carries its own copy in `registry/`.
    let registry = [manifest_dir.join("registry"), manifest_dir.join("../../registry")]
        .into_iter()
        .find(|dir| dir.join("registry.toml").exists())
        .expect("registry/registry.toml not found")
        .canonicalize()
        .unwrap();

    println!("cargo:rerun-if-changed={}", registry.display());
    let mut out = String::new();
    out.push_str(&format!(
        "pub static MANIFEST: &str = include_str!({:?});\n",
        registry.join("registry.toml")
    ));
    out.push_str("pub static FILES: &[(&str, &str)] = &[\n");
    let ui = registry.join("ui");
    let mut entries: Vec<_> = fs::read_dir(&ui).unwrap().map(|e| e.unwrap().path()).collect();
    entries.sort();
    for path in entries {
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !name.ends_with(".rs") || name == "mod.rs" {
            continue;
        }
        println!("cargo:rerun-if-changed={}", path.display());
        out.push_str(&format!("    ({name:?}, include_str!({path:?})),\n"));
    }
    out.push_str("];\n");
    fs::write(PathBuf::from(env::var("OUT_DIR").unwrap()).join("embedded.rs"), out).unwrap();
}
