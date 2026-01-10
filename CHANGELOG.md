# Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-01-10

### Added

#### Interactive TUI Views
- **List View**: Main commit list with selection and navigation
- **Details View**: Full commit information (hash, author, date, message, parents)
- **Diff View**: Placeholder for viewing commit diffs
- **Help Overlay**: Comprehensive keyboard shortcuts reference

#### TUI Features
- ViewMode enum with List, Details, Diff, Help states
- Smooth transitions between views
- Enter key to view commit details
- Shift+D for diff view (placeholder)
- Escape key to return to list view

#### TUI Tests
- Tests for view mode transitions
- Tests for commit details formatting
- Tests for selected commit access
- Total TUI tests: 11 (up from 10)

### Changed
- Refactored TUI into app.rs, theme.rs, lib.rs modules
- Exported App, ViewMode, Theme types from library
- Improved help text with keyboard shortcuts

## [0.2.0] - 2026-01-10

### Added

#### Interactive TUI
- Ratatui-based terminal user interface
- `openisl tui` command to launch interactive viewer
- App struct with commits, selection, and scroll state
- Keyboard navigation (j/k, arrows, PageUp/PageDown, Home/End)
- Help overlay (press ? to toggle)
- Commit list widget with selection highlighting
- Commit hash, summary, and branch tag display
- Support for 100 commits with 20-line viewport
- Mouse capture and raw mode terminal handling
- Dark and light theme presets with toggle ('t' key)
- Theme struct with configurable colors (title, text, border, help)

#### TUI Testing
- Unit tests for App navigation (move_down, move_up, go_to_start, go_to_end)
- Tests for visible commits pagination
- Tests for help overlay toggle
- Tests for keyboard quit handling
- Tests for theme toggle and color presets

#### TUI Documentation
- docs/tui-reference/tui.md with complete usage guide
- Keyboard shortcuts table
- Interface layout diagram
- Color scheme documentation
- Performance notes and requirements

#### TUI Library Structure
- Split into lib.rs, app.rs, and theme.rs modules
- Public API exports for App, run_tui, and Theme
- Integration with openisl-git for commit fetching

### Changed
- CLI now includes `tui` command (placeholder for binary launch)
- Updated Roadmap in README to show v0.2.0 progress

### Performance
- Optimized for 100+ commits with smooth scrolling
- 50ms poll interval for responsive input handling

## [0.1.0] - 2026-01-10

### Changed
- Update project name meaning to "Interactive Smart Log" (not "Open Infrastructure & Stack Layer")
- Update README with "smart" vision and git operations focus
- Update ARCHITECTURE.md to reflect smart log and git abstraction
- Update tagline to emphasize smart git operations

### Added

#### Core Git Abstraction Layer
- `git/` crate with CLI wrapper for git operations
- Repository detection (`is_git_repo`, `find_repo_root`)
- Commit parsing from `git log` output
- Branch detection and current branch lookup
- Status parsing with FileStatus and StatusType
- Diff retrieval (staged, commit-specific, working directory)

#### Data Models
- `Commit` struct with hash, message, author, date, parents
- `GitRef` struct for branches, tags, HEAD
- `RefType` enum (Head, Branch, Tag, Remote)
- Serialization support with serde
- Display implementations for all models

#### CLI Commands
- Full CLI structure with clap derive
- `openisl log` - Show commit history
- `openisl branch` - List/create branches
- `openisl checkout` - Switch branches/commits (placeholder)
- `openisl status` - Show working tree status
- `openisl diff` - Show changes between commits
- `openisl help` - Show help information

#### Smart Log Visualization
- ASCII smart log formatter
- Graph character rendering (o, ~)
- Commit hash and summary display
- Branch tag support
- Configurable terminal width

#### Documentation
- CLI commands documentation (log, branch, checkout, status, diff)
- Data Models section in ARCHITECTURE.md
- Repository detection documentation
- Performance targets documented

#### Testing
- Unit tests for repository detection
- Unit tests for data models
- Unit tests for log parsing
- Unit tests for smart log formatting
- CLI argument parsing tests

### Performance
- Target: 100 commits < 50ms
- Target: 1000 commits < 200ms
- max_count parameter for limiting results

### Platform
- Linux and macOS support
- No Windows support (initial release)

## [0.0.1] - 2026-01-09

### Added
- Project initialization
- Documentation foundation
- Open source standards research
