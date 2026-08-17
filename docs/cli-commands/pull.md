# openisl pull

Fetch remote changes and integrate them into the current branch.

## Synopsis

```bash
openisl pull
openisl pull --rebase
```

## Description

Fetches from the current branch's upstream and merges (or, with `--rebase`, replays your local commits on top of) the remote changes. Equivalent to `git pull`.

## Options

- `--rebase`: Rebase local commits onto the fetched changes instead of merging
- `-h, --help`: Show help

## Examples

```bash
openisl pull
openisl pull --rebase   # keep a linear history
```

## See Also

- [openisl fetch](fetch.md) - Download without integrating
- [openisl merge](merge.md) - Integrate a branch manually
- [openisl push](push.md) - Send local commits upstream