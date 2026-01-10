# Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
