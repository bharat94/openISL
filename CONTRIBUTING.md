# Contributing to openISL

Thank you for your interest in contributing to openISL! We welcome contributions from everyone and value all feedback and contributions.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Submiting Changes](#submiting-changes)
- [Style Guidelines](#style-guidelines)
- [Testing](#testing)
- [Documentation](#documentation)

## Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md) in all interactions with the project.

## Getting Started

### Prerequisites
- **Rust** (latest stable version) - Primary language
- **Git** - For version control
- **Cargo** - Rust package manager

### Setting Up Development Environment

```bash
# Clone your fork
git clone https://github.com/your-username/openISL.git
cd openISL

# Add upstream remote
git remote add upstream https://github.com/bharat94/openISL.git

# Install dependencies
cargo build

# Run tests to verify setup
cargo test
```

## Project Structure

```
openISL/
├── crates/
│   ├── cli/              # Command-line interface
│   ├── tui/              # Terminal user interface
│   ├── stack/             # Stack detection and analysis
│   └── git/              # Git abstraction layer
├── docs/                   # Documentation
├── templates/               # Issue and PR templates
├── tests/                   # Integration and unit tests
└── Cargo.toml             # Workspace configuration
```

### Module Responsibilities
- **cli/**: Command parsing, argument handling, command execution
- **tui/**: Terminal UI components, interactive navigation, visualization
- **stack/**: Technology detection algorithms, parser implementations
- **git/**: Git operations abstraction, command mapping

## Making Changes

### 1. Find an Issue or Create One

Check our [issue tracker](https://github.com/bharat94/openISL/issues) for open issues. Use our [issue templates](templates/issue-templates/) for new issues.

### 2. Create a Branch

Use our unified git interface or conventional naming:

```bash
# Using openISL
openisl branch feature/descriptive-name

# Or using git
git checkout -b feature/descriptive-name
```

Branch naming conventions:
- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation changes
- `refactor/description` - Code refactoring
- `test/description` - Test additions

### 3. Make Your Changes

#### Code Standards
- Follow [Rust style guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` before committing
- Pass `cargo clippy` checks
- Write clear, self-documenting code

#### Conventional Commits

We use [Conventional Commits](https://www.conventionalcommits.org/) for structured commit messages:

```bash
# Feature
git commit -m "feat: add stack visualization for dependencies"

# Bug fix
git commit -m "fix: resolve branch detection issue in monorepos"

# Documentation
git commit -m "docs: update installation guide for Windows"

# Breaking change
git commit -m "feat(cli)!: change command syntax to be more intuitive"
```

Commit types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements
- `ci`: CI/CD changes

Add scope in parentheses: `feat(stack)`, `fix(cli)`, `docs(git)`.

Add `!` after type for breaking changes: `feat(api)!`.

### 4. Write Tests

We require tests for all new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_detection() {
        // Test implementation
    }
}
```

Run tests before committing:
```bash
cargo test
cargo test --all-features
```

### 5. Update Documentation

- Update relevant documentation in `docs/`
- Add examples for new features
- Update [CHANGELOG.md](CHANGELOG.md) if user-facing
- Update API reference if applicable

### 6. Submit Your Changes

```bash
# Sync with upstream
git fetch upstream
git rebase upstream/main

# Push to your fork
git push origin feature/descriptive-name

# Create pull request
openisl pr
# Or visit: https://github.com/bharat94/openISL/compare
```

## Submiting Changes

### Pull Request Guidelines

Use our [PR template](templates/pr-templates/pr-template.md) for new pull requests.

### PR Checklist

Before submitting, ensure:

- [ ] Code follows project style guidelines
- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (if user-facing)
- [ ] Commit messages follow Conventional Commits
- [ ] PR description references linked issue
- [ ] Breaking changes are clearly documented

### PR Review Process

Maintainers will review your PR within 5 business days. Expect feedback on:

- Code quality and style
- Test coverage
- Documentation completeness
- Architecture and design decisions

Respond to feedback promptly. If significant changes are requested, make them in the same branch.

## Style Guidelines

### Rust Code
- Use `cargo fmt` for consistent formatting
- Address `cargo clippy` warnings
- Prefer idiomatic Rust patterns
- Document public APIs with `///` comments
- Use `unwrap_or_default()` carefully - handle errors appropriately

### Documentation
- Write clear, concise explanations
- Include code examples
- Follow [Diátaxis Framework](https://diataxis.fr/) (tutorials, how-to, reference, explanation)
- Use inclusive language

### TUI Components
- Follow accessibility guidelines (keyboard navigation, screen reader support)
- Provide helpful error messages
- Show progress for long operations
- Use consistent color schemes

## Testing

### Test Coverage

We aim for high test coverage. Run:
```bash
# Generate coverage report
cargo tarpaulin --out Html

# Check coverage
open tarpaulin-report.html
```

### Integration Tests

For integration tests:
```bash
# Run integration tests
cargo test --test integration

# Test with real git repositories
cargo test --test integration --features git-tests
```

## Documentation

### Writing Docs

Follow [Google Technical Writing](https://developers.google.com/tech-writing/) guidelines:
- Write for your audience
- Use clear, simple language
- Provide concrete examples
- Explain "why", not just "what"

### Doc Types

- **Tutorials**: Step-by-step learning paths
- **How-to guides**: Task-focused instructions
- **Reference**: Complete command/API documentation
- **Explanation**: Deep dives into concepts

## Getting Help

- **Questions**: Ask in [Discussions](https://github.com/bharat94/openISL/discussions)
- **Bug Reports**: Use [issue templates](templates/issue-templates/bug.md)
- **Feature Requests**: Use [issue templates](templates/issue-templates/feature.md)

## Recognition

Contributors are recognized in:
- [CONTRIBUTORS.md](CONTRIBUTORS.md) - List of contributors
- [CHANGELOG.md](CHANGELOG.md) - Credit in release notes
- GitHub release notes - Highlighted contributions

Thank you for contributing to openISL! 🚀

---

See also:
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Open Source Standards](OPEN_SOURCE_STANDARDS.md)
- [Governance Model](GOVERNANCE.md)
- [Security Policy](SECURITY.md)
