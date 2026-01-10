use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

/// Template structure describing a project architecture.
///
/// - `name`: human friendly template name
/// - `description`: short description
/// - `structure`: list of folders to create
/// - `files`: mapping of filename -> kind/hint
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub structure: Vec<String>,
    pub files: HashMap<String, String>,
}

/// Load templates from the embedded `templates/architectures.json` file.
pub fn load_templates() -> Result<HashMap<String, Template>> {
    let json_data = include_str!("../templates/architectures.json");
    let templates: HashMap<String, Template> =
        serde_json::from_str(json_data).context("Failed to parse templates")?;

    // Merge user templates from the user's local_templates.json (user wins)
    let mut merged = templates;
    // Migrate any per-key files in the user templates dir into local_templates.json
    if let Ok(dir) = get_user_templates_dir()
        && dir.exists()
    {
        let mut migrated_any = false;
        let mut migrated_map: HashMap<String, Template> = HashMap::new();
        for entry in std::fs::read_dir(&dir)
            .context("Failed to read user templates directory for migration")?
        {
            let entry = entry.context("Failed to read directory entry during migration")?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                if path.file_name().and_then(|s| s.to_str()) == Some("local_templates.json") {
                    continue;
                }
                // Try to interpret file as either a single template (object) or a map
                let content = std::fs::read_to_string(&path).with_context(|| {
                    format!("Failed to read {} during migration", path.display())
                })?;
                // First try as map
                if let Ok(map) = serde_json::from_str::<HashMap<String, Template>>(&content) {
                    for (k, v) in map {
                        migrated_map.insert(k, v);
                    }
                } else if let Ok(t) = serde_json::from_str::<Template>(&content) {
                    // Use filename (without ext) as key
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        migrated_map.insert(stem.to_string(), t);
                    }
                } else {
                    // skip unknown format
                    continue;
                }
                migrated_any = true;
                // remove old per-key file after ingesting
                std::fs::remove_file(&path).with_context(|| {
                    format!("Failed to remove migrated file {}", path.display())
                })?;
            }
        }
        if migrated_any {
            // Merge migrated_map into existing local_templates.json (write complete file)
            let local_file = get_user_local_templates_file()?;
            // load existing local map if present
            let mut existing: HashMap<String, Template> = if local_file.exists() {
                let s = std::fs::read_to_string(&local_file)
                    .with_context(|| format!("Failed to read existing {}", local_file.display()))?;
                serde_json::from_str(&s)
                    .with_context(|| format!("Failed to parse existing {}", local_file.display()))?
            } else {
                HashMap::new()
            };
            for (k, v) in migrated_map {
                existing.insert(k, v);
            }
            let out = serde_json::to_string_pretty(&existing)
                .context("Failed to serialize merged local templates")?;
            if let Some(p) = local_file.parent() {
                std::fs::create_dir_all(p).ok();
            }
            std::fs::write(&local_file, out).with_context(|| {
                format!(
                    "Failed to write merged local templates to {}",
                    local_file.display()
                )
            })?;
        }
    }

    if let Ok(file) = get_user_local_templates_file()
        && file.exists()
    {
        let mut s = String::new();
        let mut f = File::open(&file)
            .with_context(|| format!("Failed to open local templates file {}", file.display()))?;
        f.read_to_string(&mut s)
            .context("Failed to read local templates file")?;
        let map: HashMap<String, Template> = serde_json::from_str(&s)
            .with_context(|| format!("Failed to parse local templates JSON {}", file.display()))?;
        for (k, v) in map {
            merged.insert(k, v);
        }
    }

    Ok(merged)
}

/// Pretty-print available templates to stdout.
pub fn list_templates() -> Result<()> {
    let templates = load_templates()?;

    println!("\n📦 Available architectures:\n");
    for (key, template) in templates.iter() {
        println!("  {} - {}", key, template.name);
        println!("    {}\n", template.description);
    }

    Ok(())
}

