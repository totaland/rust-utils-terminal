use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;
use tabled::Tabled;

#[derive(Tabled)]
pub struct FunctionEntry {
    #[tabled(rename = "Function Name")]
    pub name: String,
    #[tabled(rename = "Description")]
    pub description: String,
    #[tabled(rename = "Usage")]
    pub usage: String,
    #[tabled(rename = "Source")]
    pub source: String,
}

pub fn get_all_functions() -> Result<Vec<FunctionEntry>> {
    let mut functions = Vec::new();
    let home_dir = env::var("HOME").context("HOME environment variable not set")?;
    
    // Common shell configuration files that might contain functions
    let config_files = vec![
        ".zshrc",
        ".bashrc",
        ".bash_profile",
        ".profile",
        ".zsh_functions",
        ".bash_functions",
    ];
    
    for config_file in config_files {
        let file_path = PathBuf::from(&home_dir).join(config_file);
        
        if file_path.exists() {
            if let Ok(content) = fs::read_to_string(&file_path) {
                let file_functions = parse_shell_functions(&content);
                for (name, description, usage) in file_functions {
                    functions.push(FunctionEntry {
                        name,
                        description,
                        usage,
                        source: config_file.to_string(),
                    });
                }
            }
        }
    }
    
    // Sort functions alphabetically
    functions.sort_by(|a, b| a.name.cmp(&b.name));
    
    Ok(functions)
}

fn parse_shell_functions(content: &str) -> Vec<(String, String, String)> {
    let mut functions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        // Look for function definitions (various formats)
        if let Some(func_name) = extract_function_name(line) {
            let mut description = String::new();
            let mut usage = String::new();
            let mut function_body = Vec::new();
            
            // Look backwards for comments that might be documentation
            let mut j = i.saturating_sub(1);
            let mut comments = Vec::new();
            let mut in_comment_block = false;
            
            // Collect comments above the function
            while j > 0 {
                let comment_line = lines[j].trim();
                
                if comment_line.starts_with('#') {
                    let comment = comment_line.trim_start_matches('#').trim();
                    if !comment.is_empty() {
                        comments.insert(0, comment);
                        in_comment_block = true;
                    }
                } else if in_comment_block && comment_line.is_empty() {
                    // Allow empty lines within comment blocks
                    comments.insert(0, "");
                } else if in_comment_block && !comment_line.is_empty() {
                    // Hit non-comment, non-empty line - end of comment block
                    break;
                } else if !in_comment_block && !comment_line.is_empty() {
                    // No comments found
                    break;
                }
                
                if j == 0 { break; }
                j -= 1;
            }
            
            // Parse comments for description and usage
            for comment in &comments {
                let lower_comment = comment.to_lowercase();
                if lower_comment.starts_with("usage:") || lower_comment.starts_with("use:") {
                    let start = if lower_comment.starts_with("usage:") { 6 } else { 4 };
                    usage = comment[start..].trim().to_string();
                } else if lower_comment.starts_with("desc:") || 
                         lower_comment.starts_with("description:") ||
                         lower_comment.starts_with("@desc") ||
                         lower_comment.starts_with("@description") {
                    let start = match lower_comment {
                        s if s.starts_with("desc:") => 5,
                        s if s.starts_with("description:") => 12,
                        s if s.starts_with("@desc") => 5,
                        s if s.starts_with("@description") => 12,
                        _ => 0,
                    };
                    description = comment[start..].trim().to_string();
                } else if lower_comment.starts_with("@param") || lower_comment.starts_with("@arg") {
                    // JSDoc-style parameter documentation - add to usage if empty
                    if usage.is_empty() {
                        let param_info = comment[6..].trim();
                        if !param_info.is_empty() {
                            usage = format!("{} {}", func_name, param_info);
                        }
                    }
                } else if description.is_empty() && !comment.is_empty() {
                    // Use first non-empty comment as description if no explicit description
                    description = comment.to_string();
                }
            }
            
            // Find the function body to extract more info
            i += 1;
            let mut brace_count = 0;
            let mut in_function = false;
            
            while i < lines.len() {
                let current_line = lines[i].trim();
                
                if current_line.contains('{') {
                    in_function = true;
                    brace_count += current_line.matches('{').count();
                }
                
                if in_function {
                    brace_count -= current_line.matches('}').count();
                    function_body.push(current_line);
                    
                    if brace_count == 0 {
                        break;
                    }
                }
                
                i += 1;
            }
            
            // Extract usage from function body if not found in comments
            if usage.is_empty() {
                usage = extract_usage_from_body(&function_body, &func_name);
            }
            
            // Use function name as description if no description found
            if description.is_empty() {
                description = format!("Function: {}", func_name);
            }
            
            // Clean up descriptions that are too long
            if description.len() > 80 {
                description = format!("{}...", &description[..77]);
            }
            
            functions.push((func_name, description, usage));
        }
        
        i += 1;
    }
    
    functions
}

