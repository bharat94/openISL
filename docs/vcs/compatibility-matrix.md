# openISL VCS Compatibility Matrix

This matrix maps the functionality of the major version control systems — **Git**, **Mercurial (hg)**, **Subversion (svn)**, **Fossil**, **Darcs**, **Pijul**, and **Jujutsu (jj)** — to the openISL command that covers it. It is used to (1) document what openISL already offers, and (2) find gaps so we can build the missing commands.

## How to Read This Matrix

openISL currently targets Git repositories (it wraps the `git` CLI), but is designed to grow. Each row is a **functional capability** with the command each VCS uses to provide it, followed by the openISL equivalent and its status.

Status legend:

- ✅ **Implemented** — exposed as an `openisl` CLI command
- 🟡 **In the library** — implemented in `openisl-git` (and often used by the TUI), but not yet a CLI command
- 🔶 **Partial** — some aspect exists (e.g. TUI-only, or only a subset of flags)
- ⬜ **Gap** — not implemented anywhere in openISL

> **No staging area**: Mercurial, Jujutsu, Pijul, and Darcs commit the working copy directly (`hg commit`, `jj commit`, `pijul record`, `darcs record`). Git, Subversion, and Fossil stage first. openISL follows Git's model.

---

## 1. Snapshot & History

| Operation | Git | Mercurial | Subversion | Fossil | Darcs | Pijul | Jujutsu | openISL | Status |
|---|---|---|---|---|---|---|---|---|---|
| Create repository | `git init` | `hg init` | `svnadmin create` | `fossil new` | `darcs init` | `pijul init` | `jj git init` | `openisl init` | ✅ |
| Copy a remote repo | `git clone` | `hg clone` | `svn checkout` | `fossil clone` | `darcs get` | `pijul clone` | `jj git clone` | `openisl clone` | ✅ |
| Show working-tree status | `git status` | `hg status` | `svn status` | `fossil changes` | `darcs whatsnew` | `pijul diff` | `jj status` | `openisl status` | ✅ |
| Stage a file (add to index) | `git add` | *(none)* | `svn add` | `fossil add` | *(none)* | `pijul add` | *(none)* | `openisl add` | ✅ |
| Remove a file | `git rm` | `hg remove` | `svn delete` | `fossil rm` | `darcs remove` | `pijul remove` | `jj file untrack` | `openisl rm` | ✅ |
| Move/rename a file | `git mv` | `hg rename` | `svn move` | `fossil mv` | `darcs move` | `pijul mv` | `jj file chmod`/`mv` | `openisl mv` | ✅ |
| Commit changes | `git commit` | `hg commit` | `svn commit` | `fossil commit` | `darcs record` | `pijul record` | `jj describe` + `jj new` | `openisl commit` | ✅ |
| Show commit history | `git log` | `hg log` | `svn log` | `fossil timeline` | `darcs changes` | `pijul log` | `jj log` | `openisl log` | ✅ |
| History as an interactive graph | `git log --graph` / `tig` | `hg log -G` | *(none)* | web UI | *(none)* | *(none)* | `jj log` | `openisl tui` | ✅ |
| Show working-tree diff | `git diff` | `hg diff` | `svn diff` | `fossil diff` | `darcs diff` | `pijul diff` | `jj diff` | `openisl diff` | ✅ |
| Diff between commits / show commit | `git show` | `hg export` | `svn diff -c` | `fossil diff` | `darcs diff --from` | `pijul log`/`diff` | `jj show` | `openisl show` | ✅ |
| Show staged diff | `git diff --cached` | *(none)* | `svn diff --cached`¹ | *(none)* | *(none)* | *(none)* | *(none)* | `openisl diff --staged` | ✅ |
| Annotate / blame a file | `git blame` | `hg annotate` | `svn blame` | web UI² | `darcs annotate` | `pijul credit` | `jj file annotate` | `openisl blame` | ✅ |
| Print file contents at a revision | `git show <rev>:<file>` | `hg cat` | `svn cat` | `fossil cat` | `darcs show contents` | *(none)* | `jj file show` | `openisl cat` | ✅ |
| Untracked files list | `git status --short` | `hg status -u` | `svn status` | `fossil extra` | `darcs whatsnew` | `pijul diff` | `jj status` | `openisl status` | ✅ |

¹ Subversion has no staging area either; `--cached` shown for Git comparison only. ² Fossil tracks annotations in its web interface.

---

## 2. Branching, Merging & Tags

