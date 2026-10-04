//! The user's crate: config, where components live, and how they're wired
//! into the module tree.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::registry::Registry;

pub const CONFIG_FILE: &str = "gpuicn.toml";
pub const DEFAULT_PATH: &str = "src/components/ui";
const BEGIN: &str = "// gpuicn:begin";
const END: &str = "// gpuicn:end";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub components: ComponentsConfig,
    /// Optional path to a custom registry directory (relative to the project).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentsConfig {
    /// Directory the components are copied into, relative to the project root.
    pub path: String,
}

pub struct Project {
    pub root: PathBuf,
    pub config: Option<Config>,
}

impl Project {
    /// Find the nearest `Cargo.toml` with a `[package]` at or above `start`.
    pub fn discover(start: &Path) -> Result<Self> {
        let mut dir = start.canonicalize().with_context(|| format!("resolving {}", start.display()))?;
        loop {
            let manifest = dir.join("Cargo.toml");
            if manifest.exists() {
                let text = fs::read_to_string(&manifest)?;
                let doc: toml::Table = toml::from_str(&text).with_context(|| format!("parsing {}", manifest.display()))?;
                if doc.contains_key("package") {
                    let config = match fs::read_to_string(dir.join(CONFIG_FILE)) {
                        Ok(text) => Some(toml::from_str(&text).with_context(|| format!("parsing {CONFIG_FILE}"))?),
                        Err(_) => None,
                    };
                    return Ok(Self { root: dir, config });
                }
            }
            if !dir.pop() {
                bail!("no Cargo package found in {} or its parents (run `cargo new my-app` first)", start.display());
            }
        }
    }

    pub fn components_dir(&self) -> PathBuf {
        let path = self.config.as_ref().map(|c| c.components.path.as_str()).unwrap_or(DEFAULT_PATH);
        self.root.join(path)
    }

    pub fn registry(&self, override_dir: Option<&Path>) -> Result<Registry> {
        if let Some(dir) = override_dir {
            return Registry::from_dir(dir);
        }
        match self.config.as_ref().and_then(|c| c.registry.as_ref()) {
            Some(dir) => Registry::from_dir(&self.root.join(dir)),
            None => Registry::embedded(),
        }
    }

    pub fn write_config(&mut self, path: &str) -> Result<()> {
        let config = Config { components: ComponentsConfig { path: path.to_string() }, registry: None };
        let body = toml::to_string_pretty(&config)?;
        fs::write(
            self.root.join(CONFIG_FILE),
            format!("# gpuicn configuration. Components are copied into `components.path`.\n# Restyle everything by editing `theme.rs` in that folder.\n\n{body}"),
        )?;
        self.config = Some(config);
        Ok(())
    }

    pub fn has_dependency(&self, name: &str) -> Result<bool> {
        let doc: toml::Table = toml::from_str(&fs::read_to_string(self.root.join("Cargo.toml"))?)?;
        Ok(["dependencies", "dev-dependencies"]
            .iter()
            .any(|section| doc.get(*section).and_then(|d| d.as_table()).is_some_and(|d| d.contains_key(name))))
    }

    /// The crate root (`src/main.rs` or `src/lib.rs`).
    pub fn crate_root(&self) -> Option<PathBuf> {
        ["src/main.rs", "src/lib.rs"].iter().map(|p| self.root.join(p)).find(|p| p.exists())
    }

    /// Module path of the components dir, e.g. `crate::components::ui`.
    pub fn module_path(&self) -> Result<String> {
        let rel = self.components_dir().strip_prefix(self.root.join("src")).map(Path::to_path_buf).map_err(|_| {
            anyhow::anyhow!("components.path must be inside src/ (got {})", self.components_dir().display())
        })?;
        let parts: Vec<String> = rel.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect();
        Ok(format!("crate::{}", parts.join("::")))
    }

    /// Make sure `src/…/ui` is reachable: declare each directory in its
    /// parent module and the top one in the crate root. Returns edited files.
    pub fn wire_modules(&self) -> Result<Vec<PathBuf>> {
        let src = self.root.join("src");
        let dir = self.components_dir();
        let rel = dir.strip_prefix(&src).map_err(|_| anyhow::anyhow!("components.path must be inside src/"))?;
        let parts: Vec<String> = rel.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect();
        let mut edited = Vec::new();

        for depth in 0..parts.len() {
            let name = &parts[depth];
            let parent_file = if depth == 0 {
                self.crate_root().context("no src/main.rs or src/lib.rs")?
            } else {
                let parent_dir = src.join(parts[..depth].join("/"));
                let as_file = parent_dir.with_extension("rs");
                if as_file.exists() { as_file } else { parent_dir.join("mod.rs") }
            };
            let is_lib_root = depth == 0 && parent_file.ends_with("lib.rs");
            let decl = if depth == 0 && !is_lib_root { format!("mod {name};") } else { format!("pub mod {name};") };
            let text = fs::read_to_string(&parent_file).unwrap_or_default();
            if declares_module(&text, name) {
                continue;
            }
            fs::create_dir_all(parent_file.parent().unwrap())?;
            fs::write(&parent_file, insert_declaration(&text, &decl))?;
            edited.push(parent_file);
        }
        Ok(edited)
    }

    /// Rewrite the managed region of `<components>/mod.rs`, declaring every
    /// `.rs` file in the folder and calling `init` of components that need it.
    pub fn write_mod_rs(&self, registry: &Registry) -> Result<()> {
        let dir = self.components_dir();
        let path = dir.join("mod.rs");
        let existing = fs::read_to_string(&path).ok();
        let contents = render_mod_rs(&installed_modules(&dir)?, registry, existing.as_deref());
        fs::write(&path, contents)?;
        Ok(())
    }
}

