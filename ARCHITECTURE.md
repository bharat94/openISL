# Architecture

## About This Document

This document describes the openISL system architecture, its components, and the key design decisions behind them. It reflects the current implementation.

## System Overview

openISL (Interactive Smart Log) is a terminal-first Git client. It provides:

1. **A Git abstraction layer** (`git/`) — a thin, safe wrapper around the `git` CLI that parses output into typed models.
2. **A terminal user interface** (`tui/`) — a ratatui-based interactive UI for exploring history, diffs, and stashes, and for staging changes.
3. **A command-line interface** (`cli/`) — clap-based commands that use both of the above and produce the `openisl` binary.

### High-Level Architecture

```
                    ┌─────────────────────────────────┐
                    │        User / Terminal          │
                    └───────────────┬─────────────────┘
                                    │
                 ┌──────────────────┴──────────────────┐
                 │            openisl binary           │
                 │               (cli/)                │
                 │   clap commands ── config manager   │
                 └───────────────┬─────────────────────┘
                                 │
              ┌──────────────────┴──────────────────┐
              │                 tui/                │
              │  app (state + view modes)           │
              │  handlers (keyboard, mouse, ops)    │
              │  render (commits, diff, panels,     │
              │          status_bar)                │
              │  tree (graph layout), theme, diff   │
              └──────────────────┬──────────────────┘
                                 │
              ┌──────────────────┴──────────────────┐
              │                 git/                │
              │  command (git subprocess runner)    │
              │  operations (log, branch, checkout, │
              │            status, diff, stash, tag,│
              │            hunk staging, commit ops)│
              │  models (Commit, GitRef, FileStatus)│
              │  vcs   (Change, Ref, SyncState)     │
              └──────────────────┬──────────────────┘
                                 │
                              git CLI
```

## Workspace Layout

```
openISL/
├── cli/     Command-line interface; produces the `openisl` binary
├── tui/     Terminal user interface (ratatui + crossterm)
├── git/     Git abstraction layer (git subprocess wrapper)
├── docs/    Documentation
└── Cargo.toml  Workspace manifest (shared deps, version, metadata)
```

### 1. The Git Abstraction Layer (`git/`)

**Purpose**: provide a safe, typed interface over the `git` CLI.

**Modules**:
- `command.rs` — `run`/`run_raw` subprocess helpers plus repository detection (`is_git_repo`, `find_repo_root`).
- `models.rs` — typed data models: `Commit`, `GitRef`, `RefType`, `FileStatus`, `StatusType`, `Remote`, `Tag`.
- `operations/` — one module per concern:
  - `log.rs` — `get_commits`, `get_commits_filtered` (branch/remote scoping), date parsing.
  - `branch.rs`, `checkout.rs`, `diff.rs`, `status.rs`, `remote.rs`, `tag.rs`, `stash.rs`, `editor.rs`.
  - `commit_ops.rs` — amend, drop, squash, cherry-pick, revert, reword.
  - `hunk.rs` — file hunks and line-level staging via `git apply --cached`.
  - `smartlog.rs` — ASCII tree formatter (`SmartLogFormatter`).
- `vcs/` — VCS-agnostic types (`Change`, `Ref`, `SyncState`) intended to decouple the UI from Git specifics.

**Key design decisions**:
- **Subprocess over bindings**: shell out to `git` (via `std::process::Command`) rather than linking `libgit2`. This keeps behavior identical to the user's installed Git and avoids FFI complexity.
- **Typed parsing**: `git log`/`git status`/`git branch` output is parsed into `Commit`, `FileStatus`, etc., so callers never touch raw text.
- **Real operations**: TUI actions map to real Git commands (`apply`, `checkout`, `branch`, `stash`, …); nothing is display-only.

### 2. The Terminal User Interface (`tui/`)

**Purpose**: keyboard-driven exploration and manipulation of a repository.

