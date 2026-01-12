# Architecture Documentation

## About This Document

This document provides an overview of the openISL system architecture, its components, and key design decisions.

## Document Information

| Field | Value |
|--------|-------|
| Status | Active |
| Author | openISL Maintainers |
| Version | 0.6.0 |
| Last Updated | 2026-01-11 |

## System Overview

openISL (Interactive Smart Log) is an intelligent command-line tool that provides:
1. **Advanced Git Visualization**: Interactive TUI with enhanced commit graphs and syntax-highlighted diffs
2. **Git Command Wrapper**: Unified interface for common git operations with enhanced features
3. **Adaptive TUI**: Progressive terminal UI with multiple themes and keyboard-driven navigation

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   User Interface Layer                 │
│  ┌────────────────────────────────────────────┐     │
│  │         CLI & Interactive TUI          │     │
│  └────────────────────┬───────────────────┘     │
│                       │                              │
└───────────────────────┼───────────────────────────┘
                          │
┌───────────────────────┴───────────────────────────┐
│                 Application Layer               │
│  ┌────────────────────────────────────────────┐     │
│  │ Command Dispatcher & TUI Engine       │     │
│  └───────────┬───────────────────────────┘     │
│                │                                   │
└────────────────┼───────────────────────────────────┘
                   │
┌────────────────┴───────────────────────────────────┐
│               Core Modules Layer                │
│  ┌──────────┬──────────┬──────────┬────────┐│
│  │  Diff     │  Commit  │ Git     │ Config ││
│  │ Parser    │  Tree    │ Wrapper │ Manager ││
│  │ w/        │          │         │        ││
│  │  Syntax    │          │         │        ││
│  │ Highlight │          │         │        ││
│  └──────────┴──────────┴──────────┴────────┘│
└──────────────────────────────────────────────────────┘
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

### 1. Syntax-Highlighted Diff Module

**Purpose**: Parse and display git diffs with language-aware syntax highlighting

**Responsibilities**:
- Parse unified diff format
- Detect programming language from file extension (30+ languages)
- Highlight keywords, types, strings, comments, numbers
- Apply theme-aware colors for dark/light themes
- Provide diff statistics (additions, deletions, files changed)

**Key Design Decisions**:
- **Tokenizer-based**: Custom tokenizer for lightweight syntax highlighting
- **Language dictionaries**: Separate keyword/type lists per language
- **Theme support**: Two color palettes (dark/light)
- **Performance**: Fast tokenization without external dependencies

**Supported Languages**:
Rust, Python, JavaScript/TypeScript, Go, Java, C/C++, C#, Swift, Kotlin, Ruby, PHP, Lua, Perl, Elixir, Erlang, Clojure, Haskell, OCaml, F#, Nim, V, Zig, HTML, CSS, SCSS, JSON, YAML, XML, Bash, TOML, Markdown, SQL, R

**Data Flow**:
```
Git Diff Output
     ↓
Diff Parser (unified format)
     ↓
Language Detector (file extension)
     ↓
Syntax Highlighter (tokenizer)
     ↓
Themed Output (dark/light)
     ↓
TUI Display
```

### 2. Commit Tree Module

**Purpose**: Build and visualize git commit history with enhanced graph representation

**Responsibilities**:
- Parse commit graph with parent-child relationships
- Detect commit type (Initial, Merge, Tag, Revert, Squash, Branch, Regular)
- Track branch lanes and assign colors
- Generate visual commit tree with ASCII/Unicode symbols
- Format commit details with time, author, branch info

**Key Design Decisions**:
- **Commit type classification**: Analyze commit message and structure
- **Lane-based visualization**: Track branch lanes for merge visualization
- **Distinct symbols**: Unicode characters for different commit types
- **Color assignment**: 8 distinct lane colors for better visualization

**Commit Types**:
- Initial (┌●): First commit with no parents
- Merge (┼●): Merge commit with multiple parents
- Tag (◆●): Commit with tag reference
- Revert (↩●): Commit that reverts a previous commit
- Squash (≡●): Squash commit
- Branch (┬●): Branch point with multiple children
- Regular (─●): Normal commit

**Data Flow**:
```
Git Log Output
     ↓
Commit Parser (parent relationships)
     ↓
Commit Type Detector (message/structure)
     ↓
Tree Builder (lane assignment)
     ↓
Graph Renderer (symbols + colors)
     ↓
TUI Display
```

### 3. Smart Git Abstraction Module

**Purpose**: Provide intelligent, user-friendly commands with context-aware suggestions for git operations

**Responsibilities**:
- Detect git repositories and find repo root from any subdirectory
- Parse openISL commands
- Map to git subcommands with safe execution
- Provide helpful error messages with actionable suggestions
- Execute git operations (log, branch, checkout, status, diff, remote, tag, stash)
- Handle commit operations (amend, drop, squash, cherry-pick, revert)
- Handle file operations (stage, unstage, stash)

