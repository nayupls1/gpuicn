//! `gpuicn` — shadcn/ui for gpui.
//!
//! Copies component source code into your crate (it's yours to edit) instead
//! of adding a dependency.

mod diff;
mod project;
mod registry;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as Process;

use project::{DEFAULT_PATH, Project, installed_modules};
use registry::{Component, Registry};

#[derive(Parser)]
#[command(name = "gpuicn", version, about = "Add beautiful, editable gpui components to your app — shadcn/ui for gpui.")]
struct Cli {
    /// Run as if started in this directory.
    #[arg(long, short = 'C', global = true, value_name = "DIR")]
    cwd: Option<PathBuf>,

    /// Use a registry directory (registry.toml + ui/) instead of the built-in one.
    #[arg(long, global = true, value_name = "DIR")]
    registry: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Set up gpuicn in the current crate: config, theme and module wiring.
    Init {
        /// Where components are copied, relative to the crate root.
        #[arg(long, default_value = DEFAULT_PATH)]
        path: String,
        /// Don't run `cargo add` for gpui.
        #[arg(long)]
        no_install: bool,
        /// Re-create the config and overwrite the core files.
        #[arg(long)]
        force: bool,
    },
    /// Add components (and their dependencies) to your crate.
    Add {
        /// Component names, e.g. `button dialog alert-dialog`.
        components: Vec<String>,
        /// Add every component.
        #[arg(long, short)]
        all: bool,
        /// Overwrite files that already exist (yours will be replaced!).
        #[arg(long, short)]
        overwrite: bool,
        /// Show what would happen without writing anything.
        #[arg(long)]
        dry_run: bool,
        /// Don't run `cargo add` for crate dependencies.
        #[arg(long)]
        no_install: bool,
    },
    /// List available components.
    List,
    /// Print a component's source.
    View { component: String },
    /// Show which installed components differ from the registry, or the diff of one.
    Diff { component: Option<String> },
    /// Remove components from your crate.
    Remove {
        components: Vec<String>,
        /// Remove even if other installed components depend on them.
        #[arg(long)]
        force: bool,
    },
}

