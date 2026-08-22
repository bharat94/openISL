# openisl cat

Print the contents of a file at a given revision.

## Synopsis

```bash
openisl cat <revision> <path>
```

## Description

Prints the file `path` exactly as it was at `revision` (a commit hash, branch, tag, or `HEAD`). Equivalent to `git show <revision>:<path>`.

## Arguments

- `revision`: Revision, e.g. `HEAD`, `main`, `v1.0.0`, or `abc1234`
- `path`: Path to print

## Options

- `-h, --help`: Show help

## Examples

```bash
openisl cat HEAD~1 src/main.rs
openisl cat v1.0.0 Cargo.toml
```

## See Also

- [openisl show](show.md) - Show a commit and its changes
- [openisl blame](blame.md) - Annotate a file line by line