# openISL — Interactive Smart Log

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://img.shields.io/github/actions/workflow/status/bharat94/openISL/ci.yml?branch=main&label=CI)](https://github.com/bharat94/openISL/actions)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.1%20adopted-ff69b4.svg)](CODE_OF_CONDUCT.md)
[![Rust](https://img.shields.io/badge/Rust-2021%20Edition-orange.svg)](https://www.rust-lang.org/)

**openISL** (Interactive Smart Log) is a fast, keyboard-driven Git client for the terminal. It pairs a rich interactive TUI with a complete command-line wrapper around Git: visualize commit history with an enhanced commit graph, browse syntax-highlighted diffs, and stage changes hunk-by-hunk — all without leaving your terminal.

## Features

- **Interactive TUI** — commit graph with type-specific symbols and branch lanes, a sidebar (branches, file status, stashes), and multiple view modes: details, diff, statistics, search, filter, and hunk staging.
- **Syntax-highlighted diffs** — language-aware coloring for 30+ languages with auto-detection from file extensions.
- **Hunk & line staging** — stage individual hunks or even single lines of a file, unstaged and staged, directly from the TUI.
- **Search, filter & stats** — find commits by message/author/hash, filter by author/message/date, and get repository statistics.
- **Commit operations** — amend, drop, squash, cherry-pick, and revert from the TUI.
- **Command palette & themes** — searchable actions, plus dark, light, Monokai, and Nord themes.
- **Git command wrapper** — `log`, `branch`, `checkout`, `status`, `diff`, `remote`, and `tag` with smart output.

## Installation

### Prerequisites

- Rust (1.70 or later) — [rustup](https://rustup.rs/)
- Git (2.0 or later)

### From source

```bash
git clone https://github.com/bharat94/openISL.git
cd openISL
cargo install --path cli
```

This builds and installs the `openisl` binary. Verify with:

```bash
openisl --version
```

For development, use `cargo run -p openisl-cli` instead of installing.

## Quick Start

```bash
# Navigate to a Git repository
cd /path/to/your/project

# Explore history in the interactive TUI
openisl tui

# Or as text / an ASCII tree
openisl log
openisl log --simple

# Limit history
openisl log -n 20
openisl log --branch develop

# Branches
openisl branch            # list local branches
openisl branch --all      # list local + remote
openisl branch feat/x     # create a branch

# Switch branches / commits
openisl checkout develop

# Working tree
openisl status
openisl diff              # unstaged changes
openisl diff --staged     # staged changes
openisl diff abc1234      # changes introduced by a commit

# Remotes and tags
openisl remote --list
openisl tag               # list tags
openisl tag v1.0.0        # create a tag
```

## Interactive TUI

Run `openisl tui` in any repository. The most common keys:

| Key | Action |
|-----|--------|
| `j` / `k` or `↑` / `↓` | Move |
| `Enter` | Commit details |
| `Shift+D` | Diff view |
| `i` | Hunk staging (on a file in Diff view) |
| `Tab` / `Shift+Tab` | Switch panel |
| `/` | Search |
| `f` | Filter |
| `s` | Statistics |
| `Ctrl+P` | Command palette |
| `?` | Help overlay |
| `q` / `Esc` | Quit / go back |

Press `?` inside the TUI for the complete, up-to-date keymap. View modes include **List**, **Details**, **Diff**, **Hunk staging**, **Statistics**, **Search**, **Filter**, **Stash**, and a **Command palette**.

## Configuration

openISL reads `~/.config/openisl/config.toml`. View or change it with:

```bash
openisl config --show
openisl config --theme dark
openisl config --max-commits 500
```

See the [Configuration reference](docs/cli-commands/config.md) for all fields and environment variable overrides.

## Documentation

- [CLI Commands](docs/cli-commands/) — full reference for every command
- [TUI Reference](docs/tui-reference/tui.md) — layout, keymap, themes
- [Architecture](ARCHITECTURE.md) — design and component overview
- [Changelog](CHANGELOG.md) — release history
- [Contributing](CONTRIBUTING.md) — how to get involved

## Project Structure

```
openISL/
├── cli/        # Command-line interface (openisl binary)
├── tui/        # Terminal user interface (ratatui)
├── git/        # Git abstraction layer
├── docs/       # Documentation
└── Cargo.toml  # Workspace configuration
```

## Contributing

We welcome contributions! Start with [CONTRIBUTING.md](CONTRIBUTING.md), use the [issue templates](templates/issue-templates/), and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

## Project Standards

- **Commits**: [Conventional Commits](https://www.conventionalcommits.org/)
- **Changelog**: [Keep a Changelog](https://keepachangelog.com/)
- **Versioning**: [Semantic Versioning](https://semver.org/)
- **Security**: [Security Policy](SECURITY.md)
- **Governance**: [Governance Model](GOVERNANCE.md)

See [Open Source Standards](OPEN_SOURCE_STANDARDS.md) for complete details.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgments

- [ratatui](https://github.com/ratatui-org/ratatui) — TUI framework
- [crossterm](https://github.com/crossterm-rs/crossterm) — terminal handling
- [clap](https://github.com/clap-rs/clap) — command-line parsing
- [CLI Guidelines](https://clig.dev/) — CLI design principles