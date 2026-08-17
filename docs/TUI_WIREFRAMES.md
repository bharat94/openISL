# openISL TUI Wireframes and UI Specifications

> **Note:** This document is a historical design artifact. The wireframes and sample
> file paths (e.g. `src/stack/`) predate the current implementation and may not
> reflect the actual TUI. See the [live TUI Reference](tui-reference/tui.md) for the
> current layout, keymap, and behavior.

This document contains detailed ASCII wireframes and UI specifications for all panels and flows in the openISL Terminal User Interface.

---

## 1. Main Multi-Panel Layout (5 Panels)

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ openISL v1.0.0                               git flow feature/auth  [+2 -1 ~1]   │
├──────────────────┬──────────────────────────────────────────────────────────────────┤
│ STATUS PANEL     │ FILES PANEL                                                   │
│                  │                                                                │
│ 📍 On branch     │ 🔸 STAGED (2)                                                 │
│ main             │ [+] src/core/parser.rs          +234 -45                       │
│                  │ [+] tests/integration.rs        +89 -12                        │
│ 📊 Changes       │                                                                │
│ Staged:    2     │ 🔸 UNSTAGED (3)                                               │
│ Unstaged:  3     │ [~] src/ui/components.rs        +56 -23                        │
│ Untracked: 1     │ [~] src/stack/mod.rs            +12 -8                         │
│                  │ [?] docs/new-feature.md         ?                              │
│ 📦 Stash:   1    │                                                                │
│                  │                                                                │
│ 🔀 Branches      │                                                                │
│ Local:     8     │                                                                │
│ Remote:    12    │                                                                │
│                  │                                                                │
│ 📝 Commits       │                                                                │
│ Ahead:     1     │                                                                │
│ Behind:    3     │                                                                │
│                  │                                                                │
├──────────────────┴──────────────────────────────────────────────────────────────────┤
│ BRANCHES PANEL                           │ COMMITS PANEL                           │
│                                          │                                         │
│ 🔀 LOCAL (8)                             │ ●● a7f3d2e (HEAD → main) Fix parser    │
│   main     ✓                            │ │   stack.rs:23-45                      │
│   develop  ✓                            │ │                                         │
│ ● feature/auth  ✓                       │ ●● 9b2c4d1 Merge branch 'develop'       │
│   feature/ui  ✗                         │ │ |\                                    │
│ ● feature/api  ✗                       │ │ *│ 8f1a2b3 Add user API endpoints      │
│   hotfix/critical ✗                    │ │ *│                                     │
│   release/v1.0  ✗                      │ │ | ● 7d3e5f2 Stack analyzer perf       │
│                                          │ │ |/                                    │
│ 🌍 REMOTE (12)                          │ ●● 6c4b8a1 Initial commit              │
│   origin/main                           │                                         │
│   origin/develop                        │ [Selected: 8f1a2b3 - Add user API]      │
│   origin/feature/auth                   │                                         │
│                                          │                                         │
├──────────────────────────────────────────┴─────────────────────────────────────────┤
│ STASH PANEL                                                                   [0/1]│
│                                                                                   │
│ ● WIP: feature/auth - src/core/parser.rs:45-67    (2 files, +89 -23)             │
│                                                                                   │
├─────────────────────────────────────────────────────────────────────────────────────┤
│ STATUS BAR  [Mode: NORMAL]  │ ↑k/↓j Navigate  │ :w Save  │ :q Quit  │ ? Help    │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
SPECIFICATIONS
==================================================================================

PANEL DIMENSIONS:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Panel                  │ Width      │ Height       │ Position              │
├────────────────────────┼────────────┼──────────────┼───────────────────────┤
│ Status Panel (Left)    │ 25 chars   │ 11 lines     │ Top-left              │
│ Files Panel (Right)    │ 55 chars   │ 11 lines     │ Top-right             │
│ Branches Panel (Left)  │ 25 chars   │ 12 lines     │ Bottom-left           │
│ Commits Panel (Right)  │ 55 chars   │ 12 lines     │ Bottom-right          │
│ Stash Panel            │ Full width │ 3 lines      │ Bottom bar            │
│ Status Bar             │ Full width │ 1 line       │ Bottom                │
├────────────────────────┼────────────┼──────────────┼───────────────────────┤
│ Total Layout           │ 80 chars   │ 27 lines     │ Standard terminal     │
└────────────────────────────────────────┴─────────────────────────────────────┘

COLOR SCHEME (Dark Theme):
┌─────────────────────────────────────────────────────────────────────────────┐
│ Element                │ Foreground  │ Background   │ Style               │
├────────────────────────┼─────────────┼──────────────┼─────────────────────┤
│ Panel Title            │ White       │ Dark Gray    │ Bold                │
│ Panel Border           │ Gray        │ -            │ -                   │
│ Selected Item          │ Black       │ Cyan         │ Bold                │
│ Staged File            │ Green       │ -            │ Bold                │
│ Unstaged File          │ Yellow      │ -            │ -                   │
│ Untracked File         │ Magenta     │ -            │ Italic              │
│ Current Branch         │ Cyan        │ -            │ Bold                │
│ Branch Indicator       │ Green (✓)   │ -            │ -                   │
│                       │ Red (✗)     │ -            │ -                   │
│ Commit (HEAD)          │ Black       │ Green        │ Bold                │
│ Commit Parent          │ White       │ -            │ -                   │
│ Merge Commit           │ Yellow      │ -            │ Bold                │
│ Stash WIP              │ Red         │ -            │ Italic              │
│ Status Bar             │ White       │ Dark Blue    │ -                   │
│ Vim Mode Indicator     │ White       │ Dark Red     │ Bold                │
└────────────────────────┴─────────────┴──────────────┴─────────────────────┘

KEYBINDINGS (Global):
┌─────────────────────────────────────────────────────────────────────────────┐
│ Key             │ Action                                                      │
├─────────────────┼─────────────────────────────────────────────────────────────┤
│ h / ←           │ Move focus left (panel navigation)                          │
│ j / ↓           │ Move selection down                                         │
│ k / ↑           │ Move selection up                                           │
│ l / →           │ Move focus right (panel navigation)                         │
│ Tab             │ Cycle focus through panels                                  │
│ Ctrl+w          │ Switch panel (alternative)                                  │
│ :               │ Enter command mode                                          │
│ i               │ Enter insert mode (vim)                                     │
│ Escape          │ Return to normal mode                                       │
│ q / :q          │ Quit application                                            │
│ ?               │ Toggle help panel                                           │
└─────────────────┴─────────────────────────────────────────────────────────────┘

STATUS INDICATORS:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Indicator        │ Meaning                                                    │
├──────────────────┼────────────────────────────────────────────────────────────┤
│ [+]              │ File staged for commit                                     │
│ [~]              │ File modified but unstaged                                 │
│ [?]              │ File untracked                                             │
│ [D]              │ File deleted                                               │
│ [M]              │ File renamed                                                │
│ ✓                │ Branch up-to-date with upstream                           │
│ ✗                │ Branch diverged from upstream                             │
│ ●                │ Commit in history                                          │
│ ◉                │ HEAD commit (current position)                             │
│ ●○               │ Merge commit (parent commits)                              │
│ ↔               │ Branch ahead/behind remote                                 │
└──────────────────┴────────────────────────────────────────────────────────────┘

NAVIGATION FLOW:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Panel Focus Order:                                                            │
│                                                                               │
│     ┌─────────┐                                                               │
│     │ STATUS  │  ← Focus when in Branches/Commits                            │
│     │ PANEL   │                                                               │
│     └────┬────┘                                                               │
│          │                                                                    │
│     ┌────┴────┬─────────┐                                                     │
│     │         │         │                                                     │
│     │FILES  BRANCHES   │  ← Focus with h/l keys                               │
│     │PANEL   PANEL     │                                                     │
│     │         │         │                                                     │
│     └────┬────┴────┬────┘                                                     │
│          │         │                                                          │
│     ┌────┴─────────┴────┐                                                     │
│     │   COMMITS PANEL   │                                                     │
│     └─────────┬─────────┘                                                     │
│               │                                                               │
│     ┌─────────┴─────────┐                                                     │
│     │   STASH PANEL     │                                                     │
│     └───────────────────┘                                                     │
└─────────────────────────────────────────────────────────────────────────────┘

