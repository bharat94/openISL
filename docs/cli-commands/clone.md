# openisl clone

Copy a remote repository into a local directory.

## Synopsis

```bash
openisl clone <url> [destination]
```

## Description

Clones a repository from `url` into `destination` (defaults to the repository name derived from the URL). Equivalent to `git clone`.

## Arguments

- `url`: Remote repository URL or local path
- `destination`: Destination directory (optional)

## Options

- `-h, --help`: Show help

## Examples

```bash
openisl clone https://github.com/bharat94/openISL.git
openisl clone git@github.com:bharat94/openISL.git my-copy
```

## See Also

- [openisl init](init.md) - Create a new repository from scratch
- [openisl remote](remote.md) - Manage remotes on an existing repository
- [openisl pull](pull.md) - Fetch and merge remote changes