# openisl fetch

Download commits and refs from a remote without merging them.

## Synopsis

```bash
openisl fetch [remote]
openisl fetch --prune
```

## Description

Downloads the latest commits and branches from `remote` (defaults to `origin`) into the local repository. The working tree is untouched; fetched commits are referenced by remote-tracking branches like `origin/main`. Equivalent to `git fetch`.

## Arguments

- `remote`: Remote to fetch from (defaults to `origin`)

## Options

- `--prune`: Delete remote-tracking branches whose upstream was removed
- `-h, --help`: Show help

## Examples

```bash
openisl fetch
openisl diff main origin/main   # review what changed upstream
openisl merge origin/main       # then integrate it
```

## See Also

- [openisl pull](pull.md) - Fetch and merge in one step
- [openisl remote](remote.md) - Manage remotes
- [openisl branch](branch.md) - See remote-tracking branches with `--all`