```

---

## 2. Files Panel - Staged/Unstaged/Untracked Sections

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ FILES PANEL                                                                 [2/6] │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│ 🔸 STAGED (2)                                                              [+323]  │
│ ─────────────────────────────────────────────────────────────────────────────────  │
│ > [+] src/core/parser.rs                       +234 -45      [src/core/]         │
│   [+] tests/integration.rs                     +89 -12      [tests/]             │
│                                                                                     │
│ 🔸 UNSTAGED (3)                                                           [+68]   │
│ ─────────────────────────────────────────────────────────────────────────────────  │
│   [~] src/ui/components.rs                     +56 -23      [src/ui/]            │
│   [~] src/stack/mod.rs                         +12 -8       [src/]              │
│   [?] docs/new-feature.md                      ?           [docs/]              │
│                                                                                     │
│ 🔸 UNTRACKED (1)                                                                   │
│ ─────────────────────────────────────────────────────────────────────────────────  │
│   [?] Cargo.lock                                ?           [.]                  │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
DETAILED STAGING ACTIONS VIEW
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ FILES PANEL - INTERACTIVE STAGING MODE                                     [i]   │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│ 🔸 STAGED (1)                                                              [+234]  │
│ ─────────────────────────────────────────────────────────────────────────────────  │
│ > [+] src/core/parser.rs                       +234 -45      [parser.rs:1-200]   │
│   [-] tests/integration.rs                     +89 -12      [tests/]             │
│                                                                                     │
│ 🔸 UNSTAGED (2)                                                           [+68]   │
│ ─────────────────────────────────────────────────────────────────────────────────  │
│   [~] src/ui/components.rs                     +56 -23      [components.rs:45]   │
│                                                                                     │
│   ────────────────────────────────────────────────────────────────────────────────  │
│   ▼ HUNK 1/3: src/ui/components.rs:45-78                                          │
│   ────────────────────────────────────────────────────────────────────────────────  │
│                                                                                     │
│   @@ -45,12 +45,14 @@ impl Component {                                            │
│     45  │-    fn render(&mut self, ctx: &mut Context) {                           │
│     46  │-        self.clear();                                                   │
│     47  │-        self.render_title();                                            │
│     48  │-        // TODO: Implement new rendering                                │
│     49  │+    fn render(&mut self, ctx: &mut Context) -> Result<()> {             │
│     50  │+        self.clear()?;                                                  │
│     51  │+        self.render_title()?;                                           │
│     51  │+        self.render_content()?;                                         │
│     52  │         self.draw_border();                                             │
│     53  │-        // FIXME: This is a hack                                       │
│     54  │+        self.render_children(ctx)?;                                     │
│     55  │+        Ok(())                                                          │
│     56  │     }                                                                   │
│                                                                                     │
│   ────────────────────────────────────────────────────────────────────────────────  │
│   Actions: [y] Stage hunk  │ [n] Discard hunk  │ [s] Split hunk  │ [q] Quit       │
│   ────────────────────────────────────────────────────────────────────────────────  │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
SPECIFICATIONS
==================================================================================

FILES PANEL LAYOUT:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Section           │ Height     │ Content                                    │
├───────────────────┼────────────┼────────────────────────────────────────────┤
│ Header            │ 1 line     │ Panel title + file count                   │
│ Staged Section    │ Variable   │ List of staged files                       │
│ Unstaged Section  │ Variable   │ List of modified files                     │
│ Untracked Section │ Variable   │ List of untracked files                    │
│ Hunk View (when   │ 15 lines   │ Diff hunks with staging actions            │
│  in interactive   │            │                                            │
│  mode)            │            │                                            │
└────────────────────────────┬────────────────────────────────────────────────┘
                             │ Panel Width: 55 chars
                             │ Max visible files: ~8 (depending on hunk view)
└────────────────────────────┴────────────────────────────────────────────────┘

FILE LIST ITEM FORMAT:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Position │ Content                          │ Width                         │
├──────────┼──────────────────────────────────┼──────────────────────────────┤
│ 1-3      │ Status icon ([+]/[~]/[?]/[D]/[M]) │ 3 chars                       │
│ 5-6      │ Space + staging arrow (>)        │ 2 chars                       │
│ 7-50     │ File path                        │ 44 chars (truncated)          │
│ 52-57    │ Diff stats (+/-)                 │ 6 chars                       │
│ 59-70    │ Additional info (directory)      │ 12 chars (right-aligned)      │
└──────────┴──────────────────────────────────┴────────────────────────────────┘

KEYBINDINGS (Files Panel):
┌─────────────────────────────────────────────────────────────────────────────┐
│ Key             │ Action                                                      │
├─────────────────┼─────────────────────────────────────────────────────────────┤
│ s               │ Stage file (from unstaged)                                 │
│ u               │ Unstage file (from staged)                                 │
│ Space           │ Toggle stage/unstage file                                  │
│ i               │ Enter interactive staging mode                             │
│ y               │ Stage by name (yank)                                       │
│ r               │ Revert file changes                                        │
│ d               │ Delete untracked file                                      │
│ Enter           │ View file diff                                             │
│ o               │ Open file in editor                                        │
│ Ctrl+o          │ Open file in system editor                                 │
│ ]              │ Next file section (staged→unstaged→untracked)              │
│ [              │ Previous file section                                       │
└─────────────────┴─────────────────────────────────────────────────────────────┘

HUNK-LEVEL KEYBINDINGS:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Key             │ Action                                                      │
├─────────────────┼─────────────────────────────────────────────────────────────┤
│ y               │ Stage this hunk                                            │
│ n               │ Discard this hunk                                          │
│ s               │ Split hunk into smaller hunks                              │
│ a               │ Stage all remaining hunks in file                          │
│ d               │ Discard all hunks in file                                  │
│ q               │ Quit interactive mode (return to file list)                │
│ j               │ Move to next hunk                                          │
│ k               │ Move to previous hunk                                      │
│ g               │ Jump to first hunk                                         │
│ G               │ Jump to last hunk                                          │
└─────────────────┴─────────────────────────────────────────────────────────────┘

COLOR SCHEME:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Element              │ Foreground  │ Background   │ Style                   │
├──────────────────────┼─────────────┼──────────────┼─────────────────────────┤
│ Section Header       │ White       │ Dark Gray    │ Bold                    │
│ Section Divider      │ Gray        │ -            │ -                       │
│ Selected File        │ Black       │ Cyan         │ Bold                    │
│ Staged File          │ Green       │ -            │ Bold                    │
│ Unstaged File        │ Yellow      │ -            │ -                       │
│ Untracked File       │ Magenta     │ -            │ Italic                  │
│ Hunk Header          │ White       │ Dark Blue    │ Bold                    │
│ Addition Line        │ Green       │ -            │ -                       │
│ Deletion Line        │ Red         │ -            │ -                       │
│ Action Buttons       │ Black       │ Green        │ Bold                    │
│ Selected Action      │ White       │ Green        │ Bold                    │
└──────────────────────┴─────────────┴──────────────┴─────────────────────────┘

STATUS INDICATORS:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Icon   │ Meaning                        │ Color                            │
├────────┼────────────────────────────────┼──────────────────────────────────┤
│ [+]    │ Staged for commit              │ Green                            │
│ [~]    │ Modified, unstaged             │ Yellow                           │
│ [?]    │ Untracked                      │ Magenta                          │
│ [D]    │ Deleted                        │ Red                              │
│ [R]    │ Renamed                        │ Cyan                             │
│ >      │ Currently selected file        │ Cyan (with selection highlight)  │
│ -      │ Deletion in diff               │ Red                              │
│ +      │ Addition in diff               │ Green                            │
│ @@     │ Hunk delimiter                 │ Gray                             │
└────────┴────────────────────────────────┴──────────────────────────────────┘

```

---

## 3. Commits Panel - Commit Tree with Selection

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ COMMITS PANEL                                                                  [15] │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│ ●● a7f3d2e (HEAD → main, origin/main)  Fix stack.rs null pointer        2h ago    │
│ │   stack.rs:23-45                                                               │
│ │   • Handle None case in stack analyzer                                         │
│ │   • Add unit tests for edge cases                                              │
│ │                                                                                 │
│ ●● 9b2c4d1  Merge branch 'develop'                                    4h ago    │
│ │ \                                                                            │
│ │ ●● 8f1a2b3  Add user API endpoints                                5h ago    │
│ │ │ │   api/user.rs:1-100                                                        │
│ │ │ │   • Implement /users GET endpoint                                           │
│ │ │ │   • Add authentication middleware                                           │
│ │ │ │   • Add request validation                                                 │
│ │ │                                                                            │
│ │ ●● 7d3e5f2  Stack analyzer performance improvements               6h ago    │
│ │ │ │   src/stack/analyzer.rs:45-89                                              │
│ │ │ │   • Optimize hot path in dependency resolution                              │
│ │ │ │   • Reduce memory allocations                                              │
│ │ │ │   • Add caching for repeated lookups                                       │
│ │ │                                                                            │
│ │ * 6b8c4d1  Update dependencies                                     7h ago    │
│ │ |                                                                            │
│ * │ 5a7b3d2  Refactor CLI argument parsing                          8h ago    │
│ │/                                                                         │      │
│ ●● 4c6d2e1  Bump version to v1.0.0-rc1                             10h ago   │
│    Cargo.toml:1-15                                                             │
│    • Update version to 1.0.0-rc1                                               │
│    • Update MSRV to Rust 1.70                                                  │
│                                                                                 │
│ [Selected: 8f1a2b3 - Add user API endpoints]                                    │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
COMMIT DETAIL VIEW
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ COMMIT DETAIL: 8f1a2b3                                         [← Back │ → Next] │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│ 📝 Commit Message:                                                                 │
│ ─────────────────────────────────────────────────────────────────────────────────  │
│ Add user API endpoints                                                            │
│                                                                                     │
│ 📊 Statistics:                                                                    │
│ ─────────────────────────────────────────────────────────────────────────────────  │
│ 5 files changed, 234 insertions(+), 45 deletions(-)                               │
│                                                                                     │
│ 📋 Changed Files (5):                                                             │
│ ─────────────────────────────────────────────────────────────────────────────────  │
│ M  src/api/user.rs         +189 -23     ████████████████████░░░░░░░░░  80%       │
│ M  src/api/mod.rs          +23 -12      ██████░░░░░░░░░░░░░░░░░░░░░░  10%       │
│ M  src/auth/middleware.rs  +12 -8       ████░░░░░░░░░░░░░░░░░░░░░░░░  5%        │
│ A  tests/api/user_test.rs  +10 -2       ███░░░░░░░░░░░░░░░░░░░░░░░░░  4%        │
│    docs/api.md             +? -?        █░░░░░░░░░░░░░░░░░░░░░░░░░░░  1%        │
│                                                                                     │
│ 🔍 Diff Preview:                                                                  │
│ ─────────────────────────────────────────────────────────────────────────────────  │
│ @@ -0,0 +1,100 @@                                                                │
│ +// User API endpoints                                                            │
│ +// Generated by openISL - DO NOT EDIT                                            │
│ +                                                                                  │
│ +use crate::auth::{AuthContext, Permission};                                       │
│ +use crate::database::DbPool;                                                      │
│ +                                                                                  │
│ +#[get("/users")]                                                                  │
│ +async fn list_users(ctx: &State<AuthContext>) -> impl Responder {                │
│ +    let db = ctx.database().await;                                               │
│ +    let users = db.query("SELECT * FROM users").fetch_all().await?;              │
│ +    Ok(Json(users))                                                               │
│ +}                                                                                  │
│ +                                                                                  │
│ +#[get("/users/{id}")]                                                             │
│ +async fn get_user(                                                                │
│ +    id: Path<i32>,                                                                │
│ +    ctx: &State<AuthContext>,                                                     │
│ +) -> impl Responder {                                                            │
│ +    // ... implementation                                                        │
│ +}                                                                                  │
│                                                                                     │
│ Actions: [c] Copy SHA  │ [p] Patch  │ [e] Edit message  │ [r] Revert  │ [t] Tag  │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
SPECIFICATIONS
==================================================================================

