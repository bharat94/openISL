# openisl stash

Set uncommitted changes aside and restore them later.

## Synopsis

```bash
openisl stash push [-m <message>]
openisl stash list
openisl stash pop [stash]
openisl stash apply [stash]
openisl stash drop [stash]
```

## Description

`openisl stash push` saves the working tree changes and reverts the working tree to the last commit, so you can switch branches or start clean. `list` shows saved stashes, `pop` restores and removes the newest (or named) stash, `apply` restores without removing it, and `drop` removes a stash without restoring it.

## Arguments

- `stash`: Stash reference such as `stash@{0}` (optional, defaults to the newest)

## Options (push)

- `-m, --message <message>`: Stash message
- `-h, --help`: Show help

## Examples

```bash
openisl stash push -m "wip: login refactor"
openisl checkout main
# ... work on main ...
openisl checkout feature
openisl stash pop
```

## See Also

- [openisl checkout](checkout.md) - Switch branches while work is stashed
- [openisl reset](reset.md) - Discard changes permanently instead