# openisl diff

Show changes between commits, the staging area, and the working directory.

## Synopsis

```bash
openisl diff [OPTIONS] [COMMIT]
```

## Description

Displays differences in unified diff format. With no arguments, shows changes between the working directory and the staging area. Use `--staged` to compare the staging area against the last commit, or pass a `COMMIT` to compare that commit against its parent.

## Arguments

- `COMMIT`: Full or short commit hash to show the diff for (optional)

## Options

| Option | Description |
|--------|-------------|
| `--staged` | Show staged changes (index vs. last commit) |
| `-h, --help` | Show help |

## Examples

Show unstaged changes in the working directory:

```bash
openisl diff
```

Show staged changes:

```bash
openisl diff --staged
```

Show the changes introduced by a specific commit:

```bash
openisl diff abc1234
```

## Output Format

Changes are shown in unified diff format:

```
diff --git a/src/main.rs b/src/main.rs
index 1234567..89abcde 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,5 +1,6 @@
 fn main() {
+    println!("Hello");
     println!("World");
 }
```

## See Also

- [openisl status](status.md) - Overview of changes