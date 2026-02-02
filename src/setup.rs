use anyhow::{Context, Result, bail};
use dialoguer::{MultiSelect, theme::ColorfulTheme};
use std::fs;
use std::path::{Path, PathBuf};

use crate::transcript::{Tool, codex_home_dir};

// Embed files at compile time
const CLAUDE_SKILL: &str = include_str!("../skills/claude/agentexport.md");
const CODEX_SKILL: &str = include_str!("../skills/codex/agentexport.md");

pub fn run() -> Result<()> {
    let theme = ColorfulTheme::default();

    // Detect installed tools
    let claude_path = find_in_path("claude");
    let codex_path = find_in_path("codex");

    if claude_path.is_none() && codex_path.is_none() {
        bail!("Neither claude nor codex found in PATH");
    }

    // Show what will be installed
    println!("This will install:");
    if claude_path.is_some() {
        println!("  Claude Code: agentexport skill");
    }
    if codex_path.is_some() {
        println!("  Codex: agentexport skill");
    }
    println!();

    // Build selection list (only installed tools)
    let mut items: Vec<(Tool, String)> = Vec::new();
    let mut defaults = Vec::new();

    if let Some(path) = &claude_path {
        items.push((Tool::Claude, format!("Claude Code ({})", path.display())));
        defaults.push(true);
    }
    if let Some(path) = &codex_path {
        items.push((Tool::Codex, format!("Codex ({})", path.display())));
        defaults.push(true);
    }

    let labels: Vec<&str> = items.iter().map(|(_, label)| label.as_str()).collect();

    let selected = MultiSelect::with_theme(&theme)
        .with_prompt("Select tools to configure")
        .items(&labels)
        .defaults(&defaults)
        .interact()?;

    if selected.is_empty() {
        println!("Nothing selected.");
        return Ok(());
    }

    println!();

    for index in selected {
        let (tool, _) = &items[index];
        match tool {
            Tool::Claude => {
                install_claude_skill()?;
            }
            Tool::Codex => {
                install_codex_skill()?;
            }
        }
    }

    println!();
    println!("Done! Restart Claude Code / Codex to pick up changes.");

    Ok(())
}

fn install_claude_skill() -> Result<()> {
    let skill_dir = ensure_claude_skills_dir()?.join("agentexport");
    let dest = skill_dir.join("SKILL.md");

    // Clean up old command location if it exists
    let old_command = claude_home_dir()?.join("commands").join("agentexport.md");
    if old_command.exists() {
        fs::remove_file(&old_command)?;
        println!("Removed old command at {}.", old_command.display());
    }

    if dest.exists() {
        println!(
            "Skipping Claude skill (already installed at {}).",
            dest.display()
        );
        return Ok(());
    }
    fs::create_dir_all(&skill_dir)?;
    fs::write(&dest, CLAUDE_SKILL)?;
    println!("Installed Claude skill to {}.", dest.display());
    Ok(())
}

fn install_codex_skill() -> Result<()> {
    let skill_dir = ensure_codex_skills_dir()?.join("agentexport");
    let dest = skill_dir.join("SKILL.md");

    // Clean up old prompt location if it exists
    let old_prompt = codex_home_dir()?.join("prompts").join("agentexport.md");
    if old_prompt.exists() {
        fs::remove_file(&old_prompt)?;
        println!("Removed old prompt at {}.", old_prompt.display());
    }

    if dest.exists() {
        println!(
            "Skipping Codex skill (already installed at {}).",
            dest.display()
        );
        return Ok(());
    }
    fs::create_dir_all(&skill_dir)?;
    fs::write(&dest, CODEX_SKILL)?;
    println!("Installed Codex skill to {}.", dest.display());
    Ok(())
}

fn claude_home_dir() -> Result<PathBuf> {
    let home = std::env::var("HOME").context("HOME not set")?;
    Ok(PathBuf::from(home).join(".claude"))
}

fn ensure_claude_skills_dir() -> Result<PathBuf> {
    let dir = claude_home_dir()?.join("skills");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn ensure_codex_skills_dir() -> Result<PathBuf> {
    let dir = codex_home_dir()?.join("skills");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn find_in_path(binary: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(binary);
        if candidate.is_file() && is_executable(&candidate) {
            return Some(candidate);
        }
    }
    None
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|meta| meta.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}
