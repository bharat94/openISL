# openisl cherry-pick

Apply the changes from a single commit onto the current branch.

## Synopsis

```bash
openisl cherry-pick <commit>
```

## Description

Copies the changes introduced by `commit` onto the current branch as a new commit. Useful for porting a specific fix without merging a whole branch. Equivalent to `git cherry-pick`.

## Arguments

- `commit`: Commit to cherry-pick

## Options

- `-h, --help`: Show help

## Examples

```bash
openisl checkout release/1.0
openisl cherry-pick a9c4719
```

## See Also

- [openisl revert](revert.md) - Undo a cherry-picked commit
- [openisl merge](merge.md) - Bring over a whole branch