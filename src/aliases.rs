use anyhow::{Context, Result};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tabled::Tabled;

#[derive(Tabled)]
pub struct AliasEntry {
    #[tabled(rename = "Alias")]
    pub alias: String,
    #[tabled(rename = "Command")]
    pub command: String,
    #[tabled(rename = "Source")]
    pub source: String,
}

pub fn get_all_aliases() -> Result<Vec<AliasEntry>> {
    let mut aliases = Vec::new();
    
    // Get aliases from current shell session
    if let Ok(shell_aliases) = get_shell_aliases() {
        for (alias, command) in shell_aliases {
            aliases.push(AliasEntry {
                alias,
                command,
                source: "Shell Session".to_string(),
            });
        }
    }
    
    // Get aliases from shell configuration files
    let config_aliases = get_config_file_aliases()?;
    for (alias, command, source) in config_aliases {
        // Avoid duplicates by checking if alias already exists
        if !aliases.iter().any(|a| a.alias == alias) {
            aliases.push(AliasEntry {
                alias,
                command,
                source,
            });
        }
    }
    
    // Sort aliases alphabetically
    aliases.sort_by(|a, b| a.alias.cmp(&b.alias));
    
    Ok(aliases)
}

fn get_shell_aliases() -> Result<HashMap<String, String>> {
    // Try to get current shell from environment
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    
    let output = Command::new(&shell)
        .arg("-c")
        .arg("alias")
        .output()
        .context("Failed to execute alias command")?;
    
    if !output.status.success() {
        return Ok(HashMap::new());
    }
    
    let alias_output = String::from_utf8(output.stdout)
        .context("Failed to parse alias output")?;
    
    parse_alias_output(&alias_output)
}

fn parse_alias_output(output: &str) -> Result<HashMap<String, String>> {
    let mut aliases = HashMap::new();
    
    for line in output.lines() {
        if let Some((alias, command)) = parse_alias_line(line) {
            aliases.insert(alias, command);
        }
    }
    
    Ok(aliases)
}

fn parse_alias_line(line: &str) -> Option<(String, String)> {
    // Handle different alias formats:
    // alias name='command'
    // alias name="command"
    // alias name=command
    
    if !line.starts_with("alias ") {
        return None;
    }
    
    let line = &line[6..]; // Remove "alias "
    
    if let Some(eq_pos) = line.find('=') {
        let alias = line[..eq_pos].trim().to_string();
        let mut command = line[eq_pos + 1..].trim().to_string();
        
        // Remove quotes if present
        if (command.starts_with('\'') && command.ends_with('\''))
            || (command.starts_with('"') && command.ends_with('"'))
        {
            command = command[1..command.len() - 1].to_string();
        }
        
        Some((alias, command))
    } else {
        None
    }
}

fn get_config_file_aliases() -> Result<Vec<(String, String, String)>> {
    let mut aliases = Vec::new();
    let home_dir = env::var("HOME").context("HOME environment variable not set")?;
    
    // Common shell configuration files
    let config_files = vec![
        ".bashrc",
        ".bash_profile",
        ".bash_aliases",
        ".zshrc",
        ".zsh_aliases",
        ".profile",
        ".aliases",
    ];
    
    for config_file in config_files {
        let file_path = PathBuf::from(&home_dir).join(config_file);
        
        if file_path.exists() {
            if let Ok(content) = fs::read_to_string(&file_path) {
                let file_aliases = parse_config_file_aliases(&content);
                for (alias, command) in file_aliases {
                    aliases.push((alias, command, config_file.to_string()));
                }
            }
        }
    }
    
    Ok(aliases)
}

fn parse_config_file_aliases(content: &str) -> Vec<(String, String)> {
    let mut aliases = Vec::new();
    
    for line in content.lines() {
        let line = line.trim();
        
        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        // Look for alias definitions
        if line.starts_with("alias ") {
            if let Some((alias, command)) = parse_alias_line(line) {
                aliases.push((alias, command));
            }
        }
    }
    
    aliases
}