| Operation | Git | Mercurial | Subversion | Fossil | Darcs | Pijul | Jujutsu | openISL | Status |
|---|---|---|---|---|---|---|---|---|---|
| List branches | `git branch` | `hg branches` | `svn list ^/branches` | `fossil branch` | *(none)* | *(none)* | `jj bookmark list` | `openisl branch` | ✅ |
| Create a branch | `git branch` / `git switch -c` | `hg branch` | `svn copy` | `fossil commit --branch` | *(none)* | `pijul fork` (channel) | `jj bookmark create` | `openisl branch <name>` | ✅ |
| Switch branch / checkout | `git checkout` / `git switch` | `hg update` | `svn switch` | `fossil update` | *(none)* | `pijul apply` (channel) | `jj edit` | `openisl checkout` | ✅ |
| Merge branches | `git merge` | `hg merge` | `svn merge` | `fossil merge` | `darcs pull` (implicit) | `pijul apply` | *(conflicts only)* | `openisl merge` | ✅ |
| Rebase | `git rebase` | `hg rebase` | `svn merge --reintegrate` | `fossil merge --cherrypick` | *(patch model)* | *(patch model)* | `jj rebase` | `openisl rebase` | ✅ |
| Cherry-pick a change | `git cherry-pick` | `hg graft` | `svn merge -c` | `fossil cherry-pick` | `darcs pull`/`apply` | `pijul apply` | `jj duplicate` | `openisl cherry-pick` | ✅ |
| Revert a change | `git revert` | `hg backout` | `svn merge --reverse` | `fossil merge --backout` | `darcs revert` | `pijul unrecord` | `jj revert` | `openisl revert` | ✅ |
| Tag a revision | `git tag` | `hg tag` | `svn copy` | `fossil tag` | `darcs tag` | *(none)* | `jj tag set` | `openisl tag` | ✅ |
| Reset/move HEAD | `git reset` | `hg strip`/`hg update -C` | *(centralized)* | `fossil update` | `darcs obliterate` | `pijul unrecord` | `jj abandon`/`jj undo` | `openisl reset` | ✅ |
| Undo the last operation | `git reset --hard` / reflog | `hg strip` | *(none)* | `fossil undo` | `darcs unrecord` | `pijul unrecord` | `jj undo` | `openisl undo` | ✅ |

---

## 3. Collaboration & Remotes

| Operation | Git | Mercurial | Subversion | Fossil | Darcs | Pijul | Jujutsu | openISL | Status |
|---|---|---|---|---|---|---|---|---|---|
| List/add/remove remotes | `git remote` | `hg paths` | *(none)* | `fossil config`/`push` | *(none)* | *(none)* | `jj git remote` | `openisl remote` | ✅ |
| Fetch (download without merging) | `git fetch` | `hg pull` | `svn update` | `fossil pull` | `darcs pull` | `pijul pull` | `jj git fetch` | `openisl fetch` | ✅ |
| Pull (fetch + merge) | `git pull` | `hg pull --update` | `svn update` | `fossil update --latest` | `darcs pull` | `pijul pull` | `jj git fetch` + rebase | `openisl pull` | ✅ |
| Push | `git push` | `hg push` | `svn commit` | `fossil push` | `darcs push` | `pijul push` | `jj git push` | `openisl push` | ✅ |
| Sync state (ahead/behind) | `git status -sb` | `hg incoming`/`outgoing` | *(centralized)* | `fossil info` | *(none)* | *(none)* | `jj log` | *(TUI status bar)* | 🔶 |

---

## 4. History Rewriting & Advanced

| Operation | Git | Mercurial | Subversion | Fossil | Darcs | Pijul | Jujutsu | openISL | Status |
|---|---|---|---|---|---|---|---|---|---|
| Amend the last commit | `git commit --amend` | `hg amend` | *(none)* | *(none)* | `darcs amend-record` | `pijul record --amend` | `jj describe` | `openisl commit --amend` | ✅ |
| Squash commits | `git rebase -i` | `hg histedit`/`fold` | *(none)* | *(none)* | `darcs pull --compress` | `pijul record` (re-record) | `jj squash` | `openisl squash` | ✅ |
| Stash uncommitted work | `git stash` | `hg shelve` | `svn copy` | *(none)* | *(none)* | *(none)* | `jj new` (implied) | `openisl stash` | ✅ |
| Bisect (find a bad commit) | `git bisect` | `hg bisect` | *(none)* | `fossil bisect` | *(none)* | *(none)* | `jj bisect` | `openisl bisect` | ✅ |
| Resolve merge conflicts | `git mergetool` | `hg resolve` | `svn resolve` | `fossil 3-way-merge` | `darcs mark-conflicts` | `pijul resolve` | `jj resolve` | `openisl resolve` | ✅ |
| Split a change | `git add -p`/`rebase -i` | `hg split` | *(none)* | *(none)* | `darcs record` (partial) | `pijul record` (partial) | `jj split` | `openisl split` | 🟡 |
| Interactive history rewrite | `git rebase -i` | `hg histedit` | *(none)* | *(none)* | `darcs record --edit` | `pijul record` | `jj squash`/`split`/`rebase` | *(TUI commit ops)* | 🔶 |
| Drop a commit | `git reset --hard` | `hg strip` | *(none)* | *(none)* | `darcs obliterate` | `pijul obliterate` | `jj abandon` | `openisl reset` | ✅ |
| Apply a patch | `git apply` / `git am` | `hg import` | `svn patch` | *(none)* | `darcs apply` | `pijul apply` | `jj diff` + edit | `openisl apply` | ✅ |
| Submodules / nested repos | `git submodule` | `hg subrepos` | `svn externals` | *(none)* | *(none)* | *(none)* | `jj submodule` | *(none)* | ⬜ |

