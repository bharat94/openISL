# openISL

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![CI/CD](https://img.shields.io/badge/CI%2FCD-Passing-success.svg)](https://github.com/bharat94/openISL/actions)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.1%20adopted-ff69b4.svg)](https://www.contributor-covenant.org/version/2/1/)

**openISL** - Interactive Smart Log - An intelligent CLI/TUI tool that enhances git workflow with advanced visualization, keyboard-driven navigation, and powerful commit history exploration.

## Overview

openISL is a modern Rust-based command-line tool that provides:
- **Advanced Git Visualization**: Interactive TUI with enhanced commit graph showing branch relationships
- **Comprehensive Git Operations**: Complete git wrapper for all common workflows
- **Syntax-Highlighted Diffs**: Color-coded diffs with 30+ language support
- **Interactive TUI**: Progressive terminal UI for exploring git history
- **Theme Support**: Dark, light, Monokai, and Nord themes
- **Keyboard-Driven Workflow**: Full keyboard navigation and command palette

## Features

### 🚀 Core Functionality
- **Smart Log Visualization**: Enhanced commit tree with distinct symbols for different commit types:
  - Initial commits (┌●)
  - Merge commits (┼●)
  - Tagged commits (◆●)
  - Revert commits (↩●)
  - Squash commits (≡●)
  - Branch points (┬●)
  - Regular commits (─●)
- **Syntax-Highlighted Diffs**: Auto-detects 30+ languages with keyword, type, string, and comment highlighting
- **Interactive TUI**: Full keyboard-driven exploration with multiple view modes
- **Git Operations Wrapper**: Complete git commands (log, branch, checkout, status, diff, remote, tag)
- **Theme System**: 4 built-in themes with dark/light variants
- **Customizable Keybindings**: Configure keyboard shortcuts via config file

### 🎯 Key Benefits
- **Clear Visualization**: Enhanced commit graph with commit type indicators
- **Fast Navigation**: Vim-style keyboard shortcuts and efficient panel switching
- **Rich Diff View**: Syntax-highlighted code changes with language detection
- **Search & Filter**: Search commits by author, message, or hash
- **Statistics**: Repository insights (commits by author, activity timeline)
- **Flexible Configuration**: Per-project and user-level config files

## Installation

### From Source
```bash
git clone https://github.com/bharat94/openISL.git
cd openISL
cargo build --release
cargo install --path .
```

### Using Cargo
```bash
cargo install openisl
```

## Quick Start

### Prerequisites
- Rust 1.70 or later
- Git 2.0 or later

### Installation

From source:
```bash
git clone https://github.com/bharat94/openISL.git
cd openISL
cargo build --release
cargo install --path .
```

Or using cargo:
```bash
cargo install openisl
```

### Usage

```bash
# Navigate to a git repository
cd /path/to/your/project

# View commit log (interactive TUI)
openisl log

# View commit log as ASCII in terminal
openisl log --simple

# View commits from specific branch
openisl log --branch develop

# View last 20 commits
openisl log -n 20

# List all branches
openisl branch

# List all branches including remotes
openisl branch --all

# Create a new branch
openisl branch feature/new-feature

# Checkout a branch or commit
openisl checkout develop

# View repository status
openisl status

# View changes (diff)
openisl diff

# View staged changes
openisl diff --staged

# View changes for specific commit
openisl diff --commit abc1234

# Configure settings
openisl config --show

# Set theme
openisl config --theme dark

# Manage remotes
openisl remote --list

# Manage tags
openisl tag --list
openisl tag --create v1.0.0
```

### Interactive TUI

Launch interactive smart log viewer:
```bash
openisl log
```

Keyboard shortcuts (in TUI):

Navigation:
- `j` / `k` or `↑` / `↓`: Navigate commits
- `h` / `l` or `←` / `→`: Navigate panels (files, branches, commits)
- `gg` / `Home`: Go to first commit
- `G` / `End`: Go to last commit
- `PageUp` / `PageDown`: Page through commits

Panel Controls:
- `Tab` / `Shift+Tab`: Switch between panels
- `Ctrl+B`: Toggle sidebar
- `Space`: Stage/unstage selected file

Commit Operations:
- `Enter`: View commit details
- `c`: Create branch from commit
- `b`: Create branch from commit
- `d` / `Shift+D`: View diff of selected commit
- `A`: Amend last commit
- `D`: Drop selected commit
- `S`: Squash selected commit into previous
- `C`: Cherry-pick selected commit
- `R`: Revert selected commit

File Operations:
- `Ctrl+S`: Stage all files
- `Ctrl+U`: Unstage all files

Search & Filter:
- `/`: Search commits (by message, author, hash)
- `Ctrl+N` / `Ctrl+P`: Navigate search results
- `f`: Filter commits (by author, message, date)

UI Controls:
- `t`: Toggle theme (dark/light/monokai/nord)
- `m`: Toggle mouse mode
- `s`: View repository statistics
- `?`: Show help
- `Ctrl+P`: Command palette
- `q` / `Esc`: Quit / go back

View Modes:
- List view: Commits with tree visualization
- Details view: Full commit information
- Diff view: Syntax-highlighted diff changes
- Help overlay: Full keyboard shortcuts
- Statistics view: Commits by author, activity timeline

## Documentation

Our documentation follows the [Diátaxis Framework](https://diataxis.fr/) for clear, user-centric content:

### Tutorials
- [Getting Started](docs/tutorials/getting-started.md) - Your first steps with openISL
- [Understanding Smart Log](docs/tutorials/understanding-smartlog.md) - How to read the commit tree
- [Git Operations](docs/tutorials/git-operations.md) - Common workflows

### How-to Guides
- [Installing openISL](docs/how-to-guides/installation.md) - Installation methods
- [Configuring openISL](docs/how-to-guides/configuration.md) - Customize behavior
- [Integration with CI/CD](docs/how-to-guides/ci-integration.md) - Use in pipelines

### Reference
- [CLI Commands](docs/cli-commands/) - Complete command reference
- [TUI Reference](docs/tui-reference/) - Terminal UI components
- [Configuration](docs/reference/configuration.md) - All configuration options

### Explanation
- [Architecture Overview](docs/explanation/architecture.md) - System design
- [Smart Log Algorithm](docs/explanation/smartlog-algorithm.md) - How commit tree works
- [Git Abstraction Layer](docs/explanation/git-abstraction.md) - Command mapping

## Contribution

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Reporting Issues
- Use our [issue templates](templates/issue-templates/)
- Follow our [code of conduct](CODE_OF_CONDUCT.md)
- Check existing issues first

### Development Workflow
1. Fork the repository
2. Create a feature branch (`openisl branch feature/my-feature`)
3. Follow [CONTRIBUTING.md](CONTRIBUTING.md) guidelines
4. Submit a pull request using our [PR template](templates/pr-templates/)

## Project Standards

openISL follows industry best practices for open source projects:

- **Documentation**: [Diátaxis Framework](https://diataxis.fr/)
- **Commits**: [Conventional Commits](https://www.conventionalcommits.org/)
- **Versioning**: [Semantic Versioning](https://semver.org/)
- **Changelog**: [Keep a Changelog](https://keepachangelog.com/)
- **Security**: [OpenSSF Security Baseline](https://baseline.openssf.org/)
- **Governance**: [Open Governance Model](GOVERNANCE.md)

See [Open Source Standards](OPEN_SOURCE_STANDARDS.md) for complete details.

## Roadmap

### v0.1.0 (Released!)
- [x] Workspace structure setup
- [x] Git abstraction layer (CLI wrapper)
- [x] Core CLI commands (log, branch, checkout, status, diff)
- [x] ASCII smart log visualization
- [x] Unit tests and documentation
- **Released**: 2026-01-10

### v0.2.0 (Released!)
- [x] Interactive TUI with commit tree
- [x] Keyboard navigation
- [x] TUI documentation
- [x] Dark/Light theme presets with toggle
- [x] TUI unit tests (10 tests)
- **Released**: 2026-01-10

### v0.3.0 (Released!)
- [x] Interactive commit details view
- [x] Help overlay with keyboard shortcuts
- [x] ViewMode enum (List, Details, Diff, Help)
- [x] View transitions (Enter, Esc, Shift+D)
- [x] Interactive checkout from TUI (display only)
- [x] Interactive branch creation from TUI (display only)
- [x] Diff viewer with actual diff content
- **Released**: 2026-01-10

### v0.4.0 (Released!)
- [x] Configuration file support (config.toml in ~/.config/openisl/)
- [x] Branch filtering options (--remote, --all flags)
- [x] Enhanced graph visualization (●○│ characters, main branch marker)
- [x] Keyboard customization (keybindings.toml)
- **Released**: 2026-01-10

### v0.5.0 (Released!)
- [x] Stash support (list, push, pop, apply, drop, show)
- [x] Commit search functionality in TUI
- [x] Ctrl+N/P for search result navigation
- [x] Search by message, author, hash, summary
- **Released**: 2026-01-11

### v0.6.0 (Released!)
- [x] Enhanced commit graph visualization with commit type symbols
- [x] Commit type detection (Initial, Merge, Tag, Revert, Squash, Branch, Regular)
- [x] Branch point indicators (┬) for multi-child commits
- [x] Lane color assignment (8 distinct colors)
- [x] Tag display in commit details
- [x] Syntax highlighting for code diffs (30+ languages)
- [x] Language auto-detection from file extension
- [x] Theme-aware syntax colors (dark/light)
- **Released**: 2026-01-11

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [CLI Guidelines](https://clig.dev/) - CLI design principles
- [Better CLI](https://bettercli.org/) - Interface design patterns
- [Open Source Guides](https://opensource.guide/) - Community practices
- [Diátaxis Framework](https://diataxis.fr/) - Documentation structure

## Contact

- **Website**: https://github.com/bharat94/openISL
- **Issues**: https://github.com/bharat94/openISL/issues
- **Discussions**: https://github.com/bharat94/openISL/discussions

---

**openISL** - Your intelligent companion for git visualization and workflow enhancement. It doesn't just simplify git - it provides advanced visualization, syntax-highlighted diffs, and an efficient keyboard-driven interface that adapts to how you work.
