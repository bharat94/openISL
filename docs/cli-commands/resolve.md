# openisl resolve

List or mark resolved merge conflicts.

## Synopsis

```bash
openisl resolve --list
openisl resolve <path>...
```

## Description

With `--list`, prints the files that currently have merge conflicts. With paths, stages each file to mark its conflict as resolved after you've edited it. Equivalent to `git mergetool` (marking) plus listing conflicted files via `git diff --diff-filter=U`.

## Arguments

- `path`: Paths to mark as resolved

## Options

- `--list`: List conflicted files
- `-h, --help`: Show help

## Examples

```bash
openisl resolve --list
# edit the conflicted files...
openisl resolve src/main.rs
openisl commit -m "resolve merge"
```

## See Also

- [openisl merge](merge.md) - Produces the conflicts
- [openisl status](status.md) - See conflicted files in context
- [openisl commit](commit.md) - Record the resolution