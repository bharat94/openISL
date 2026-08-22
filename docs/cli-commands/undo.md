# openisl undo

Undo the last operation.

## Synopsis

```bash
openisl undo
```

## Description

Moves HEAD, the index, and the working tree back to the previous state recorded in the reflog (`git reset --hard HEAD@{1}`). This undoes the most recent commit, merge, rebase, or reset.

> **Warning:** destructive. Working-tree changes made since the last operation are discarded. Prefer `openisl revert` on shared branches.

## Options

- `-h, --help`: Show help

## Examples

```bash
openisl commit -m "oops"
openisl undo          # removes that commit and restores the previous state
```

## See Also

- [openisl revert](revert.md) - Undo a commit without rewriting history
- [openisl reset](reset.md) - Reset to an explicit revision