/// Return the user templates directory, e.g. $XDG_CONFIG_HOME/forge/templates or %APPDATA%\forge\templates
pub fn get_user_templates_dir() -> Result<PathBuf> {
    // Prefer XDG_CONFIG_HOME on unix, APPDATA on windows, fallback to HOME/.config
    if cfg!(windows) {
        if let Ok(appdata) = env::var("APPDATA") {
            let p = PathBuf::from(appdata).join("forge").join("templates");
            return Ok(p);
        }
    } else {
        if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
            let p = PathBuf::from(xdg).join("forge").join("templates");
            return Ok(p);
        }
        if let Ok(home) = env::var("HOME") {
            let p = PathBuf::from(home)
                .join(".config")
                .join("forge")
                .join("templates");
            return Ok(p);
        }
    }

    anyhow::bail!("Could not determine user config directory for templates")
}

pub fn get_user_local_templates_file() -> Result<PathBuf> {
    let dir = get_user_templates_dir()?;
    Ok(dir.join("local_templates.json"))
}

pub fn save_local_template(key: &str, template: &Template) -> Result<()> {
    validate_template(template)?;
    let file = get_user_local_templates_file()?;

    if let Some(dir) = file.parent() {
        create_dir_all(dir)
            .with_context(|| format!("Failed to create templates directory {}", dir.display()))?;
    }

    // Load existing map if present, otherwise create empty map
    let mut map: HashMap<String, Template> = if file.exists() {
        let content = std::fs::read_to_string(&file)
            .with_context(|| format!("Failed to read {}", file.display()))?;

        // Try to parse, if fails start with empty map
        serde_json::from_str(&content).unwrap_or_else(|_| HashMap::new())
    } else {
        HashMap::new()
    };

    // Insert or update the template
    map.insert(key.to_string(), template.clone());

    // Write back with pretty formatting
    let json_content =
        serde_json::to_string_pretty(&map).context("Failed to serialize templates")?;

    std::fs::write(&file, json_content)
        .with_context(|| format!("Failed to write to {}", file.display()))?;

    Ok(())
}

pub fn remove_local_template(key: &str) -> Result<bool> {
    let file = get_user_local_templates_file()?;
    if !file.exists() {
        return Ok(false);
    }
    let mut s = String::new();
    let mut f = File::open(&file)
        .with_context(|| format!("Failed to open local templates file {}", file.display()))?;
    f.read_to_string(&mut s)
        .context("Failed to read local templates file")?;
    let mut map: HashMap<String, Template> = serde_json::from_str(&s)
        .with_context(|| format!("Failed to parse local templates JSON {}", file.display()))?;
    if map.remove(key).is_some() {
        let s2 = serde_json::to_string_pretty(&map)
            .context("Failed to serialize updated local templates")?;
        let mut f2 = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file)
            .with_context(|| {
                format!(
                    "Failed to open local templates file {} for writing",
                    file.display()
                )
            })?;
        f2.write_all(s2.as_bytes())
            .context("Failed to write updated local templates")?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Validate template fields for safety (no parent dir components, no absolute paths)
pub fn validate_template(t: &Template) -> Result<()> {
    for s in &t.structure {
        let p = Path::new(s);
        if p.is_absolute() {
            anyhow::bail!("structure entry '{}' must not be absolute", s);
        }
        for comp in p.components() {
            if let Component::ParentDir = comp {
                anyhow::bail!(
                    "structure entry '{}' must not contain parent directory '..'",
                    s
                );
            }
        }
    }
    for fname in t.files.keys() {
        let p = Path::new(fname);
        if p.is_absolute() {
            anyhow::bail!("file '{}' must not be absolute", fname);
        }
        for comp in p.components() {
            if let Component::ParentDir = comp {
                anyhow::bail!("file '{}' must not contain parent directory '..'", fname);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_templates() {
        let templates = load_templates().expect("Failed to load templates");
        assert!(templates.contains_key("backend-api"));
        assert!(templates.contains_key("frontend-react"));
    }

    #[test]
    fn test_invalid_template_key() {
        let templates = load_templates().expect("Failed to load templates");
        assert!(!templates.contains_key("nonexistent-template"));
    }
}
