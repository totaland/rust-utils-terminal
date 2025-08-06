# 🔍 Shell Alias Explorer

A beautiful and powerful Rust command-line tool for discovering and displaying all your shell aliases in an elegant table format on macOS.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![macOS](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=macos&logoColor=F0F0F0)

## ✨ Features

- 🎨 **Beautiful table display** with colors and rounded borders
- 🔍 **Smart alias detection** from both shell sessions and config files
- 📁 **Multi-source support** (.zshrc, .bashrc, .bash_profile, .aliases, etc.)
- 🔎 **Powerful filtering** by alias name or command content
- 📋 **Source tracking** to see where each alias is defined
- 🎯 **Duplicate elimination** across different sources
- 🌈 **Colorized output** with option for plain text
- ⚡ **Fast and lightweight** native Rust performance

## 🚀 Installation

### Prerequisites

- Rust (install from [rustup.rs](https://rustup.rs/))
- macOS (tested on macOS)

### Build from source

```bash
git clone <your-repo-url>
cd utils
cargo build --release
```

### Install globally

```bash
cargo install --path .
```

## 📖 Usage

### Basic usage

Display all aliases with beautiful formatting:

```bash
cargo run
```

### Command line options

```bash
# Show help
cargo run -- --help

# Filter aliases by name or command (case-insensitive)
cargo run -- --filter "git"
cargo run -- --filter "docker"

# Filter by source file
cargo run -- --source ".zshrc"
cargo run -- --source ".bashrc"

# Plain text output (no colors)
cargo run -- --plain

# Combine filters
cargo run -- --filter "npm" --source ".zshrc"
```

### Examples

```bash
# Find all git-related aliases
cargo run -- --filter "git"

# Show only aliases from .zshrc
cargo run -- --source ".zshrc"

# Find aliases containing "docker" with plain output
cargo run -- --filter "docker" --plain
```

## 🛠️ Technical Details

### Dependencies

- **tabled** - Beautiful table formatting with colors and styling
- **colored** - Terminal color output
- **clap** - Command-line argument parsing
- **anyhow** - Error handling
- **regex** - Pattern matching for alias parsing

### Supported Shell Configuration Files

The tool automatically scans these common shell configuration files:

- `.bashrc`
- `.bash_profile`
- `.bash_aliases`
- `.zshrc`
- `.zsh_aliases`
- `.profile`
- `.aliases`

### How it works

1. **Shell Session Aliases**: Executes the shell's `alias` command to get currently active aliases
2. **Configuration Files**: Parses shell configuration files to find alias definitions
3. **Deduplication**: Removes duplicate aliases, prioritizing shell session over config files
4. **Formatting**: Creates a beautiful table with proper column widths and colors
5. **Filtering**: Applies any user-specified filters before display

## 🎨 Output Format

```
🔍 Shell Alias Explorer
──────────────────────────────────────────────────
╭──────────────────────┬────────────────────────────────────────────────────┬────────╮
│        Alias         │                      Command                       │ Source │
├──────────────────────┼────────────────────────────────────────────────────┼────────┤
│ cat                  │ bat                                                │ .zshrc │
│ audit                │ npm audit --json | npx npm-audit-html --output... │ .zshrc │
│ rustapp              │ cd ~/git/desktop-app/my-next-app && nvm use...    │ .zshrc │
╰──────────────────────┴────────────────────────────────────────────────────┴────────╯

✨ Found 13 aliases
```

## 🔧 Development

### Project Structure

```
src/
├── main.rs          # Main application logic
└── ...

Cargo.toml           # Dependencies and project config
```

### Key Functions

- `get_all_aliases()` - Collects aliases from all sources
- `get_shell_aliases()` - Gets aliases from current shell session
- `get_config_file_aliases()` - Parses configuration files
- `display_aliases_table()` - Renders the beautiful table output
- `parse_alias_line()` - Parses individual alias definitions

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [tabled](https://github.com/zhiburt/tabled) - For the beautiful table formatting
- [colored](https://github.com/colored-rs/colored) - For terminal colors
- [clap](https://github.com/clap-rs/clap) - For command-line interface
- Rust community for excellent crates and documentation

---

Made with ❤️ and ☕ for macOS developers who love beautiful terminal tools!
