# openisl rebase

Replay the current branch's commits on top of an upstream revision.

## Synopsis

```bash
openisl rebase [upstream]
openisl rebase -i [upstream]
```

## Description

Moves the current branch's commits so they sit on top of `upstream` (defaults to the branch's upstream). This produces a linear history. With `-i`, opens an interactive editor to reorder, squash, edit, and drop commits.

## Arguments

- `upstream`: Upstream branch or revision (optional)

## Options

- `-i, --interactive`: Interactive rebase
- `-h, --help`: Show help

## Examples

```bash
openisl checkout feature
openisl rebase main
openisl rebase -i main   # squash or reorder commits
```

## See Also

- [openisl merge](merge.md) - Merge instead of rebase
- [openisl reset](reset.md) - Discard or reposition commits
- [openisl cherry-pick](cherry-pick.md) - Copy a single commit