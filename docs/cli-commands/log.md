# openisl log

Show commit history with optional filtering and formatting.

## Synopsis

```bash
openisl log [OPTIONS]
```

## Description

Displays the commit history for the current repository. By default it shows commits from **all** branches (local and remote-tracking) with the author, date, and summary of each commit.

## Options

| Option | Description |
|--------|-------------|
| `--simple` | Render the history as an ASCII commit tree instead of the text list |
| `-b, --branch <name>` | Show commits reachable from the given branch only |
| `--remote` | Show commits from remote-tracking branches only |
| `-n, --max-count <N>` | Limit to the N most recent commits |
| `-h, --help` | Show help |

Precedence: `--branch` wins over `--remote`; otherwise the default is all branches.

## Examples

Show all commits:

```bash
openisl log
```

Show the last 10 commits:

```bash
openisl log -n 10
```

Show commits on a specific branch:

```bash
openisl log --branch develop
```

Show only remote-tracking commits:

```bash
openisl log --remote
```

Render the history as an ASCII tree:

```bash
openisl log --simple
```

## Output Format

Text mode:

```
Commit Log (N commits):

abc123d - First commit summary
  Author: John Doe <john@example.com>
  Date:   2024-01-10 12:00:00 UTC

def456g - Second commit summary
  Author: Jane Doe <jane@example.com>
  Date:   2024-01-09 12:00:00 UTC
```

## Performance

`--max-count` limits how many commits are read from Git:

- 100 commits: < 50ms
- 1000 commits: < 200ms
- 10000+ commits: use `-n` for faster results

## See Also

- [openisl tui](tui.md) - Interactive terminal UI for the same history
- [openisl branch](branch.md) - List or create branches