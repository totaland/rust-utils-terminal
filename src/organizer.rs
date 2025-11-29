use anyhow::{Context, Result};
use colored::Colorize;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};
use std::collections::HashMap;
use std::fs;
use std::io::{Write, stdout};
use std::path::{Path, PathBuf};
use tabled::Tabled;

/// Markers that indicate a development/project folder that should be skipped
const DEV_MARKERS: &[&str] = &[
    // Node.js / JavaScript
    "node_modules",
    "package.json",
    "package-lock.json",
    "yarn.lock",
    "pnpm-lock.yaml",
    // Java
    "pom.xml",
    "build.gradle",
    "gradlew",
    ".mvn",
    // Rust
    "Cargo.toml",
    "Cargo.lock",
    // Python
    "requirements.txt",
    "setup.py",
    "pyproject.toml",
    "Pipfile",
    ".venv",
    "venv",
    // Go
    "go.mod",
    "go.sum",
    // Ruby
    "Gemfile",
    "Gemfile.lock",
    // PHP
    "composer.json",
    "composer.lock",
    // .NET
    "*.csproj",
    "*.sln",
    // Git
    ".git",
    // IDE
    ".idea",
    ".vscode",
];

/// File categories for organization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileCategory {
    Documents,
    Images,
    Videos,
    Audio,
    Archives,
    Code,
    Data,
    Executables,
    Fonts,
    Ebooks,
    Other,
}

impl std::fmt::Display for FileCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileCategory::Documents => write!(f, "Documents"),
            FileCategory::Images => write!(f, "Images"),
            FileCategory::Videos => write!(f, "Videos"),
            FileCategory::Audio => write!(f, "Audio"),
            FileCategory::Archives => write!(f, "Archives"),
            FileCategory::Code => write!(f, "Code"),
            FileCategory::Data => write!(f, "Data"),
            FileCategory::Executables => write!(f, "Executables"),
            FileCategory::Fonts => write!(f, "Fonts"),
            FileCategory::Ebooks => write!(f, "Ebooks"),
            FileCategory::Other => write!(f, "Other"),
        }
    }
}

impl FileCategory {
    pub fn folder_name(&self) -> &str {
        match self {
            FileCategory::Documents => "Documents",
            FileCategory::Images => "Images",
            FileCategory::Videos => "Videos",
            FileCategory::Audio => "Audio",
            FileCategory::Archives => "Archives",
            FileCategory::Code => "Code",
            FileCategory::Data => "Data",
            FileCategory::Executables => "Executables",
            FileCategory::Fonts => "Fonts",
            FileCategory::Ebooks => "Ebooks",
            FileCategory::Other => "Other",
        }
    }

    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            // Documents
            "pdf" | "doc" | "docx" | "txt" | "rtf" | "odt" | "xls" | "xlsx" | "ppt" | "pptx"
            | "csv" | "md" | "pages" | "numbers" | "key" => FileCategory::Documents,

            // Images
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" | "ico" | "tiff" | "tif"
            | "raw" | "cr2" | "nef" | "heic" | "heif" | "psd" | "ai" | "eps" => {
                FileCategory::Images
            }

            // Videos
            "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "mpeg" | "mpg"
            | "3gp" | "ogv" => FileCategory::Videos,

            // Audio
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" | "aiff" | "opus" => {
                FileCategory::Audio
            }

            // Archives
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "tgz" | "tbz2" | "dmg" | "iso" => {
                FileCategory::Archives
            }

            // Code (standalone scripts, not part of projects)
            "sh" | "bash" | "zsh" | "fish" | "sql" | "lua" => FileCategory::Code,

            // Data
            "json" | "xml" | "yaml" | "yml" | "toml" | "ini" | "cfg" | "conf" | "plist"
            | "sqlite" | "db" => FileCategory::Data,

            // Executables
            "exe" | "msi" | "app" | "deb" | "rpm" | "pkg" | "appimage" | "run" => {
                FileCategory::Executables
            }

            // Fonts
            "ttf" | "otf" | "woff" | "woff2" | "eot" => FileCategory::Fonts,

            // Ebooks
            "epub" | "mobi" | "azw" | "azw3" | "fb2" | "djvu" => FileCategory::Ebooks,

            // Other
            _ => FileCategory::Other,
        }
    }
}

