# AGENTS.md

This file provides guidance for agentic coding tools working in the openISL repository.

## Build Commands

### Building
```bash
cargo build              # Debug build
cargo build --release    # Release build (optimized)
```

### Linting & Formatting
```bash
cargo fmt               # Format code
cargo fmt --check       # Check formatting (CI)
cargo clippy            # Run linter
cargo clippy -- -D warnings  # Treat warnings as errors
```

### Testing
```bash
cargo test              # Run all tests
cargo test --all-features  # Run with all features enabled
cargo test --test integration  # Run integration tests only
cargo test --test integration --features git-tests  # Integration with git tests

# Run a single test
cargo test test_name           # Test by name
cargo test -- --exact test_name  # Exact match

# Run tests for specific module
cargo test --lib module_name
cargo test --package crate_name
```

### Coverage
```bash
cargo tarpaulin --out Html    # Generate coverage report
open tarpaulin-report.html     # View coverage
```

### Installation
```bash
cargo install --path .         # Install from local source
cargo build --release && cargo install --path .  # Release build + install
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
use crate::stack::StackAnalyzer;
```

### Types & Naming
- Use `PascalCase` for types, structs, enums
- Use `snake_case` for functions, variables, modules
- Use `SCREAMING_SNAKE_CASE` for constants
- Avoid abbreviations unless widely understood
- Name boolean variables with `is_`, `has_`, `can_` prefix
- Use descriptive names that reveal intent

```rust
struct StackAnalyzer { }
fn detect_stack(path: &Path) -> Result<Stack> { }
const MAX_DEPTH: usize = 10;
let is_valid = true;
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

fn parse_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config from {}", path.display()))?;
    toml::from_str(&content).context("Failed to parse config")
}
```

### Documentation
- Document all public APIs with `///` doc comments
- Include examples for public functions
- Explain "why" not just "what"
- Use Markdown formatting in docs
- Run `cargo doc --no-deps` to verify docs

```rust
/// Analyzes a git repository to detect the technology stack.
///
/// Scans dependency files and source code to identify:
/// - Programming languages used
/// - Frameworks and libraries
/// - Build tools and package managers
///
/// # Arguments
///
/// * `path` - Path to the git repository root
///
/// # Returns
///
/// A `Stack` object containing detected technologies and relationships
///
/// # Example
///
/// ```no_run
/// use openisl_stack::detect_stack;
/// let stack = detect_stack(Path::new("./my-project"))?;
/// println!("Detected: {:?}", stack.languages);
/// ```
pub fn detect_stack(path: &Path) -> Result<Stack> {
    // Implementation
}
```

### Testing
- Write unit tests in `#[cfg(test)]` modules
- Use descriptive test names that describe what is being tested
- Use `assert_eq!`/`assert_ne!` with meaningful messages
- Test both success and error paths
- Mock external dependencies (git, file system)
- Follow Arrange-Act-Assert pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_stack_with_nodejs_project() {
        // Arrange
        let temp_dir = create_test_repo();
        write_package_json(&temp_dir);

        // Act
        let result = detect_stack(&temp_dir);

        // Assert
        assert!(result.is_ok());
        let stack = result.unwrap();
        assert!(stack.contains("nodejs"));
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
- `feat(stack): add Python 3.12 support`
- `fix(cli): resolve branch detection in monorepos`
- `docs(tui): update installation guide`

## Project Structure

This is a Cargo workspace with 4 crates:
- `cli/` - Command-line interface (argument parsing, command execution)
- `tui/` - Terminal user interface (interactive UI, visualization)
- `stack/` - Stack detection and analysis (technology detection, dependency parsing)
- `git/` - Git abstraction layer (git operations, command mapping)

Each crate should remain focused on its responsibilities. Share common types via workspace dependencies.

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
