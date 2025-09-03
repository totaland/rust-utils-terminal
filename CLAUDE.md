# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust command-line utility called "shell-explorer" that analyzes and displays shell aliases, functions, and package versions from configuration files and the current shell session. The project is structured as both a library (`utils`) and a binary (`shell-explorer`).

## Build and Development Commands

```bash
# Build the project (development)
cargo build

# Build release version
cargo build --release

# Run the project
cargo run -- --mode aliases
cargo run -- --mode functions
cargo run -- --mode packages --package react --min-version 17.0.0

# Run with specific arguments
cargo run -- --mode functions --filter git
cargo run -- --filter "ssh" --source .zshrc
cargo run -- --mode packages --package typescript --min-version 4.0.0 --path ./src

# Install using the provided script
./install.sh

# Run tests (if present)
cargo test

# Check code formatting
cargo fmt --check

# Run clippy linter
cargo clippy
```

## Architecture

### Core Components

- **`main.rs`**: Entry point that handles CLI parsing and delegates to mode handlers
- **`cli.rs`**: Command-line interface definition using clap, with modes for aliases/functions/packages and filtering options
- **`aliases.rs`**: Shell alias discovery and parsing from both live shell sessions and config files
- **`functions.rs`**: Shell function discovery with sophisticated parsing of function definitions and documentation
- **`packages.rs`**: Package version discovery and comparison from various package management files
- **`display.rs`**: Table formatting and output rendering using the tabled crate
- **`lib.rs`**: Module exports and public API

### Key Features

**Multi-Mode Operation**: 
- Aliases mode: Discovers aliases from shell session (`alias` command) and config files
- Functions mode: Parses shell functions from config files with documentation extraction
- Packages mode: Finds package versions greater than a specified threshold in various package files

**Shell Config File Support**: 
Automatically searches common shell configuration files:
- `.zshrc`, `.bashrc`, `.bash_profile`, `.profile`
- `.bash_aliases`, `.zsh_aliases`, `.aliases`
- `.zsh_functions`, `.bash_functions`

**Advanced Function Parsing**:
- Detects multiple function definition formats (`function name()`, `name()`)
- Extracts documentation from comment blocks above functions
- Recognizes JSDoc-style comments (`@desc`, `@param`)
- Infers usage patterns from function body analysis
- Handles parameter detection and usage string generation

**Package Version Analysis**:
- Supports multiple package file formats: `package.json`, `Cargo.toml`, `requirements.txt`, `pyproject.toml`, `composer.json`, `go.mod`, and more
- Semantic version parsing with support for pre-release versions
- Recursive directory searching with smart exclusions (node_modules, target, .git)
- Version comparison using semantic versioning rules

**Output Features**:
- Colored terminal output with emoji indicators
- Filtering by name/command patterns and source files
- Plain text mode for scripting
- Tabulated display with source attribution

## Dependencies

- `tabled`: Table formatting and display
- `colored`: Terminal color output
- `clap`: Command-line argument parsing with derive features
- `anyhow`: Error handling and context
- `regex`: Pattern matching for parsing

## Binary Output

The built binary is named `utils` but installs as `shell-explorer`. The install script provides multiple installation options including system-wide and user-local paths.