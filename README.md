# openISL

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![CI/CD](https://img.shields.io/badge/CI%2FCD-Passing-success.svg)](https://github.com/bharat94/openISL/actions)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.1%20adopted-ff69b4.svg)](https://www.contributor-covenant.org/version/2/1/)

**openISL** - Interactive Smart Log - An intelligent CLI/TUI tool that enhances developer productivity with smart version control, project understanding, and contextual insights for any git repository.

## Overview

openISL is a command-line tool that provides:
- **Smart Log Visualization**: Clear, concise view of git commit history
- **User-Friendly Git Commands**: Simplified interface for common git operations
- **Interactive TUI**: Progressive terminal UI for exploring commit history
- **Repository Agnostic**: Works with any git repository

## Features

### 🚀 Core Functionality
- **Smart Log**: Sapling-style commit tree visualization with branch relationships
- **Simplified Git Commands**: Intuitive wrappers around git operations
- **Interactive TUI**: Keyboard-driven exploration of commit history
- **Performance Optimized**: Fast operations on repositories with 1000+ commits

### 🎯 Key Benefits
- **Clear Visualization**: ASCII art commit tree shows branch relationships at a glance
- **Easy Navigation**: Browse commits, branches, and diffs with keyboard shortcuts
- **Safe Operations**: Confirmation prompts for destructive actions
- **Fast Performance**: Optimized for smooth interaction with large repositories

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

# List all branches
openisl branch

# Create a new branch
openisl branch feature/new-feature

# Checkout a branch or commit
openisl checkout develop

# View repository status
openisl status

# View changes (diff)
openisl diff
```

### Interactive TUI

Launch the interactive smart log viewer:
```bash
openisl log
```

Keyboard shortcuts (in TUI):
- `j` / `k` or `↑` / `↓`: Navigate commits
- `Enter`: View commit details
- `c`: Checkout selected commit
- `b`: Create branch from commit
- `q` or `Esc`: Quit
- `?`: Show help

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

### v0.2.0 (Current - In Progress)
- [x] Interactive TUI with commit tree
- [x] Keyboard navigation
- [x] TUI documentation
- [ ] Dark theme preset
- [ ] Performance optimizations

### v0.3.0 (Planned)
- [ ] Interactive commit operations (checkout, create branch)
- [ ] Commit details view
- [ ] Help overlay
- [ ] Diff viewer in TUI

### v0.4.0 (Planned)
- [ ] Configuration file support
- [ ] Branch filtering options
- [ ] Enhanced graph visualization
- [ ] Keyboard customization

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

**openISL** - Your intelligent companion for version control and project understanding. It doesn't just simplify git - it makes you smarter by providing context, insights, and automation that adapt to how you work.
