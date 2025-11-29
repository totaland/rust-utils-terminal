use anyhow::{Context, Result};
use colored::Colorize;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};
use rayon::prelude::*;
use std::fs;
use std::io::{Write, stdout};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tabled::Tabled;

#[derive(Clone)]
pub struct NodeModuleEntry {
    pub path: PathBuf,
    pub size: u64,
    pub selected: bool,
    pub status: CleanStatus,
}

#[derive(Clone, PartialEq)]
pub enum CleanStatus {
    Found,
    Deleting,
    Deleted,
    Error(String),
}

impl std::fmt::Display for CleanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CleanStatus::Found => write!(f, "Found"),
            CleanStatus::Deleting => write!(f, "Deleting..."),
            CleanStatus::Deleted => write!(f, "✓ Deleted"),
            CleanStatus::Error(e) => write!(f, "✗ {}", e),
        }
    }
}

#[derive(Tabled, Clone)]
pub struct CleanedEntry {
    #[tabled(rename = "Path")]
    pub path: String,
    #[tabled(rename = "Size")]
    pub size: String,
    #[tabled(rename = "Status")]
    pub status: String,
}

/// Recursively find all node_modules directories
fn find_node_modules(root: &Path, verbose: bool) -> Vec<PathBuf> {
    let mut results = Vec::new();
    find_node_modules_recursive(root, &mut results, verbose);
    results
}

fn find_node_modules_recursive(dir: &Path, results: &mut Vec<PathBuf>, verbose: bool) {
    if !dir.is_dir() {
        return;
    }

    let dir_name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if matches!(dir_name, ".git" | "target" | ".cache" | ".Trash") {
        return;
    }

    if verbose {
        println!("{} Scanning: {}", "🔍".dimmed(), dir.display());
    }

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "node_modules" {
                results.push(path);
            } else {
                find_node_modules_recursive(&path, results, verbose);
            }
        }
    }
}

/// Calculate directory size recursively using parallel traversal
fn calculate_dir_size(path: &Path) -> u64 {
    if !path.is_dir() {
        return path.metadata().map(|m| m.len()).unwrap_or(0);
    }

    // Use parallel iteration for top-level entries
    let entries: Vec<_> = fs::read_dir(path)
        .map(|iter| iter.flatten().collect())
        .unwrap_or_default();

    entries
        .par_iter()
        .map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                calculate_dir_size_recursive(&path)
            } else {
                path.metadata().map(|m| m.len()).unwrap_or(0)
            }
        })
        .sum()
}

/// Non-parallel recursive helper (parallel at top level is enough)
fn calculate_dir_size_recursive(path: &Path) -> u64 {
    fs::read_dir(path)
        .map(|entries| {
            entries
                .flatten()
                .map(|entry| {
                    let path = entry.path();
                    if path.is_dir() {
                        calculate_dir_size_recursive(&path)
                    } else {
                        path.metadata().map(|m| m.len()).unwrap_or(0)
                    }
                })
                .sum()
        })
        .unwrap_or(0)
}

/// Format bytes into human-readable string
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Remove a directory and all its contents
fn remove_directory(path: &Path) -> Result<()> {
    fs::remove_dir_all(path)
        .with_context(|| format!("Failed to remove directory: {}", path.display()))
}

