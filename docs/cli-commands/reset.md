# openisl reset

Move the current branch to a different revision.

## Synopsis

```bash
openisl reset [target]
openisl reset --soft [target]
openisl reset --hard [target]
```

## Description

Moves HEAD (and by default the index) to `target` (defaults to `HEAD`). Equivalent to `git reset`.

- Default (mixed): moves HEAD and the index; working tree changes are kept
- `--soft`: moves HEAD only; index and working tree are kept
- `--hard`: moves HEAD, index, and working tree; discards local changes

## Arguments

- `target`: Revision to reset to (defaults to `HEAD`)

## Options

- `--hard`: Discard changes in the working tree and index
- `--soft`: Keep index and working tree changes
- `-h, --help`: Show help

## Examples

```bash
openisl reset --hard HEAD~1    # drop the last commit and its changes
openisl reset --soft HEAD~1    # undo the last commit but keep changes staged
openisl reset HEAD             # unstage everything
```

## See Also

- [openisl revert](revert.md) - Undo a commit without rewriting history
- [openisl rebase](rebase.md) - Rewrite history by replaying commits
- [openisl stash](stash.md) - Set changes aside instead of discarding them