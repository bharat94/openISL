# openisl remote

List, add, or remove Git remotes.

## Synopsis

```bash
openisl remote --list
openisl remote <name> <url>
openisl remote --remove <name>
```

## Description

Manages the remotes of the current repository. Use `--list` to show all remotes with their URLs and fetch/push type, give a `NAME` and `URL` pair to add a remote, or use `--remove` to delete one. Equivalent to `git remote`.

## Arguments

- `name`: Name of the remote to add
- `url`: URL of the remote to add

## Options

| Option | Description |
|--------|-------------|
| `--list` | List all remotes |
| `--remove <name>` | Remove a remote by name |
| `-h, --help` | Show help |

## Examples

List remotes:

```bash
openisl remote --list
```

Add a remote:

```bash
openisl remote origin https://github.com/bharat94/openISL.git
```

Remove a remote:

```bash
openisl remote --remove origin
```

## Output Format

```
origin  https://github.com/bharat94/openISL.git  (fetch)
```

## See Also

- [openisl tag](tag.md) - Manage tags
- [openisl branch](branch.md) - Manage branches
- [openisl fetch](fetch.md) / [openisl push](push.md) - Sync with a remote