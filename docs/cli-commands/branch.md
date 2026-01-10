# openisl branch

List, create, or delete branches.

## Synopsis

```bash
openisl branch [BRANCH_NAME]
```

## Description

With no arguments, lists all branches in the repository. With a branch name, creates a new branch.

## Options

- `BRANCH_NAME`: Name of branch to create
- `-h, --help`: Show help for branch command

## Examples

List all branches:
```bash
openisl branch
```

Create a new branch:
```bash
openisl branch feature/new-feature
```

Create a branch from a specific commit:
```bash
openisl branch hotfix/fix-bug abc1234
```

## Output Format

```
Branches:
  main
* develop
  feature/new-feature
```

Current branch is marked with `*`.

## Branch Naming

Best practices for branch names:
- Use lowercase with hyphens: `feature/new-feature`
- Prefix with type: `bugfix/`, `hotfix/`, `feature/`
- Include issue number: `bugfix/123-fix-login`

## See Also

- [openisl checkout](checkout.md) - Switch to a different branch
