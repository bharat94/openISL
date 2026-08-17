# openisl blame

Annotate a file line by line with the commit that last touched each line.

## Synopsis

```bash
openisl blame <path>
```

## Description

Prints each line of the file prefixed with the abbreviated commit hash and author of the last change to that line. Useful for tracing when and why a line was introduced. Equivalent to `git blame`.

## Arguments

- `path`: Path to annotate

## Options

- `-h, --help`: Show help

## Examples

```bash
openisl blame src/main.rs
```

## See Also

- [openisl log](log.md) - Browse history
- [openisl show](show.md) - Inspect the commit that changed a line