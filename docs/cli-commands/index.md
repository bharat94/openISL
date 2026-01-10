# CLI Commands

Complete reference for all openisl command-line commands.

## Commands

### Core Commands

- [log](log.md) - Show commit log and history
- [branch](branch.md) - List, create, or delete branches
- [checkout](checkout.md) - Switch branches or commits
- [status](status.md) - Show working tree status
- [diff](diff.md) - Show changes between commits

## Quick Reference

| Command | Description |
|---------|-------------|
| `openisl log` | View commit history |
| `openisl branch` | List all branches |
| `openisl branch <name>` | Create a new branch |
| `openisl checkout <branch>` | Switch to a branch |
| `openisl status` | Check repository state |
| `openisl diff` | View changes |
| `openisl help` | Show help |

## Common Options

All commands support `-h` or `--help` to show usage information:

```bash
openisl log --help
openisl branch --help
```

## Exit Codes

- `0`: Success
- `1`: Error occurred

## See Also

- [TUI Reference](../tui-reference/) - Interactive terminal interface
- [Configuration](../reference/configuration.md) - Customizing openisl
