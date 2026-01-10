# openISL

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![CI/CD](https://img.shields.io/badge/CI%2FCD-Passing-success.svg)](https://github.com/bharat94/openISL/actions)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-v2.1%20adopted-ff69b4.svg)](https://www.contributor-covenant.org/version/2/1/)

**openISL** - Open Infrastructure & Stack Layer - A CLI tool that provides a unified, user-friendly interface for version control and project stack visualization in any git repository.

## Overview

openISL is a command-line tool that provides:
- **Stack Detection**: Automatically analyzes and displays the technology stack of any git repository
- **Unified Git Interface**: Abstracts away complex git commands with intuitive, consistent alternatives
- **Terminal UI (TUI)**: Interactive visual interface for stack browsing and project management
- **Repository Agnostic**: Works with any git repository, regardless of language or framework

## Features

### 🚀 Core Functionality
- **Stack Visualization**: Automatically detect and display project technologies (languages, frameworks, databases, tools)
- **Git Abstraction Layer**: Provide user-friendly commands that map to standard git operations
- **Interactive TUI**: Navigate your project stack and git operations through an intuitive terminal interface
- **Universal Compatibility**: Works with any git repository - monorepo, polyrepo, or single project

### 🎯 Key Benefits
- **Reduce Git Complexity**: Common operations become simple, intuitive commands
- **Immediate Stack Understanding**: Visual representation of your project's technology footprint
- **Unified Workflow**: Single tool for git operations and stack management
- **Developer-Friendly**: Designed following CLI/TUI best practices for modern developers

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

### Analyze Project Stack
```bash
# Navigate to any git repository
cd /path/to/your/project

# Display stack information
openisl stack

# Interactive stack viewer with detailed breakdown
openisl stack --interactive
```

### Using Unified Git Commands
```bash
# Save changes with consistent interface
openisl save "Add user authentication feature"

# Create new branch
openisl branch feature/new-feature

# Sync with remote
openisl sync

# View changes
openisl changes
```

### Interactive TUI
```bash
# Launch interactive terminal UI
openisl tui

# Shows:
# - Project structure visualization
# - Stack composition
# - Git history
# - Branch management
# - Quick actions
```

## Documentation

Our documentation follows the [Diátaxis Framework](https://diataxis.fr/) for clear, user-centric content:

### Tutorials
- [Getting Started](docs/tutorials/getting-started.md) - Your first steps with openISL
- [Stack Detection](docs/tutorials/stack-detection.md) - Understanding stack analysis
- [Git Operations](docs/tutorials/git-operations.md) - Unified git commands

### How-to Guides
- [Installing openISL](docs/how-to-guides/installation.md) - Installation methods
- [Configuring Detection](docs/how-to-guides/configuring-detection.md) - Customize stack analysis
- [Integration with CI/CD](docs/how-to-guides/ci-integration.md) - Use in pipelines

### Reference
- [CLI Commands](docs/cli-commands/) - Complete command reference
- [TUI Reference](docs/tui-reference/) - Terminal UI components
- [Configuration](docs/reference/configuration.md) - All configuration options

### Explanation
- [Architecture Overview](docs/explanation/architecture.md) - System design
- [Stack Detection Algorithm](docs/explanation/detection-algorithm.md) - How analysis works
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

### v0.1.0 (Current)
- [ ] Basic stack detection
- [ ] Core git abstraction commands
- [ ] Basic TUI interface
- [ ] Repository analysis

### v0.2.0 (Planned)
- [ ] Advanced stack visualization
- [ ] Custom detection rules
- [ ] Plugin system
- [ ] Configuration profiles

### v1.0.0 (Future)
- [ ] Full git porcelain implementation
- [ ] Multi-repo support
- [ ] Stack comparison tool
- [ ] Export stack as diagram

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

**openISL** - Making version control and stack understanding accessible to all developers.