COMMIT TREE LAYOUT:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Element               │ Format                                                   │
├───────────────────────┼─────────────────────────────────────────────────────┤
│ Commit Node           │ ●● (2 chars: node type + count)                       │
│                       │ ● (single commit)                                      │
│                       │ ◉ (HEAD commit)                                        │
│                       │ ○ (not loaded/filtered)                                │
├───────────────────────┼─────────────────────────────────────────────────────┤
│ Commit SHA            │ ab3d2e (7 chars, abbreviated)                         │
│                       │ ab3d2e (HEAD → main, origin/main) (refs shown)        │
├───────────────────────┼─────────────────────────────────────────────────────┤
│ Branch Tags           │ (tag: v1.0.0)                                          │
│                       │ (HEAD → main)                                          │
│                       │ (origin/develop ↑2 ↓1)                                │
├───────────────────────┼─────────────────────────────────────────────────────┤
│ Commit Message        │ First line only (subject)                             │
│                       │ Wrapped at terminal width                              │
├───────────────────────┼─────────────────────────────────────────────────────┤
│ Metadata              │ Timestamp (relative: "2h ago")                        │
│                       │ Filename:line numbers                                  │
├───────────────────────┼─────────────────────────────────────────────────────┤
│ Commit Body           │ Shown as bullet points under commit                   │
│                       │ • Line 1                                               │
│                       │ • Line 2                                               │
├───────────────────────┼─────────────────────────────────────────────────────┤
│ Tree Structure        │ │ (vertical continuation)                              │
│                       │ \ (branch divergence)                                  │
│                       │ / (branch merge)                                       │
└───────────────────────┴─────────────────────────────────────────────────────┘

COMMIT LIST ITEM FORMAT:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Position │ Content                          │ Width                         │
├──────────┼──────────────────────────────────┼──────────────────────────────┤
│ 1-2      │ Tree character (│/\)             │ 2 chars                       │
│ 3        │ Space                            │ 1 char                        │
│ 4-5      │ Commit node (●●/●/◉)             │ 2 chars                       │
│ 6        │ Space                            │ 1 char                        │
│ 7-13     │ SHA (7 chars)                    │ 7 chars                       │
│ 14       │ Space                            │ 1 char                        │
│ 15-50    │ Refs and message                 │ 36 chars                      │
│ 52-60    │ Timestamp                        │ 9 chars                       │
│ 61-end   │ Location hint                    │ Remaining                     │
└──────────┴──────────────────────────────────┴────────────────────────────────┘

