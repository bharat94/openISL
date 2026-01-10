# openisl log

Show commit log with optional filtering and formatting.

## Synopsis

```bash
openisl log [OPTIONS]
```

## Description

Displays the commit history for the current repository. By default, shows all commits with author, date, and message summary.

## Options

- `--simple`: Show as ASCII text instead of launching TUI
- `--all`: Show commits from all branches
- `--no-remote`: Hide remote branch commits
- `-n, --max-count <N>`: Limit to N most recent commits
- `-h, --help`: Show help for log command

## Examples

Show all commits:
```bash
openisl log
```

Show last 10 commits:
```bash
openisl log -n 10
```

Show commits as text (no TUI):
```bash
openisl log --simple
```

Show only local commits:
```bash
openisl log --no-remote
```

## Output Format

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

For large repositories (1000+ commits), use `--max-count` to limit output:
- 100 commits: < 50ms
- 1000 commits: < 200ms
- 10000+ commits: Use `--max-count` for faster results
