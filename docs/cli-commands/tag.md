# openisl tag

List, create, or delete Git tags.

## Synopsis

```bash
openisl tag [OPTIONS] [TAG_NAME]
```

## Description

Manages tags in the current repository. With no arguments, lists all tags. Provide a tag name to create a tag at the current HEAD (annotated if `--message` is given), or use `--delete` to remove one.

## Options

| Option | Description |
|--------|-------------|
| `--list` | List all tags |
| `--delete <name>` | Delete a tag |
| `-m, --message <msg>` | Create an annotated tag with this message |
| `-h, --help` | Show help |

## Arguments

- `TAG_NAME`: Name of the tag to create

## Examples

List tags:

```bash
openisl tag
```

Create a lightweight tag:

```bash
openisl tag v1.0.0
```

Create an annotated tag:

```bash
openisl tag v1.0.0 -m "Release 1.0.0"
```

Delete a tag:

```bash
openisl tag --delete v1.0.0
```

## See Also

- [openisl remote](remote.md) - Manage remotes
- [openisl branch](branch.md) - Manage branches