**Key design decisions**:
- **Framework**: [ratatui](https://github.com/ratatui-org/ratatui) with [crossterm](https://github.com/crossterm-rs/crossterm) for events.
- **Central app state**: a single `App` struct owns all state and view-mode transitions.
- **Module layout** (result of splitting a monolithic `app.rs`):
  - `app/state.rs` — types: `App`, `ViewMode`, `PanelType`, filters, selection state.
  - `app/handlers/` — `keyboard.rs`, `mouse.rs`, `commit_ops.rs`: key/mouse dispatch and operations. Global keys (e.g. `?` for help) are intercepted in `handle_key` before per-view dispatch.
  - `app/render/` — `commits.rs`, `diff.rs`, `panels.rs`, `status_bar.rs`: rendering per view. `render/mod.rs` re-exports them `pub(crate)`.
  - `tree.rs` — commit graph layout (lanes, branch points, colors).
  - `theme.rs` — 4 themes (dark, light, Monokai, Nord).
  - `keybindings.rs` — the keybinding config model (TOML-loadable).
  - `diff.rs` — language-aware syntax highlighting for diffs.
- **View modes**: `List`, `Details`, `Diff`, `Help`, `InputBranch`, `Search`, `BranchSearch`, `Filter`, `Stats`, `CommandPalette`, `Stash`, `HunkStaging`.

### 3. The Command-Line Interface (`cli/`)

**Purpose**: the `openisl` binary.

- **Argument parsing**: clap derive; commands mirror common Git workflows (`log`, `tui`, `branch`, `checkout`, `status`, `diff`, `config`, `remote`, `tag`).
- **Config**: `cli/src/config/` loads `~/.config/openisl/config.toml` (plus `OPENISL_*` environment overrides) into a typed `Config` with `general`, `tui`, and `git` sections.

## Data Models

```rust
Commit {
    hash, short_hash: String,
    message, summary:   String,
    author, email:      String,
    date:               DateTime<Utc>,
    parent_hashes:      Vec<String>,
    refs:               Vec<GitRef>,
}

GitRef { name: String, ref_type: RefType }
// RefType: Head | Branch | Tag | Remote

FileStatus { path: String, status: StatusType }
// StatusType: Modified, Added, Deleted, Untracked,
//              ModifiedStaged, AddedStaged, DeletedStaged,
//              Renamed, Conflicted

Hunk { old_start, old_lines, new_start, new_lines, lines: Vec<HunkLine>, is_staged }
HunkLine { line_type: HunkLineType, content: String, is_selected }
// HunkLineType: Context | Addition | Deletion
```

All models implement `Serialize`/`Deserialize` and `Display` where useful.

## Hunk Staging

The TUI's hunk-staging mode performs real, line-precise staging:

1. `get_file_diff_hunks` returns hunks for a file — unstaged hunks come from the working tree (`git diff`), staged hunks from the index (`git diff --cached`).
2. `stage_hunk`/`unstage_hunk` apply a single hunk's patch with `git apply --cached` (or `--cached --reverse`).
3. `stage_hunk_lines`/`unstage_hunk_lines` build a partial patch for the selected lines (recomputing the `---`/`+++` headers and hunk ranges) and apply it with `--unidiff-zero`.

This gives true per-line staging without a full index rewrite.

## Configuration

Configuration is owned by the CLI crate and consumed by the TUI at launch:

- File: `~/.config/openisl/config.toml`
- Precedence (lowest → highest): defaults → config file → `OPENISL_*` environment variables → CLI flags.

See [Configuration reference](docs/cli-commands/config.md).

## Testing Strategy

- **Unit tests** live in `#[cfg(test)]` modules across all crates (parsing, view-mode transitions, navigation, rendering logic).
- **Integration tests** (`git/tests/`) create real temporary repositories with `tempfile` and exercise actual Git operations (hunk staging, branch filtering, status parsing).
- **CI** (`.github/workflows/ci.yml`) runs `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --all` on every push/PR.

## Error Handling

- The `git/` crate defines `GitError` (thiserror) for its own errors and uses `anyhow::Context` to enrich messages with the operation and path.
- CLI and TUI propagate errors with `anyhow::Result`; the CLI prints actionable messages on failure and exits non-zero.

## Future Enhancements

- Wire the `keybindings.toml` model into the actual key dispatch (today the config model exists but handlers match raw keycodes).
- Async Git operations to keep the UI responsive on very large repositories.
- Interactive rebase, blame viewer, and reflog browser.
- Custom user-defined themes.
- Multi-repo support and remote integrations.