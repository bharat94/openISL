# openisl add

Stage files so their changes are included in the next commit.

## Synopsis

```bash
openisl add <path>...
openisl add --all
```

## Description

Adds the given paths (files or directories) to the Git index, staging new, modified, and deleted files for commit. Equivalent to `git add`.

## Arguments

- `path`: One or more paths to stage

## Options

- `-A, --all`: Stage all changes in the repository (new, modified, and deleted)
- `-h, --help`: Show help

## Examples

```bash
openisl add src/main.rs tests/
openisl add --all
```

## See Also

- [openisl status](status.md) - Review what is staged
- [openisl diff](diff.md) - Review staged changes with `--staged`
- [openisl commit](commit.md) - Create a commit from staged changes