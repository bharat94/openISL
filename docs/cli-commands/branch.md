# openisl branch

List or create branches.

## Synopsis

```bash
openisl branch [BRANCH_NAME] [OPTIONS]
```

## Description

With no arguments, lists all branches and marks the current one with `*`. With a branch name, creates a new branch from the current HEAD.

## Options

| Option | Description |
|--------|-------------|
| `--remote` | Show remote-tracking branches only |
| `--all` | Show local and remote-tracking branches |
| `-h, --help` | Show help |

## Examples

List local branches:

```bash
openisl branch
```

List all branches including remotes:

```bash
openisl branch --all
```

List remote branches only:

```bash
openisl branch --remote
```

Create a new branch:

```bash
openisl branch feature/new-feature
```

## Output Format

```
Branches:
  main
* develop
  feature/new-feature
```

The current branch is marked with `*`.

## Branch Naming

Best practices for branch names:

- Use lowercase with hyphens: `feature/new-feature`
- Prefix with type: `bugfix/`, `hotfix/`, `feature/`
- Include issue number: `bugfix/123-fix-login`

## See Also

- [openisl checkout](checkout.md) - Switch to a branch or commit