# openisl status

Show the current state of the working directory.

## Synopsis

```bash
openisl status
```

## Description

Displays the status of every changed file in the working directory: modified, added, deleted, and untracked files, including staged changes.

## Options

- `-h, --help`: Show help

## Examples

Check repository status:

```bash
openisl status
```

## Output Format

```
Changes:
Modified (staged): src/main.rs
Added (staged):    new-feature.txt
Deleted:           old-file.txt
Untracked:         temp/
```

Or for a clean repository:

```
Working tree is clean
```

## Status Types

| Status | Description |
|--------|-------------|
| `Modified` | File changed in the working directory |
| `Added` | File newly added in the working directory |
| `Deleted` | File deleted from the working directory |
| `Untracked` | File not tracked by Git |
| `Modified (staged)` | File modified and staged |
| `Added (staged)` | File newly staged |
| `Deleted (staged)` | File staged for deletion |
| `Renamed` | File renamed |
| `Conflicted` | Merge conflict present |

## See Also

- [openisl diff](diff.md) - Show detailed changes