# openisl push

Send local commits to a remote repository.

## Synopsis

```bash
openisl push [remote] [branch]
openisl push --tags
openisl push --set-upstream
```

## Description

Uploads the current branch's commits to `remote` (defaults to `origin`) at `branch` (defaults to the current branch). Equivalent to `git push`.

## Arguments

- `remote`: Remote to push to (defaults to `origin`)
- `branch`: Branch to push (defaults to the current branch)

## Options

- `--tags`: Also push tags
- `--set-upstream`: Record the remote branch as the upstream for future pull/push
- `-h, --help`: Show help

## Examples

```bash
openisl push
openisl push origin feature/login --set-upstream
openisl push --tags
```

## See Also

- [openisl pull](pull.md) - Fetch and integrate remote changes
- [openisl remote](remote.md) - Manage remotes
- [openisl fetch](fetch.md) - Download without pushing