#[derive(Tabled, Clone)]
pub struct OrganizeEntry {
    #[tabled(rename = "File")]
    pub file_name: String,
    #[tabled(rename = "Category")]
    pub category: String,
    #[tabled(rename = "Destination")]
    pub destination: String,
    #[tabled(rename = "Status")]
    pub status: String,
}

#[derive(Clone)]
pub struct FileToOrganize {
    pub path: PathBuf,
    pub file_name: String,
    pub category: FileCategory,
    pub selected: bool,
}

/// Check if a directory is a development/project folder
pub fn is_dev_folder(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }

    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return false,
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        for marker in DEV_MARKERS {
            if marker.starts_with('*') {
                // Handle wildcard patterns like *.csproj
                let suffix = &marker[1..];
                if name_str.ends_with(suffix) {
                    return true;
                }
            } else if name_str == *marker {
                return true;
            }
        }
    }

    false
}

/// Get files to organize in a directory (non-recursive, top-level files only)
pub fn get_files_to_organize(path: &Path) -> Result<Vec<FileToOrganize>> {
    let mut files = Vec::new();

    let entries = fs::read_dir(path)
        .with_context(|| format!("Failed to read directory: {}", path.display()))?;

    for entry in entries.flatten() {
        let file_path = entry.path();

        // Only process files, not directories
        if !file_path.is_file() {
            continue;
        }

        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        // Skip hidden files
        if file_name.starts_with('.') {
            continue;
        }

        // Get extension and determine category
        let extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let category = FileCategory::from_extension(extension);

        files.push(FileToOrganize {
            path: file_path,
            file_name,
            category,
            selected: true, // Default to selected
        });
    }

    // Sort by category then by name
    files.sort_by(|a, b| {
        a.category
            .folder_name()
            .cmp(b.category.folder_name())
            .then(a.file_name.cmp(&b.file_name))
    });

    Ok(files)
}

/// Organize files in a directory
pub fn organize_files(
    search_path: Option<&str>,
    dry_run: bool,
    verbose: bool,
    interactive: bool,
) -> Result<Vec<OrganizeEntry>> {
    let root = search_path
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    println!(
        "{} Checking directory: {}",
        "🔍".cyan(),
        root.display().to_string().yellow()
    );

    // Check if this is a dev folder
    if is_dev_folder(&root) {
        println!(
            "{} {} is a development folder. Skipping organization.",
            "⚠️".yellow(),
            root.display().to_string().cyan()
        );
        println!(
            "{}",
            "Development markers found (node_modules, package.json, Cargo.toml, etc.)".dimmed()
        );
        return Ok(Vec::new());
    }

    println!(
        "{} Not a development folder. Scanning for files to organize...",
        "✓".green()
    );

    let files = get_files_to_organize(&root)?;

    if files.is_empty() {
        println!("{}", "No files found to organize.".yellow());
        return Ok(Vec::new());
    }

    // Count files by category
    let mut category_counts: HashMap<&FileCategory, usize> = HashMap::new();
    for file in &files {
        *category_counts.entry(&file.category).or_insert(0) += 1;
    }

    println!("\n{} Files found by category:", "📊".cyan());
    for (category, count) in &category_counts {
        println!(
            "  {} {}: {}",
            "•".dimmed(),
            category,
            count.to_string().green()
        );
    }
    println!();

    if interactive {
        return interactive_organize(&root, files, dry_run);
    }

    if dry_run {
        println!("{} Dry run mode - no files will be moved\n", "🔍".cyan());
    }

    let mut results = Vec::new();

    for file in files {
        let category_folder = root.join(file.category.folder_name());
        let destination = category_folder.join(&file.file_name);

        let status = if dry_run {
            "Would move".to_string()
        } else {
            // Create category folder if it doesn't exist
            if !category_folder.exists() {
                fs::create_dir(&category_folder).with_context(|| {
                    format!("Failed to create directory: {}", category_folder.display())
                })?;
                if verbose {
                    println!(
                        "{} Created folder: {}",
                        "📁".green(),
                        category_folder.display()
                    );
                }
            }

            // Move the file
            match fs::rename(&file.path, &destination) {
                Ok(_) => {
                    if verbose {
                        println!(
                            "{} Moved: {} → {}",
                            "✓".green(),
                            file.file_name,
                            destination.display()
                        );
                    }
                    "✓ Moved".to_string()
                }
                Err(e) => {
                    // Try copy + delete if rename fails (cross-device move)
                    match fs::copy(&file.path, &destination) {
                        Ok(_) => {
                            fs::remove_file(&file.path).ok();
                            if verbose {
                                println!(
                                    "{} Moved: {} → {}",
                                    "✓".green(),
                                    file.file_name,
                                    destination.display()
                                );
                            }
                            "✓ Moved".to_string()
                        }
                        Err(_) => format!("✗ Error: {}", e),
                    }
                }
            }
        };

        results.push(OrganizeEntry {
            file_name: file.file_name,
            category: file.category.to_string(),
            destination: destination.display().to_string(),
            status,
        });
    }

    if !dry_run {
        let moved_count = results
            .iter()
            .filter(|r| r.status.contains("Moved"))
            .count();
        println!(
            "\n{} Successfully organized {} files",
            "✨".green(),
            moved_count.to_string().bold()
        );
    }

    Ok(results)
}

