# AGENTS.md

This file provides guidance for agentic coding tools working in the openISL repository.

## Build Commands

### Building
```bash
cargo build              # Debug build
cargo build --release    # Release build (optimized)
cargo run -p openisl-cli -- <args>   # Run the CLI without installing
```

### Linting & Formatting
```bash
cargo fmt               # Format code
cargo fmt --check       # Check formatting (CI)
cargo clippy            # Run linter
cargo clippy --all-targets -- -D warnings  # Treat warnings as errors (CI)
```

### Testing
```bash
cargo test              # Run all workspace tests
cargo test -p openisl-git    # Test the git crate only
cargo test -p openisl-tui    # Test the TUI crate only
cargo test -p openisl-cli    # Test the CLI crate only

# Run a single test
cargo test test_name           # Test by name
cargo test -- --exact test_name  # Exact match
```

CI runs `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`,
and `cargo test --all` (see `.github/workflows/ci.yml`). Keep all three green.

### Coverage
```bash
cargo tarpaulin --out Html    # Generate coverage report
open tarpaulin-report.html     # View coverage
```

### Installation
```bash
cargo install --path cli      # Install the openisl binary from source
```

## Code Style Guidelines

### Rust-Specific
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Always run `cargo fmt` before committing
- Address all `cargo clippy` warnings
- Use idiomatic Rust patterns over C-style approaches
- Prefer `&str` over `String` for function arguments when ownership not needed
- Use `Cow<str>` when you might need owned or borrowed strings
- Leverage `Option` and `Result` instead of null/exceptions

### Imports
- Group imports: std → external → local
- Use `use` statements at top of file
- Prefer qualified paths for rare imports to avoid name conflicts
- Re-export commonly used types at module level

```rust
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use crate::operations::get_commits;
```

### Types & Naming
- Use `PascalCase` for types, structs, enums
- Use `snake_case` for functions, variables, modules
- Use `SCREAMING_SNAKE_CASE` for constants
- Avoid abbreviations unless widely understood
- Name boolean variables with `is_`, `has_`, `can_` prefix
- Use descriptive names that reveal intent

```rust
struct CommitList { }
fn get_commits(path: &Path) -> Result<Vec<Commit>> { }
const MAX_COMMITS: usize = 100;
let is_dirty = true;
```

### Error Handling
- Use `Result<T, E>` for recoverable errors
- Use `anyhow::Result` for application errors with context
- Use `thiserror` for library error types
- Add context with `.context()` from anyhow
- Avoid `unwrap()`/`expect()` except in tests or when truly infallible
- Use `?` operator for error propagation

```rust
use anyhow::{Context, Result};

fn get_diff(path: &Path) -> Result<String> {
    run(&["diff"], Some(path))
        .with_context(|| format!("Failed to get diff in {}", path.display()))
}
```

### Documentation
- Document all public APIs with `///` doc comments
- Include examples for public functions
- Explain "why" not just "what"
- Use Markdown formatting in docs
- Run `cargo doc --no-deps` to verify docs

```rust
/// Fetches commits, optionally scoped to a single branch.
///
/// # Arguments
///
/// * `repo_path` - Path to the git repository
/// * `max_count` - Maximum number of commits to return
///
/// # Returns
///
/// A `Vec<Commit>` ordered newest first.
pub fn get_commits(repo_path: &Path, max_count: Option<usize>) -> Result<Vec<Commit>> {
    // Implementation
}
```

### Testing
- Write unit tests in `#[cfg(test)]` modules
- Use descriptive test names that describe what is being tested
- Use `assert_eq!`/`assert_ne!` with meaningful messages
- Test both success and error paths
- Mock external dependencies (git, file system); the `git` crate has
  integration tests that create real temporary repos with `tempfile`
- Follow Arrange-Act-Assert pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_commits_returns_commits() {
        // Arrange
        let repo_path = std::env::current_dir().unwrap();

        // Act
        let result = get_commits(&repo_path, Some(5));

        // Assert
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}
```

### Commit Messages
Follow Conventional Commits format:
```
type[optional scope]: description

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `perf`, `ci`
Add `!` for breaking changes: `feat(api)!`

Examples:
- `feat(git): add branch-filtered commit history`
- `fix(tui): make help reachable from all views`
- `docs: update CLI reference`

## Project Structure

This is a Cargo workspace with 3 crates:
- `cli/` - Command-line interface (arg parsing, command execution; produces the `openisl` binary)
- `tui/` - Terminal user interface (interactive UI, keyboard/mouse handling, rendering)
- `git/` - Git abstraction layer (git operations, command mapping, parsing)

Each crate should remain focused on its responsibilities. Share common types via workspace dependencies.

The TUI crate is organized as:
- `tui/src/app/` - `mod.rs` (app state), `state.rs` (types), `handlers/` (keyboard, mouse, commit_ops), `render/` (commits, diff, panels, status_bar)
- `tui/src/tree.rs` - commit graph layout
- `tui/src/theme.rs` - color themes
- `tui/src/keybindings.rs` - keybinding config model
- `tui/src/diff.rs` - syntax highlighting

## Best Practices

- Keep functions small and focused (< 50 lines preferred)
- Use meaningful variable names over single letters
- Prefer composition over inheritance
- Use `impl From`/`impl Into` for type conversions
- Leverage iterators and functional combinators
- Avoid `unsafe` code unless absolutely necessary
- Prefer `match` over nested `if-else`
- Use `dbg!()` for debugging, remove before committing
- Never commit `.env` files or secrets
- Always run tests before committing changes
- Update documentation (README, CHANGELOG, docs/) when behavior changes