KEYBINDINGS (Commits Panel):
┌─────────────────────────────────────────────────────────────────────────────┐
│ Key             │ Action                                                      │
├─────────────────┼─────────────────────────────────────────────────────────────┤
│ Enter           │ View commit details                                         │
│ o               │ Open commit in pager (git)                             │
│ show c               │ Copy commit SHA to clipboard                                │
│ p               │ Create patch from commit                                    │
│ r               │ Revert commit                                               │
│ t               │ Create tag at commit                                        │
│ C               │ Cherry-pick commit                                          │
│ R               │ Rebase interactive (if in HEAD)                             │
│ E               │ Edit commit message (if amendable)                          │
│ f               │ Fetch commit details (lazy load)                            │
│ /               │ Search commits                                              │
│ n               │ Next search result                                          │
│ N               │ Previous search result                                      │
│ ]              │ Next commit in current branch                                │
│ [              │ Previous commit in current branch                            │
│ {              │ First ancestor in current branch                             │
│ }              │ Last commit (HEAD)                                           │
└─────────────────┴─────────────────────────────────────────────────────────────┘

COLOR SCHEME:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Element              │ Foreground  │ Background   │ Style                   │
├──────────────────────┼─────────────┼──────────────┼─────────────────────────┤
│ Commit Node (HEAD)   │ Black       │ Green        │ Bold                    │
│ Commit Node          │ White       │ -            │ -                       │
│ Merge Commit         │ Yellow      │ -            │ Bold                    │
│ Commit SHA           │ Cyan        │ -            │ -                       │
│ Branch Refs          │ Magenta     │ -            │ Italic                  │
│ Tag Refs             │ Yellow      │ -            │ Bold                    │
│ Tree Lines           │ Dark Gray   │ -            │ -                       │
│ Timestamp            │ Dark Gray   │ -            │ -                       │
│ Selected Commit      │ Black       │ Cyan         │ Bold                    │
│ Diff Add (+)         │ Green       │ -            │ -                       │
│ Diff Del (-)         │ Red         │ -            │ -                       │
└──────────────────────┴─────────────┴──────────────┴─────────────────────────┘

TREE NAVIGATION:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Graphical Representation:                                                      │
│                                                                               │
│   ●● a7f3d2e (HEAD)                                                           │
│   │                                                                            │
│   ●● 9b2c4d1  Merge                                                           │
│   │ \                                                                           │
│   │ ●● 8f1a2b3  Feature  ◄── Current selection (↓ moves here)                  │
│   │ │                                                                           │
│   │ ●● 7d3e5f2  Perf                                                              │
│   │ │                                                                           │
│   │ * 6b8c4d1  Update                                                            │
│   │                                                                            │
│   * 5a7b3d2  Refactor                                                           │
│   │                                                                            │
│   ●● 4c6d2e1  Version                                                           │
│                                                                               │
│ Navigation Rules:                                                              │
│   j/k: Move up/down commit list                                                │
│   h/l: Switch between branches at merge point                                  │
│   Tab: Cycle through parent commits at merge                                   │
└─────────────────────────────────────────────────────────────────────────────┘

```

---

## 4. Interactive Staging Flow with Hunk-Level View

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ INTERACTIVE STAGING MODE                                           [Stage: 1/3]    │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│ File: src/core/stack.rs                                                     67% █│
│ ─────────────────────────────────────────────────────────────────────────────────  │
│                                                                                     │
│ @@ -23,15 +23,18 @@ impl StackAnalyzer {                                          │
│ 23  │     fn detect_language(&self, file: &Path) -> Option<Language> {            │
│ 24  │-        let ext = file.extension()?.to_str()?;                              │
│ 25  │-        match ext {                                                         │
│ 26  │-            "rs" => Some(Language::Rust),                                   │
│ 27  │-            "py" => Some(Language::Python),                                 │
│ 28  │-            "js" => Some(Language::JavaScript),                             │
│ 29  │-            "ts" => Some(Language::TypeScript),                             │
│ 30  │-            _ => None,                                                      │
│ 31  │-        }                                                                   │
│ 23  │+    fn detect_language(&self, file: &Path) -> Option<Language> {            │
│ 24  │+        let ext = file.extension().and_then(|e| e.to_str())?;              │
│ 25  │+        match ext {                                                         │
│ 26  │+            "rs" => Some(Language::Rust),                                   │
│ 27  │+            "py" => Some(Language::Python),                                 │
│ 28  │+            "js" => Some(Language::JavaScript),                             │
│ 29  │+            "ts" => Some(Language::TypeScript),                             │
│ 30  │+            "go" => Some(Language::Go),                                     │
│ 31  │+            "rs" | "toml" => Some(Language::Rust),  // Special case         │
│ 32  │+            _ => None,                                                      │
│ 33  │+        }                                                                   │
│ 34  │     }                                                                       │
│                                                                                     │
│ ────────────────────────────────────────────────────────────────────────────────  │
│ Hunk 1/3: Lines 23-36 (+12/-8)                                                 │
│ ────────────────────────────────────────────────────────────────────────────────  │
│                                                                                     │
│ @@ -67,10 +70,12 @@ impl StackAnalyzer {                                        │
│ 67  │-    fn resolve_deps(&mut self, deps: &[Dependency]) -> Result<Vec<Package>> │
│ 68  │-        let mut packages = Vec::new();                                      │
│ 69  │-        for dep in deps {                                                   │
│ 69  │+    fn resolve_deps(&mut self, deps: &[Dependency]) -> Result<Vec<Package>>│
│ 70  │+        let mut packages: Vec<Package> = Vec::new();                        │
│ 71  │+        // TODO: Add parallel processing for large dependency sets          │
│ 72  │+        for dep in deps.iter() {                                            │
│ 73  │             let pkg = self.fetch_package(dep)?;                             │
│ 73  │+            packages.push(pkg);                                             │
│ 74  │-            packages.push(pkg);                                             │
│ 75  │         }                                                                   │
│ 76  │         Ok(packages)                                                        │
│ 76  │     }                                                                       │
│                                                                                     │
│ ────────────────────────────────────────────────────────────────────────────────  │
│ Hunk 2/3: Lines 67-76 (+5/-3)                                                 │
│ ────────────────────────────────────────────────────────────────────────────────  │
│                                                                                     │
│ @@ -89,5 +92,5 @@ impl StackAnalyzer {                                        │
│ 89  │-    fn cache_key(&self, pkg: &Package) -> String {                          │
│ 90  │-        format!("{}-{}", pkg.name(), pkg.version())                         │
│ 89  │+    fn cache_key(&self, pkg: &Package) -> Cow<str> {                        │
│ 90  │+        Cow::Owned(format!("{}-{}", pkg.name(), pkg.version()))             │
│ 91  │     }                                                                       │
│                                                                                     │
│ ────────────────────────────────────────────────────────────────────────────────  │
│ Hunk 3/3: Lines 89-91 (+2/-2)                                                 │
│ ────────────────────────────────────────────────────────────────────────────────  │
│                                                                                     │
│ ────────────────────────────────────────────────────────────────────────────────  │
│ Stage: 1 hunks staged, 1 hunks discarded, 1 hunks remaining                      │
│ ────────────────────────────────────────────────────────────────────────────────  │
│                                                                                     │
│ [y] Stage this hunk   [n] Discard  [s] Split   [a] Stage all   [d] Discard all  │
│ [p] Partial (edit)    [e] Edit     [q] Quit   [j] Next hunk   [k] Previous      │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
SPLIT HUNK VIEW
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ SPLIT HUNK MODE - Edit individual lines                              [Edit Mode]  │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│ Hunk: Lines 23-36 (+12/-8)                                                        │
│ ────────────────────────────────────────────────────────────────────────────────  │
│                                                                                     │
│  23 │+    fn detect_language(&self, file: &Path) -> Option<Language> {            │
│  24 │+        let ext = file.extension().and_then(|e| e.to_str())?;              │
│  25 │+        match ext {                                                         │
│  26 │+            "rs" => Some(Language::Rust),                                   │
│  27 │+            "py" => Some(Language::Python),                                 │
│  28 │+            "js" => Some(Language::JavaScript),                             │
│  29 │+            "ts" => Some(Language::TypeScript),                             │
│  30 │+            "go" => Some(Language::Go),                                     │
│  31 │+            "rs" | "toml" => Some(Language::Rust),  // Special case         │
│  32 │+            _ => None,                                                      │
│  33 │+        }                                                                   │
│  34 │+    }                                                                       │
│                                                                                     │
│ ────────────────────────────────────────────────────────────────────────────────  │
│ Select lines to stage (use space to toggle, y to confirm):                        │
│ ────────────────────────────────────────────────────────────────────────────────  │
│                                                                                     │
│ [ ] Line 23: fn detect_language signature change                                 │
│ [ ] Line 24: More idiomatic extension handling                                   │
│ [ ] Line 30: Add Go language support                                             │
│ [✓] Line 31: Special case for Rust files (important bug fix)                     │
│ [ ] Line 32: Default case unchanged                                              │
│                                                                                     │
│ [y] Confirm selection   [r] Reset selection   [q] Cancel                          │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
SPECIFICATIONS
==================================================================================

INTERACTIVE STAGING WORKFLOW:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Step 1: Enter Interactive Mode                                               │
│    From Files panel: Press 'i' to enter interactive staging                 │
│    Command: :git add -i (alternative)                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│ Step 2: File Selection                                                      │
│    - Navigate to file in unstaged section                                   │
│    - Press Enter to view hunks                                              │
│    - Or press 's' to select file directly                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│ Step 3: Hunk Review                                                         │
│    - Review each hunk's diff                                                │
│    - Use j/k to navigate hunks                                              │
│    - Use y/n/s/a/d for actions                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│ Step 4: Confirm Staging                                                     │
│    - Press q to finish when done                                            │
│    - Review staged changes in Files panel                                   │
│    - Commit with :git commit or cc keybinding                               │
└─────────────────────────────────────────────────────────────────────────────┘

HUNK VIEW DIMENSIONS:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Component              │ Lines    │ Content                                   │
├────────────────────────┼──────────┼───────────────────────────────────────────┤
│ File header            │ 1        │ File path + progress indicator            │
│ Hunk header            │ 1        │ @@ line reference @@                      │
│ Hunk diff context      │ Variable │ Lines before change                       │
│ Changed lines          │ Variable │ Additions (+ green) / Deletions (- red)  │
│ Hunk footer            │ 1        │ Line counts (+/-)                         │
│ Action bar             │ 2        │ Keybindings + descriptions                │
├────────────────────────┼──────────┼───────────────────────────────────────────┤
│ Default visible hunks  │ 3        │ Fits in ~20 line terminal                 │
│ Max hunks              │ 10       │ Depends on terminal height                │
└────────────────────────┴──────────┴───────────────────────────────────────────┘

KEYBINDINGS (Interactive Mode):
┌─────────────────────────────────────────────────────────────────────────────┐
│ Key             │ Action                                                      │
├─────────────────┼─────────────────────────────────────────────────────────────┤
│ y               │ Stage current hunk                                          │
│ n               │ Discard current hunk (git checkout --)                     │
│ s               │ Split hunk into smaller pieces                             │
│ a               │ Stage all remaining hunks in file                          │
│ d               │ Discard all hunks in file                                  │
│ p               │ Partial edit (enter line-by-line mode)                     │
│ e               │ Edit hunk manually (in editor)                             │
│ j               │ Move to next hunk                                           │
│ k               │ Move to previous hunk                                       │
│ g               │ Jump to first hunk                                          │
│ G               │ Jump to last hunk                                           │
│ q               │ Quit interactive mode                                       │
│ Ctrl+c          │ Cancel without saving                                       │
│ ?               │ Show help                                                   │
└─────────────────┴─────────────────────────────────────────────────────────────┘

SPLIT MODE KEYBINDINGS:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Key             │ Action                                                      │
├─────────────────┼─────────────────────────────────────────────────────────────┤
│ Space           │ Toggle line selection                                       │
│ v               │ Toggle visual selection mode                                │
│ j/k             │ Move selection down/up                                      │
│ y               │ Confirm and stage selected lines                            │
│ r               │ Reset all selections                                        │
│ q               │ Cancel split mode                                           │
└─────────────────┴─────────────────────────────────────────────────────────────┘

COLOR SCHEME (Hunk View):
┌─────────────────────────────────────────────────────────────────────────────┐
│ Element              │ Foreground  │ Background   │ Style                   │
├──────────────────────┼─────────────┼──────────────┼─────────────────────────┤
│ File Header          │ White       │ Dark Gray    │ Bold                    │
│ Hunk Header          │ White       │ Dark Blue    │ Bold                    │
│ Context Line         │ Gray        │ -            │ -                       │
│ Addition Line        │ Black       │ Green        │ -                       │
│ Deletion Line        │ Black       │ Red          │ -                       │
│ Selected Hunk        │ Black       │ Yellow       │ Bold                    │
│ Current Line         │ -           │ Dark Gray    │ Underline               │
│ Action Bar           │ White       │ Dark Red     │ Bold                    │
│ Key Hint             │ Black       │ Green        │ -                       │
│ Description          │ Gray        │ -            │ Italic                  │
└──────────────────────┴─────────────┴──────────────┴─────────────────────────┘

STATUS INDICATORS:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Indicator        │ Meaning                                                    │
├──────────────────┼────────────────────────────────────────────────────────────┤
│ [Stage: 1/3]     │ 1 of 3 hunks staged                                        │
│ [+12/-8]         │ 12 additions, 8 deletions                                  │
│ @@ -23,15 +23,18 │ Original lines 23-15, new lines 23-18                     │
│ [y]              │ Green button hint for stage                                │
│ [ ]              │ Unselected line in split mode                              │
│ [✓]              │ Selected line in split mode                                │
└──────────────────┴────────────────────────────────────────────────────────────┘

```

---

## 5. Branch Panel - Local/Remote Branches

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ BRANCHES PANEL                                                              [20] │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│ 🔀 LOCAL (8)                                                                         │
│ ─────────────────────────────────────────────────────────────────────────────────  │
│ > main     ✓                    ↑0 ↓0  | 2 commits ahead, 3 behind  │ 2d ago      │
│   develop  ✓                    ↑2 ↓1  | 5 commits ahead              │ 1w ago      │
│ ● feature/auth                  ↑3 ↓0  | 7 commits ahead, work in     │ 3h ago      │
│   feature/ui                    ↑1 ↓2  | 4 commits ahead, 5 behind    │ 5d ago      │
│ ● feature/api   ✗               ↑5 ↓3  | Diverged: 5/3               │ 6h ago      │
│   feature/cli                   ↑0 ↓0  | Same as origin               │ 2w ago      │
│   hotfix/critical ✗             ↑2 ↓1  | 2 commits ahead, local work  │ 4h ago      │
│   release/v1.0                  ↑0 ↓0  | Tag: v1.0.0-rc1              │ 3w ago      │
│                                                                                     │
│ 🌍 REMOTE (12)                                                                        │
│ ─────────────────────────────────────────────────────────────────────────────────  │
│   origin/main                  1234abc  | 5 commits behind             │ 2d ago      │
│   origin/develop               a7f3d2e  | 2 commits ahead              │ 1d ago      │
│   origin/feature/auth          9b2c4d1  | Synced with local            │ 3h ago      │
│   origin/feature/ui            8f1a2b3  | 1 commit ahead               │ 5d ago      │
│   origin/feature/api           7d3e5f2  | 3 commits behind             │ 6h ago      │
│   origin/feature/cli           6c4b8a1  | 5 commits ahead              │ 1w ago      │
│   origin/hotfix/critical       5a7b3d2  | 1 commit behind              │ 4h ago      │
│   origin/release/v1.0          4c6d2e1  | Tag: v1.0.0                  │ 3w ago      │
│   upstream/main                b3d2e1f  | 10 commits ahead             │ 1mo ago     │
│   upstream/develop             c4e2f1a  | 2 commits behind             │ 2w ago      │
│   upstream/feature/auth        d5f3a2b  | Same as local                │ 3d ago      │
│   fork/cli-fix                 e6a4b3c  | 15 commits ahead             │ 1mo ago     │
│                                                                                     │
│ Actions: [c] Create  │ [d] Delete  │ [r] Rename  │ [m] Merge  │ [p] Publish  │ [f] Fetch │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
BRANCH CREATE DIALOG
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ CREATE NEW BRANCH                                                       [Dialog] │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│                                                                                     │
│    ┌─────────────────────────────────────────────────────────────────────────┐     │
│    │ Branch Name: feature/                                                   │     │
│    │ [feature/new-ui________]                                                │     │
│    │                                                                         │     │
│    │ Base Branch: develop                                                    │     │
│    │ [develop___▼]                                                           │     │
│    │                                                                         │     │
│    │ Start Point:                                                            │     │
│    │ (●) From current HEAD (a7f3d2e)                                        │     │
│    │ ( ) From branch: [develop___▼]                                         │     │
│    │ ( ) From tag: [v1.0.0___▼]                                              │     │
│    │ ( ) From commit: [a7f3d2e______]                                        │     │
│    │                                                                         │     │
│    │ [✓] Switch to new branch immediately                                    │     │
│    │                                                                         │     │
│    └─────────────────────────────────────────────────────────────────────────┘     │
│                                                                                     │
│    [Enter] Confirm  │  [Tab] Switch field  │  [Esc] Cancel  │  [Ctrl+n] Autocomplete│
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
BRANCH OPERATIONS PANEL
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ BRANCH OPERATIONS: feature/auth                                        [Selected] │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│ Branch: feature/auth                                                              │
│ HEAD: a7f3d2e (Fix parser null pointer)                                          │
│ Upstream: origin/feature/auth                                                     │
│                                                                                     │
│ ┌─ Compare ─────────────────────────────────────────────────────────────────────┐   │
│ │ compare: feature/auth [▼]  vs  develop [▼]                                   │   │
│ │                                                                               │   │
│ │ Behind: 3 commits        │ Ahead: 5 commits         │ Diverged: ✓           │   │
│ │ ↓ 9b2c4d1 → 8f1a2b3     │ ↑ 7d3e5f2 → 6c4b8a1      │ ✓ Common: 5a7b3d2     │   │
│ │ ↓ 8f1a2b3 → 7d3e5f2     │ ↑ 6c4b8a1 → 5a7b3d2      │ ✓ Ahead: 5 commits    │   │
│ │ ↓ 7d3e5f2 → 6c4b8a1     │ ↑ 5a7b3d2 → 4c6d2e1      │ ✓ Behind: 3 commits   │   │
│ │                                                                               │   │
│ └───────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                     │
│ ┌─ Actions ─────────────────────────────────────────────────────────────────────┐   │
│ │ [m] Merge feature/auth into current       [r] Rebase feature/auth on develop │   │
│ │ [p] Push feature/auth to origin           [c] Compare branches...            │   │
│ │ [d] Delete feature/auth (local)           [C] Delete feature/auth (remote)   │   │
│ │ [R] Rename feature/auth                   [t] Track origin/feature/auth       │   │
│ └───────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
SPECIFICATIONS
==================================================================================

BRANCH LIST ITEM FORMAT:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Position │ Content                          │ Width                         │
├──────────┼──────────────────────────────────┼──────────────────────────────┤
│ 1-3      │ Status (●/ /✗/✓/○)               │ 3 chars                       │
│ 4        │ Space                            │ 1 char                        │
│ 5-30     │ Branch name                      │ 26 chars                      │
│ 32-45    │ Sync status (↑0 ↓0)              │ 14 chars                      │
│ 47-62    │ Description                      │ 16 chars                      │
│ 64-70    │ Timestamp                        │ 7 chars                       │
└──────────┴──────────────────────────────────┴────────────────────────────────┘

BRANCH STATUS INDICATORS:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Icon   │ Meaning                        │ Color                            │
├────────┼────────────────────────────────┼──────────────────────────────────┤
│ >      │ Current branch                 │ Cyan                             │
│ ●      │ Selected branch                │ Yellow                           │
│ ✓      │ Up-to-date with upstream       │ Green                            │
│ ✗      │ Diverged from upstream         │ Red                              │
│ ○      │ No upstream tracking           │ Gray                             │
│ ↑n     │ n commits ahead                │ Green                            │
│ ↓n     │ n commits behind               │ Yellow                           │
│ ↕n/m   │ n ahead, m behind              │ Red                              │
└────────┴────────────────────────────────┴──────────────────────────────────┘

KEYBINDINGS (Branches Panel):
┌─────────────────────────────────────────────────────────────────────────────┐
│ Key             │ Action                                                      │
├─────────────────┼─────────────────────────────────────────────────────────────┤
│ Enter           │ View branch details                                         │
│ c               │ Create new branch                                           │
│ d               │ Delete branch (with confirmation)                           │
│ D               │ Force delete branch                                         │
│ r               │ Rename branch                                               │
│ m               │ Merge selected branch into current                          │
│ R               │ Rebase current branch on selected                           │
│ p               │ Push branch to remote                                       │
│ P               │ Push all branches to remote                                 │
│ f               │ Fetch from remote                                           │
│ F               │ Fetch all remotes                                           │
│ t               │ Set upstream tracking                                       │
│ o               │ Checkout branch                                             │
│ O               │ Create and checkout branch                                  │
│ b               │ Branch from selected branch                                 │
│ /               │ Search branches                                             │
│ ]              │ Next branch (local→remote)                                   │
│ [              │ Previous branch (remote→local)                               │
│ Tab             │ Switch between local/remote lists                           │
└─────────────────┴─────────────────────────────────────────────────────────────┘

COLOR SCHEME (Branches Panel):
┌─────────────────────────────────────────────────────────────────────────────┐
│ Element              │ Foreground  │ Background   │ Style                   │
├──────────────────────┼─────────────┼──────────────┼─────────────────────────┤
│ Current Branch (>)   │ Black       │ Cyan         │ Bold                    │
│ Selected Branch (●)  │ Black       │ Yellow       │ Bold                    │
│ Local Branch         │ White       │ -            │ -                       │
│ Remote Branch        │ Gray        │ -            │ Italic                  │
│ Up-to-date (✓)       │ Green       │ -            │ Bold                    │
│ Diverged (✗)         │ Red         │ -            │ Bold                    │
│ Ahead (↑)            │ Green       │ -            │ -                       │
│ Behind (↓)           │ Yellow      │ -            │ -                       │
│ Section Header       │ White       │ Dark Gray    │ Bold                    │
│ Section Divider      │ Gray        │ -            │ -                       │
└──────────────────────┴─────────────┴──────────────┴─────────────────────────┘

REMOTE MANAGEMENT:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Remote Actions:                                                              │
│                                                                               │
│   [a] Add remote    - Add new remote repository                              │
│   [r] Remove remote - Remove existing remote                                  │
│   [p] Prune remote  - Delete stale remote-tracking branches                  │
│   [R] Rename remote - Rename remote reference                                 │
│   [v] View remotes  - Show all configured remotes                            │
│                                                                               │
│ Remote List Format:                                                          │
│   origin    https://github.com/user/repo.git  (fetch: +push: ✓)              │
│   upstream  https://github.com/org/repo.git   (fetch: +push: ✗)              │
│   fork      git@github.com:user/repo.git     (fetch: ✗push: ✓)               │
└─────────────────────────────────────────────────────────────────────────────┘

```

---

## 6. Onboarding Tour for First-Time Users

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ openISL - WELCOME TO YOUR TECHNOLOGY STACK DETECTOR                          [1/6] │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│                                                                                     │
│    ┌─────────────────────────────────────────────────────────────────────────┐     │
│    │                                                                         │     │
│    │     ██████╗  █████╗ ██████╗  █████╗ ██╗     ██╗     ███████╗██╗       │     │
│    │     ██╔══██╗██╔══██╗██╔══██╗██╔══██╗██║     ██║     ██╔════╝██║       │     │
│    │     ██████╔╝███████║██████╔╝███████║██║     ██║     █████╗  ██║       │     │
│    │     ██╔═══╝ ██╔══██║██╔══██╗██╔══██║██║     ██║     ██╔══╝  ██║       │     │
│    │     ██║     ██║  ██║██║  ██║██║  ██║███████╗███████╗███████╗███████╗  │     │
│    │     ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝╚══════╝  │     │
│    │                                                                         │     │
│    │                   TECHNOLOGY STACK DETECTOR                              │     │
│    │                                                                         │     │
│    │              ────────────────────────────────────────                   │     │
│    │                                                                         │     │
│    │              Detect • Analyze • Visualize • Understand                  │     │
│    │                                                                         │     │
│    │              ────────────────────────────────────────                   │     │
│    │                                                                         │     │
│    │                    Press [Enter] to continue →                          │     │
│    │                                                                         │     │
│    └─────────────────────────────────────────────────────────────────────────┘     │
│                                                                                     │
│    [Skip Tour] [← Back]                      [6/6]                     [Next →]     │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
TOUR STEP 2: PANEL OVERVIEW
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ openISL - TOUR: PANEL OVERVIEW                                              [2/6] │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│    YOUR TERMINAL, YOUR STACK                                                      │
│    ─────────────────────────────────────────────────────────────────────────────   │
│                                                                                     │
│    ┌─────────────┬─────────────────────────────────────────────────────────────┐    │
│    │  STATUS     │  FILES                                                    │    │
│    │  • Branch   │  • Staged files                                           │    │
│    │  • Changes  │  • Modified files                                         │    │    │
│    │  • Remote   │  • Untracked files                                        │    │
│    ├─────────────┴─────────────────────────────────────────────────────────────┤    │
│    │  BRANCHES   │  COMMITS                                                  │    │
│    │  • Local    │  • History tree                                           │    │
│    │  • Remote   │  • Commit details                                         │    │
│    ├───────────────────────────────────────────────────────────────────────────┤    │
│    │  STASH (bottom panel)                                                   │    │
│    └───────────────────────────────────────────────────────────────────────────┘    │
│                                                                                     │
│    ┌─ Quick Actions ─────────────────────────────────────────────────────────┐     │
│    │                                                                         │     │
│    │   s: Stage    │   c: Commit   │   p: Push   │   q: Quit               │     │
│    │                                                                         │     │
│    └─────────────────────────────────────────────────────────────────────────┘     │
│                                                                                     │
│    [Skip Tour] [← Back]                      [6/6]                     [Next →]     │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
TOUR STEP 3: NAVIGATION
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ openISL - TOUR: NAVIGATION                                                    [3/6] │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│    NAVIGATE LIKE A PRO                                                            │
│    ─────────────────────────────────────────────────────────────────────────────   │
│                                                                                     │
│         ┌─────────────────────────────┐                                         │
│         │     PANEL NAVIGATION        │                                         │
│         │                             │                                         │
│         │    h  k  j  l              │    Like Vim!                            │
│         │    ←  ↑  ↓  →              │                                         │
│         │                             │                                         │
│         │    Tab: Next panel         │                                         │
│         │    Ctrl+w: Switch panel    │                                         │
│         └─────────────────────────────┘                                         │
│                                                                                     │
│    ┌───────────────────────┐  ┌─────────────────────────────────────────────────┐  │
│    │ COMMON NAV KEYS       │  │ COMMAND MODE (:)                                │  │
│    ├───────────────────────┤  ├─────────────────────────────────────────────────┤  │
│    │ j / ↓  : Down         │  │ :w      Save changes                           │  │
│    │ k / ↑  : Up           │  │ :q      Quit                                   │  │
│    │ gg     : First item   │  │ :wq     Save & quit                           │  │
│    │ G      : Last item    │  │ :commit Create commit                         │  │
│    │ /      : Search       │  │ :push   Push to remote                        │  │
│    │ n      : Next match   │  │ :help   Show help                            │  │
│    │ N      : Prev match   │  │ :git <any> Run git command                    │  │
│    └───────────────────────┘  └─────────────────────────────────────────────────┘  │
│                                                                                     │
│    [Skip Tour] [← Back]                      [6/6]                     [Next →]     │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
TOUR STEP 4: STACK DETECTION
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ openISL - TOUR: STACK DETECTION                                              [4/6] │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│    DETECT YOUR TECHNOLOGY STACK                                                   │
│    ─────────────────────────────────────────────────────────────────────────────   │
│                                                                                     │
│    ┌───────────────────────────────────────────────────────────────────────────┐    │
│    │  openISL automatically detects:                                           │    │
│    │                                                                           │    │
│    │  📦 PACKAGE MANAGERS                                                     │    │
│    │     Cargo.toml (Rust) • package.json (Node.js) • requirements.txt        │    │
│    │     pom.xml (Java) • go.mod (Go) • Gemfile (Ruby) • pyproject.toml       │    │
│    │                                                                           │    │
│    │  🏗️  BUILD TOOLS                                                        │    │
│    │     CMake • Makefile • Cargo • Webpack • Maven • Gradle • Bazel          │    │
│    │                                                                           │    │
│    │  🔧 FRAMEWORKS & LIBRARIES                                               │    │
│    │     Actix • Rocket (Rust) • Express • NestJS (Node) • Flask • Django     │    │
│    │                                                                           │    │
│    │  📝 CONFIG FILES                                                         │    │
│    │     .eslintrc • .prettierrc • tsconfig • rust-toolchain • .python-version │    │
│    └───────────────────────────────────────────────────────────────────────────┘    │
│                                                                                     │
│    [Skip Tour] [← Back]                      [6/6]                     [Next →]     │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
TOUR STEP 5: INTERACTIVE FEATURES
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ openISL - TOUR: INTERACTIVE FEATURES                                          [5/6] │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│    POWER FEATURES                                                                 │
│    ─────────────────────────────────────────────────────────────────────────────   │
│                                                                                     │
│    ┌──────────────────────┐  ┌──────────────────────────────────────────────────┐  │
│    │  INTERACTIVE STAGING  │  │  COMMIT TREE NAVIGATION                         │  │
│    ├──────────────────────┤  ├──────────────────────────────────────────────────┤  │
│    │  i: Enter mode       │  │  o: View commit details                         │  │
│    │  y: Stage hunk       │  │  c: Copy SHA                                    │  │
│    │  n: Discard hunk     │  │  p: Create patch                                │  │
│    │  s: Split hunk       │  │  r: Revert commit                               │  │
│    │  a: Stage all        │  │  R: Rebase interactive                          │  │
│    └──────────────────────┘  └──────────────────────────────────────────────────┘  │
│                                                                                     │
│    ┌──────────────────────┐  ┌──────────────────────────────────────────────────┐  │
│    │  BRANCH MANAGEMENT    │  │  SEARCH & FILTER                               │  │
│    ├──────────────────────┤  ├──────────────────────────────────────────────────┤  │
│    │  c: Create branch     │  │  /: Search files/commits                       │  │
│    │  d: Delete branch     │  │  f: Filter by pattern                          │  │
│    │  m: Merge branch      │  │  *: Filter by status                           │  │
│    │  p: Push to remote    │  │  r: Refresh view                               │  │
│    └──────────────────────┘  └──────────────────────────────────────────────────┘  │
│                                                                                     │
│    [Skip Tour] [← Back]                      [6/6]                     [Next →]     │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
TOUR STEP 6: GETTING HELP
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ openISL - TOUR: GETTING HELP                                                 [6/6] │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│    YOU'RE ALL SET!                                                                │
│    ─────────────────────────────────────────────────────────────────────────────   │
│                                                                                     │
│    ┌───────────────────────────────────────────────────────────────────────────┐    │
│    │                                                                           │    │
│    │   START USING openISL:                                                    │    │
│    │                                                                           │    │
│    │      1. Navigate with h/j/k/l or arrow keys                              │    │
│    │      2. Press ? for help at any time                                     │    │
│    │      3. Press : for command mode                                         │    │
│    │      4. Run 'openisl --help' for CLI options                             │    │
│    │                                                                           │    │
│    └───────────────────────────────────────────────────────────────────────────┘    │
│                                                                                     │
│    ┌─────────────────────────┐  ┌───────────────────────────────────────────────┐   │
│    │ HELP RESOURCES          │  │ KEYBINDINGS QUICK REFERENCE                  │   │
│    ├─────────────────────────┤  ├───────────────────────────────────────────────┤   │
│    │ ?         Show help     │  │ ?: Help           q: Quit                    │   │
│    │ :help      Full docs    │  │ h/j/k/l: Move    Tab: Focus                 │   │
│    │ :keybindings Reference  │  │ i: Interactive    s: Stage                  │   │
│    │ GitHub      Report bug  │  │ c: Commit         p: Push                   │   │
│    └─────────────────────────┘  └───────────────────────────────────────────────┘   │
│                                                                                     │
│                              [✓ FINISH TOUR]                           [Next →]     │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
SPECIFICATIONS
==================================================================================

ONBOARDING TOUR STRUCTURE:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Step   │ Title                  │ Duration  │ Content                       │
├────────┼────────────────────────┼───────────┼───────────────────────────────┤
│ 1      │ Welcome                │ 5 sec     │ Logo + welcome message        │
│ 2      │ Panel Overview         │ 30 sec    │ Layout explanation            │
│ 3      │ Navigation             │ 45 sec    │ Keys + command mode           │
│ 4      │ Stack Detection        │ 30 sec    │ What openISL detects          │
│ 5      │ Interactive Features   │ 45 sec    │ Key features demo             │
│ 6      │ Getting Help           │ 20 sec    │ Resources + reference         │
├────────┴────────────────────────┴───────────┴───────────────────────────────┤
│ Total Tour Duration: ~3 minutes                                            │
└─────────────────────────────────────────────────────────────────────────────┘

TOUR NAVIGATION:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Key             │ Action                                                      │
├─────────────────┼─────────────────────────────────────────────────────────────┤
│ Enter           │ Next step / Finish tour                                    │
│ → / j / n       │ Next step                                                  │
│ ← / k / p       │ Previous step                                              │
│ Tab             │ Skip Tour / Finish Tour (toggle)                           │
│ Escape          │ Exit tour (with confirmation)                              │
│ q               │ Quit tour (with confirmation)                              │
└─────────────────┴─────────────────────────────────────────────────────────────┘

AUTOMATIC ADVANCEMENT:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Condition                    │ Action                                        │
├──────────────────────────────┼───────────────────────────────────────────────┤
│ Timer expires (per step)     │ Auto-advance to next step                    │
│ User presses key             │ Stop timer, wait for input                   │
│ All steps completed          │ Show completion message, enable openISL      │
│ User skips tour              │ Save preference, start normally              │
│ Second launch (saved pref)   │ Skip tour automatically                      │
└──────────────────────────────┴───────────────────────────────────────────────┘

TOUR CONFIGURATION:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Config Option     │ Default    │ Description                                 │
├───────────────────┼────────────┼─────────────────────────────────────────────┤
│ tour_on_first_run │ true       │ Enable tour on first launch                │
│ tour_auto_advance │ true       │ Auto-advance steps                          │
│ tour_step_time    │ 5000ms     │ Time per step (if auto-advance)            │
│ tour_show_on_update │ major    │ Show on major version updates              │
└───────────────────┴────────────┴─────────────────────────────────────────────┘

COLOR SCHEME (Tour Screens):
┌─────────────────────────────────────────────────────────────────────────────┐
│ Element              │ Foreground  │ Background   │ Style                   │
├──────────────────────┼─────────────┼──────────────┼─────────────────────────┤
│ Logo Text            │ Cyan        │ -            │ Bold                    │
│ Tour Progress        │ White       │ Dark Gray    │ Bold                    │
│ Section Headers      │ Yellow      │ -            │ Bold                    │
│ Keybindings          │ Black       │ Green        │ Bold                    │
│ Descriptions         │ Gray        │ -            │ -                       │
│ Navigation Hints     │ Dark Gray   │ -            │ Italic                  │
│ Progress Bar         │ Green       │ -            │ -                       │
│ Finish Button        │ Black       │ Green        │ Bold                    │
└──────────────────────┴─────────────┴──────────────┴─────────────────────────┘

```

---

## 7. Vim Editor Integration Flow

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ ─────────────────────────────────────────────────────────────────────────────────  │
│   src/commit_message.txt                                                         │
│   ───────────────────────────────────────────────────────────────────────────────  │
│   1│# Please enter the commit message for your changes.                           │
│   2│# Lines starting with '#' will be ignored.                                    │
│   3│#                                                                            │
│   4│# Changes to be committed:                                                    │
│   5│#   new file:   Cargo.toml                                                   │
│   6│#   modified:   src/main.rs                                                  │
│   7│#   modified:   src/cli/mod.rs                                               │
│   8│#                                                                            │
│   9│                                                                            │
│   10│                                                                            │
│   11│                                                                            │
│   12│                                                                            │
│   13│                                                                            │
│   14│                                                                            │
│   15│                                                                            │
│   ───────────────────────────────────────────────────────────────────────────────  │
│   -- INSERT --                                                              15,1   │
│   ───────────────────────────────────────────────────────────────────────────────  │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
VIM EDITOR - EDIT MODE
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ ─────────────────────────────────────────────────────────────────────────────────  │
│   src/commit_message.txt                                                         │
│   ───────────────────────────────────────────────────────────────────────────────  │
│   1│feat(stack): Add Go language support                                          │
│   2│                                                                            │
│   3│- Detect .go files as Go language                                             │
│   4│- Add Go package manager detection (go.mod)                                   │
│   5│- Support Go modules in stack analyzer                                        │
│   6│- Add unit tests for Go detection                                            │
│   7│                                                                            │
│   8│Co-authored-by: Jane Developer <jane@example.com>                            │
│   9│                                                                            │
│   10│                                                                            │
│   11│                                                                            │
│   12│                                                                            │
│   -- INSERT --                                                              9,1    │
│   ───────────────────────────────────────────────────────────────────────────────  │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
VIM EDITOR - NORMAL MODE
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ ─────────────────────────────────────────────────────────────────────────────────  │
│   src/commit_message.txt                                                         │
│   ───────────────────────────────────────────────────────────────────────────────  │
│   1│feat(stack): Add Go language support                                          │
│   2│                                                                            │
│   3│- Detect .go files as Go language                                             │
│   4│- Add Go package manager detection (go.mod)                                   │
│   5│- Support Go modules in stack analyzer                                        │
│   6│- Add unit tests for Go detection                                            │
│   7│                                                                            │
│   8│Co-authored-by: Jane Developer <jane@example.com>                            │
│   9│                                                                            │
│   10│                                                                            │
│   11│                                                                            │
│   12│                                                                            │
│   -- NORMAL --                                                               9,1   │
│   ───────────────────────────────────────────────────────────────────────────────  │
│   Commands: :w Save  │ :q Cancel  │ :cq Abort  │ :wq Save & Quit  │ ZZ Save & Exit│
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
VIM EDITOR - COMMIT CONFIRMATION
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ ─────────────────────────────────────────────────────────────────────────────────  │
│   Confirm Commit                                                                │
│   ───────────────────────────────────────────────────────────────────────────────  │
│                                                                                     │
│   ┌───────────────────────────────────────────────────────────────────────────┐    │
│   │  Repository: /Users/dev/project/openisl                                    │    │
│   │                                                                           │    │
│   │  Changes to commit:                                                       │    │
│   │    ✨ feat(stack): Add Go language support                                 │    │
│   │                                                                           │    │
│   │  Files staged (3):                                                        │    │
│   │    + Cargo.toml (version bump)                                            │    │
│   │    + src/stack/languages.rs (+120 -0)                                     │    │
│   │    + tests/go_detection.rs (+45 -0)                                       │    │
│   │                                                                           │    │
│   │  Statistics: 3 files changed, 165 insertions(+), 0 deletions(-)           │    │
│   └───────────────────────────────────────────────────────────────────────────┘    │
│                                                                                     │
│   ┌─────────────────────────────────────────────────────────────────────────────┐  │
│   │                                                                             │  │
│   │   [✓] Confirm and commit           [✗] Cancel                              │  │
│   │                                                                             │  │
│   │   Y: Commit with this message     N: Edit message                          │  │
│   │   A: Amend last commit            X: Show extended info                    │  │
│   │                                                                             │  │
│   └─────────────────────────────────────────────────────────────────────────────┘  │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
VIM EDITOR - KEYBINDINGS MAPPING
==================================================================================

┌─────────────────────────────────────────────────────────────────────────────────────┐
│ VIM KEYBINDINGS IN openISL                                                      │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│ ┌──────────────────────┬─────────────────────────────────────────────────────────┐ │
│ │ VIM KEY              │ openISL ACTION                                         │ │
│ ├──────────────────────┼─────────────────────────────────────────────────────────┤ │
│ │ NORMAL MODE          │                                                         │ │
│ ├──────────────────────┼─────────────────────────────────────────────────────────┤ │
│ │ h / ←               │ Move focus left (panel)                                │ │
│ │ j / ↓               │ Move selection down                                    │ │
│ │ k / ↑               │ Move selection up                                      │ │
│ │ l / →               │ Move focus right (panel)                               │ │
│ │ gg                  │ Jump to first item                                     │ │
│ │ G                   │ Jump to last item                                      │ │
│ │ i                   │ Enter insert mode / interactive staging                │ │
│ │ v                   │ Visual mode (select multiple)                          │ │
│ │ y                   │ Yank (copy) file/commit SHA                            │ │
│ │ p                   │ Paste (if applicable)                                  │ │
│ │ d                   │ Delete branch/stash/commit                             │ │
│ │ c                   │ Create branch/commit                                   │ │
│ │ r                   │ Refresh / revert                                       │ │
│ │ /                   │ Search mode                                            │ │
│ │ n                   │ Next search result                                     │ │
│ │ N                   │ Previous search result                                 │ │
│ │ u                   │ Undo staging / Undo last action                        │ │
│ │ Ctrl+r              │ Redo                                                   │ │
│ │ .                   │ Repeat last command                                    │ │
│ │ :                   │ Enter command mode                                     │ │
│ │ ;                   │ Enter command mode (alternative)                       │ │
│ │ q                   │ Quit openISL                                           │ │
│ │ q!                  │ Force quit (discard changes)                           │ │
│ │ w                   │ Save (write) changes                                   │ │
│ │ wq / ZZ             │ Save and quit                                          │ │
│ │ x                   │ Stage file / Toggle selection                          │ │
│ │ a                   │ Add all / Stage all                                    │ │
│ │ o                   │ Open in editor / View details                          │ │
│ │ enter               │ View commit/file details                               │ │
│ │ tab                 │ Switch focus                                           │ │
│ │ esc                 │ Return to normal mode                                  │ │
│ ├──────────────────────┼─────────────────────────────────────────────────────────┤ │
│ │ INSERT MODE          │                                                         │ │
│ ├──────────────────────┼─────────────────────────────────────────────────────────┤ │
│ │ esc                  │ Return to normal mode                                 │ │
│ │ ctrl+c               │ Return to normal mode                                 │ │
│ │ ctrl+h               │ Delete character (backspace)                          │ │
│ │ ctrl+w               │ Delete word                                           │ │
│ │ ctrl+u               │ Delete to beginning of line                           │ │
│ │ ctrl+a               │ Beginning of line                                     │ │
│ │ ctrl+e               │ End of line                                           │ │
│ │ ctrl+b               │ Back one character                                    │ │
│ │ ctrl+f               │ Forward one character                                 │ │
│ └──────────────────────┴─────────────────────────────────────────────────────────┘ │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘

==================================================================================
SPECIFICATIONS
==================================================================================

VIM EDITOR INTEGRATION:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Component              │ Specification                                        │
├────────────────────────┼─────────────────────────────────────────────────────┤
│ Editor Path            │ $VISUAL, $EDITOR, or vim, vi, nano, emacs           │
│ Commit Message File    │ ~/.git/COMMIT_EDITMSG or temp file                   │
│ Temporary File Format  │ /tmp/openisl-commit-XXXXX.txt                        │
│ Encoding               │ UTF-8                                                │
│ Line Endings           │ Unix (LF)                                            │
│ Backup Files           │ None (commit messages are sensitive)                 │
├────────────────────────┼─────────────────────────────────────────────────────┤
│ Exit Codes:            │                                                      │
│   0                    │ Commit created successfully                          │
│   1                    │ Error (empty message, validation failed)             │
│   128+                 │ Git error                                            │
└────────────────────────┴─────────────────────────────────────────────────────┘

VIM MODES IN openISL:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Mode            │ Cursor Style │ Indicator    │ Description                  │
├─────────────────┼──────────────┼──────────────┼──────────────────────────────┤
│ Normal          │ Block        │ (empty)      │ Navigation + commands        │
│ Insert          │ Line         │ -- INSERT -- │ Text editing                 │
│ Visual          │ Block        │ -- VISUAL -- │ Selection mode               │
│ Command (:)     │ Block        │ :            │ Ex commands                  │
│ Search (/)      │ Block        │ /            │ Search input                 │
│ Replace         │ Underline    │ -- REPLACE --│ Overwrite mode               │
└─────────────────┴──────────────┴──────────────┴──────────────────────────────┘

VIM EMULATION CONFIGURATION:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Config Option       │ Default    │ Description                                 │
├─────────────────────┼────────────┼─────────────────────────────────────────────┤
│ vim_mode            │ true       │ Enable vim keybindings                      │
│ hjkl_navigation     │ true       │ Use h/j/k/l for navigation                  │
│ esc_timeout         │ 100ms      │ Time before esc key takes effect            │
│ underline_cursor    │ false      │ Use underline cursor in insert mode         │
│ show_mode           │ true       │ Show current mode in status bar             │
│ statusline_mode     │ true       │ Show mode in status line                    │
│ ctrl_keys           │ true       │ Enable Ctrl keybindings                     │
│ relative_numbers    │ true       │ Use relative line numbers                   │
│ line_numbers        │ true       │ Show line numbers                           │
└─────────────────────┴────────────┴─────────────────────────────────────────────┘

COLOR SCHEME (Vim Integration):
┌─────────────────────────────────────────────────────────────────────────────┐
│ Element              │ Foreground  │ Background   │ Style                   │
├──────────────────────┼─────────────┼──────────────┼─────────────────────────┤
│ Normal Mode          │ White       │ -            │ -                       │
│ Insert Mode          │ Black       │ Green        │ Bold                    │
│ Visual Mode          │ Black       │ Yellow       │ Bold                    │
│ Command Mode (:)     │ Black       │ Cyan         │ Bold                    │
│ Cursor (Normal)      │ White       │ Cyan         │ -                       │
│ Cursor (Insert)      │ Black       │ Green        │ -                       │
│ Cursor (Visual)      │ Black       │ Yellow       │ -                       │
│ Line Numbers         │ Dark Gray   │ -            │ -                       │
│ Comment Lines (#)    │ Gray        │ -            │ Italic                  │
│ Status Bar           │ White       │ Dark Blue    │ Bold                    │
└──────────────────────┴─────────────┴──────────────┴─────────────────────────┘

INTEGRATION WITH GIT:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Git Command                    │ Vim Integration                             │
├────────────────────────────────┼─────────────────────────────────────────────┤
│ git commit                     │ Opens $EDITOR with template                 │
│ git commit --amend             │ Opens last commit message in vim            │
│ git rebase -i                  │ Opens editor for rebase commands            │
│ git filter-branch              │ Opens editor for filter commands            │
│ git tag -a                     │ Opens editor for tag annotation             │
│ git merge --no-ff              │ Opens editor for merge message              │
└────────────────────────────────┴─────────────────────────────────────────────┘

TEMPLATE PLACEHOLDERS:
┌─────────────────────────────────────────────────────────────────────────────┐
│ Placeholder              │ Replaced With                                     │
├──────────────────────────┼───────────────────────────────────────────────────┤
│ %s                       │ First line (subject)                             │
│ %b                       │ Body (all lines after subject)                   │
│ %h                       │ Abbreviated commit SHA                           │
│ %H                       │ Full commit SHA                                  │
│ %an                      │ Author name                                      │
│ %ae                      │ Author email                                     │
│ %cn                      │ Committer name                                   │
│ %ce                      │ Committer email                                  │
│ %d                       │ Ref names (branches, tags)                       │
│ %D                       │ Ref names (full)                                 │
│ %H                       │ Commit hash                                      │
│ %h                       │ Abbreviated commit hash                          │
│ %T                       │ Tree hash                                        │
│ %P                       │ Parent hashes                                    │
│ %p                       │ Abbreviated parent hashes                        │
│ %ar                      │ Author date (relative)                           │
│ %ai                      │ Author date (ISO)                                │
│ %cr                      │ Committer date (relative)                        │
│ %ci                      │ Committer date (ISO)                             │
└──────────────────────────┴───────────────────────────────────────────────────┘

```

---

## Appendix: Complete Keybinding Reference

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ COMPLETE KEYBINDING REFERENCE                                                      │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│ NAVIGATION                                                                          │
│ ─────────────────────────────────────────────────────────────────────────────────   │
│ h / ←              Move focus left (panel)                    │                     │
│ j / ↓              Move selection down                           │                     │
│ k / ↑              Move selection up                               │                     │
│ l / →              Move focus right (panel)                      │                     │
│ Tab                Next panel                                     │                     │
│ Ctrl+w             Switch panel (alternative)                     │                     │
│ gg                 Jump to first item                             │                     │
│ G                  Jump to last item                              │                     │
│ ]                  Next section/branch/parent                     │                     │
│ [                  Previous section/branch/parent                 │                     │
│                                                                                     │
│ FILE OPERATIONS                                                                    │
│ ─────────────────────────────────────────────────────────────────────────────────   │
│ s                  Stage file (from unstaged)                     │                     │
│ u                  Unstage file (from staged)                       │                     │
│ Space              Toggle stage/unstage file                        │                     │
│ i                  Enter interactive staging mode                   │                     │
│ y                  Stage hunk (interactive mode)                   │                     │
│ n                  Discard hunk (interactive mode)                 │                     │
│ r                  Revert file changes                              │                     │
│ d                  Delete untracked file                            │                     │
│ o                  Open file in editor                              │                     │
│ Enter              View file diff                                   │                     │
│                                                                                     │
│ COMMIT OPERATIONS                                                                  │
│ ─────────────────────────────────────────────────────────────────────────────────   │
│ c                  Create commit                                    │                     │
│ C                  Create commit (with message)                     │                     │
│ a                  Amend last commit                                │                     │
│ Enter              View commit details                              │                     │
│ o                  Open commit in pager                             │                     │
│ f                  Fetch commit details                             │                     │
│ p                  Create patch                                      │                     │
│ r                  Revert commit                                     │                     │
│ R                  Rebase interactive (if HEAD)                     │                     │
│ t                  Create tag at commit                             │                     │
│ C                  Cherry-pick commit                                │                     │
│                                                                                     │
│ BRANCH OPERATIONS                                                                  │
│ ─────────────────────────────────────────────────────────────────────────────────   │
│ c                  Create new branch                                 │                     │
│ C                  Create and checkout branch                        │                     │
│ o                  Checkout branch                                   │                     │
│ O                  Create and checkout branch                        │                     │
│ d                  Delete branch (safe)                              │                     │
│ D                  Force delete branch                               │                     │
│ r                  Rename branch                                      │                     │
│ m                  Merge branch                                       │                     │
│ R                  Rebase on branch                                   │                     │
│ p                  Push branch                                        │                     │
│ P                  Push all branches                                 │                     │
│ f                  Fetch from remote                                  │                     │
│ t                  Set upstream tracking                              │                     │
│ b                  Branch from selected                               │                     │
│                                                                                     │
│ STASH OPERATIONS                                                                   │
│ ─────────────────────────────────────────────────────────────────────────────────   │
│ z                  Show stash menu                                   │                     │
│ a                  Apply stash                                       │                     │
│ A                  Apply and drop stash                              │                     │
│ p                  Pop stash                                         │                     │
│ d                  Drop stash                                        │                     │
│ D                  Drop all stashes                                  │                     │
│                                                                                     │
│ SEARCH & FILTER                                                                   │
│ ─────────────────────────────────────────────────────────────────────────────────   │
│ /                  Search mode                                        │                     │
│ n                  Next search result                                 │                     │
│ N                  Previous search result                             │                     │
│ f                  Filter by pattern                                  │                     │
│ *                  Filter by status                                   │                     │
│                                                                                     │
│ COMMAND MODE (:)                                                                   │
│ ─────────────────────────────────────────────────────────────────────────────────   │
│ :w                  Save changes                                       │                     │
│ :q                  Quit                                               │                     │
│ :wq / :x            Save and quit                                     │                     │
│ :q!                 Quit without saving                               │                     │
│ :commit             Create commit                                     │                     │
│ :commit -m "msg"   Create commit with message                        │                     │
│ :amend              Amend last commit                                 │                     │
│ :push               Push current branch                               │                     │
│ :push --all         Push all branches                                 │                     │
│ :fetch              Fetch from remote                                  │                     │
│ :merge <branch>     Merge branch                                       │                     │
│ :rebase <branch>    Rebase on branch                                   │                     │
│ :branch <name>      Create branch                                      │                     │
│ :checkout <branch>  Checkout branch                                    │                     │
│ :status             Show git status                                    │                     │
│ :log                Show commit log                                    │                     │
│ :diff               Show unstaged changes                              │                     │
│ :diff --cached      Show staged changes                                │                     │
│ :help               Show help                                          │                     │
│ :keybindings        Show keybindings                                   │                     │
│ :config             Open configuration                                 │                     │
│ :toggle <option>    Toggle option                                     │                     │
│ :git <cmd>          Run git command                                    │                     │
│                                                                                     │
│ MODE SWITCHING                                                                     │
│ ─────────────────────────────────────────────────────────────────────────────────   │
│ i                  Enter insert mode                                   │                     │
│ v                  Enter visual mode                                   │                     │
│ V                  Enter visual line mode                              │                     │
│ Ctrl+v             Enter visual block mode                             │                     │
│ :                  Enter command mode                                  │                     │
│ /                  Enter search mode                                   │                     │
│ esc                Return to normal mode                               │                     │
│ Ctrl+c             Cancel / Return to normal mode                      │                     │
│                                                                                     │
│ UNDO/REDO                                                                        │
│ ─────────────────────────────────────────────────────────────────────────────────   │
│ u                  Undo last action                                    │                     │
│ Ctrl+r              Redo                                                │                     │
│                                                                                     │
│ HELP & INFO                                                                       │
│ ─────────────────────────────────────────────────────────────────────────────────   │
│ ?                  Toggle help panel                                   │                     │
│ :help               Show full help                                      │                     │
│ :keybindings        Show keybindings                                    │                     │
│ :version            Show version                                        │                     │
│ :about              Show about                                          │                     │
│                                                                                     │
│ QUIT & EXIT                                                                       │
│ ─────────────────────────────────────────────────────────────────────────────────   │
│ q                  Quit application                                     │                     │
│ Q                  Force quit (discard all)                            │                     │
│ :q                  Quit                                                │                     │
│ :q!                 Quit without saving                                 │                     │
│ :wq / :x            Save and quit                                       │                     │
│ ZZ                  Save and quit (vim style)                           │                     │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

---

## Document Information

- **Version**: 1.0
- **Created**: January 2025
- **Author**: openISL Development Team
- **Last Updated**: January 2025
- **Format**: ASCII Wireframes with Specifications