fn main() {
    if let Err(err) = run(Cli::parse()) {
        eprintln!("\x1b[31merror:\x1b[0m {err:#}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    let cwd = match &cli.cwd {
        Some(dir) => dir.clone(),
        None => std::env::current_dir()?,
    };
    let registry_dir = cli.registry.as_deref();
    match cli.command {
        Command::Init { path, no_install, force } => {
            let mut project = Project::discover(&cwd)?;
            init(&mut project, &path, no_install, force, registry_dir)
        }
        Command::Add { components, all, overwrite, dry_run, no_install } => {
            let mut project = Project::discover(&cwd)?;
            if project.config.is_none() {
                if dry_run {
                    println!("would run `gpuicn init` first (no {} found)", project::CONFIG_FILE);
                } else {
                    println!("No {} found — running `gpuicn init` with defaults.\n", project::CONFIG_FILE);
                    init(&mut project, DEFAULT_PATH, no_install, false, registry_dir)?;
                    println!();
                }
            }
            let registry = project.registry(registry_dir)?;
            let names: Vec<String> = if all {
                registry.components.iter().map(|c| c.name.clone()).collect()
            } else if components.is_empty() {
                bail!("which components? e.g. `gpuicn add button card` (or `--all`; see `gpuicn list`)");
            } else {
                components
            };
            add(&project, &registry, &names, overwrite, dry_run, no_install)
        }
        Command::List => {
            let project = Project::discover(&cwd).ok();
            let registry = match &project {
                Some(project) => project.registry(registry_dir)?,
                None => load_registry(registry_dir)?,
            };
            list(project.as_ref(), &registry)
        }
        Command::View { component } => {
            let registry = match Project::discover(&cwd) {
                Ok(project) => project.registry(registry_dir)?,
                Err(_) => load_registry(registry_dir)?,
            };
            let component = registry.get(&component).with_context(|| format!("unknown component `{component}`"))?;
            for file in &component.files {
                if component.files.len() > 1 {
                    println!("// ── {file} ──");
                }
                print!("{}", registry.file(file).unwrap());
            }
            Ok(())
        }
        Command::Diff { component } => {
            let project = Project::discover(&cwd)?;
            let registry = project.registry(registry_dir)?;
            diff_cmd(&project, &registry, component.as_deref())
        }
        Command::Remove { components, force } => {
            let project = Project::discover(&cwd)?;
            let registry = project.registry(registry_dir)?;
            remove(&project, &registry, &components, force)
        }
    }
}

fn load_registry(dir: Option<&Path>) -> Result<Registry> {
    match dir {
        Some(dir) => Registry::from_dir(dir),
        None => Registry::embedded(),
    }
}

fn rel(project: &Project, path: &Path) -> String {
    path.strip_prefix(&project.root).unwrap_or(path).display().to_string()
}

fn init(project: &mut Project, path: &str, no_install: bool, force: bool, registry_dir: Option<&Path>) -> Result<()> {
    if project.config.is_some() && !force {
        println!("{} already exists (use --force to re-create it).", project::CONFIG_FILE);
    } else {
        project.write_config(path)?;
        println!("✔ Wrote {}", project::CONFIG_FILE);
    }
    let registry = project.registry(registry_dir)?;
    let dir = project.components_dir();
    fs::create_dir_all(&dir)?;

    let core: Vec<&Component> = registry.core().collect();
    for component in &core {
        for file in &component.files {
            let target = dir.join(file);
            if target.exists() && !force {
                continue;
            }
            fs::write(&target, registry.file(file).unwrap())?;
            println!("✔ Created {}", rel(project, &target));
        }
    }
    project.write_mod_rs(&registry)?;
    for edited in project.wire_modules()? {
        println!("✔ Declared the components module in {}", rel(project, &edited));
    }

    if !project.has_dependency("gpui")? {
        if no_install {
            println!("• Add gpui to Cargo.toml: cargo add gpui@0.2");
        } else {
            cargo_add(project, "gpui@0.2")?;
        }
    }

    let module = project.module_path()?;
    println!(
        "\n\x1b[1mgpuicn is ready.\x1b[0m\n\
         \n  1. Call `{module}::init(cx)` once in `Application::run`.\n  \
         2. Add components: `gpuicn add button card dialog`\n  \
         3. Restyle everything in `{}`\n",
        rel(project, &dir.join("theme.rs"))
    );
    Ok(())
}

fn cargo_add(project: &Project, spec: &str) -> Result<()> {
    println!("• cargo add {spec}");
    let status = Process::new("cargo")
        .arg("add")
        .arg(spec)
        .current_dir(&project.root)
        .status()
        .context("running `cargo add` (is cargo on PATH?)")?;
    if !status.success() {
        bail!("`cargo add {spec}` failed");
    }
    Ok(())
}

fn add(project: &Project, registry: &Registry, names: &[String], overwrite: bool, dry_run: bool, no_install: bool) -> Result<()> {
    let requested: BTreeSet<String> =
        names.iter().filter_map(|n| registry.get(n)).map(|c| c.name.clone()).collect();
    let components = registry.resolve(names)?;
    let dir = project.components_dir();
    if !dry_run {
        fs::create_dir_all(&dir)?;
    }

    let (mut created, mut updated, mut skipped) = (Vec::new(), Vec::new(), Vec::new());
    for component in &components {
        for file in &component.files {
            let source = registry.file(file).unwrap();
            let target = dir.join(file);
            match fs::read_to_string(&target) {
                Ok(existing) if existing == source => {}
                Ok(_) if overwrite => updated.push((target, source)),
                Ok(_) => skipped.push((component.name.clone(), target)),
                Err(_) => created.push((target, source)),
            }
        }
    }

    let verb = if dry_run { "Would create" } else { "Created" };
    for (target, source) in &created {
        if !dry_run {
            fs::write(target, source)?;
        }
        println!("✔ {verb} {}", rel(project, target));
    }
    let verb = if dry_run { "Would overwrite" } else { "Overwrote" };
    for (target, source) in &updated {
        if !dry_run {
            fs::write(target, source)?;
        }
        println!("✔ {verb} {}", rel(project, target));
    }
    for (name, target) in &skipped {
        let hint = if requested.contains(name) { " (use --overwrite to replace it)" } else { "" };
        println!("• Kept your {}{hint}", rel(project, target));
    }
    if created.is_empty() && updated.is_empty() {
        println!("Everything is already up to date.");
    }

    // Crate dependencies.
    let mut crates = BTreeSet::new();
    for component in &components {
        crates.extend(component.dependencies.iter().cloned());
    }
    for spec in crates {
        let name = spec.split('@').next().unwrap();
        if project.has_dependency(name)? {
            continue;
        }
        if dry_run || no_install {
            println!("• Needs crate dependency: cargo add {spec}");
        } else {
            cargo_add(project, &spec)?;
        }
    }

    if dry_run {
        return Ok(());
    }
    project.write_mod_rs(registry)?;
    project.wire_modules()?;

    let notes: Vec<&String> = components
        .iter()
        .filter(|c| created.iter().any(|(t, _)| c.files.iter().any(|f| t.ends_with(f))))
        .filter_map(|c| c.note.as_ref())
        .collect();
    let module = project.module_path()?;
    if !created.is_empty() {
        let example = components.iter().find(|c| requested.contains(&c.name)).and_then(|c| c.modules().next().map(String::from));
        if let Some(example) = example {
            println!("\nImport with: use {module}::{example}::*;");
        }
    }
    if !notes.is_empty() {
        println!("\nSetup:");
        for note in notes {
            println!("  • {note}");
        }
    }
    Ok(())
}

fn list(project: Option<&Project>, registry: &Registry) -> Result<()> {
    let installed: BTreeSet<String> = match project {
        Some(project) => installed_modules(&project.components_dir())?.into_iter().collect(),
        None => BTreeSet::new(),
    };
    let width = registry.components.iter().map(|c| c.name.len()).max().unwrap_or(0);
    for component in &registry.components {
        let is_installed = component.modules().all(|m| installed.contains(m));
        let mark = if is_installed { "\x1b[32m✔\x1b[0m" } else { " " };
        let description = component.description.split(". ").next().unwrap_or("").trim_end_matches('.');
        println!("{mark} {:<width$}  \x1b[2m{description}\x1b[0m", component.name);
    }
    Ok(())
}

fn installed_components<'a>(project: &Project, registry: &'a Registry) -> Result<Vec<&'a Component>> {
    let installed: BTreeSet<String> = installed_modules(&project.components_dir())?.into_iter().collect();
    Ok(registry.components.iter().filter(|c| c.modules().all(|m| installed.contains(m))).collect())
}

