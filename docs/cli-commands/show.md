# openisl show

Display a commit, its metadata, and the changes it introduced.

## Synopsis

```bash
openisl show <commit>
```

## Description

Shows the commit message, author, date, and the diff against its parent for the given commit or revision (e.g. a branch name, tag, or hash). Equivalent to `git show`.

## Arguments

- `commit`: Commit hash or revision to show

## Options

- `-h, --help`: Show help

## Examples

```bash
openisl show HEAD
openisl show a9c4719
openisl show main
```

## See Also

- [openisl log](log.md) - List commits
- [openisl diff](diff.md) - Compare working tree, staged, or commit changes