use anyhow::Result;
use clap::{Arg, ArgMatches, Command as ClapCommand};
use colored::Colorize;

use crate::{display_aliases_table, display_functions_table, display_packages_table, get_all_aliases, get_all_functions, find_packages_with_version_greater_than};

pub fn build_cli() -> ClapCommand {
    ClapCommand::new("shell-explorer")
        .about("🔍 Beautiful shell alias, function, and package explorer for macOS")
        .long_about("A comprehensive tool for exploring shell aliases, functions, and package versions.

MODES:
  aliases   - Show shell aliases from config files and current session
  functions - Show shell functions with documentation from config files  
  packages  - Find package versions greater than a specified threshold

EXAMPLES:
  shell-explorer                                    # Show all aliases (default)
  shell-explorer --mode functions --filter git     # Show functions containing 'git'
  shell-explorer --mode packages --package react --min-version 17.0.0
  shell-explorer --mode packages --package typescript --min-version 4.0.0 --path ./src")
        .version("1.0.0")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .help("Mode: 'aliases' (default), 'functions', or 'packages'")
                .default_value("aliases")
        )
        .arg(
            Arg::new("filter")
                .short('f')
                .long("filter")
                .value_name("PATTERN")
                .help("Filter aliases/functions by name or command (case-insensitive, not used in packages mode)")
        )
        .arg(
            Arg::new("source")
                .short('s')
                .long("source")
                .value_name("SOURCE")
                .help("Filter by source file (.zshrc, .bashrc, etc. - not used in packages mode)")
        )
        .arg(
            Arg::new("plain")
                .short('p')
                .long("plain")
                .help("Plain text output without colors")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("package")
                .long("package")
                .value_name("PACKAGE_NAME")
                .help("Package name to search for (required for packages mode)")
                .long_help("Package name to search for across all discovered package files. Case-insensitive matching.")
                .required_if_eq("mode", "packages")
        )
        .arg(
            Arg::new("min_version")
                .long("min-version")
                .value_name("VERSION")
                .help("Minimum version threshold - show packages with versions greater than this (required for packages mode)")
                .long_help("Minimum version threshold using semantic versioning. Only packages with versions greater than this will be shown. Supports formats like: 1.0.0, 2.1.3, 0.5.0-beta, etc.")
                .required_if_eq("mode", "packages")
        )
        .arg(
            Arg::new("path")
                .long("path")
                .value_name("SEARCH_PATH")
                .help("Path to search for package files (defaults to current directory)")
                .long_help("Directory path to search for package files. Recursively searches subdirectories but excludes common build/cache directories (node_modules, target, .git, etc.)")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Show verbose output including directories and files being scanned")
                .action(clap::ArgAction::SetTrue)
        )
}

pub fn handle_aliases_mode(matches: &ArgMatches) -> Result<()> {
    let mut aliases = get_all_aliases()?;
    
    // Apply filters
    if let Some(filter_pattern) = matches.get_one::<String>("filter") {
        let pattern = filter_pattern.to_lowercase();
        aliases.retain(|alias| {
            alias.alias.to_lowercase().contains(&pattern) || 
            alias.command.to_lowercase().contains(&pattern)
        });
        println!("{} Filtering by: {}", "🔍".cyan(), filter_pattern.yellow());
    }
    
    if let Some(source_filter) = matches.get_one::<String>("source") {
        aliases.retain(|alias| alias.source.contains(source_filter));
        println!("{} Filtering by source: {}", "📁".cyan(), source_filter.yellow());
    }
    
    if aliases.is_empty() {
        println!("{}", "No aliases found matching your criteria.".yellow());
        return Ok(());
    }

    let alias_count = aliases.len();
    let use_colors = !matches.get_flag("plain");
    display_aliases_table(aliases, use_colors)?;
    
    println!("\n{} Found {} aliases", "✨".green(), alias_count.to_string().bold());
    Ok(())
}

pub fn handle_functions_mode(matches: &ArgMatches) -> Result<()> {
    let mut functions = get_all_functions()?;
    
    // Apply filters
    if let Some(filter_pattern) = matches.get_one::<String>("filter") {
        let pattern = filter_pattern.to_lowercase();
        functions.retain(|func| {
            func.name.to_lowercase().contains(&pattern) || 
            func.description.to_lowercase().contains(&pattern) ||
            func.usage.to_lowercase().contains(&pattern)
        });
        println!("{} Filtering by: {}", "🔍".cyan(), filter_pattern.yellow());
    }
    
    if let Some(source_filter) = matches.get_one::<String>("source") {
        functions.retain(|func| func.source.contains(source_filter));
        println!("{} Filtering by source: {}", "📁".cyan(), source_filter.yellow());
    }
    
    if functions.is_empty() {
        println!("{}", "No functions found matching your criteria.".yellow());
        return Ok(());
    }

    let function_count = functions.len();
    let use_colors = !matches.get_flag("plain");
    display_functions_table(functions, use_colors)?;
    
    println!("\n{} Found {} functions", "✨".green(), function_count.to_string().bold());
    Ok(())
}

pub fn handle_packages_mode(matches: &ArgMatches) -> Result<()> {
    let package_name = matches.get_one::<String>("package").unwrap();
    let min_version = matches.get_one::<String>("min_version").unwrap();
    let search_path = matches.get_one::<String>("path").map(|s| s.as_str());
    let verbose = matches.get_flag("verbose");
    
    println!("{} Searching for package '{}' with version > {}", 
        "🔍".cyan(), 
        package_name.yellow(), 
        min_version.green()
    );
    
    if let Some(path) = search_path {
        println!("{} Search path: {}", "📁".cyan(), path.yellow());
    }
    
    if verbose {
        println!("{} Verbose mode enabled - showing scan details", "🔍".cyan());
    }
    
    let packages = find_packages_with_version_greater_than(package_name, min_version, search_path, verbose)?;
    
    if packages.is_empty() {
        println!("{}", format!(
            "No packages named '{}' found with version greater than '{}'", 
            package_name, min_version
        ).yellow());
        return Ok(());
    }
    
    let package_count = packages.len();
    let use_colors = !matches.get_flag("plain");
    display_packages_table(packages, use_colors)?;
    
    println!("\n{} Found {} package instances", "✨".green(), package_count.to_string().bold());
    Ok(())
}
