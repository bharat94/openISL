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

openISL (Interactive Smart Log) is an intelligent command-line tool that provides:
1. **Smart Stack Analysis**: Analyzes git repositories to identify technology stack, relationships, patterns, and insights
2. **Intelligent Git Interface**: Provides unified interface for git operations with context-aware suggestions
3. **Adaptive TUI**: Progressive terminal UI that adapts to user expertise and workflow

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   User Interface Layer                 │
│  ┌────────────────────────────────────────────┐     │
│  │         CLI & Adaptive TUI          │     │
│  └────────────────────┬───────────────────┘     │
│                       │                              │
└───────────────────────┼──────────────────────────┘
                         │
┌───────────────────────┴──────────────────────────┐
│                 Application Layer               │
│  ┌────────────────────────────────────────────┐     │
│  │ Command Dispatcher & Intelligence      │     │
│  └───────────┬──────────────────────────┘     │
│                │                                   │
└────────────────┼───────────────────────────────────┘
                  │
┌────────────────┴───────────────────────────────────┐
│               Core Modules Layer                │
│  ┌──────────┬──────────┬──────────┬────────┐│
│  │  Smart    │  Smart   │ Adaptive│ Config ││
│  │ Stack     │   Git   │   TUI   │ Manager ││
│  │ Analysis  │Abstraction│ Engine  │        ││
│  └──────────┴──────────┴──────────┴────────┘│
└──────────────────────────────────────────────────────┘
```

## Core Components

### 1. Smart Stack Analysis Module

**Purpose**: Analyze repository and identify technologies, relationships, patterns, and insights

**Responsibilities**:
- Scan repository files (package.json, Cargo.toml, go.mod, etc.)
- Parse dependency files and detect relationships
- Detect languages, frameworks, databases, tools
- Build technology tree/graph with relationship mapping
- Generate stack summary with insights (best practices, anti-patterns, security concerns)
- Analyze code patterns and usage statistics

**Key Design Decisions**:
- **Parser-based architecture**: Separate parser for each ecosystem (Node, Rust, Python, etc.)
- **Intelligence layer**: Pattern recognition and context-aware analysis
- **Extensibility**: New ecosystem support via plugin system (future)
- **Caching**: Cache intelligent analysis results for performance

**Data Flow**:
```
Repository File System
         ↓
    File Parser (ecosystem-specific)
         ↓
    Dependency Extractor + Relationship Analyzer
         ↓
    Technology Classifier + Pattern Detector
         ↓
    Smart Stack Analysis (relationships, insights, best practices)
         ↓
    Stack Model (JSON/Tree with intelligence)
```

### 2. Smart Git Abstraction Module

**Purpose**: Provide intelligent, user-friendly commands with context-aware suggestions for git operations

**Responsibilities**:
- Parse openISL commands
- Map to git subcommands with smart enhancements
- Execute git operations safely with intelligence
- Provide helpful error messages with actionable suggestions
- Handle edge cases with smart resolution guidance
- Detect and prevent common mistakes
- Suggest optimal workflows based on context

**Key Design Decisions**:
- **Smart command mapping**: Enhanced mapping with intelligence (`openisl save` → `git commit` + auto-staging + suggestions)
- **Context-aware suggestions**: Suggest commands and workflows based on project state
- **Validation**: Validate operations before execution
- **Dry-run mode**: Preview changes without executing
- **Safety checks**: Confirm destructive operations, detect conflicts early

**Command Examples**:
```
openisl save [message]  → git commit -m [message] + auto-stage related files + suggest message
openisl branch [name]   → git checkout -b [name] + conflict detection
openisl sync            → git pull + status check + conflict resolution guidance
openisl undo            → git reset HEAD~1 + backup creation + impact preview
openisl analyze         → Smart analysis of repository state + suggestions
```

### 3. Adaptive TUI Engine Module

**Purpose**: Progressive terminal user interface that adapts to user expertise and workflow

**Responsibilities**:
- Render components (lists, trees, forms)
- Handle keyboard input and navigation
- Display smart stack visualization with relationships
- Show git history and operations with context
- Manage application state with progressive disclosure
- Adapt interface complexity based on user expertise

**Key Design Decisions**:
- **Framework**: Use [ratatui](https://github.com/ratatui-org/ratatui) for Rust TUI
- **Component-based**: Reusable UI components
- **State management**: Central state with reactive updates
- **Progressive disclosure**: Show advanced features based on context/expertise
- **Accessibility**: Keyboard-first, screen reader support

**UI Components**:
- Stack tree viewer with relationships
- File browser with context-aware preview
- Git history viewer with commit graph
- Smart command palette with suggestions
- Context-aware help system
- Progressive complexity modes (beginner/intermediate/advanced)

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
