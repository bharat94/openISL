# openisl commit

Create a commit from the staged changes.

## Synopsis

```bash
openisl commit -m <message>
openisl commit --amend -m <message>
```

## Description

Records the staged changes as a new commit on the current branch. A commit message is required via `-m`. Equivalent to `git commit`.

## Options

- `-m, --message <message>`: Commit message
- `--amend`: Replace the last commit instead of creating a new one
- `-h, --help`: Show help

## Examples

```bash
openisl add --all
openisl commit -m "feat: add build script"
openisl commit --amend -m "feat: add build script and fix typo"
```

## See Also

- [openisl add](add.md) - Stage changes first
- [openisl status](status.md) - Review what will be committed
- [openisl log](log.md) - View the resulting history