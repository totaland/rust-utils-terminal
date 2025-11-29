# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust command-line utility called "shell-explorer" that analyzes and displays shell aliases, functions, package versions, and Chrome bookmarks from configuration files and the current shell session. The project is structured as both a library (`utils`) and a binary (`shell-explorer`).

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
cargo run -- --mode bookmarks --subcommand stats

# Run with specific arguments
cargo run -- --mode functions --filter git
cargo run -- --filter "ssh" --source .zshrc
cargo run -- --mode packages --package typescript --min-version 4.0.0 --path ./src

# Bookmark commands
cargo run -- --mode bookmarks --subcommand stats
cargo run -- --mode bookmarks --subcommand duplicates
cargo run -- --mode bookmarks --subcommand domains
cargo run -- --mode bookmarks --subcommand categories
cargo run -- --mode bookmarks --subcommand search --query "github"
cargo run -- --mode bookmarks --subcommand organize
cargo run -- --mode bookmarks --subcommand export --output bookmarks.md

# Install globally
cargo install --path .

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
- **`cli.rs`**: Command-line interface definition using clap, with modes for aliases/functions/packages/bookmarks and filtering options
- **`aliases.rs`**: Shell alias discovery and parsing from both live shell sessions and config files
- **`functions.rs`**: Shell function discovery with sophisticated parsing of function definitions and documentation
- **`packages.rs`**: Package version discovery and comparison from various package management files
- **`bookmarks.rs`**: Chrome bookmarks parsing, analysis, and organization with AI/ML category detection
- **`organizer.rs`**: File organization by type for non-development folders
- **`cleaner.rs`**: Node modules cleanup utility
- **`display.rs`**: Table formatting and output rendering using the tabled crate
- **`lib.rs`**: Module exports and public API

### Key Features

**Multi-Mode Operation**: 
- Aliases mode: Discovers aliases from shell session (`alias` command) and config files
- Functions mode: Parses shell functions from config files with documentation extraction
- Packages mode: Finds package versions greater than a specified threshold in various package files
- Clean mode: Removes node_modules directories recursively with interactive selection
- Organize mode: Organizes files in non-development folders by file type
- Bookmarks mode: Analyzes and organizes Chrome bookmarks with smart categorization

**Chrome Bookmarks Features**:
- Parses Chrome's JSON bookmarks file (`~/Library/Application Support/Google/Chrome/Default/Bookmarks`)
- Automatic categorization based on URL and title analysis
- AI/ML specialized categories with subcategories:
  - 🤖 AI/ML General - TensorFlow, PyTorch, Kaggle
  - 🧠 LLMs & Models - OpenAI, Anthropic, Claude, GPT, Mistral, Llama
  - ✍️ Prompt Engineering - Chain-of-thought, few-shot, DSPy
  - 🤝 AI Agents - AutoGPT, CrewAI, LangChain agents, MCP
  - 📚 RAG - LlamaIndex, semantic search, document retrieval
  - 🔗 Context & Memory - Context windows, mem0, MemGPT
  - 🎯 Fine-Tuning - LoRA, QLoRA, PEFT, RLHF
  - 📊 Embeddings - Sentence transformers, text-embedding
  - 🗄️ Vector Databases - Pinecone, Weaviate, Milvus, ChromaDB
  - ⚙️ MLOps - MLflow, W&B, model deployment
  - 👁️ Computer Vision - Stable Diffusion, DALL-E, YOLO
  - 💬 NLP - spaCy, NLTK, sentiment analysis
  - 🔬 AI Research - arXiv, Papers with Code
- General categories: Development, Social, News, Shopping, Entertainment, Education, Reference, Tools, Finance, Health, Travel, Food, Sports, Gaming, Music, Video
- Duplicate detection and domain/category statistics
- Organization suggestions based on content analysis
- Export to markdown format

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
- `serde_json`: JSON parsing for Chrome bookmarks
- `rayon`: Parallel processing
- `crossterm`: Terminal manipulation for interactive modes
- `reqwest`: HTTP client (for future link checking features)

## Binary Output

The built binary is named `utils` but installs as `shell-explorer`. The install script provides multiple installation options including system-wide and user-local paths.

Note: If you have an older version in `~/.local/bin`, you may need to copy the new binary there after `cargo install`:
```bash
cp ~/.cargo/bin/shell-explorer ~/.local/bin/shell-explorer
```