/// Scan and display node_modules without cleaning (list mode)
pub fn list_node_modules(search_path: Option<&str>, verbose: bool) -> Result<Vec<NodeModuleEntry>> {
    let root = search_path
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    println!(
        "{} Searching for node_modules in: {}",
        "🔍".cyan(),
        root.display().to_string().yellow()
    );

    let node_modules_dirs = find_node_modules(&root, verbose);

    if node_modules_dirs.is_empty() {
        println!("{}", "No node_modules directories found.".yellow());
        return Ok(Vec::new());
    }

    let total_dirs = node_modules_dirs.len();
    println!(
        "{} Found {} node_modules directories. Calculating sizes in parallel...",
        "📦".cyan(),
        total_dirs.to_string().green()
    );

    // Shared state for progress tracking
    let completed = Arc::new(AtomicUsize::new(0));
    let total_size_so_far = Arc::new(AtomicU64::new(0));
    let current_path: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let done = Arc::new(AtomicBool::new(false));

    // Clones for progress thread
    let completed_clone = Arc::clone(&completed);
    let total_size_clone = Arc::clone(&total_size_so_far);
    let current_path_clone = Arc::clone(&current_path);
    let done_clone = Arc::clone(&done);

    let progress_handle = thread::spawn(move || {
        let mut stdout = stdout();
        while !done_clone.load(Ordering::Relaxed) {
            let count = completed_clone.load(Ordering::Relaxed);
            let size = total_size_clone.load(Ordering::Relaxed);
            let path = current_path_clone
                .lock()
                .map(|p| p.clone())
                .unwrap_or_default();

            // Truncate path for display
            let display_path = if path.len() > 40 {
                format!("...{}", &path[path.len() - 37..])
            } else {
                path
            };

            print!(
                "\r{} [{}/{}] {} found | {}",
                "⏳".cyan(),
                count.to_string().green(),
                total_dirs.to_string().cyan(),
                format_size(size).yellow(),
                display_path.dimmed()
            );
            // Clear rest of line
            print!("{}", " ".repeat(20));
            stdout.flush().ok();
            thread::sleep(Duration::from_millis(50));
        }
        print!("\r{}\r", " ".repeat(100)); // Clear the line
        stdout.flush().ok();
    });

    let entries: Vec<NodeModuleEntry> = node_modules_dirs
        .par_iter()
        .map(|path| {
            // Update current path being processed
            if let Ok(mut current) = current_path.lock() {
                *current = path.display().to_string();
            }

            let size = calculate_dir_size(path);
            completed.fetch_add(1, Ordering::Relaxed);
            total_size_so_far.fetch_add(size, Ordering::Relaxed);

            NodeModuleEntry {
                path: path.clone(),
                size,
                selected: false,
                status: CleanStatus::Found,
            }
        })
        .collect();

    done.store(true, Ordering::Relaxed);
    progress_handle.join().ok();

    let total_size: u64 = entries.iter().map(|e| e.size).sum();

    println!(
        "\n{} Total space that can be freed: {}",
        "💾".green(),
        format_size(total_size).bold().yellow()
    );

    Ok(entries)
}

/// Interactive mode - select and delete node_modules
pub fn interactive_clean(search_path: Option<&str>, verbose: bool) -> Result<Vec<CleanedEntry>> {
    let mut entries = list_node_modules(search_path, verbose)?;

    if entries.is_empty() {
        return Ok(Vec::new());
    }

    entries.sort_by(|a, b| b.size.cmp(&a.size));

    println!("\n{}", "Interactive Mode".bold().cyan());
    println!("{}", "─".repeat(60).dimmed());
    println!("  {}    Navigate up/down", "↑/↓".yellow());
    println!("  {}  Toggle selection", "Space".yellow());
    println!("  {}      Select all", "a".yellow());
    println!("  {}      Deselect all", "n".yellow());
    println!("  {}  Delete selected", "Enter".yellow());
    println!("  {}      Quit without deleting", "q".yellow());
    println!("{}", "─".repeat(60).dimmed());
    println!("\nPress any key to continue...");

    terminal::enable_raw_mode()?;
    let _ = event::read();
    terminal::disable_raw_mode()?;

    let selected_entries = run_interactive_selection(&mut entries)?;

    if selected_entries.is_empty() {
        println!("{}", "No directories selected for deletion.".yellow());
        return Ok(Vec::new());
    }

    delete_with_live_updates(selected_entries)
}

