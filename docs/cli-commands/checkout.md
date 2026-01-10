# openisl checkout

Switch to a different branch or commit.

## Synopsis

```bash
openisl checkout <BRANCH_NAME | COMMIT_HASH>
```

## Description

Updates the working directory to match the specified branch or commit. Uncommitted changes may prevent checkout.

## Arguments

- `BRANCH_NAME`: Name of branch to switch to
- `COMMIT_HASH`: Full or short commit hash to checkout

## Options

- `-h, --help`: Show help for checkout command

## Examples

Switch to a branch:
```bash
openisl checkout develop
```

Switch to a specific commit (detached HEAD):
```bash
openisl checkout abc1234
```

## Safety

Before checkout, openisl will:
1. Check for uncommitted changes
2. Warn if changes would be lost
3. Require confirmation for destructive operations

## Detached HEAD

When checking out a commit (not a branch), you enter "detached HEAD" state. This is normal for viewing old commits, but:
- New commits won't be on any branch
- Use `openisl branch <name>` to save work
- Use `openisl checkout <branch>` to return to a branch

## See Also

- [openisl branch](branch.md) - Create or list branches
- [openisl status](status.md) - Check current state
