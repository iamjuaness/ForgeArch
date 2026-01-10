// ForgeArch binary entrypoint. This file wires the CLI to the library
// modules that implement template loading and project creation.
//
// Usage examples are documented in README.md. The CLI is intentionally
// small: `forge new <name> [--arch <key>] [--git-init] [--no-readme] [--force]`.
mod generator;
mod templates;

use anyhow::Context;
use anyhow::Result;
use clap::{Parser, Subcommand};
use dialoguer::Select;
use std::env;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "forge", bin_name = "forge")]
#[command(about = "Universal project initializer", long_about = None)]
#[command(version)]
struct Cli {
    #[arg(short = 'v', long = "v", global = true, action = clap::ArgAction::SetTrue)]
    version_flag: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project with specified architecture
    New {
        /// Project name
        name: String,

        /// Architecture template to use
        #[arg(short, long)]
        arch: Option<String>,

        /// Initialize git repository
        #[arg(long)]
        git_init: bool,

        /// By default generate README.md. Pass --no-readme to disable.
        #[arg(long = "no-readme", default_value_t = true, action = clap::ArgAction::SetFalse)]
        readme_gen: bool,

        /// Overwrite destination if it already exists (force)
        #[arg(long)]
        force: bool,
    },

    /// List available architecture templates
    List,
    /// Add a local (private) template available only on this machine
    AddTemplate {
        /// Key used to reference the template (e.g. "my-backend")
        key: String,
    },
    /// Remove a local template stored in user templates dir
    RemoveTemplate {
        /// Key of the template to remove
        key: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // If the user used the -v/--v shorthand (we also expose -V/--version via Clap),
    // print the version and exit early.
    if cli.version_flag {
        // Print the canonical bin name plus version for clarity
        println!("forge {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    match cli.command {
        Commands::New {
            name,
            arch,
            git_init,
            readme_gen,
            force,
        } => {
            let templates = templates::load_templates()?;

            // Determinar clave de arquitectura: flag o prompt interactivo
            let arch_key = if let Some(a) = arch {
                a
            } else {
                // Prompt interactivo
                let items: Vec<String> = templates.keys().cloned().collect();
                let selection = Select::new()
                    .with_prompt("Select architecture")
                    .items(&items)
                    .interact()?;
                items[selection].clone()
            };

            let template = templates
                .get(&arch_key)
                .ok_or_else(|| anyhow::anyhow!("Template not found: {}", arch_key))?;

            // Pass flags (git_init, readme_gen, force) to the generator
            generator::create_project(&name, template, git_init, readme_gen, force)?;
            Ok(())
        }
        Commands::List => templates::list_templates(),
        Commands::AddTemplate { key } => {
            // Determine local templates file path
            let file = templates::get_user_local_templates_file()?;
            if let Some(dir) = file.parent() {
                std::fs::create_dir_all(dir)
                    .with_context(|| format!("Failed to create templates dir {}", dir.display()))?;
            }

            // Load existing templates to check if key exists
            let templates_map = templates::load_templates()?;
            if templates_map.contains_key(&key) {
                println!("Template '{}' already exists in:", key);
                println!("  {}", file.display());
                println!("\nOpening for edit...");
                open_in_editor(&file)?;
                return Ok(());
            }

            // Create skeleton template
            let skeleton = templates::Template {
                name: format!("{} Template", key),
                description: String::from("Edit this description"),
                structure: vec![String::from("src"), String::from("tests")],
                files: {
                    let mut map = std::collections::HashMap::new();
                    map.insert(".gitignore".to_string(), "backend".to_string());
                    map
                },
            };

            // Save using the proper save function
            templates::save_local_template(&key, &skeleton)?;

            println!("✓ Added template '{}' to:", key);
            println!("  {}", file.display());
            println!("\n📝 Edit the template structure below:");
            println!("   1. Save your changes");
            println!("   2. Run: forge new <project> --arch {}\n", key);
            println!("Opening editor...");

            open_in_editor(&file)?;

            Ok(())
        }
        Commands::RemoveTemplate { key } => {
            match templates::remove_local_template(&key)? {
                true => println!("Removed local template '{}'", key),
                false => println!("Local template '{}' not found", key),
            }
            Ok(())
        }
    }
}

/// Open the given path in a user editor. Priority:
/// - $FORGE_EDITOR, $VISUAL, $EDITOR
/// - `code --wait` (with WSL path conversion)
/// - `vim`
/// - `nano`
fn open_in_editor(path: &Path) -> Result<()> {
    // Check env vars
    let editor_env = env::var("FORGE_EDITOR")
        .ok()
        .or_else(|| env::var("VISUAL").ok())
        .or_else(|| env::var("EDITOR").ok());

    if let Some(cmdline) = editor_env {
        let mut parts = cmdline.split_whitespace();
        if let Some(cmd) = parts.next() {
            let args: Vec<&str> = parts.collect();
            let status = Command::new(cmd).args(&args).arg(path).status();
            match status {
                Ok(s) if s.success() => return Ok(()),
                Ok(_) | Err(_) => {
                    eprintln!("Failed to launch editor from {}", cmdline);
                }
            }
        }
    }

    // Try VS Code (with WSL support)
    if Command::new("code").arg("--version").output().is_ok() {
        // Convert WSL path to Windows path for VS Code if needed
        let path_str = if is_wsl() {
            convert_wsl_path_to_windows(path)?
        } else {
            path.to_string_lossy().to_string()
        };

        println!("Opening in VS Code: {}", path_str);
        let status = Command::new("code").arg("--wait").arg(&path_str).status();
        if let Ok(s) = status
            && s.success()
        {
            return Ok(());
        }
    }

    // Try vim
    if Command::new("vim").arg("--version").output().is_ok() {
        let status = Command::new("vim").arg(path).status();
        if let Ok(s) = status
            && s.success()
        {
            return Ok(());
        }
    }

    // Try nano
    if Command::new("nano").arg("--version").output().is_ok() {
        let status = Command::new("nano").arg(path).status();
        if let Ok(s) = status
            && s.success()
        {
            return Ok(());
        }
    }

    // On Windows fallback to notepad
    if cfg!(windows) {
        let status = Command::new("notepad.exe").arg(path).status();
        if let Ok(s) = status
            && s.success()
        {
            return Ok(());
        }
    }

    anyhow::bail!("Failed to open editor for {}", path.display())
}

/// Detect if running in WSL
fn is_wsl() -> bool {
    std::fs::read_to_string("/proc/version")
        .map(|s| s.to_lowercase().contains("microsoft"))
        .unwrap_or(false)
}

/// Convert WSL path to Windows path for VS Code
fn convert_wsl_path_to_windows(path: &Path) -> Result<String> {
    let output = Command::new("wslpath")
        .arg("-w")
        .arg(path)
        .output()
        .context("Failed to execute wslpath")?;

    if output.status.success() {
        let windows_path = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 from wslpath")?
            .trim()
            .to_string();
        Ok(windows_path)
    } else {
        // Fallback: use the original path
        Ok(path.to_string_lossy().to_string())
    }
}
