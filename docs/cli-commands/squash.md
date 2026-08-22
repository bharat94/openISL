# openisl squash

Combine a range of commits into a single commit.

## Synopsis

```bash
openisl squash <commit> -m <message>
```

## Description

Squashes all commits after `commit` (inclusive of it) into one new commit with the given message, using a soft reset and re-commit. The earlier history before `commit` is preserved. Equivalent to `git reset --soft <commit>` plus `git commit`.

## Arguments

- `commit`: Commit to squash up to (inclusive), e.g. `HEAD~3`

## Options

- `-m, --message <message>`: Message for the squashed commit
- `-h, --help`: Show help

## Examples

```bash
openisl squash HEAD~2 -m "one tidy commit"
```

## See Also

- [openisl rebase](rebase.md) - Interactive history rewriting
- [openisl commit](commit.md) - `--amend` for the last commit