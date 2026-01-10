# openisl diff

Show changes between commits, staging area, and working directory.

## Synopsis

```bash
openisl diff [OPTIONS] [COMMIT]
```

## Description

Displays differences between commits, the staging area, and working directory in unified diff format.

## Arguments

- `COMMIT`: Specific commit to show diff for (optional)

## Options

- `--staged`: Show staged changes (vs. last commit)
- `-h, --help`: Show help for diff command

## Examples

Show all changes in working directory:
```bash
openisl diff
```

Show staged changes:
```bash
openisl diff --staged
```

Show changes in a specific commit:
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

## Color Legend

- Green lines: Added
- Red lines: Deleted
- Cyan headers: File metadata

## See Also

- [openisl status](status.md) - Overview of changes
