# CLI Commands

Complete reference for all `openisl` command-line commands. openISL is a Git wrapper — every command runs inside an existing Git repository and maps to one or more underlying `git` operations.

## Commands

| Command | Description | Maps to |
|---------|-------------|---------|
| [`openisl log`](log.md) | Show commit history (text or ASCII tree) | `git log` |
| [`openisl tui`](tui.md) | Launch the interactive terminal UI | — |
| [`openisl branch`](branch.md) | List or create branches | `git branch`, `git checkout -b` |
| [`openisl checkout`](checkout.md) | Switch to a branch or commit | `git checkout` |
| [`openisl status`](status.md) | Show working tree status | `git status` |
| [`openisl diff`](diff.md) | Show changes (working tree, staged, or a commit) | `git diff` |
| [`openisl config`](config.md) | View and update settings | — |
| [`openisl remote`](remote.md) | List, add, or remove remotes | `git remote` |
| [`openisl tag`](tag.md) | List, create, or delete tags | `git tag` |

## Common Behavior

All commands:

- Run in the **current directory**; the enclosing Git repository is detected automatically.
- Exit with code `0` on success and `1` on error (including when run outside a repository).
- Support `-h` / `--help` for usage and `-V` / `--version` for the version.

```bash
openisl log --help
openisl branch --help
```

## See Also

- [TUI Reference](../tui-reference/tui.md) — interactive terminal interface
- [Configuration](config.md) — the `~/.config/openisl/config.toml` file