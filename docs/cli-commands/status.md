# openisl status

Show the current state of the working directory.

## Synopsis

```bash
openisl status
```

## Description

Displays the status of modified, added, deleted, and untracked files in the working directory.

## Options

- `-h, --help`: Show help for status command

## Examples

Check repository status:
```bash
openisl status
```

## Output Format

```
Changes:
  Modified: src/main.rs
  Added:    new-feature.txt
  Deleted:  old-file.txt
  Untracked: temp/
```

Or for a clean repository:
```
Working tree is clean
```

## Status Types

| Status | Description |
|--------|-------------|
| Modified | File changed in working directory |
| Added | File staged for commit |
| Deleted | File removed from working directory |
| Untracked | File not tracked by git |
| Modified (staged) | File staged with changes |
| Added (staged) | File newly staged |
| Deleted (staged) | File staged for deletion |
| Renamed | File renamed |
| Conflicted | Merge conflict present |

## See Also

- [openisl diff](diff.md) - Show detailed changes
