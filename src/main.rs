use anyhow::Result;
use colored::Colorize;
use utils::{
    build_cli, handle_aliases_mode, handle_bookmarks_mode, handle_clean_mode,
    handle_functions_mode, handle_organize_mode, handle_packages_mode,
};

fn main() -> Result<()> {
    let matches = build_cli().get_matches();
    let mode = matches.get_one::<String>("mode").unwrap();

    match mode.as_str() {
        "functions" => {
            println!("{}", "🔧 Shell Function Explorer".bold().cyan());
            println!("{}", "─".repeat(60).dimmed());
            handle_functions_mode(&matches)
        }
        "packages" => {
            println!("{}", "📦 Package Version Explorer".bold().cyan());
            println!("{}", "─".repeat(60).dimmed());
            handle_packages_mode(&matches)
        }
        "clean" => {
            println!("{}", "🧹 Node Modules Cleaner".bold().cyan());
            println!("{}", "─".repeat(60).dimmed());
            handle_clean_mode(&matches)
        }
        "organize" => {
            println!("{}", "📂 File Organizer".bold().cyan());
            println!("{}", "─".repeat(60).dimmed());
            handle_organize_mode(&matches)
        }
        "bookmarks" => {
            println!("{}", "🔖 Chrome Bookmarks Organizer".bold().cyan());
            println!("{}", "─".repeat(60).dimmed());
            handle_bookmarks_mode(&matches)
        }
        "aliases" | _ => {
            println!("{}", "🔍 Shell Alias Explorer".bold().cyan());
            println!("{}", "─".repeat(50).dimmed());
            handle_aliases_mode(&matches)
        }
    }
}
