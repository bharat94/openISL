# openisl checkout

Switch to a different branch or commit.

## Synopsis

```bash
openisl checkout <BRANCH_NAME | COMMIT_HASH>
```

## Description

Updates the working directory to match the specified branch or commit. Uncommitted changes may prevent the checkout from succeeding; Git will report an error if so.

## Arguments

- `BRANCH_NAME`: Name of the branch to switch to
- `COMMIT_HASH`: Full or short commit hash to check out

## Examples

Switch to a branch:

```bash
openisl checkout develop
```

Switch to a specific commit (detached HEAD):

```bash
openisl checkout abc1234
```

## Detached HEAD

When checking out a commit rather than a branch, you enter "detached HEAD" state. This is normal for inspecting old commits, but:

- New commits will not belong to any branch
- Use `openisl branch <name>` to save your work onto a branch
- Use `openisl checkout <branch>` to return to a branch

## See Also

- [openisl branch](branch.md) - List or create branches
- [openisl status](status.md) - Check the current state