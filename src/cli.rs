use anyhow::Result;
use clap::{Arg, ArgMatches, Command as ClapCommand};
use colored::Colorize;

use crate::{display_aliases_table, display_functions_table, get_all_aliases, get_all_functions};

pub fn build_cli() -> ClapCommand {
    ClapCommand::new("shell-explorer")
        .about("🔍 Beautiful shell alias and function explorer for macOS")
        .version("1.0.0")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .help("Mode: 'aliases' (default) or 'functions'")
                .default_value("aliases")
        )
        .arg(
            Arg::new("filter")
                .short('f')
                .long("filter")
                .value_name("PATTERN")
                .help("Filter aliases/functions by name or command (case-insensitive)")
        )
        .arg(
            Arg::new("source")
                .short('s')
                .long("source")
                .value_name("SOURCE")
                .help("Filter by source (.zshrc, .bashrc, etc.)")
        )
        .arg(
            Arg::new("plain")
                .short('p')
                .long("plain")
                .help("Plain text output without colors")
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