fn run_interactive_selection(entries: &mut Vec<NodeModuleEntry>) -> Result<Vec<NodeModuleEntry>> {
    let mut cursor_pos = 0;
    let mut scroll_offset = 0;

    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)?;

    loop {
        let (_, term_height) = terminal::size().unwrap_or((80, 24));
        let visible_rows = (term_height as usize).saturating_sub(8);

        if cursor_pos < scroll_offset {
            scroll_offset = cursor_pos;
        } else if cursor_pos >= scroll_offset + visible_rows {
            scroll_offset = cursor_pos - visible_rows + 1;
        }

        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::All)
        )?;

        let total_size: u64 = entries.iter().map(|e| e.size).sum();
        let selected_size: u64 = entries.iter().filter(|e| e.selected).map(|e| e.size).sum();
        let selected_count = entries.iter().filter(|e| e.selected).count();

        writeln!(
            stdout,
            "{}",
            "🧹 Node Modules Cleaner - Interactive Mode".bold().cyan()
        )?;
        writeln!(stdout, "{}", "─".repeat(80).dimmed())?;
        writeln!(
            stdout,
            "Total: {} ({})  |  Selected: {} ({})  |  Will free: {}",
            entries.len().to_string().cyan(),
            format_size(total_size).cyan(),
            selected_count.to_string().green(),
            format_size(selected_size).green(),
            format_size(selected_size).bold().yellow()
        )?;
        writeln!(stdout, "{}", "─".repeat(80).dimmed())?;

        for (i, entry) in entries
            .iter()
            .enumerate()
            .skip(scroll_offset)
            .take(visible_rows)
        {
            let is_current = i == cursor_pos;
            let checkbox = if entry.selected { "[✓]" } else { "[ ]" };
            let size_str = format!("{:>10}", format_size(entry.size));
            let path_str = entry.path.display().to_string();

            let max_path_len = 55;
            let display_path = if path_str.len() > max_path_len {
                format!("...{}", &path_str[path_str.len() - max_path_len + 3..])
            } else {
                path_str
            };

            let line = format!(" {} {} {}", checkbox, size_str, display_path);

            if is_current {
                writeln!(stdout, "{}", line.on_blue().white())?;
            } else if entry.selected {
                writeln!(stdout, "{}", line.green())?;
            } else {
                writeln!(stdout, "{}", line)?;
            }
        }

        writeln!(stdout)?;
        writeln!(stdout, "{}", "─".repeat(80).dimmed())?;
        writeln!(
            stdout,
            "{}  {}  {}  {}  {}  {}",
            "↑↓:Navigate".dimmed(),
            "Space:Toggle".dimmed(),
            "a:All".dimmed(),
            "n:None".dimmed(),
            "Enter:Delete".dimmed(),
            "q:Quit".dimmed()
        )?;

        stdout.flush()?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if cursor_pos < entries.len() - 1 {
                        cursor_pos += 1;
                    }
                }
                KeyCode::Char(' ') => {
                    entries[cursor_pos].selected = !entries[cursor_pos].selected;
                }
                KeyCode::Char('a') => {
                    for entry in entries.iter_mut() {
                        entry.selected = true;
                    }
                }
                KeyCode::Char('n') => {
                    for entry in entries.iter_mut() {
                        entry.selected = false;
                    }
                }
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    for entry in entries.iter_mut() {
                        entry.selected = false;
                    }
                    break;
                }
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    for entry in entries.iter_mut() {
                        entry.selected = false;
                    }
                    break;
                }
                _ => {}
            }
        }
    }

    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;

    let selected: Vec<NodeModuleEntry> = entries.iter().filter(|e| e.selected).cloned().collect();
    Ok(selected)
}

