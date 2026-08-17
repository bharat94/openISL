# openisl remote

List, add, or remove Git remotes.

## Synopsis

```bash
openisl remote [OPTIONS] [NAME] [URL]
```

## Description

Manages the remotes of the current repository. Use `--list` to show all remotes with their URLs and fetch/push type.

## Options

| Option | Description |
|--------|-------------|
| `--list` | List all remotes |
| `-h, --help` | Show help |

## Arguments

- `NAME`: Name of the remote to add or remove
- `URL`: URL of the remote to add

## Examples

List remotes:

```bash
openisl remote --list
```

Remove a remote:

```bash
openisl remote remove origin
```

> **Note:** Adding a remote with `openisl remote add <name> <url>` is accepted but currently prints a message asking for the URL argument; prefer the underlying `git remote add <name> <url>` until add support is implemented.

## Output Format

```
origin  https://github.com/bharat94/openISL.git  (fetch)
```

## See Also

- [openisl tag](tag.md) - Manage tags
- [openisl branch](branch.md) - Manage branches