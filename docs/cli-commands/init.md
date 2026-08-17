# openisl init

Initialize a new Git repository in the current directory.

## Synopsis

```bash
openisl init
```

## Description

Creates a new Git repository (`.git` directory) in the current directory. The directory must not already contain a repository.

## Options

- `-h, --help`: Show help

## Examples

Initialize a repository in the current directory:

```bash
mkdir my-project
cd my-project
openisl init
openisl status   # working tree is clean, no commits yet
```

## See Also

- [openisl clone](clone.md) - Copy an existing remote repository
- [openisl add](add.md) - Stage the first files
- [openisl commit](commit.md) - Create the first commit