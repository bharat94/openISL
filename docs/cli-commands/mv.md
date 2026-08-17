# openisl mv

Move (rename) a tracked file.

## Synopsis

```bash
openisl mv <from> <to>
```

## Description

Moves the file from `from` to `to` and stages the rename. Git records the operation as a rename (detected via content similarity). Equivalent to `git mv`.

## Arguments

- `from`: Source path
- `to`: Destination path

## Options

- `-h, --help`: Show help

## Examples

```bash
openisl mv README.txt README.md
openisl commit -m "convert README to markdown"
```

## See Also

- [openisl rm](rm.md) - Remove a tracked file
- [openisl commit](commit.md) - Record the rename