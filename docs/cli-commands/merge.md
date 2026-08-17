# openisl merge

Merge another branch (or commit) into the current branch.

## Synopsis

```bash
openisl merge <target>
```

## Description

Merges `target` (a branch name or commit) into the current branch. If the branches have diverged, a merge commit is created (using the default commit message); otherwise Git fast-forwards. Conflicting files are left in the working tree and shown by `openisl status`.

## Arguments

- `target`: Branch or commit to merge

## Options

- `-h, --help`: Show help

## Examples

```bash
openisl checkout main
openisl merge feature/login
openisl status   # review any conflicts
```

## See Also

- [openisl rebase](rebase.md) - Replay commits onto an upstream instead
- [openisl status](status.md) - Review merge conflicts
- [openisl checkout](checkout.md) - Switch branches before merging