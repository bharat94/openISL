# openisl TUI Reference

The interactive terminal UI launched by `openisl tui`.

## Synopsis

```bash
openisl tui
```

## Description

The TUI is a keyboard-driven, multi-panel interface for exploring Git history. It shows a commit graph with type-specific symbols and branch lanes, plus a sidebar with branches, file status, and stashes. From the list view you can open commit details, syntax-highlighted diffs, statistics, and a hunk-staging mode for precise staging of changes.

The TUI loads up to `general.max_commits` commits (100 by default); see [Configuration](../cli-commands/config.md).

## Interface

A simplified layout of the main view:

```
┌──────────────────────────────────────────────────────────────────────────┐
│ openISL                                    [main] status bar            │
├──────────────┬───────────────────────────────────────────────────────────┤
│ Branches     │ Commits Panel                                           │
│              │                                                          │
│ main ✓       │ > ┼● a7f3d2e Merge PR #123                               │
│ develop ✓    │   │   docs: update API documentation                     │
│              │   │   Author: Jane Doe <jane@example.com>                │
│              │   │   Date: 2h ago                                       │
│              │   ─● 9b2c4d1 Fix authentication bug                      │
├──────────────┴──────────────────────────────────────────────────────────┤
│ Files Panel                                                             │
│ STAGED (2)                                   [SPACE] to stage           │
│ [+] src/core/parser.rs        +234 -45                                  │
│ UNSTAGED (3)                                                            │
│ [~] src/ui/components.rs      +56 -23                                   │
├──────────────────────────────────────────────────────────────────────────┤
│ Status: 2 staged, 3 unstaged      ↑k/j↓  Enter  ? Help                 │
└──────────────────────────────────────────────────────────────────────────┘
```

## View Modes

| Mode | Access | Purpose |
|------|--------|---------|
| **List** | default | Commit tree with navigation |
| **Details** | `Enter` | Full commit info: hash, author, date, message, parents, tags |
| **Diff** | `Shift+D` | Syntax-highlighted diff of the selected commit |
| **Hunk staging** | `i` (in Diff view, on a file) | Select and stage/unstage individual hunks or lines |
| **Statistics** | `s` | Commits by author, activity timeline |
| **Search** | `/` | Search commits by message, author, hash |
| **Filter** | `f` | Filter by author (`a`), message (`m`), or date (`d`) |
| **Stash** | from command palette (`Ctrl+P`) | View stashes, apply/drop/pop |
| **Command palette** | `Ctrl+P` | Searchable list of actions |
| **Help** | `?` | Keyboard shortcuts overlay (from any non-input view) |

## Keyboard Shortcuts

Press `?` in the TUI for an always-up-to-date reference. The complete map:

### Navigation

| Key | Action |
|-----|--------|
| `j` / `k` or `↓` / `↑` | Move down/up |
| `PageDown` / `PageUp` | Page down/up |
| `Home` / `End` | First / last commit |

### Panels

| Key | Action |
|-----|--------|
| `Tab` / `Shift+Tab` | Next / previous panel |
| `←` / `→` (sidebar visible) | Next / previous panel |
| `Ctrl+B` | Toggle sidebar |
| `Space` (Files panel) | Toggle file staged |
| `Ctrl+U` | Unstage all files |

### Views & Search

| Key | Action |
|-----|--------|
| `Enter` | Commit details |
| `Shift+D` | Diff view |
| `i` | Hunk staging (Diff view, file selected) |
| `s` | Statistics |
| `/` | Search commits (Branches panel: search branches) |
| `Ctrl+N` / `Ctrl+P` | Next / previous search result |
| `f` | Filter mode |
| `Ctrl+P` | Command palette |
| `?` | Help overlay |

### Commit Operations

| Key | Action |
|-----|--------|
| `c` / `b` | Checkout / create branch from selected commit |
| `A` | Amend last commit |
| `D` | Drop selected commit |
| `S` | Squash selected commit into previous |
| `C` | Cherry-pick selected commit |
| `R` | Revert selected commit |
| `r` | Re-apply filter |

### Hunk Staging Mode (`i`)

| Key | Action |
|-----|--------|
| `j` / `k` or `↓` / `↑` | Move line/hunk |
| `Space` | Toggle line selection |
| `s` / `u` | Stage / unstage selected lines |
| `Esc` | Exit |

### Stash View

| Key | Action |
|-----|--------|
| `a` / `d` / `p` | Apply / drop / pop stash |
| `Enter` | View stash diff |

### UI Controls

| Key | Action |
|-----|--------|
| `t` | Cycle theme (dark, light, Monokai, Nord) |
| `m` | Toggle mouse mode |
| `q` / `Esc` | Quit / go back |

## Themes

The TUI ships with 4 built-in color themes, cycled with `t`:

- **Dark** (default) — high-contrast dark theme
- **Light** — clean theme for bright environments
- **Monokai** — classic Monokai palette
- **Nord** — arctic, north-bluish palette

The default theme can be set with `openisl config --theme dark|light`.

## Syntax Highlighting

The diff viewer highlights keywords, types, strings, comments, and numbers for 30+ languages. The language is auto-detected from the file extension, and colors adapt to the active theme. Supported languages include Rust, Python, JavaScript, TypeScript, Go, Java, C/C++, C#, Swift, Kotlin, Ruby, PHP, HTML, CSS, SCSS, JSON, YAML, Markdown, Bash, SQL, TOML, Lua, Perl, Elixir, Erlang, Clojure, Haskell, OCaml, F#, Nim, V, and Zig.

## Commit Types

The commit graph uses distinct symbols:

| Type | Symbol | Description |
|------|--------|-------------|
| Initial | `┌●` | First commit with no parents |
| Merge | `┼●` | Merge commit with multiple parents |
| Tagged | `◆●` | Commit with an associated tag |
| Revert | `↩●` | Reverts a previous commit |
| Squash | `≡●` | Squash commit |
| Branch point | `┬●` | Commit with multiple children |
| Regular | `─●` | Normal commit |

## Requirements

- Terminal with true color support (24-bit or 256 colors)
- Minimum 80x24 terminal size; 120x40 recommended
- Mouse support is optional (`m` toggles it)

## See Also

- [openisl tui](../cli-commands/tui.md) - Launch the TUI
- [openisl log](../cli-commands/log.md) - Text and ASCII-tree views of the same history
- [CLI Commands](../cli-commands/) - All command-line options
- [Configuration](../cli-commands/config.md) - Config file reference