**Key Design Decisions**:
- **Safe execution**: Validate operations before execution
- **Dry-run mode**: Preview changes without executing (display only)
- **Error handling**: Clear messages with suggestions
- **Commit operations**: Wrapper around rebase/cherry-pick for advanced operations

**Command Examples**:
```
openisl log [options]     → git log [options] with enhanced visualization
openisl branch [name]      → git branch [name] or git checkout -b [name]
openisl checkout <target>    → git checkout <target>
openisl status              → git status
openisl diff [options]      → git diff [options]
openisl remote --list       → git remote -v
openisl tag --list          → git tag -l
openisl tag --create v1.0.0 → git tag v1.0.0
```

**Data Flow**:
```
User Command
     ↓
Repository Detection (find .git directory)
     ↓
Git Command Execution (via CLI wrapper)
     ↓
Output Parsing
     ↓
Data Models (Commit, GitRef, etc.)
     ↓
UI/CLI Display
```

### 3. Interactive TUI Engine Module

**Purpose**: Progressive terminal user interface for exploring git history and managing files

**Responsibilities**:
- Render commit tree with enhanced visualization
- Handle keyboard input and navigation
- Display syntax-highlighted diffs
- Show git history with commit graph
- Manage application state with multiple view modes
- Support multiple panels (commits, branches, files)
- Implement search and filter functionality
- Display repository statistics

**Key Design Decisions**:
- **Framework**: Use [ratatui](https://github.com/ratatui-org/ratatui) for Rust TUI
- **Component-based**: Reusable UI components
- **State management**: Central state with view modes
- **Theme system**: 4 built-in themes (dark, light, monokai, nord)
- **Keyboard-first**: Full keyboard navigation with optional mouse support
- **Multiple view modes**: List, Details, Diff, Help, Stats, Filter, CommandPalette

**View Modes**:
- **List**: Commit tree with navigation
- **Details**: Full commit information
- **Diff**: Syntax-highlighted diff viewer
- **Help**: Keyboard shortcuts overlay
- **Stats**: Repository statistics (commits by author, activity)
- **Filter**: Filter commits by author, message, date
- **CommandPalette**: Searchable command list

**UI Components**:
- Enhanced commit tree with type-specific symbols
- Syntax-highlighted diff viewer
- File status panel with stage/unstage actions
- Branch list panel
- Repository statistics display
- Command palette for quick access
- Help overlay with all shortcuts
- Status bar with current mode info

**Theme System**:
- Dark theme (default)
- Light theme
- Monokai theme
- Nord theme
- Theme-aware syntax highlighting colors

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

### Data Models

openISL uses several core data models for representing git data:

#### Commit
Represents a single git commit with full metadata:
```
Commit {
    hash: String,           // Full SHA-1 hash
    short_hash: String,     // 7-character abbreviated hash
    message: String,        // Full commit message
    summary: String,        // First line of message
    author: String,         // Author name
    email: String,          // Author email
    date: DateTime<Utc>,    // Commit timestamp
    parent_hashes: Vec<String>,  // Parent commit hashes
    refs: Vec<GitRef>,      // Associated refs (branches, tags)
}
```

#### GitRef
Represents a git reference (branch, tag, HEAD):
```
GitRef {
    name: String,      // Ref name (e.g., "main", "v1.0.0")
    ref_type: RefType, // Branch, Tag, Remote, or Head
}
```

#### RefType Enum
```
enum RefType {
    Head,      // HEAD pointer
    Branch,    // Local branch
    Tag,       // Tag reference
    Remote,    // Remote branch
}
```

All models implement `Serialize` and `Deserialize` for potential export and `Display` for debugging.

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

- **Status Bar Enhancements**: More useful status information
- **Collapsible Diff Sections**: Better handling of large diffs
- **Loading States and Progress Indicators**: Better UX during operations
- **Interactive Rebase**: Visual rebase conflict resolution
- **Blame Viewer**: Line-by-line commit history
- **Reflog Browser**: Navigate repository reflog
- **Custom Themes**: User-defined theme files
- **Gitignore Integration**: Interactive .gitignore management
- **Stash Browser**: Visual stash viewer with diff preview

### Architecture Evolution

- **Async I/O**: Improve performance with async git operations
- **Plugin System**: Custom diff parsers and themes
- **Multi-Repo Support**: View and compare multiple repositories
- **Export Formats**: Export commit graph as SVG, Mermaid, or JSON
- **Remote Operations**: Direct integration with remote repositories
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

**Last Updated**: 2026-01-11
**Next Review**: After next feature release