fn extract_function_name(line: &str) -> Option<String> {
    // Match various function definition patterns:
    // function name() { ... }
    // name() { ... }
    // function name { ... }
    
    if line.starts_with("function ") {
        // function name() or function name
        let after_function = &line[9..];
        if let Some(space_or_paren) = after_function.find(|c| c == ' ' || c == '(' || c == '{') {
            return Some(after_function[..space_or_paren].trim().to_string());
        }
    } else if line.contains("()") && (line.contains('{') || line.ends_with("()")) {
        // name() format
        if let Some(paren_pos) = line.find("()") {
            let potential_name = line[..paren_pos].trim();
            // Make sure it's a valid function name (starts with letter or underscore)
            if potential_name.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_') &&
               potential_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Some(potential_name.to_string());
            }
        }
    }
    
    None
}

fn extract_usage_from_body(body: &[&str], func_name: &str) -> String {
    let mut params = Vec::new();
    
    // Look for common usage patterns in function body
    for line in body {
        let line = line.trim();
        
        // Look for echo statements that might show usage
        if line.starts_with("echo") && (line.contains("Usage:") || line.contains("usage:")) {
            if let Some(usage_start) = line.to_lowercase().find("usage:") {
                let usage_part = &line[usage_start + 6..];
                return usage_part.trim_matches('"').trim_matches('\'').trim().to_string();
            }
        }
        
        // Look for printf statements with usage
        if line.starts_with("printf") && (line.contains("Usage:") || line.contains("usage:")) {
            if let Some(usage_start) = line.to_lowercase().find("usage:") {
                let usage_part = &line[usage_start + 6..];
                return usage_part.trim_matches('"').trim_matches('\'').trim().to_string();
            }
        }
        
        // Look for variable assignments that indicate parameters
        if line.starts_with("local ") && (line.contains("=$1") || line.contains("=${1") || line.contains("$1")) {
            params.push("arg1");
        }
        if line.contains("=$2") || line.contains("=${2") {
            params.push("arg2");
        }
        if line.contains("=$3") || line.contains("=${3") {
            params.push("arg3");
        }
        
        // Look for parameter checks
        if line.contains("$#") && (line.contains("-eq") || line.contains("-lt") || line.contains("-gt")) {
            // Function checks argument count
            if line.contains("-eq 1") {
                params.push("<arg>");
            } else if line.contains("-eq 2") {
                params.push("<arg1> <arg2>");
            } else if line.contains("-eq 3") {
                params.push("<arg1> <arg2> <arg3>");
            } else if line.contains("-lt") || line.contains("-gt") {
                params.push("[args...]");
            }
        }
        
        // Look for getopts usage
        if line.contains("getopts") {
            return format!("{} [options]", func_name);
        }
        
        // Look for shift commands (indicates parameter processing)
        if line.trim() == "shift" || line.contains("shift ") {
            if !params.contains(&"[args...]") {
                params.push("[args...]");
            }
        }
    }
    
    // Construct usage string
    if !params.is_empty() {
        format!("{} {}", func_name, params.join(" "))
    } else {
        func_name.to_string()
    }
}
