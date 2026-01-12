# openisl log (TUI Mode)

Launches interactive terminal UI for browsing commit history.

## Synopsis

```bash
openisl log
```

## Description

Opens an interactive terminal interface for viewing and navigating git commit history. The TUI provides enhanced commit tree visualization with syntax-highlighted diffs, multiple view modes, and comprehensive keyboard navigation.

## Options

- `--simple`: Display as ASCII text instead of TUI
- `--branch <name>`: Show commits from specific branch
- `--all`: Show commits from all branches
- `--remote`: Include remote branch commits
- `-n, --max-count <N>`: Limit to N most recent commits
- `-h, --help`: Show help

## Interface

The TUI displays a multi-panel layout with commit tree, branch list, file status, and various view modes.

### Layout

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ openisl log v0.6.0                           [main] +23 -45 ~1  │
├──────────────────┬──────────────────────────────────────────────────────────┤
│ Branches Panel   │ Commits Panel                                       │
│                  │                                                       │
│ LOCAL (8)        │ > ┼● a7f3d2e (HEAD → main) Merge PR #123        │
│   main ✓         │   │   docs: update API documentation                │
│   develop ✓       │   │   Author: Jane Doe <jane@example.com>       │
│ ● feature/auth ✓  │   │   Date: 2h ago                              │
│   feature/ui ✗     │   │   [tags: v1.2.0, release]                   │
│ ● feature/api ✗     │   │                                                   │
│   hotfix/crit ✗   │ ●● 9b2c4d1 Fix authentication bug               │
│   release/v1.0 ✗     │   │   src/auth.rs:45-67                         │
│                  │   │                                                   │
│ REMOTE (12)       │ ●● 8f1a2b3 Add user API endpoints              │
│   origin/main ✓   │   │   src/api/user.rs:12-34                     │
│   origin/develop ✓ │   │                                                   │
│                  │   │   │                                                   │
├──────────────────┴──────────────────────────────────────────────────────────┤
│ Files Panel                                                              │
│                                                                         │
│ 🔸 STAGED (2)                                     [SPACE] to stage   │
│ [+] src/core/parser.rs        +234 -45                                 │
│ [+] tests/integration.rs        +89 -12                                   │
│                                                                         │
│ 🔸 UNSTAGED (3)                                                   │
│ [~] src/ui/components.rs        +56 -23                                    │
│ [~] src/stack/mod.rs            +12 -8                                     │
│ [?] docs/new-feature.md         ?                                          │
│                                                                         │
├─────────────────────────────────────────────────────────────────────────────────┤
│ Status: 2 staged, 3 unstaged, 1 untracked      ↑k/j↓   Enter  ? Help  │
└───────────────────────────────────────────────────────────────────────────────────┘
```

## View Modes

The TUI supports multiple view modes accessible via keyboard shortcuts:

### List View
Default view showing commit tree with navigation.

### Details View
Full commit information including hash, author, date, message, and parent commits.

### Diff View
Syntax-highlighted diff viewer showing code changes with:
- Language auto-detection (30+ languages)
- Keyword, type, string, and comment highlighting
- Theme-aware colors
- Diff statistics (additions, deletions, files changed)

### Help Overlay
Complete keyboard shortcuts reference organized by category.

### Statistics View
Repository insights including:
- Total commits
- Commits by author
- Activity timeline (today, this week, this month)
- Authors ranked by commit count

### Filter Mode
Filter commits by:
- Author name
- Commit message
- Date range

### Command Palette
Searchable command list for quick access to actions.

## Commit Types

The commit graph uses distinct symbols for different commit types:

| Type | Symbol | Description |
|------|---------|-------------|
| Initial | ┌● | First commit with no parents |
| Merge | ┼● | Merge commit with multiple parents |
| Tagged | ◆● | Commit with associated tag |
| Revert | ↩● | Reverts a previous commit |
| Squash | ≡● | Squash commit |
| Branch Point | ┬● | Commit with multiple children |
| Regular | ─● | Normal commit |
| (detached) | ○ | Regular commit not on HEAD |

## Navigation

### Commit Navigation

| Key | Action |
|-----|--------|
| `j` / `k` or `↓` / `↑` | Move down/up one commit |
| `PageDown` / `PageUp` | Page down/up (20 commits) |
| `gg` / `Home` | Jump to first commit |
| `G` / `End` | Jump to last commit |
| `Enter` | View commit details |
| `d` / `Shift+D` | View diff for commit |

### Panel Navigation

| Key | Action |
|-----|--------|
| `Tab` / `→` / `l` | Next panel (commits → branches → files) |
| `Shift+Tab` / `←` / `h` | Previous panel |
| `Ctrl+B` | Toggle sidebar visibility |

### File Operations

| Key | Action |
|-----|--------|
| `Space` | Stage/unstage selected file |
| `Ctrl+S` | Stage all files |
| `Ctrl+U` | Unstage all files |

### Search & Filter

| Key | Action |
|-----|--------|
| `/` | Enter search mode (search by message, author, hash) |
| `Ctrl+N` / `Ctrl+P` | Navigate to next/previous search result |
| `f` | Enter filter mode (filter by author, message, date) |
| `a` / `m` / `d` | Filter by author/message/date |

### Commit Operations

| Key | Action |
|-----|--------|
| `c` / `b` | Create branch from selected commit |
| `A` | Amend last commit |
| `D` | Drop selected commit |
| `S` | Squash selected commit into previous |
| `C` | Cherry-pick selected commit |
| `R` | Revert selected commit |

### UI Controls

| Key | Action |
|-----|--------|
| `t` | Toggle theme (dark → light → monokai → nord → dark) |
| `m` | Toggle mouse mode |
| `s` | Show statistics view |
| `?` | Show help overlay |
| `Ctrl+P` | Open command palette |
| `q` / `Esc` | Quit / go back to previous view |

## Themes

The TUI includes 4 built-in color themes:

### Dark (Default)
High-contrast dark theme optimized for readability

### Light
Clean light theme for bright environments

### Monokai
Classic Monokai color scheme

### Nord
Arctic, north-bluish color palette

Toggle themes with `t` key or configure in `~/.config/openisl/config.toml`.

## Syntax Highlighting

Diff viewer includes syntax highlighting for 30+ programming languages:

| Language | Extensions |
|----------|------------|
| Rust | .rs |
| Python | .py |
| JavaScript | .js, .jsx |
| TypeScript | .ts, .tsx |
| Go | .go |
| Java | .java |
| C/C++ | .c, .cpp, .h, .hpp |
| C# | .cs |
| Swift | .swift |
| Kotlin | .kt, .kts |
| Ruby | .rb |
| PHP | .php |
| HTML | .html, .htm |
| CSS | .css |
| SCSS/SASS | .scss, .sass |
| JSON | .json |
| YAML | .yaml, .yml |
| Markdown | .md |
| Bash | .sh, .bash, .zsh |
| SQL | .sql |
| TOML | .toml |
| Lua | .lua |
| Perl | .pl |
| Elixir | .ex, .exs |
| Erlang | .erl, .hrl |
| Clojure | .clj, .cljs, .cljc |
| Haskell | .hs |
| OCaml | .ml, .mli |
| F# | .fs, .fsi, .fsx |
| Nim | .nim |
| V | .v, .vv |
| Zig | .zig |

## Configuration

Customize TUI behavior with config file at `~/.config/openisl/config.toml`:

```toml
[theme]
name = "dark"  # dark, light, monokai, nord

[tui]
max_commits = 100
show_line_numbers = true
mouse_enabled = false

[keybindings]
# Custom keyboard shortcuts (see AGENTS.md for format)
```

## Performance

The TUI displays up to 100 commits by default for optimal performance. For larger repositories:
- Use `--max-count` to limit displayed commits
- TUI maintains 60fps refresh rate with smooth scrolling
- Syntax highlighting is optimized for fast rendering

## Requirements

- Terminal with true color support (24-bit or 256 colors)
- Minimum 80x24 terminal size
- Recommended 120x40 or larger for optimal experience
- Mouse support (optional, toggle with `m` key)

## See Also

- [openisl log --simple](cli-commands/log.md) - ASCII commit viewer
- [CLI Commands](cli-commands/) - All command-line options
- [Configuration](reference/configuration.md) - Config file reference
