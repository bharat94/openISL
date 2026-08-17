# openisl revert

Undo a commit by creating a new, inverse commit.

## Synopsis

```bash
openisl revert <commit>
```

## Description

Creates a new commit that undoes the changes introduced by `commit`. Unlike `openisl reset`, it does not rewrite history, so it is safe on shared branches. Equivalent to `git revert`.

## Arguments

- `commit`: Commit to revert

## Options

- `-h, --help`: Show help

## Examples

```bash
openisl revert a9c4719
openisl log   # a new "Revert" commit on top
```

## See Also

- [openisl reset](reset.md) - Undo commits by moving HEAD (rewrites history)
- [openisl cherry-pick](cherry-pick.md) - Apply a commit's changes forward