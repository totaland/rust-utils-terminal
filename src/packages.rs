use anyhow::{Context, Result};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use tabled::Tabled;
use regex::Regex;

#[derive(Tabled)]
pub struct PackageEntry {
    #[tabled(rename = "Package")]
    pub name: String,
    #[tabled(rename = "Version")]
    pub version: String,
    #[tabled(rename = "File")]
    pub file_path: String,
    #[tabled(rename = "Type")]
    pub package_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: String,
}

impl Version {
    pub fn parse(version_str: &str) -> Result<Self> {
        let clean_version = version_str.trim_start_matches('v')
            .trim_start_matches('^')
            .trim_start_matches('~')
            .trim_start_matches('=')
            .trim_start_matches('>')
            .trim_start_matches('<')
            .trim_start_matches('*');

        let re = Regex::new(r"^(\d+)(?:\.(\d+))?(?:\.(\d+))?(?:-(.+))?").unwrap();
        
        if let Some(captures) = re.captures(clean_version) {
            let major = captures.get(1).unwrap().as_str().parse().unwrap_or(0);
            let minor = captures.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
            let patch = captures.get(3).map_or(0, |m| m.as_str().parse().unwrap_or(0));
            let pre_release = captures.get(4).map_or(String::new(), |m| m.as_str().to_string());

            Ok(Version {
                major,
                minor,
                patch,
                pre_release,
            })
        } else {
            Err(anyhow::anyhow!("Invalid version format: {}", version_str))
        }
    }

    pub fn is_greater_than(&self, other: &Version) -> bool {
        if self.major != other.major {
            return self.major > other.major;
        }
        if self.minor != other.minor {
            return self.minor > other.minor;
        }
        if self.patch != other.patch {
            return self.patch > other.patch;
        }
        
        // Handle pre-release versions
        match (&self.pre_release.is_empty(), &other.pre_release.is_empty()) {
            (true, false) => true,   // 1.0.0 > 1.0.0-beta
            (false, true) => false,  // 1.0.0-beta < 1.0.0
            (true, true) => false,   // Equal versions
            (false, false) => self.pre_release > other.pre_release, // Compare pre-release strings
        }
    }
}

pub fn find_packages_with_version_greater_than(
    package_name: &str,
    min_version: &str,
    search_path: Option<&str>,
    verbose: bool,
) -> Result<Vec<PackageEntry>> {
    let min_ver = Version::parse(min_version)
        .with_context(|| format!("Invalid version format: {}", min_version))?;
    
    let search_dir = search_path.unwrap_or(".");
    let mut packages = Vec::new();
    
    // Find all package files
    let package_files = find_package_files(search_dir, verbose)?;
    
    // Process files in parallel
    let matching_packages: Vec<PackageEntry> = package_files
        .par_iter()
        .filter_map(|file_path| {
            // Parse each file in parallel
            match parse_package_file(file_path) {
                Ok(file_packages) => {
                    if verbose && !file_packages.is_empty() {
                        println!("✅ Parsed {} packages from {}", file_packages.len(), file_path.display());
                    }
                    let mut matches = Vec::new();
                    for (name, version, pkg_type) in file_packages {
                        if name.to_lowercase() == package_name.to_lowercase() {
                            if let Ok(pkg_version) = Version::parse(&version) {
                                if pkg_version.is_greater_than(&min_ver) {
                                    if verbose {
                                        println!("🎯 Found match: {} v{} in {} ({})", name, version, file_path.display(), pkg_type);
                                    }
                                    matches.push(PackageEntry {
                                        name: name.clone(),
                                        version: version.clone(),
                                        file_path: file_path.to_string_lossy().to_string(),
                                        package_type: pkg_type.clone(),
                                    });
                                }
                            }
                        }
                    }
                    if matches.is_empty() { None } else { Some(matches) }
                }
                Err(e) => {
                    if verbose {
                        println!("❌ Failed to parse {}: {}", file_path.display(), e);
                    }
                    None
                }
            }
        })
        .flatten()
        .collect();
    
    packages.extend(matching_packages);
    
    if verbose {
        println!("📊 Summary: Found {} package files, discovered {} matching packages", 
                 package_files.len(), 
                 packages.len());
    }
    
    // Sort by version (descending)
    packages.sort_by(|a, b| {
        let ver_a = Version::parse(&a.version).unwrap_or(Version { major: 0, minor: 0, patch: 0, pre_release: String::new() });
        let ver_b = Version::parse(&b.version).unwrap_or(Version { major: 0, minor: 0, patch: 0, pre_release: String::new() });
        ver_b.cmp(&ver_a)
    });
    
    Ok(packages)
}

