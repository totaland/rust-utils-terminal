# 🔧 Shell Function Explorer - New Feature

The shell explorer utility now supports exploring shell functions in addition to aliases! This powerful new feature can parse and display all functions defined in your shell configuration files with their descriptions and usage information.

## 🆕 What's New

### Function Explorer Mode

You can now use the tool to explore shell functions with:
```bash
./utils --mode functions
```

## Features

✅ **Smart Function Parsing**: Automatically detects function definitions in various formats
✅ **Documentation Extraction**: Pulls descriptions and usage from comments
✅ **Multiple Comment Formats**: Supports standard comments, JSDoc-style, and explicit tags
✅ **Usage Detection**: Automatically infers parameter usage from function bodies
✅ **Rich Table Display**: Beautiful colored tables with proper text wrapping
✅ **Filtering Support**: Filter functions by name, description, usage, or source
✅ **Multiple Sources**: Scans .zshrc, .bashrc, .zsh_functions, .bash_functions, etc.

## Documentation Formats Supported

### 1. Standard Comments
```bash
# This function kills processes running on a specific port
# Usage: killport 3000
function killport() {
    lsof -ti:$1 | xargs kill -9
}
```

### 2. JSDoc-Style Comments
```bash
# @description Generate a tree structure of repository files
# @param output_file The output file path
# @param repo_path The repository path
function generate_repo_structure() {
    # function implementation
}
```

### 3. Explicit Tags
```bash
# desc: List all files excluding build directories
# usage: listFiles [directory]
function listFiles() {
    # function implementation
}
```

## Command Examples

### Basic Usage
```bash
# Show all functions
./utils --mode functions

# Filter functions containing 'git'
./utils --mode functions --filter git

# Filter by source file
./utils --mode functions --source .zshrc

# Plain text output (no colors)
./utils --mode functions --plain
```

### Example Output
```
🔧 Shell Function Explorer
────────────────────────────────────────────────────────────
🔍 Filtering by: git
┌──────────────────┬──────────────────────────────────────────┬─────────────────────────────────┬────────┐
│  Function Name   │               Description                │             Usage               │ Source │
├──────────────────┼──────────────────────────────────────────┼─────────────────────────────────┼────────┤
│ git_commit_count │ Show commit count for each author in a G │ git_commit_count .              │ .zshrc │
│                  │ it repository, sorted by commit count    │                                 │        │
│ listFiles        │ List all files in a directory excluding  │ listFiles    #  Will prompt for │ .zshrc │
│                  │ common build folders (node_modules, d... │ directory                       │        │
└──────────────────┴──────────────────────────────────────────┴─────────────────────────────────┴────────┘
✨ Found 2 functions
```

## Automatic Parameter Detection

The function parser is smart enough to detect parameters from the function body:

- **$1, $2, $3 usage**: Detects positional parameters
- **getopts**: Recognizes option parsing → shows `[options]`
- **Argument counting**: Checks for `$#` conditions to determine required args
- **shift commands**: Indicates variable arguments → shows `[args...]`
- **local variables**: Detects parameter assignments

## Integration with Existing Tool

The function explorer seamlessly integrates with the existing alias explorer:

```bash
# Default behavior - shows aliases (unchanged)
./utils

# New function mode
./utils --mode functions

# All existing filters work with functions too
./utils --mode functions --filter "backup" --source ".zshrc"
```

## Technical Implementation

- **Robust Parsing**: Handles various function definition formats (`function name()`, `name()`, `function name`)
- **Comment Block Detection**: Intelligently groups related comments above functions
- **Multi-line Support**: Properly parses functions spanning multiple lines
- **Source Tracking**: Shows which configuration file contains each function
- **Text Wrapping**: Ensures long descriptions and usage text fit in the table
- **Performance**: Efficient parsing of large configuration files

This enhancement makes the shell explorer a comprehensive tool for understanding your entire shell environment!
