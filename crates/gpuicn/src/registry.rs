//! The component registry: which components exist, their files and
//! dependencies.

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

mod embedded {
    include!(concat!(env!("OUT_DIR"), "/embedded.rs"));
}

#[derive(Debug, Clone, Deserialize)]
pub struct Component {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub files: Vec<String>,
    /// Other registry components this one imports.
    #[serde(default)]
    pub registry_dependencies: Vec<String>,
    /// Crates to `cargo add`, e.g. `"unicode-segmentation@1"`.
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// The module exposes `pub fn init(cx: &mut App)`.
    #[serde(default)]
    pub init: bool,
    /// Installed by `gpuicn init`.
    #[serde(default)]
    pub core: bool,
    /// Printed after installing (setup steps).
    #[serde(default)]
    pub note: Option<String>,
}

impl Component {
    /// Rust module names for this component's files.
    pub fn modules(&self) -> impl Iterator<Item = &str> {
        self.files.iter().filter_map(|f| f.strip_suffix(".rs"))
    }
}

#[derive(Debug, Deserialize)]
struct Manifest {
    #[serde(rename = "component", default)]
    components: Vec<Component>,
}

pub struct Registry {
    pub components: Vec<Component>,
    files: BTreeMap<String, String>,
}

impl Registry {
    /// The registry compiled into this binary.
    pub fn embedded() -> Result<Self> {
        let manifest: Manifest = toml::from_str(embedded::MANIFEST).context("invalid embedded registry.toml")?;
        let files = embedded::FILES.iter().map(|(name, src)| (name.to_string(), src.to_string())).collect();
        Self::new(manifest.components, files)
    }

    /// A registry on disk: `<dir>/registry.toml` + `<dir>/ui/*.rs`.
    pub fn from_dir(dir: &Path) -> Result<Self> {
        let manifest_path = dir.join("registry.toml");
        let manifest: Manifest = toml::from_str(
            &fs::read_to_string(&manifest_path).with_context(|| format!("reading {}", manifest_path.display()))?,
        )
        .with_context(|| format!("parsing {}", manifest_path.display()))?;
        let mut files = BTreeMap::new();
        for component in &manifest.components {
            for file in &component.files {
                let path = dir.join("ui").join(file);
                let src = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
                files.insert(file.clone(), src);
            }
        }
        Self::new(manifest.components, files)
    }

    fn new(components: Vec<Component>, files: BTreeMap<String, String>) -> Result<Self> {
        let registry = Self { components, files };
        for component in &registry.components {
            for file in &component.files {
                if !registry.files.contains_key(file) {
                    bail!("registry: `{}` lists missing file `{file}`", component.name);
                }
            }
            for dep in &component.registry_dependencies {
                if registry.get(dep).is_none() {
                    bail!("registry: `{}` depends on unknown component `{dep}`", component.name);
                }
            }
        }
        Ok(registry)
    }

    /// Look up by name; accepts `alert-dialog`, `alert_dialog` or `AlertDialog`.
    pub fn get(&self, name: &str) -> Option<&Component> {
        let wanted = normalize(name);
        self.components.iter().find(|c| normalize(&c.name) == wanted)
    }

    pub fn file(&self, name: &str) -> Option<&str> {
        self.files.get(name).map(String::as_str)
    }

    pub fn core(&self) -> impl Iterator<Item = &Component> {
        self.components.iter().filter(|c| c.core)
    }

    /// The component that owns `file`.
    pub fn owner_of(&self, file: &str) -> Option<&Component> {
        self.components.iter().find(|c| c.files.iter().any(|f| f == file))
    }

