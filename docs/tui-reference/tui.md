# openisl tui

Launch the interactive terminal UI for browsing commit history.

## Synopsis

```bash
openisl tui
```

## Description

Opens an interactive terminal interface for viewing and navigating the commit history. The TUI provides a visual representation of the commit tree with keyboard navigation.

## Options

- `-h, --help`: Show help for tui command

## Usage

Launch the TUI from any git repository:

```bash
openisl tui
```

## Interface

The TUI displays a scrollable list of commits with the following information:
- Commit hash (7 characters)
- Commit summary
- Branch tags (if any)

### Layout

```
┌─────────────────────────────────────────────────────┐
│ openisl log                            [current: main]│
├─────────────────────────────────────────────────────┤
│ > abc123d - Most recent commit                    │
│   def456g - Previous commit                        │
│   ghi789j - Another commit                         │
│   ...                                              │
│   xyz0001 - Older commit                           │
├─────────────────────────────────────────────────────┤
│ Press ? for help | q to quit                       │
└─────────────────────────────────────────────────────┘
```

## Navigation

| Key | Action |
|-----|--------|
| `j` or `↓` | Move down one commit |
| `k` or `↑` | Move up one commit |
| `PageDown` | Move down 20 commits |
| `PageUp` | Move up 20 commits |
| `Home` | Jump to first commit |
| `End` | Jump to last commit |
| `?` | Toggle help overlay |
| `q` | Quit TUI |

## Help Overlay

Press `?` to show the keyboard shortcuts overlay:

```
┌─────────────────────────────────────────────────────┐
│ Keyboard Shortcuts                                  │
├─────────────────────────────────────────────────────┤
│ j / k        Move up and down                       │
│ ↑ / ↓        Arrow keys                             │
│ PageUp/Down  Jump 20 commits                        │
│ Home/End     Jump to start/end                      │
│ ?            Toggle this help                       │
│ q            Quit                                   │
└─────────────────────────────────────────────────────┘
```

## Colors

The TUI uses the following color scheme:

| Element | Color |
|---------|-------|
| Selected commit | Yellow hash, white bold summary |
| Commit hash | Dim white |
| Commit summary | White |
| Branch tags | Green |
| Help text | Gray |
| Title | Cyan |

## Performance

The TUI displays up to 100 commits by default for optimal performance. For larger repositories, use:

```bash
# Display more commits
openisl tui
```

The TUI maintains 60fps refresh rate with smooth scrolling.

## Requirements

- Terminal with true color support (24-bit color)
- Minimum 80x24 terminal size
- Mouse support (optional)

## See Also

- [openisl log](cli-commands/log.md) - Command-line commit viewer
- [CLI Commands](cli-commands/) - All command-line options