/// Module names (file stems) present in the components folder.
pub fn installed_modules(dir: &Path) -> Result<Vec<String>> {
    let mut modules = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            let name = entry?.file_name().to_string_lossy().to_string();
            if let Some(stem) = name.strip_suffix(".rs")
                && stem != "mod"
            {
                modules.push(stem.to_string());
            }
        }
    }
    modules.sort();
    Ok(modules)
}

pub fn render_mod_rs(modules: &[String], registry: &Registry, existing: Option<&str>) -> String {
    let mut region = String::new();
    region.push_str(BEGIN);
    region.push_str(" — managed by `gpuicn add`; edits inside this block are overwritten.\n");
    for module in modules {
        region.push_str(&format!("pub mod {module};\n"));
    }
    region.push_str("\n/// Install the theme and key bindings. Call once at startup.\n");
    region.push_str("pub fn init(cx: &mut gpui::App) {\n");
    // theme first: other components read it during init.
    let mut inits: Vec<&String> = modules
        .iter()
        .filter(|m| registry.owner_of(&format!("{m}.rs")).is_some_and(|c| c.init))
        .collect();
    inits.sort_by_key(|m| (m.as_str() != "theme", m.to_string()));
    for module in inits {
        region.push_str(&format!("    {module}::init(cx);\n"));
    }
    region.push_str("}\n");
    region.push_str(END);
    region.push('\n');

    match existing.and_then(|text| Some((text.find(BEGIN)?, text.find(END)?, text))) {
        Some((start, end, text)) if start < end => {
            let after = &text[end + END.len()..];
            format!("{}{}{}", &text[..start], region, after.strip_prefix('\n').unwrap_or(after))
        }
        _ => format!(
            "//! UI components installed by `gpuicn`.\n\
             //!\n\
             //! These files are yours: edit them freely. Restyle everything from `theme.rs`.\n\
             #![allow(dead_code, unused_imports)]\n\n{region}"
        ),
    }
}

fn declares_module(text: &str, name: &str) -> bool {
    text.lines().any(|line| {
        let line = line.trim();
        let line = line.strip_prefix("pub(crate) ").or_else(|| line.strip_prefix("pub ")).unwrap_or(line);
        line == format!("mod {name};") || line.starts_with(&format!("mod {name} {{")) || line.starts_with(&format!("mod {name}{{"))
    })
}

/// Insert `decl` after the leading doc comments/attributes and existing `mod` lines.
fn insert_declaration(text: &str, decl: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut at = 0;
    for (ix, line) in lines.iter().enumerate() {
        let t = line.trim();
        if t.starts_with("//!") || t.starts_with("#![") || t.is_empty() {
            at = ix + 1;
        } else {
            break;
        }
    }
    // Prefer to sit with other `mod` declarations.
    if let Some(last_mod) = lines.iter().rposition(|l| {
        let t = l.trim();
        (t.starts_with("mod ") || t.starts_with("pub mod ")) && t.ends_with(';')
    }) {
        at = last_mod + 1;
    }
    let mut out: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    out.insert(at.min(out.len()), decl.to_string());
    let mut joined = out.join("\n");
    joined.push('\n');
    joined
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_after_existing_mods() {
        let src = "//! App\n\nmod app;\nuse gpui::*;\n\nfn main() {}\n";
        let out = insert_declaration(src, "mod components;");
        assert!(out.contains("mod app;\nmod components;\nuse gpui::*;"), "{out}");
    }

    #[test]
    fn inserts_at_top_when_no_mods() {
        let out = insert_declaration("use gpui::*;\nfn main() {}\n", "mod components;");
        assert!(out.starts_with("mod components;\nuse gpui::*;"), "{out}");
    }

    #[test]
    fn detects_declarations() {
        assert!(declares_module("pub mod ui;\n", "ui"));
        assert!(declares_module("mod components {\n}", "components"));
        assert!(!declares_module("mod uix;", "ui"));
    }

    #[test]
    fn mod_rs_preserves_user_content() {
        let registry = Registry::embedded().unwrap();
        let first = render_mod_rs(&["button".into(), "theme".into()], &registry, None);
        let edited = format!("{first}\npub mod my_widget;\n");
        let second = render_mod_rs(&["button".into(), "input".into(), "theme".into()], &registry, Some(&edited));
        assert!(second.contains("pub mod my_widget;"));
        assert!(second.contains("pub mod input;"));
        assert!(second.contains("    theme::init(cx);\n    input::init(cx);"));
    }

    /// The gallery compiles the registry through `registry/ui/mod.rs`; keep it
    /// identical to what `gpuicn add --all` would generate.
    #[test]
    fn registry_mod_rs_is_up_to_date() {
        let registry = Registry::embedded().unwrap();
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../registry/ui");
        let path = dir.join("mod.rs");
        let current = fs::read_to_string(&path).unwrap_or_default();
        let expected = render_mod_rs(&installed_modules(&dir).unwrap(), &registry, Some(&current));
        if current != expected {
            if std::env::var("GPUICN_BLESS").is_ok() {
                fs::write(&path, &expected).unwrap();
            } else {
                panic!("registry/ui/mod.rs is stale; run `GPUICN_BLESS=1 cargo test -p gpuicn`");
            }
        }
    }
}
