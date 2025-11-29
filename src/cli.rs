use anyhow::Result;
use clap::{Arg, ArgMatches, Command as ClapCommand};
use colored::Colorize;

use crate::{
    clean_node_modules, display_aliases_table, display_bookmarks_table,
    display_category_stats_table, display_cleaned_table, display_dead_links_table,
    display_domain_stats_table, display_duplicates_table, display_functions_table,
    display_organize_suggestions_table, display_organize_table, display_packages_table,
    export_to_chrome_html, export_to_markdown, filter_by_category, filter_by_domain,
    find_dead_links, find_duplicates, find_packages_with_version_greater_than, get_all_aliases,
    get_all_functions, get_bookmark_stats, get_category_stats, get_domain_stats,
    get_organize_suggestions, organize_files, parse_bookmarks, remove_dead_links,
    remove_duplicates, search_bookmarks,
};

pub fn build_cli() -> ClapCommand {
    ClapCommand::new("shell-explorer")
        .about("🔍 Beautiful shell alias, function, and package explorer for macOS")
        .long_about("A comprehensive tool for exploring shell aliases, functions, and package versions.

MODES:
  aliases   - Show shell aliases from config files and current session
  functions - Show shell functions with documentation from config files  
  packages  - Find package versions greater than a specified threshold
  clean     - Remove all node_modules directories recursively (parallel)
  organize  - Organize files in non-development folders by type
  bookmarks - Organize and analyze Chrome bookmarks

BOOKMARK SUBCOMMANDS:
  bookmarks stats           - Show bookmark statistics (domains, categories, duplicates)
  bookmarks duplicates      - Find duplicate bookmarks
  bookmarks remove-dupes    - Remove duplicate bookmarks (interactive)
  bookmarks deadlinks       - Check for dead/broken links
  bookmarks remove-dead     - Remove dead links (interactive)
  bookmarks domains         - Show bookmarks grouped by domain
  bookmarks categories      - Show bookmarks grouped by category
  bookmarks search          - Search bookmarks by query
  bookmarks organize        - Get organization suggestions
  bookmarks export          - Export bookmarks to markdown
  bookmarks export-html     - Export organized bookmarks to Chrome-importable HTML

EXAMPLES:
  shell-explorer                                    # Show all aliases (default)
  shell-explorer --mode functions --filter git     # Show functions containing 'git'
  shell-explorer --mode packages --package react --min-version 17.0.0
  shell-explorer --mode packages --package typescript --min-version 4.0.0 --path ./src
  shell-explorer --mode clean --path ./projects    # Remove all node_modules
  shell-explorer --mode clean --dry-run            # Preview what would be removed
  shell-explorer --mode clean --interactive        # Select which node_modules to delete
  shell-explorer --mode organize --path ~/Downloads # Organize files in Downloads
  shell-explorer --mode organize --dry-run          # Preview organization
  shell-explorer --mode bookmarks --subcommand stats           # Show bookmark stats
  shell-explorer --mode bookmarks --subcommand duplicates      # Find duplicates
  shell-explorer --mode bookmarks --subcommand remove-dupes    # Remove duplicates (confirm)
  shell-explorer --mode bookmarks --subcommand deadlinks       # Check for dead links
  shell-explorer --mode bookmarks --subcommand remove-dead     # Remove dead links (confirm)
  shell-explorer --mode bookmarks --subcommand search --query github  # Search bookmarks
  shell-explorer --mode bookmarks --subcommand export --output bookmarks.md")
        .version("1.0.0")
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .help("Mode: 'aliases' (default), 'functions', 'packages', 'clean', 'organize', or 'bookmarks'")
                .default_value("aliases")
        )
        .arg(
            Arg::new("subcommand")
                .long("subcommand")
                .value_name("SUBCOMMAND")
                .help("Subcommand for bookmarks mode: 'stats', 'duplicates', 'remove-dupes', 'deadlinks', 'remove-dead', 'domains', 'categories', 'search', 'organize', 'export', 'export-html'")
        )
        .arg(
            Arg::new("query")
                .short('q')
                .long("query")
                .value_name("QUERY")
                .help("Search query for bookmarks search mode")
        )
        .arg(
            Arg::new("category")
                .short('c')
                .long("category")
                .value_name("CATEGORY")
                .help("Filter by category (for bookmarks mode)")
        )
        .arg(
            Arg::new("domain")
                .short('d')
                .long("domain")
                .value_name("DOMAIN")
                .help("Filter by domain (for bookmarks mode)")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Output file path (for bookmarks export)")
        )
        .arg(
            Arg::new("limit")
                .short('l')
                .long("limit")
                .value_name("LIMIT")
                .help("Limit number of results")
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
        .arg(
            Arg::new("dry_run")
                .long("dry-run")
                .help("Preview what would be removed without actually deleting (for clean mode)")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("Interactive mode: select which node_modules to delete (for clean mode)")
                .action(clap::ArgAction::SetTrue)
        )
}

pub fn handle_aliases_mode(matches: &ArgMatches) -> Result<()> {
    let mut aliases = get_all_aliases()?;

    // Apply filters
    if let Some(filter_pattern) = matches.get_one::<String>("filter") {
        let pattern = filter_pattern.to_lowercase();
        aliases.retain(|alias| {
            alias.alias.to_lowercase().contains(&pattern)
                || alias.command.to_lowercase().contains(&pattern)
        });
        println!("{} Filtering by: {}", "🔍".cyan(), filter_pattern.yellow());
    }

    if let Some(source_filter) = matches.get_one::<String>("source") {
        aliases.retain(|alias| alias.source.contains(source_filter));
        println!(
            "{} Filtering by source: {}",
            "📁".cyan(),
            source_filter.yellow()
        );
    }

    if aliases.is_empty() {
        println!("{}", "No aliases found matching your criteria.".yellow());
        return Ok(());
    }

    let alias_count = aliases.len();
    let use_colors = !matches.get_flag("plain");
    display_aliases_table(aliases, use_colors)?;

    println!(
        "\n{} Found {} aliases",
        "✨".green(),
        alias_count.to_string().bold()
    );
    Ok(())
}

pub fn handle_functions_mode(matches: &ArgMatches) -> Result<()> {
    let mut functions = get_all_functions()?;

    // Apply filters
    if let Some(filter_pattern) = matches.get_one::<String>("filter") {
        let pattern = filter_pattern.to_lowercase();
        functions.retain(|func| {
            func.name.to_lowercase().contains(&pattern)
                || func.description.to_lowercase().contains(&pattern)
                || func.usage.to_lowercase().contains(&pattern)
        });
        println!("{} Filtering by: {}", "🔍".cyan(), filter_pattern.yellow());
    }

    if let Some(source_filter) = matches.get_one::<String>("source") {
        functions.retain(|func| func.source.contains(source_filter));
        println!(
            "{} Filtering by source: {}",
            "📁".cyan(),
            source_filter.yellow()
        );
    }

    if functions.is_empty() {
        println!("{}", "No functions found matching your criteria.".yellow());
        return Ok(());
    }

    let function_count = functions.len();
    let use_colors = !matches.get_flag("plain");
    display_functions_table(functions, use_colors)?;

    println!(
        "\n{} Found {} functions",
        "✨".green(),
        function_count.to_string().bold()
    );
    Ok(())
}

pub fn handle_packages_mode(matches: &ArgMatches) -> Result<()> {
    let package_name = matches.get_one::<String>("package").unwrap();
    let min_version = matches.get_one::<String>("min_version").unwrap();
    let search_path = matches.get_one::<String>("path").map(|s| s.as_str());
    let verbose = matches.get_flag("verbose");

    println!(
        "{} Searching for package '{}' with version > {}",
        "🔍".cyan(),
        package_name.yellow(),
        min_version.green()
    );

    if let Some(path) = search_path {
        println!("{} Search path: {}", "📁".cyan(), path.yellow());
    }

    if verbose {
        println!(
            "{} Verbose mode enabled - showing scan details",
            "🔍".cyan()
        );
    }

    let packages =
        find_packages_with_version_greater_than(package_name, min_version, search_path, verbose)?;

    if packages.is_empty() {
        println!(
            "{}",
            format!(
                "No packages named '{}' found with version greater than '{}'",
                package_name, min_version
            )
            .yellow()
        );
        return Ok(());
    }

    let package_count = packages.len();
    let use_colors = !matches.get_flag("plain");
    display_packages_table(packages, use_colors)?;

    println!(
        "\n{} Found {} package instances",
        "✨".green(),
        package_count.to_string().bold()
    );
    Ok(())
}

pub fn handle_clean_mode(matches: &ArgMatches) -> Result<()> {
    let search_path = matches.get_one::<String>("path").map(|s| s.as_str());
    let dry_run = matches.get_flag("dry_run");
    let verbose = matches.get_flag("verbose");
    let interactive = matches.get_flag("interactive");

    let results = clean_node_modules(search_path, dry_run, verbose, interactive)?;

    if !results.is_empty() && !interactive {
        let use_colors = !matches.get_flag("plain");
        display_cleaned_table(results, use_colors)?;
    }

    Ok(())
}

pub fn handle_organize_mode(matches: &ArgMatches) -> Result<()> {
    let search_path = matches.get_one::<String>("path").map(|s| s.as_str());
    let dry_run = matches.get_flag("dry_run");
    let verbose = matches.get_flag("verbose");
    let interactive = matches.get_flag("interactive");

    let results = organize_files(search_path, dry_run, verbose, interactive)?;

    if !results.is_empty() && !interactive {
        let use_colors = !matches.get_flag("plain");
        display_organize_table(results, use_colors)?;
    }

    Ok(())
}

pub fn handle_bookmarks_mode(matches: &ArgMatches) -> Result<()> {
    let subcommand = matches
        .get_one::<String>("subcommand")
        .map(|s| s.as_str())
        .unwrap_or("stats");
    let use_colors = !matches.get_flag("plain");
    let verbose = matches.get_flag("verbose");
    let dry_run = matches.get_flag("dry_run");
    let limit = matches
        .get_one::<String>("limit")
        .and_then(|s| s.parse::<usize>().ok());

    // Parse bookmarks
    println!("{} Loading Chrome bookmarks...", "📖".cyan());
    let (bookmarks, folders) = parse_bookmarks()?;
    println!(
        "{} Found {} bookmarks in {} folders\n",
        "✅".green(),
        bookmarks.len().to_string().yellow(),
        folders.len().to_string().yellow()
    );

    match subcommand {
        "stats" => {
            let stats = get_bookmark_stats(&bookmarks, &folders);

            println!("{}", "📊 Bookmark Statistics".bold().cyan());
            println!("{}", "─".repeat(50).dimmed());
            println!(
                "  {} Total bookmarks: {}",
                "📑".cyan(),
                stats.total_bookmarks.to_string().yellow()
            );
            println!(
                "  {} Total folders: {}",
                "📁".cyan(),
                stats.total_folders.to_string().yellow()
            );
            println!(
                "  {} Duplicate URLs: {}",
                "🔄".cyan(),
                stats.duplicates.to_string().yellow()
            );
            println!(
                "  {} Empty folders: {}",
                "📂".cyan(),
                stats.empty_folders.to_string().yellow()
            );
            println!(
                "  {} Deeply nested folders (>3 levels): {}",
                "🪆".cyan(),
                stats.deep_nesting_count.to_string().yellow()
            );
            println!(
                "  {} Unique domains: {}",
                "🌐".cyan(),
                stats.by_domain.len().to_string().yellow()
            );

            // Show top domains
            println!("\n{}", "🔝 Top 10 Domains".bold().cyan());
            println!("{}", "─".repeat(50).dimmed());
            let domain_stats = get_domain_stats(&bookmarks);
            let top_domains: Vec<_> = domain_stats.into_iter().take(10).collect();
            display_domain_stats_table(top_domains, use_colors)?;

            // Show category breakdown
            println!("\n{}", "📂 Category Breakdown".bold().cyan());
            println!("{}", "─".repeat(50).dimmed());
            let category_stats = get_category_stats(&bookmarks);
            display_category_stats_table(category_stats, use_colors)?;
        }
        "duplicates" => {
            println!("{}", "🔄 Duplicate Bookmarks".bold().cyan());
            println!("{}", "─".repeat(50).dimmed());

            let duplicates = find_duplicates(&bookmarks);
            if duplicates.is_empty() {
                println!("{}", "No duplicate bookmarks found!".green());
            } else {
                let limited: Vec<_> = if let Some(lim) = limit {
                    duplicates.into_iter().take(lim).collect()
                } else {
                    duplicates
                };
                let count = limited.len();
                display_duplicates_table(limited, use_colors)?;
                println!(
                    "\n{} Found {} duplicate URL groups",
                    "📊".cyan(),
                    count.to_string().yellow()
                );
            }
        }
        "domains" => {
            println!("{}", "🌐 Bookmarks by Domain".bold().cyan());
            println!("{}", "─".repeat(50).dimmed());

            if let Some(domain_filter) = matches.get_one::<String>("domain") {
                let filtered = filter_by_domain(&bookmarks, domain_filter);
                if filtered.is_empty() {
                    println!(
                        "{}",
                        format!("No bookmarks found for domain: {}", domain_filter).yellow()
                    );
                } else {
                    let count = filtered.len();
                    display_bookmarks_table(filtered, use_colors)?;
                    println!(
                        "\n{} Found {} bookmarks for '{}'",
                        "📊".cyan(),
                        count.to_string().yellow(),
                        domain_filter.cyan()
                    );
                }
            } else {
                let domain_stats = get_domain_stats(&bookmarks);
                let limited: Vec<_> = if let Some(lim) = limit {
                    domain_stats.into_iter().take(lim).collect()
                } else {
                    domain_stats.into_iter().take(30).collect()
                };
                display_domain_stats_table(limited, use_colors)?;
            }
        }
        "categories" => {
            println!("{}", "📂 Bookmarks by Category".bold().cyan());
            println!("{}", "─".repeat(50).dimmed());

            if let Some(category_filter) = matches.get_one::<String>("category") {
                let filtered = filter_by_category(&bookmarks, category_filter);
                if filtered.is_empty() {
                    println!(
                        "{}",
                        format!("No bookmarks found for category: {}", category_filter).yellow()
                    );
                } else {
                    let count = filtered.len();
                    display_bookmarks_table(filtered, use_colors)?;
                    println!(
                        "\n{} Found {} bookmarks in category '{}'",
                        "📊".cyan(),
                        count.to_string().yellow(),
                        category_filter.cyan()
                    );
                }
            } else {
                let category_stats = get_category_stats(&bookmarks);
                display_category_stats_table(category_stats, use_colors)?;
            }
        }
        "search" => {
            if let Some(query) = matches.get_one::<String>("query") {
                println!("{} Searching for: {}", "🔍".cyan(), query.yellow());
                println!("{}", "─".repeat(50).dimmed());

                let results = search_bookmarks(&bookmarks, query);
                if results.is_empty() {
                    println!(
                        "{}",
                        format!("No bookmarks found matching: {}", query).yellow()
                    );
                } else {
                    let limited: Vec<_> = if let Some(lim) = limit {
                        results.into_iter().take(lim).collect()
                    } else {
                        results
                    };
                    let count = limited.len();
                    display_bookmarks_table(limited, use_colors)?;
                    println!(
                        "\n{} Found {} matching bookmarks",
                        "📊".cyan(),
                        count.to_string().yellow()
                    );
                }
            } else {
                println!(
                    "{}",
                    "Please provide a search query with --query <QUERY>".yellow()
                );
            }
        }
        "organize" => {
            println!("{}", "📋 Organization Suggestions".bold().cyan());
            println!("{}", "─".repeat(50).dimmed());

            let suggestions = get_organize_suggestions(&bookmarks);
            if suggestions.is_empty() {
                println!("{}", "All bookmarks are already well-organized!".green());
            } else {
                let limited: Vec<_> = if let Some(lim) = limit {
                    suggestions.into_iter().take(lim).collect()
                } else {
                    suggestions.into_iter().take(50).collect()
                };
                let count = limited.len();
                display_organize_suggestions_table(limited, use_colors)?;
                println!(
                    "\n{} Found {} bookmarks that could be reorganized",
                    "📊".cyan(),
                    count.to_string().yellow()
                );
                println!(
                    "\n{} To apply these changes, manually reorganize in Chrome or export and reimport.",
                    "💡".yellow()
                );
            }
        }
        "export" => {
            let output_path = matches.get_one::<String>("output").map(|s| s.as_str());
            let default_path = "bookmarks_export.md";
            let path = output_path.unwrap_or(default_path);

            println!("{} Exporting bookmarks to markdown...", "📝".cyan());
            export_to_markdown(&bookmarks, Some(path))?;
            println!(
                "\n{} Exported {} bookmarks to {}",
                "✅".green(),
                bookmarks.len().to_string().yellow(),
                path.cyan()
            );
        }
        "export-html" => {
            let output_path = matches.get_one::<String>("output").map(|s| s.as_str());
            let default_path = "bookmarks_organized.html";
            let path = output_path.unwrap_or(default_path);

            println!(
                "{} Exporting organized bookmarks to Chrome HTML...",
                "📝".cyan()
            );
            export_to_chrome_html(&bookmarks, Some(path))?;
        }
        "deadlinks" => {
            println!("{}", "🔗 Checking for Dead Links".bold().cyan());
            println!("{}", "─".repeat(50).dimmed());

            let dead_links = find_dead_links(&bookmarks, verbose);
            if dead_links.is_empty() {
                println!(
                    "{}",
                    "No dead links found! All bookmarks are valid.".green()
                );
            } else {
                let limited: Vec<_> = if let Some(lim) = limit {
                    dead_links.into_iter().take(lim).collect()
                } else {
                    dead_links
                };
                let count = limited.len();
                display_dead_links_table(limited, use_colors)?;
                println!(
                    "\n{} Found {} dead links",
                    "📊".cyan(),
                    count.to_string().red()
                );
                println!(
                    "\n{} Use --subcommand remove-dead to remove these dead links",
                    "💡".yellow()
                );
            }
        }
        "remove-dead" => {
            println!("{}", "🗑️  Remove Dead Links".bold().cyan());
            println!("{}", "─".repeat(50).dimmed());

            // First find dead links
            let dead_links = find_dead_links(&bookmarks, verbose);
            if dead_links.is_empty() {
                println!(
                    "{}",
                    "No dead links found! All bookmarks are valid.".green()
                );
            } else {
                let count = dead_links.len();
                println!(
                    "\n{} Found {} dead links to remove",
                    "📊".cyan(),
                    count.to_string().red()
                );
                remove_dead_links(&dead_links, dry_run, true)?;
            }
        }
        "remove-dupes" => {
            println!("{}", "🗑️  Remove Duplicate Bookmarks".bold().cyan());
            println!("{}", "─".repeat(50).dimmed());

            remove_duplicates(dry_run, true)?;
        }
        _ => {
            println!(
                "{}",
                format!(
                    "Unknown subcommand: {}. Use: stats, duplicates, remove-dupes, deadlinks, remove-dead, domains, categories, search, organize, export",
                    subcommand
                )
                .yellow()
            );
        }
    }

    Ok(())
}