fn delete_with_live_updates(entries: Vec<NodeModuleEntry>) -> Result<Vec<CleanedEntry>> {
    let entries_arc = Arc::new(Mutex::new(
        entries
            .into_iter()
            .map(|e| (e.path.clone(), e.size, CleanStatus::Found))
            .collect::<Vec<_>>(),
    ));

    let total_count = entries_arc.lock().unwrap().len();
    let deleted_count = Arc::new(AtomicUsize::new(0));
    let freed_bytes = Arc::new(AtomicU64::new(0));
    let done = Arc::new(AtomicBool::new(false));

    let entries_display = Arc::clone(&entries_arc);
    let deleted_display = Arc::clone(&deleted_count);
    let freed_display = Arc::clone(&freed_bytes);
    let done_display = Arc::clone(&done);

    let display_handle = thread::spawn(move || {
        let mut stdout = stdout();

        while !done_display.load(Ordering::Relaxed) {
            execute!(
                stdout,
                cursor::MoveTo(0, 0),
                terminal::Clear(ClearType::All)
            )
            .ok();

            writeln!(stdout, "{}", "🧹 Deleting node_modules...".bold().cyan()).ok();
            writeln!(stdout, "{}", "─".repeat(80).dimmed()).ok();

            let deleted = deleted_display.load(Ordering::Relaxed);
            let freed = freed_display.load(Ordering::Relaxed);

            writeln!(
                stdout,
                "Progress: {}/{}  |  Freed: {}",
                deleted.to_string().green(),
                total_count.to_string().cyan(),
                format_size(freed).bold().yellow()
            )
            .ok();
            writeln!(stdout, "{}", "─".repeat(80).dimmed()).ok();

            if let Ok(entries) = entries_display.lock() {
                for (path, size, status) in entries.iter() {
                    let size_str = format!("{:>10}", format_size(*size));
                    let path_str = path.display().to_string();
                    let max_path_len = 50;
                    let display_path = if path_str.len() > max_path_len {
                        format!("...{}", &path_str[path_str.len() - max_path_len + 3..])
                    } else {
                        path_str
                    };

                    let status_str = match status {
                        CleanStatus::Found => "⏳ Pending".dimmed().to_string(),
                        CleanStatus::Deleting => "🔄 Deleting...".yellow().to_string(),
                        CleanStatus::Deleted => "✓ Deleted".green().to_string(),
                        CleanStatus::Error(e) => format!("✗ {}", e).red().to_string(),
                    };

                    writeln!(stdout, " {} {} {}", size_str, display_path, status_str).ok();
                }
            }

            stdout.flush().ok();
            thread::sleep(Duration::from_millis(100));
        }
    });

    let paths_to_delete: Vec<(PathBuf, u64)> = {
        let entries = entries_arc.lock().unwrap();
        entries.iter().map(|(p, s, _)| (p.clone(), *s)).collect()
    };

    paths_to_delete.par_iter().for_each(|(path, size)| {
        if let Ok(mut entries) = entries_arc.lock() {
            if let Some(entry) = entries.iter_mut().find(|(p, _, _)| p == path) {
                entry.2 = CleanStatus::Deleting;
            }
        }

        let result = remove_directory(path);

        if let Ok(mut entries) = entries_arc.lock() {
            if let Some(entry) = entries.iter_mut().find(|(p, _, _)| p == path) {
                match result {
                    Ok(_) => {
                        entry.2 = CleanStatus::Deleted;
                        deleted_count.fetch_add(1, Ordering::Relaxed);
                        freed_bytes.fetch_add(*size, Ordering::Relaxed);
                    }
                    Err(e) => {
                        entry.2 = CleanStatus::Error(e.to_string());
                    }
                }
            }
        }
    });

    done.store(true, Ordering::Relaxed);
    display_handle.join().ok();

    let mut stdout = stdout();
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    let final_entries: Vec<CleanedEntry> = {
        let entries = entries_arc.lock().unwrap();
        entries
            .iter()
            .map(|(path, size, status)| CleanedEntry {
                path: path.display().to_string(),
                size: format_size(*size),
                status: status.to_string(),
            })
            .collect()
    };

    let total_freed = freed_bytes.load(Ordering::Relaxed);
    let total_deleted = deleted_count.load(Ordering::Relaxed);

    println!(
        "\n{} Completed! Deleted {} directories, freed {}",
        "✨".green(),
        total_deleted.to_string().bold(),
        format_size(total_freed).bold().yellow()
    );

    Ok(final_entries)
}

