# Contributing to openISL

Thank you for your interest in contributing to openISL! We welcome contributions from everyone and value all feedback and contributions.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Style Guidelines](#style-guidelines)
- [Testing](#testing)
- [Documentation](#documentation)
- [Getting Help](#getting-help)

## Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md) in all interactions with the project.

## Getting Started

### Prerequisites

- **Rust** (latest stable) via [rustup](https://rustup.rs/) — the project targets Rust 1.70+
- **Git** (2.0+)

### Setting Up the Development Environment

```bash
# Clone your fork
git clone https://github.com/your-username/openISL.git
cd openISL

# Add the upstream remote
git remote add upstream https://github.com/bharat94/openISL.git

# Build and run tests to verify the setup
cargo build
cargo test
```

## Project Structure

```
openISL/
├── cli/          # Command-line interface (produces the `openisl` binary)
├── tui/          # Terminal user interface (ratatui)
├── git/          # Git abstraction layer
├── docs/         # Documentation (CLI reference, TUI reference)
├── templates/    # Issue templates
├── .github/      # PR template and CI workflows
└── Cargo.toml    # Workspace configuration
```

### Module Responsibilities

- **cli/**: Command parsing, configuration, command execution
- **tui/**: Terminal UI components, interactive navigation, rendering, hunk staging
- **git/**: Git operations abstraction, command mapping, output parsing

## Making Changes

### 1. Find an Issue or Create One

Check the [issue tracker](https://github.com/bharat94/openISL/issues) for open issues. Use the [issue templates](templates/issue-templates/) for new issues.

### 2. Create a Branch

```bash
git checkout -b feature/descriptive-name
```

Branch naming conventions:

- `feature/description` — new features
- `fix/description` — bug fixes
- `docs/description` — documentation changes
- `refactor/description` — code refactoring
- `test/description` — test additions

### 3. Make Your Changes

#### Code Standards

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Pass `cargo clippy --all-targets -- -D warnings`
- Write clear, self-documenting code
- See [AGENTS.md](AGENTS.md) for the full style guide used by tooling

#### Conventional Commits

We use [Conventional Commits](https://www.conventionalcommits.org/) for commit messages:

```bash
# Feature
git commit -m "feat(tui): add interactive blame viewer"

# Bug fix
git commit -m "fix(git): resolve branch detection in monorepos"

# Documentation
git commit -m "docs: update installation guide"

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

Add a scope in parentheses (`feat(git)`, `fix(cli)`, `docs(tui)`) and `!` for breaking changes (`feat(api)!`).

### 4. Write Tests

We require tests for new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_commits_returns_commits() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_commits(&repo_path, Some(5));
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}
```

Run tests before committing:

```bash
cargo test
```

The `git` crate has integration tests that create real temporary repositories (with `tempfile`) and run actual Git operations; these run as part of the normal `cargo test`.

### 5. Update Documentation

- Update the relevant docs in `docs/` (CLI commands, TUI reference, or `ARCHITECTURE.md`)
- Add examples for new features
- Update [CHANGELOG.md](CHANGELOG.md) under `[Unreleased]` if the change is user-facing

### 6. Submit Your Changes

```bash
# Sync with upstream
git fetch upstream
git rebase upstream/main

# Push to your fork
git push origin feature/descriptive-name

# Create a pull request
gh pr create
# Or visit: https://github.com/bharat94/openISL/compare
```

Use our [PR template](.github/PULL_REQUEST_TEMPLATE/pr-template.md) for new pull requests.

### PR Checklist

Before submitting, ensure:

- [ ] Code follows project style guidelines
- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy --all-targets -- -D warnings`)
- [ ] Formatting is clean (`cargo fmt --check`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated under `[Unreleased]` (if user-facing)
- [ ] Commit messages follow Conventional Commits
- [ ] PR description references the linked issue
- [ ] Breaking changes are clearly documented

### PR Review Process

Maintainers will review your PR. Expect feedback on code quality and style, test coverage, documentation completeness, and design decisions. Respond to feedback promptly; if significant changes are requested, make them in the same branch.

## Style Guidelines

### Rust Code

- Use `cargo fmt` for consistent formatting
- Address all `cargo clippy` warnings
- Prefer idiomatic Rust patterns
- Document public APIs with `///` comments
- Handle errors with `anyhow::Context`/`?` instead of `unwrap()`/`expect()`

### Documentation

- Write clear, concise explanations
- Include code examples
- Explain "why", not just "what"
- Follow the existing doc structure in `docs/`
- Use inclusive language

### TUI Components

- Keep everything keyboard-navigable
- Provide helpful status/error messages
- Show progress for long operations
- Use the theme system consistently

## Testing

### Test Coverage

We aim for high test coverage. To generate a report:

```bash
cargo tarpaulin --out Html
open tarpaulin-report.html
```

### Integration Tests

The `git` crate's integration tests (`git/tests/`) use real temporary repositories created with `tempfile`. They run as part of `cargo test` and require a working `git` binary.

## Documentation

The docs follow a practical layout:

- `README.md` — landing page, quick start, TUI overview
- `docs/cli-commands/` — per-command reference
- `docs/tui-reference/` — TUI layout, keymap, themes
- `ARCHITECTURE.md` — design and component overview

## Getting Help

- **Questions**: ask in [Discussions](https://github.com/bharat94/openISL/discussions)
- **Bug Reports**: use the [bug report template](templates/issue-templates/bug.md)
- **Feature Requests**: use the [feature request template](templates/issue-templates/feature.md)

Thank you for contributing to openISL!

---

See also:

- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Open Source Standards](OPEN_SOURCE_STANDARDS.md)
- [Governance Model](GOVERNANCE.md)
- [Security Policy](SECURITY.md)