# openisl tui

Launch the interactive terminal UI for browsing commit history.

## Synopsis

```bash
openisl tui
```

## Description

Opens an interactive, keyboard-driven terminal interface for exploring a Git repository. The TUI provides:

- A commit graph with type-specific symbols and branch lanes
- A sidebar with branches, file status, and stashes
- Syntax-highlighted diffs (30+ languages)
- Search, filtering, and repository statistics
- Hunk staging with line-level selection
- A command palette and help overlay

The TUI loads up to `general.max_commits` commits (100 by default) — see [configuration](config.md).

## Options

- `-h, --help`: Show help

## Keyboard Shortcuts

Press `?` from almost any view to open the in-app help overlay, which lists the full keymap. The most common keys:

| Key | Action |
|-----|--------|
| `j` / `k` or `↑` / `↓` | Move up/down |
| `Enter` | Commit details |
| `Shift+D` | Diff view |
| `Tab` / `Shift+Tab` | Switch panel |
| `/` | Search |
| `f` | Filter |
| `s` | Statistics |
| `Ctrl+P` | Command palette |
| `?` | Help |
| `q` / `Esc` | Quit / go back |

## Requirements

- Terminal with true color support (24-bit or 256 colors)
- Minimum 80x24 terminal size (120x40 recommended)

## See Also

- [TUI Reference](../tui-reference/tui.md) - full keymap and layout
- [openisl log](log.md) - the same history as text or an ASCII tree