    /// `names` plus everything they depend on, in a stable order.
    pub fn resolve<'a>(&'a self, names: &[String]) -> Result<Vec<&'a Component>> {
        let mut seen = BTreeSet::new();
        let mut order = Vec::new();
        let mut stack: Vec<&Component> = Vec::new();
        for name in names {
            match self.get(name) {
                Some(component) => stack.push(component),
                None => bail!("unknown component `{name}`{}", self.suggest(name)),
            }
        }
        while let Some(component) = stack.pop() {
            if !seen.insert(component.name.clone()) {
                continue;
            }
            order.push(component);
            for dep in &component.registry_dependencies {
                stack.push(self.get(dep).expect("validated in Registry::new"));
            }
        }
        order.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(order)
    }

    /// Components that (directly) depend on `name`.
    pub fn dependents(&self, name: &str) -> Vec<&Component> {
        self.components.iter().filter(|c| c.registry_dependencies.iter().any(|d| normalize(d) == normalize(name))).collect()
    }

    fn suggest(&self, name: &str) -> String {
        let wanted = normalize(name);
        let best = self
            .components
            .iter()
            .map(|c| (edit_distance(&normalize(&c.name), &wanted), &c.name))
            .min_by_key(|(d, _)| *d);
        match best {
            Some((distance, name)) if distance <= 3 => format!(" — did you mean `{name}`?"),
            _ => " — run `gpuicn list` to see all components".to_string(),
        }
    }
}

fn normalize(name: &str) -> String {
    name.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect()
}

fn edit_distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut row = vec![i + 1];
        for (j, cb) in b.iter().enumerate() {
            row.push((prev[j] + (ca != *cb) as usize).min(prev[j + 1] + 1).min(row[j] + 1));
        }
        prev = row;
    }
    prev[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_registry_is_valid() {
        let registry = Registry::embedded().unwrap();
        assert!(registry.components.len() >= 60);
        assert!(registry.get("AlertDialog").is_some());
        assert!(registry.get("alert_dialog").is_some());
    }

    #[test]
    fn resolves_transitive_dependencies() {
        let registry = Registry::embedded().unwrap();
        let names: Vec<_> = registry.resolve(&["combobox".into()]).unwrap().into_iter().map(|c| c.name.as_str()).collect();
        for expected in ["combobox", "command", "input", "popover", "portal", "theme", "utils", "button", "icon"] {
            assert!(names.contains(&expected), "missing {expected} in {names:?}");
        }
    }

    #[test]
    fn suggests_close_names() {
        let registry = Registry::embedded().unwrap();
        let err = registry.resolve(&["buton".into()]).unwrap_err().to_string();
        assert!(err.contains("did you mean `button`"), "{err}");
    }

    /// The website (site/components.js) shows every component and only real ones.
    #[test]
    fn site_lists_every_component() {
        let registry = Registry::embedded().unwrap();
        let site = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../site/components.js")).unwrap();
        let listed: BTreeSet<&str> =
            site.match_indices("name: \"").map(|(ix, m)| &site[ix + m.len()..]).map(|rest| &rest[..rest.find('"').unwrap()]).collect();
        for component in &registry.components {
            assert!(listed.contains(component.name.as_str()), "site/components.js is missing `{}`", component.name);
        }
        for name in listed {
            assert!(registry.get(name).is_some(), "site/components.js lists unknown component `{name}`");
        }
    }

    /// Every `use super::x` in a component must be declared as a dependency,
    /// otherwise `gpuicn add` would produce code that doesn't compile.
    #[test]
    fn manifest_dependencies_match_imports() {
        let registry = Registry::embedded().unwrap();
        for component in &registry.components {
            for file in &component.files {
                let src = registry.file(file).unwrap();
                for (ix, _) in src.match_indices("super::") {
                    let rest = &src[ix + "super::".len()..];
                    let module: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                    if module.is_empty() || component.modules().any(|m| m == module) {
                        continue;
                    }
                    let dep = registry.owner_of(&format!("{module}.rs")).map(|c| c.name.clone());
                    assert!(
                        dep.as_ref().is_some_and(|d| component.registry_dependencies.contains(d)),
                        "`{}` uses `super::{module}` but doesn't list it in registry_dependencies",
                        component.name
                    );
                }
                if src.contains("crate::impl_styled") {
                    assert!(component.registry_dependencies.iter().any(|d| d == "utils"), "`{}` needs utils", component.name);
                }
            }
        }
    }
}