fn find_package_files(search_dir: &str, verbose: bool) -> Result<Vec<PathBuf>> {
    let mut package_files = Vec::new();
    let search_path = PathBuf::from(search_dir);
    
    if search_path.is_file() {
        if is_package_file(&search_path) {
            package_files.push(search_path);
        }
        return Ok(package_files);
    }
    
    // Recursively search for package files
    if verbose {
        println!("📁 Scanning directory: {}", search_path.display());
    }
    find_package_files_recursive(&search_path, &mut package_files, verbose)?;
    
    Ok(package_files)
}

fn find_package_files_recursive(dir: &PathBuf, package_files: &mut Vec<PathBuf>, verbose: bool) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    
    let entries = fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory: {}", dir.display()))?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            // Skip common directories that are unlikely to contain package files we care about
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                if matches!(dir_name, 
                    "node_modules" | "target" | ".git" | "build" | "dist" | 
                    ".next" | ".nuxt" | ".cache" | "coverage" | ".nyc_output" |
                    "__pycache__" | ".pytest_cache" | ".tox" | "venv" | ".venv" |
                    "vendor" | ".bundle" | "tmp" | "temp" | ".tmp" |
                    ".svn" | ".hg" | "CVS" | ".DS_Store" |
                    "bin" | "obj" | "Debug" | "Release" | 
                    ".idea" | ".vscode" | ".vs" | 
                    "logs" | "log" | "*.log"
                ) {
                    if verbose {
                        println!("⏭️  Skipping directory: {}", path.display());
                    }
                } else {
                    if verbose {
                        println!("📂 Scanning subdirectory: {}", path.display());
                    }
                    find_package_files_recursive(&path, package_files, verbose)?;
                }
            }
        } else if is_package_file(&path) {
            if verbose {
                println!("📄 Found package file: {}", path.display());
            }
            package_files.push(path);
        }
    }
    
    Ok(())
}

fn is_package_file(path: &PathBuf) -> bool {
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        matches!(
            file_name,
            "package.json" | "Cargo.toml" | "requirements.txt" | 
            "pyproject.toml" | "Pipfile" | "composer.json" | 
            "pom.xml" | "build.gradle" | "pubspec.yaml" | 
            "go.mod" | "Gemfile"
        )
    } else {
        false
    }
}

fn parse_package_file(file_path: &PathBuf) -> Result<Vec<(String, String, String)>> {
    // Try to read as UTF-8, skip file if it's not valid UTF-8
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            // Skip files that aren't valid UTF-8 (e.g., binary files, different encodings)
            return Ok(Vec::new());
        }
    };
    
    let file_name = file_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");
    
    match file_name {
        "package.json" => parse_package_json(&content),
        "Cargo.toml" => parse_cargo_toml(&content),
        "requirements.txt" => parse_requirements_txt(&content),
        "pyproject.toml" => parse_pyproject_toml(&content),
        "composer.json" => parse_composer_json(&content),
        "go.mod" => parse_go_mod(&content),
        _ => Ok(Vec::new()),
    }
}

