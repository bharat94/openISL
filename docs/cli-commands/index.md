# CLI Commands

Complete reference for all `openisl` command-line commands. openISL is a Git wrapper — every command runs inside an existing Git repository and maps to one or more underlying `git` operations.

## Commands

| Command | Description | Maps to |
|---------|-------------|---------|
| [`openisl init`](init.md) | Initialize a new repository | `git init` |
| [`openisl clone`](clone.md) | Clone a remote repository | `git clone` |
| [`openisl log`](log.md) | Show commit history (text or ASCII tree) | `git log` |
| [`openisl tui`](tui.md) | Launch the interactive terminal UI | — |
| [`openisl branch`](branch.md) | List, create, or delete branches | `git branch`, `git checkout -b` |
| [`openisl checkout`](checkout.md) | Switch to a branch or commit | `git checkout` |
| [`openisl status`](status.md) | Show working tree status | `git status` |
| [`openisl diff`](diff.md) | Show changes (working tree, staged, or a commit) | `git diff` |
| [`openisl add`](add.md) | Stage files (add to the index) | `git add` |
| [`openisl rm`](rm.md) | Remove a tracked file | `git rm` |
| [`openisl mv`](mv.md) | Move (rename) a tracked file | `git mv` |
| [`openisl commit`](commit.md) | Create a commit from staged changes | `git commit` |
| [`openisl show`](show.md) | Show a commit and its changes | `git show` |
| [`openisl blame`](blame.md) | Annotate a file line by line | `git blame` |
| [`openisl merge`](merge.md) | Merge a branch into the current branch | `git merge` |
| [`openisl rebase`](rebase.md) | Rebase the current branch | `git rebase` |
| [`openisl reset`](reset.md) | Move HEAD to a revision | `git reset` |
| [`openisl cherry-pick`](cherry-pick.md) | Cherry-pick a commit | `git cherry-pick` |
| [`openisl revert`](revert.md) | Revert a commit | `git revert` |
| [`openisl stash`](stash.md) | Manage stashed changes | `git stash` |
| [`openisl fetch`](fetch.md) | Fetch from a remote | `git fetch` |
| [`openisl pull`](pull.md) | Fetch and merge remote changes | `git pull` |
| [`openisl push`](push.md) | Push commits to a remote | `git push` |
| [`openisl config`](config.md) | View and update settings | — |
| [`openisl remote`](remote.md) | List, add, or remove remotes | `git remote` |
| [`openisl tag`](tag.md) | List, create, or delete tags | `git tag` |
| [`openisl cat`](cat.md) | Print file contents at a revision | `git show <rev>:<path>` |
| [`openisl apply`](apply.md) | Apply a patch file | `git apply` |
| [`openisl bisect`](bisect.md) | Binary-search history for a bug | `git bisect` |
| [`openisl resolve`](resolve.md) | List or resolve merge conflicts | `git diff --diff-filter=U`, `git add` |
| [`openisl undo`](undo.md) | Undo the last operation via reflog | `git reset --hard HEAD@{1}` |
| [`openisl squash`](squash.md) | Squash commits into one | `git reset --soft`, `git commit` |

## Common Behavior

All commands:

- Run in the **current directory**; the enclosing Git repository is detected automatically.
- Exit with code `0` on success and `1` on error (including when run outside a repository).
- Support `-h` / `--help` for usage and `-V` / `--version` for the version.

```bash
openisl log --help
openisl branch --help
```

## See Also

- [TUI Reference](../tui-reference/tui.md) — interactive terminal interface
- [Configuration](config.md) — the `~/.config/openisl/config.toml` file