/// Find and remove all node_modules directories in parallel (non-interactive)
pub fn clean_node_modules(
    search_path: Option<&str>,
    dry_run: bool,
    verbose: bool,
    interactive: bool,
) -> Result<Vec<CleanedEntry>> {
    // If interactive mode, use the interactive cleaner (needs sizes for selection)
    if interactive {
        return interactive_clean(search_path, verbose);
    }

    // If dry-run, we need sizes to show what would be freed
    if dry_run {
        let entries = list_node_modules(search_path, verbose)?;

        if entries.is_empty() {
            return Ok(Vec::new());
        }

        println!(
            "{} Dry run mode - no directories will be removed",
            "⚠️".yellow()
        );

        let results: Vec<CleanedEntry> = entries
            .iter()
            .map(|e| CleanedEntry {
                path: e.path.display().to_string(),
                size: format_size(e.size),
                status: "Would remove".to_string(),
            })
            .collect();

        let total_size: u64 = entries.iter().map(|e| e.size).sum();
        println!(
            "\n{} Would free {} from {} directories",
            "💾".green(),
            format_size(total_size).bold(),
            entries.len().to_string().bold()
        );

        return Ok(results);
    }

    // For clean-all mode, skip size calculation and delete immediately
    delete_all_node_modules(search_path, verbose)
}

/// Delete all node_modules without calculating sizes first (fast mode)
fn delete_all_node_modules(search_path: Option<&str>, verbose: bool) -> Result<Vec<CleanedEntry>> {
    let root = search_path
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    println!(
        "{} Searching for node_modules in: {}",
        "🔍".cyan(),
        root.display().to_string().yellow()
    );

    let node_modules_dirs = find_node_modules(&root, verbose);

    if node_modules_dirs.is_empty() {
        println!("{}", "No node_modules directories found.".yellow());
        return Ok(Vec::new());
    }

    let total_count = node_modules_dirs.len();
    println!(
        "{} Found {} node_modules directories. Deleting in parallel...",
        "📦".cyan(),
        total_count.to_string().green()
    );

    // Shared state for progress
    let deleted_count = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));
    let current_path: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let done = Arc::new(AtomicBool::new(false));

    // Clones for display thread
    let deleted_clone = Arc::clone(&deleted_count);
    let error_clone = Arc::clone(&error_count);
    let current_path_clone = Arc::clone(&current_path);
    let done_clone = Arc::clone(&done);

    let display_handle = thread::spawn(move || {
        let mut stdout = stdout();
        while !done_clone.load(Ordering::Relaxed) {
            let deleted = deleted_clone.load(Ordering::Relaxed);
            let errors = error_clone.load(Ordering::Relaxed);
            let path = current_path_clone.lock().map(|p| p.clone()).unwrap_or_default();
            
            let display_path = if path.len() > 45 {
                format!("...{}", &path[path.len() - 42..])
            } else {
                path
            };
            
            let error_str = if errors > 0 {
                format!(" | {} errors", errors.to_string().red())
            } else {
                String::new()
            };
            
            print!(
                "\r{} Deleted {}/{}{}  {}",
                "🗑️".cyan(),
                deleted.to_string().green(),
                total_count.to_string().cyan(),
                error_str,
                display_path.dimmed()
            );
            print!("{}", " ".repeat(20));
            stdout.flush().ok();
            thread::sleep(Duration::from_millis(50));
        }
        print!("\r{}\r", " ".repeat(100));
        stdout.flush().ok();
    });

    // Delete in parallel
    let results: Vec<CleanedEntry> = node_modules_dirs
        .par_iter()
        .map(|path| {
            if let Ok(mut current) = current_path.lock() {
                *current = path.display().to_string();
            }

            let status = match remove_directory(path) {
                Ok(_) => {
                    deleted_count.fetch_add(1, Ordering::Relaxed);
                    "✓ Deleted".to_string()
                }
                Err(e) => {
                    error_count.fetch_add(1, Ordering::Relaxed);
                    format!("✗ {}", e)
                }
            };

            CleanedEntry {
                path: path.display().to_string(),
                size: "-".to_string(), // Size not calculated in fast mode
                status,
            }
        })
        .collect();

    done.store(true, Ordering::Relaxed);
    display_handle.join().ok();

    let deleted = deleted_count.load(Ordering::Relaxed);
    let errors = error_count.load(Ordering::Relaxed);

    if errors > 0 {
        println!(
            "\n{} Completed! Deleted {} directories ({} errors)",
            "✨".green(),
            deleted.to_string().bold(),
            errors.to_string().red()
        );
    } else {
        println!(
            "\n{} Completed! Deleted {} directories",
            "✨".green(),
            deleted.to_string().bold()
        );
    }

    Ok(results)
}
