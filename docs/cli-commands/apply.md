# openisl apply

Apply a patch file to the working tree (or index).

## Synopsis

```bash
openisl apply <patch>
openisl apply --cached <patch>
```

## Description

Applies the changes described in `patch` (a unified diff, e.g. from `git diff` or `git format-patch`) to the working tree. With `--cached`, stages the changes in the index instead. Equivalent to `git apply`.

## Arguments

- `patch`: Patch file to apply

## Options

- `--cached`: Apply to the index instead of the working tree
- `-h, --help`: Show help

## Examples

```bash
git diff HEAD > change.patch
openisl apply change.patch
openisl apply --cached change.patch
```

## See Also

- [openisl diff](diff.md) - Produce patches
- [openisl cherry-pick](cherry-pick.md) - Apply a whole commit