/// Interactive mode for organizing files
fn interactive_organize(
    root: &Path,
    mut files: Vec<FileToOrganize>,
    dry_run: bool,
) -> Result<Vec<OrganizeEntry>> {
    if files.is_empty() {
        return Ok(Vec::new());
    }

    println!("\n{}", "Interactive Mode".bold().cyan());
    println!("{}", "─".repeat(60).dimmed());
    println!("  {}    Navigate up/down", "↑/↓".yellow());
    println!("  {}  Toggle selection", "Space".yellow());
    println!("  {}      Select all", "a".yellow());
    println!("  {}      Deselect all", "n".yellow());
    println!("  {}  Organize selected", "Enter".yellow());
    println!("  {}      Quit without organizing", "q".yellow());
    println!("{}", "─".repeat(60).dimmed());
    println!("\nPress any key to continue...");

    terminal::enable_raw_mode()?;
    let _ = event::read();

    let mut selected_idx = 0;
    let mut stdout = stdout();

    // Enter alternate screen
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    loop {
        // Clear screen and render
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::All)
        )?;

        // Header
        writeln!(
            stdout,
            "{}",
            "📁 File Organizer - Interactive Mode".bold().cyan()
        )?;
        writeln!(stdout, "{}", "─".repeat(80).dimmed())?;

        let selected_count = files.iter().filter(|f| f.selected).count();
        writeln!(
            stdout,
            "Selected: {}/{} | {}=Toggle {}=All {}=None {}=Organize {}=Quit",
            selected_count.to_string().green(),
            files.len().to_string().cyan(),
            "Space".yellow(),
            "a".yellow(),
            "n".yellow(),
            "Enter".yellow(),
            "q".yellow()
        )?;
        writeln!(stdout, "{}", "─".repeat(80).dimmed())?;

        // Calculate visible window
        let term_height = terminal::size()?.1 as usize;
        let list_height = term_height.saturating_sub(8);
        let start_idx = if selected_idx >= list_height {
            selected_idx - list_height + 1
        } else {
            0
        };
        let end_idx = (start_idx + list_height).min(files.len());

        // Render file list
        for (idx, file) in files
            .iter()
            .enumerate()
            .skip(start_idx)
            .take(end_idx - start_idx)
        {
            let is_current = idx == selected_idx;
            let checkbox = if file.selected { "[✓]" } else { "[ ]" };

            let line = format!(
                " {} {} → {}",
                checkbox,
                file.file_name,
                file.category.folder_name()
            );

            if is_current {
                writeln!(stdout, "{}", line.on_bright_blue().white())?;
            } else if file.selected {
                writeln!(stdout, "{}", line.green())?;
            } else {
                writeln!(stdout, "{}", line.dimmed())?;
            }
        }

        // Show scroll indicator
        if files.len() > list_height {
            writeln!(
                stdout,
                "\n{} {}/{}",
                "Showing:".dimmed(),
                (selected_idx + 1).to_string().cyan(),
                files.len().to_string().cyan()
            )?;
        }

        stdout.flush()?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if selected_idx > 0 {
                        selected_idx -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if selected_idx < files.len() - 1 {
                        selected_idx += 1;
                    }
                }
                KeyCode::Char(' ') => {
                    files[selected_idx].selected = !files[selected_idx].selected;
                }
                KeyCode::Char('a') => {
                    for file in &mut files {
                        file.selected = true;
                    }
                }
                KeyCode::Char('n') => {
                    for file in &mut files {
                        file.selected = false;
                    }
                }
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
                    terminal::disable_raw_mode()?;
                    println!("{}", "Cancelled.".yellow());
                    return Ok(Vec::new());
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
                    terminal::disable_raw_mode()?;
                    println!("{}", "Cancelled.".yellow());
                    return Ok(Vec::new());
                }
                _ => {}
            }
        }
    }

    // Leave alternate screen
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    // Get selected files
    let selected_files: Vec<_> = files.into_iter().filter(|f| f.selected).collect();

    if selected_files.is_empty() {
        println!("{}", "No files selected for organization.".yellow());
        return Ok(Vec::new());
    }

    println!(
        "\n{} Organizing {} selected files...",
        "📁".cyan(),
        selected_files.len().to_string().green()
    );

    if dry_run {
        println!("{} Dry run mode - no files will be moved\n", "🔍".cyan());
    }

    let mut results = Vec::new();

    for file in selected_files {
        let category_folder = root.join(file.category.folder_name());
        let destination = category_folder.join(&file.file_name);

        let status = if dry_run {
            "Would move".to_string()
        } else {
            // Create category folder if it doesn't exist
            if !category_folder.exists() {
                fs::create_dir(&category_folder).with_context(|| {
                    format!("Failed to create directory: {}", category_folder.display())
                })?;
            }

            // Move the file
            match fs::rename(&file.path, &destination) {
                Ok(_) => "✓ Moved".to_string(),
                Err(e) => {
                    // Try copy + delete if rename fails (cross-device move)
                    match fs::copy(&file.path, &destination) {
                        Ok(_) => {
                            fs::remove_file(&file.path).ok();
                            "✓ Moved".to_string()
                        }
                        Err(_) => format!("✗ Error: {}", e),
                    }
                }
            }
        };

        println!(
            "  {} {} → {}/{}",
            if status.contains("Moved") || status.contains("Would") {
                "✓".green()
            } else {
                "✗".red()
            },
            file.file_name,
            file.category.folder_name(),
            file.file_name.dimmed()
        );

        results.push(OrganizeEntry {
            file_name: file.file_name,
            category: file.category.to_string(),
            destination: destination.display().to_string(),
            status,
        });
    }

    if !dry_run {
        let moved_count = results
            .iter()
            .filter(|r| r.status.contains("Moved"))
            .count();
        println!(
            "\n{} Successfully organized {} files",
            "✨".green(),
            moved_count.to_string().bold()
        );
    }

    Ok(results)
}

/// Display table for organized files
pub fn display_organize_table(entries: Vec<OrganizeEntry>, use_colors: bool) -> Result<()> {
    use tabled::{Table, settings::Style};

    if entries.is_empty() {
        return Ok(());
    }

    let table = Table::new(&entries).with(Style::rounded()).to_string();

    if use_colors {
        // Add some color highlighting
        let colored_table = table
            .replace("✓ Moved", &"✓ Moved".green().to_string())
            .replace("Would move", &"Would move".yellow().to_string())
            .replace("✗ Error", &"✗ Error".red().to_string());
        println!("{}", colored_table);
    } else {
        println!("{}", table);
    }

    Ok(())
}