---

## openISL Command Coverage

Current CLI surface and its status:

| Command | Covers | Status |
|---|---|---|
| `openisl log` | history, ASCII tree, `--branch`/`--remote`/`-n` | ✅ |
| `openisl tui` | interactive graph, diff, hunk staging, stash, stats | ✅ |
| `openisl status` | working-tree status | ✅ |
| `openisl diff` | working-tree, `--staged`, `[COMMIT]` | ✅ |
| `openisl branch` | list / create, `--all`/`--remote` | ✅ |
| `openisl checkout` | branch or commit | ✅ |
| `openisl tag` | list / create / delete, annotated via `-m` | ✅ |
| `openisl remote` | list / remove | ✅ |
| `openisl config` | theme, max commits | ✅ |
| `openisl add`, `rm`, `mv` | staging / file tracking | ✅ |
| `openisl commit` | basic commit | ✅ |
| `openisl show` | commit diff | ✅ |
| `openisl stash` | list / push / pop / apply / drop | ✅ |
| `openisl cherry-pick` / `revert` | commit ops | ✅ |
| `openisl squash` / `amend` | commit ops | ✅ |
| `openisl fetch` / `pull` / `push` | sync | ✅ |
| `openisl init` / `clone` | repository lifecycle | ✅ |
| `openisl merge` / `rebase` | integration | ✅ |
| `openisl reset` | move HEAD | ✅ |
| `openisl blame` | file annotation | ✅ |
| `openisl bisect` | bug hunting | ✅ |
| `openisl cat` | file at revision | ✅ |
| `openisl undo` | operation undo | ✅ |
| `openisl resolve` | conflict resolution | ✅ |
| `openisl apply` | patch application | ✅ |

## Gap Analysis

### Tier 1 — Common commands to build next (backed by the `git` CLI)
These are everyday Git workflows a user expects from a Git client. Most map to functions that already exist in `openisl-git`.

1. **`openisl commit`** — commit staged changes (add a plain `commit` to `git/operations/commit.rs`).
2. **`openisl add`** — stage files (`stage_file` exists).
3. **`openisl stash`** — list/push/pop/apply/drop (`git/operations/stash.rs` exists).
4. **`openisl fetch` / `pull` / `push`** — sync (`git/operations/remote.rs` exists).
5. **`openisl show`** — commit diff (`get_commit_diff` exists).
6. **`openisl cherry-pick` / `revert`** — commit ops (`cherry_pick_commit`, `revert_commit` exist).
7. **`openisl merge`** — `git merge`.
8. **`openisl rebase`** — `git rebase`.
9. **`openisl reset`** — `git reset`.
10. **`openisl rm` / `mv`** — file tracking.

### Tier 2 — Valuable but less urgent
- **`openisl blame`** — `git blame`.
- **`openisl bisect`** — `git bisect` (start/good/bad/reset).
- **`openisl init` / `clone`** — repository lifecycle.
- **`openisl cat`** — file at a revision.

### Tier 3 — Out of scope for now
- **`openisl apply`** — Git's `apply`/`am`; niche.
- **`openisl resolve`** — conflict resolution; the TUI surfaces conflicts but a full mergetool is a large effort.
- **`openisl undo`** — Git has no true undo; `reflog`/`reset` cover most needs.
- **Submodules** — meaningful UX work; deferred.

## Unique VCS Features Worth Studying

These concepts from other VCSes are not present in Git but inspire future openISL directions:

- **Jujutsu** — the *operation log* (`jj op log`) and `jj undo`; changes identified by stable *change IDs*; conflict-as-first-class. An operation log would be a powerful addition to openISL.
- **Pijul** — *channels* instead of branches and *patch-based* merge semantics (no conflicts by construction in many cases).
- **Fossil** — a *single-file repository* plus integrated bug tracker, wiki, and forum; `fossil undo/redo`.
- **Mercurial** — `hg graft` (cherry-pick), `hg shelve` (stash), and the revset query language (which Jujutsu adopted).

## See Also

- [CLI Commands](../cli-commands/) — the reference for each `openisl` command
- [Architecture](../../ARCHITECTURE.md) — the `git` crate design that these commands build on