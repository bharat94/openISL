# Architecture Documentation

## About This Document

This document provides an overview of the openISL system architecture, its components, and key design decisions.

## Document Information

| Field | Value |
|--------|-------|
| Status | Draft |
| Author | openISL Maintainers |
| Version | 0.1.0 |
| Last Updated | 2026-01-09 |

## System Overview

openISL is a command-line tool that provides:
1. **Stack Detection**: Analyzes git repositories to identify technology stack
2. **Git Abstraction**: Provides unified interface for git operations
3. **TUI (Terminal UI)**: Interactive visualization and management

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   User Interface Layer                 │
│  ┌────────────────────────────────────────────┐     │
│  │         CLI & TUI Components          │     │
│  └────────────────────┬───────────────────┘     │
│                       │                              │
└───────────────────────┼──────────────────────────┘
                        │
┌───────────────────────┴──────────────────────────┐
│                 Application Layer               │
│  ┌────────────────────────────────────────────┐     │
│  │   Command Parser & Dispatcher            │     │
│  └───────────┬──────────────────────────┘     │
│                │                                   │
└────────────────┼───────────────────────────────────┘
                 │
┌────────────────┴───────────────────────────────────┐
│               Core Modules Layer                │
│  ┌──────────┬──────────┬──────────┬────────┐│
│  │  Stack    │   Git     │   TUI   │ Config ││
│  │ Detector  │Abstraction│ Engine  │ Manager ││
│  └──────────┴──────────┴──────────┴────────┘│
└──────────────────────────────────────────────────────┘
```

## Core Components

### 1. Stack Detector Module

**Purpose**: Analyze repository and identify technologies

**Responsibilities**:
- Scan repository files (package.json, Cargo.toml, go.mod, etc.)
- Parse dependency files
- Detect languages, frameworks, databases, tools
- Build technology tree/graph
- Generate stack summary

**Key Design Decisions**:
- **Parser-based architecture**: Separate parser for each ecosystem (Node, Rust, Python, etc.)
- **Extensibility**: New ecosystem support via plugin system (future)
- **Caching**: Cache results for performance

**Data Flow**:
```
Repository File System
         ↓
    File Parser (ecosystem-specific)
         ↓
    Dependency Extractor
         ↓
    Technology Classifier
         ↓
    Stack Model (JSON/Tree)
```

### 2. Git Abstraction Module

**Purpose**: Provide user-friendly commands mapping to git operations

**Responsibilities**:
- Parse openISL commands
- Map to git subcommands
- Execute git operations safely
- Provide helpful error messages
- Handle edge cases and edge behavior

**Key Design Decisions**:
- **Command mapping**: Simple mapping (`openisl save` → `git commit`)
- **Validation**: Validate operations before execution
- **Dry-run mode**: Preview changes without executing
- **Safety checks**: Confirm destructive operations

**Command Examples**:
```
openisl save [message]  → git commit -m [message]
openisl branch [name]   → git checkout -b [name]
openisl sync            → git pull + status check
openisl undo            → git reset HEAD~1
```

### 3. TUI Engine Module

**Purpose**: Interactive terminal user interface

**Responsibilities**:
- Render components (lists, trees, forms)
- Handle keyboard input and navigation
- Display stack visualization
- Show git history and operations
- Manage application state

**Key Design Decisions**:
- **Framework**: Use [Bubble](https://github.com/Populate/bubble) or similar Rust TUI library
- **Component-based**: Reusable UI components
- **State management**: Central state with reactive updates
- **Accessibility**: Keyboard-first, screen reader support

**UI Components**:
- Stack tree viewer
- File browser
- Git history viewer
- Command palette
- Help system

### 4. Configuration Manager

**Purpose**: Handle application configuration

**Responsibilities**:
- Load configuration from multiple sources:
  - Command-line flags
  - Config file (~/.config/openisl/config.toml)
  - Environment variables
  - Repository-specific config (.openisl/)
- Merge configuration with precedence
- Persist user preferences

**Configuration Hierarchy**:
1. Command-line flags (highest precedence)
2. Environment variables
3. Repository config (.openisl/config.toml)
4. User config (~/.config/openisl/config.toml)
5. Default values (lowest precedence)

## Cross-Cutting Concerns

### Error Handling

**Strategy**: Use Rust's `Result<T, E>` for recoverable errors

```
use std::result::Result;

pub fn detect_stack(path: &Path) -> Result<Stack, Error> {
    // Implementation
}
```

**Error Types**:
- **User errors**: Incorrect usage, missing files
- **System errors**: Permission denied, file system errors
- **Network errors**: Git operations, remote fetch failures
- **Parsing errors**: Invalid config files, corrupted data

### Logging

**Levels**:
- Error: Critical failures
- Warn: Non-critical issues
- Info: Normal operation flow
- Debug: Detailed diagnostics (disabled by default)

**Implementation**: Use `env_logger` or `tracing` crate

### Testing Strategy

**Unit Tests**:
- Test individual functions and modules
- Mock external dependencies (git, file system)
- Test edge cases and error paths

**Integration Tests**:
- Test with real git repositories
- Test stack detection on sample projects
- Verify git command mappings

**E2E Tests**:
- Complete workflow tests
- User journey testing
- Smoke tests on release candidates

## Technology Decisions

### Language and Framework

**Choice**: Rust

**Rationale**:
- Performance: Fast execution for file scanning
- Safety: Memory safety and error handling
- Ecosystem: Great CLI/TUI libraries (clap, bubble, ratatui)
- Distribution: Single binary via cargo install

### Dependencies

**Key Dependencies** (planned):
- `clap`: Command-line argument parsing
- `bubble`: Terminal UI framework
- `git2` or `libgit2`: Git operations
- `serde`/`serde_json`: Configuration and data serialization
- `tokio` or `async-std`: Async operations

## Security Considerations

### File System Access
- Validate paths to prevent directory traversal
- Check file permissions before operations
- Sanitize user input for file paths

### Git Operations
- Validate commands before execution
- Sanitize user input for branch names, commit messages
- Use safe-by-default approach (confirm destructive operations)

### Secrets Management
- Never read `.env` files
- Never log sensitive data
- Clear environment variables after use

## Performance Considerations

### Stack Detection
- Cache results per repository
- Parallel file scanning where possible
- Lazy loading of deep dependencies

### TUI Rendering
- Use incremental rendering
- Cache rendered components
- Minimize redraws

## Future Enhancements

### Planned Features

- **Plugin System**: Custom stack detectors
- **Configuration Profiles**: Per-project settings
- **Multi-Repo Support**: Analyze multiple repositories
- **Export Formats**: JSON, SVG, Markdown
- **Comparison Mode**: Compare stacks between repos/branches

### Architecture Evolution

- **Async I/O**: Improve performance with async file operations
- **Microservices**: Separate stack detection as service (future)
- **Web UI**: Browser-based interface for visualization

## References

### Documentation
- [Diátaxis Framework](https://diataxis.fr/)
- [Open Source Standards](OPEN_SOURCE_STANDARDS.md)
- [arc42 Template](https://arc42.org/)

### Rust Ecosystem
- [Rust CLI Guidelines](https://rust-lang.github.io/api-guidelines/)
- [TUI Libraries Comparison](https://github.com/rothgar/awesome-tuis)

---

This architecture document will be updated as openISL evolves.

**Last Updated**: 2026-01-09
**Next Review**: After initial implementation (Q2 2026)