fn parse_package_json(content: &str) -> Result<Vec<(String, String, String)>> {
    let mut packages = Vec::new();
    
    // Simple JSON parsing without serde to avoid dependency
    let dependencies_sections = ["dependencies", "devDependencies", "peerDependencies"];
    
    for section in dependencies_sections {
        if let Some(section_start) = content.find(&format!("\"{}\"", section)) {
            if let Some(brace_start) = content[section_start..].find('{') {
                let start_pos = section_start + brace_start + 1;
                if let Some(brace_end) = find_matching_brace(&content[start_pos..]) {
                    let deps_content = &content[start_pos..start_pos + brace_end];
                    
                    let re = Regex::new(r#""([^"]+)":\s*"([^"]+)""#).unwrap();
                    for caps in re.captures_iter(deps_content) {
                        let name = caps[1].to_string();
                        let version = caps[2].to_string();
                        packages.push((name, version, "npm".to_string()));
                    }
                }
            }
        }
    }
    
    Ok(packages)
}

fn parse_cargo_toml(content: &str) -> Result<Vec<(String, String, String)>> {
    let mut packages = Vec::new();
    
    let sections = ["dependencies", "dev-dependencies", "build-dependencies"];
    
    for section in sections {
        if let Some(section_start) = content.find(&format!("[{}]", section)) {
            let section_content = &content[section_start..];
            let section_end = section_content.find("\n[").unwrap_or(section_content.len());
            let section_text = &section_content[..section_end];
            
            // Handle both formats: package = "version" and package = { version = "version" }
            let simple_re = Regex::new(r#"([a-zA-Z0-9_-]+)\s*=\s*"([^"]+)""#).unwrap();
            let complex_re = Regex::new(r#"([a-zA-Z0-9_-]+)\s*=\s*\{[^}]*version\s*=\s*"([^"]+)""#).unwrap();
            
            for caps in simple_re.captures_iter(section_text) {
                let name = caps[1].to_string();
                let version = caps[2].to_string();
                packages.push((name, version, "cargo".to_string()));
            }
            
            for caps in complex_re.captures_iter(section_text) {
                let name = caps[1].to_string();
                let version = caps[2].to_string();
                packages.push((name, version, "cargo".to_string()));
            }
        }
    }
    
    Ok(packages)
}

fn parse_requirements_txt(content: &str) -> Result<Vec<(String, String, String)>> {
    let mut packages = Vec::new();
    
    let re = Regex::new(r"^([a-zA-Z0-9_-]+)[>=<~!]*([0-9]+(?:\.[0-9]+)*(?:\.[0-9]+)?)").unwrap();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some(caps) = re.captures(line) {
            let name = caps[1].to_string();
            let version = caps[2].to_string();
            packages.push((name, version, "pip".to_string()));
        }
    }
    
    Ok(packages)
}

fn parse_pyproject_toml(content: &str) -> Result<Vec<(String, String, String)>> {
    let mut packages = Vec::new();
    
    // Look for dependencies in [tool.poetry.dependencies] or [project.dependencies]
    let sections = ["[tool.poetry.dependencies]", "[project.dependencies]"];
    
    for section in sections {
        if let Some(section_start) = content.find(section) {
            let section_content = &content[section_start..];
            let section_end = section_content.find("\n[").unwrap_or(section_content.len());
            let section_text = &section_content[..section_end];
            
            let re = Regex::new(r#"([a-zA-Z0-9_-]+)\s*=\s*"([^"]+)""#).unwrap();
            
            for caps in re.captures_iter(section_text) {
                let name = caps[1].to_string();
                let version = caps[2].to_string();
                if name != "python" { // Skip python version specification
                    packages.push((name, version, "poetry".to_string()));
                }
            }
        }
    }
    
    Ok(packages)
}

fn parse_composer_json(content: &str) -> Result<Vec<(String, String, String)>> {
    let mut packages = Vec::new();
    
    let sections = ["require", "require-dev"];
    
    for section in sections {
        if let Some(section_start) = content.find(&format!("\"{}\"", section)) {
            if let Some(brace_start) = content[section_start..].find('{') {
                let start_pos = section_start + brace_start + 1;
                if let Some(brace_end) = find_matching_brace(&content[start_pos..]) {
                    let deps_content = &content[start_pos..start_pos + brace_end];
                    
                    let re = Regex::new(r#""([^"]+)":\s*"([^"]+)""#).unwrap();
                    for caps in re.captures_iter(deps_content) {
                        let name = caps[1].to_string();
                        let version = caps[2].to_string();
                        packages.push((name, version, "composer".to_string()));
                    }
                }
            }
        }
    }
    
    Ok(packages)
}

fn parse_go_mod(content: &str) -> Result<Vec<(String, String, String)>> {
    let mut packages = Vec::new();
    
    let re = Regex::new(r"([a-zA-Z0-9./\-_]+)\s+v([0-9]+\.[0-9]+\.[0-9]+[^\s]*)").unwrap();
    
    for caps in re.captures_iter(content) {
        let name = caps[1].to_string();
        let version = caps[2].to_string();
        packages.push((name, version, "go".to_string()));
    }
    
    Ok(packages)
}

fn find_matching_brace(content: &str) -> Option<usize> {
    let mut brace_count = 1;
    let mut in_string = false;
    let mut escaped = false;
    
    for (i, ch) in content.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        
        match ch {
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            '{' if !in_string => brace_count += 1,
            '}' if !in_string => {
                brace_count -= 1;
                if brace_count == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    
    None
}