# openisl rm

Remove a tracked file from the repository.

## Synopsis

```bash
openisl rm <path>
```

## Description

Removes the file from the working tree and stages the removal. The change is recorded with the next commit. Equivalent to `git rm`.

## Arguments

- `path`: Path of the file to remove

## Options

- `-h, --help`: Show help

## Examples

```bash
openisl rm obsolete.txt
openisl commit -m "remove obsolete file"
```

## See Also

- [openisl mv](mv.md) - Move or rename a tracked file
- [openisl commit](commit.md) - Record the removal