fn diff_cmd(project: &Project, registry: &Registry, component: Option<&str>) -> Result<()> {
    let dir = project.components_dir();
    match component {
        Some(name) => {
            let component = registry.get(name).with_context(|| format!("unknown component `{name}`"))?;
            for file in &component.files {
                let local = fs::read_to_string(dir.join(file)).with_context(|| format!("{file} is not installed"))?;
                let upstream = registry.file(file).unwrap();
                if local == upstream {
                    println!("{file}: up to date");
                } else {
                    println!("\x1b[1m--- registry/{file}\n+++ local/{file}\x1b[0m");
                    print!("{}", diff::unified(upstream, &local, 3));
                }
            }
        }
        None => {
            let mut changed = 0;
            for component in installed_components(project, registry)? {
                let modified = component
                    .files
                    .iter()
                    .any(|f| fs::read_to_string(dir.join(f)).ok().as_deref() != registry.file(f));
                if modified {
                    changed += 1;
                    println!("~ {}", component.name);
                }
            }
            if changed == 0 {
                println!("All installed components match the registry.");
            } else {
                println!("\n{changed} component(s) differ. `gpuicn diff <name>` shows the changes; `gpuicn add <name> --overwrite` resets one.");
            }
        }
    }
    Ok(())
}

fn remove(project: &Project, registry: &Registry, names: &[String], force: bool) -> Result<()> {
    if names.is_empty() {
        bail!("which components? e.g. `gpuicn remove carousel`");
    }
    let installed: Vec<String> = installed_components(project, registry)?.iter().map(|c| c.name.clone()).collect();
    let dir = project.components_dir();
    for name in names {
        let component = registry.get(name).with_context(|| format!("unknown component `{name}`"))?;
        let users: Vec<&str> = registry
            .dependents(&component.name)
            .into_iter()
            .filter(|c| installed.contains(&c.name) && !names.iter().any(|n| registry.get(n).is_some_and(|x| x.name == c.name)))
            .map(|c| c.name.as_str())
            .collect();
        if !users.is_empty() && !force {
            bail!("`{}` is used by {} (use --force to remove anyway)", component.name, users.join(", "));
        }
        for file in &component.files {
            let target = dir.join(file);
            if target.exists() {
                fs::remove_file(&target)?;
                println!("✔ Removed {}", rel(project, &target));
            }
        }
    }
    project.write_mod_rs(registry)
}
