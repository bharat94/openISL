use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use openisl_git::{
    add_paths, amend_commit, apply_patch, bisect_bad, bisect_good, bisect_reset, bisect_skip,
    bisect_start, checkout, cherry_pick_commit, clone, commit, create_branch, create_tag,
    delete_tag, fetch, get_blame, get_branches, get_commit_diff, get_commits, get_commits_filtered,
    get_conflicted_files, get_current_branch, get_diff, get_file_at_revision, get_stash_list,
    get_status, init, mark_resolved, merge, move_file, pull, push, rebase, remote_add, remote_list,
    remote_remove, remove_file, reset, revert_commit, squash_commits, stage_all, stash_apply,
    stash_drop, stash_pop, stash_push, tag_list, undo_last, ResetMode, SmartLogFormatter,
    StatusType,
};
mod config;
use config::Config;

#[derive(Parser)]
#[command(name = "openisl")]
#[command(author = "Bharat <bharat@example.com>")]
#[command(version = "0.1.0")]
#[command(about = "Interactive Smart Log - Smart git operations", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Show commit log")]
    Log {
        #[arg(long, help = "Show as ASCII in terminal")]
        simple: bool,
        #[arg(short, long, help = "Show commits from specific branch")]
        branch: Option<String>,
        #[arg(long, help = "Include remote branches")]
        remote: bool,
        #[arg(short = 'n', long, help = "Maximum number of commits to show")]
        max_count: Option<usize>,
    },

    #[command(about = "Launch interactive TUI for commit history")]
    Tui,

    #[command(about = "Initialize a new repository")]
    Init,

    #[command(about = "Clone a remote repository")]
    Clone {
        #[arg(help = "Remote repository URL or path")]
        url: String,
        #[arg(help = "Destination directory (defaults to repo name)")]
        destination: Option<String>,
    },

    #[command(about = "List, create, or delete branches")]
    Branch {
        #[arg(help = "Create a new branch with this name")]
        name: Option<String>,
        #[arg(long, help = "Show remote branches only")]
        remote: bool,
        #[arg(long, help = "Show all branches including remotes")]
        all: bool,
    },

    #[command(about = "Checkout a branch or commit")]
    Checkout {
        #[arg(help = "Branch name or commit hash to checkout")]
        target: String,
    },

    #[command(about = "Stage files (add to the index)")]
    Add {
        #[arg(help = "Paths to stage")]
        paths: Vec<String>,
        #[arg(short = 'A', long, help = "Stage all changes (new, modified, deleted)")]
        all: bool,
    },

    #[command(about = "Remove a tracked file")]
    Rm {
        #[arg(help = "Path to remove")]
        path: String,
    },

    #[command(about = "Move (rename) a tracked file")]
    Mv {
        #[arg(help = "Source path")]
        from: String,
        #[arg(help = "Destination path")]
        to: String,
    },

    #[command(about = "Create a commit from staged changes")]
    Commit {
        #[arg(short = 'm', long, help = "Commit message")]
        message: Option<String>,
        #[arg(long, help = "Amend the last commit instead")]
        amend: bool,
    },

    #[command(about = "Show a commit and its changes")]
    Show {
        #[arg(help = "Commit hash or revision to show")]
        commit: String,
    },

    #[command(about = "Show working tree status")]
    Status,

    #[command(about = "Show changes between commits")]
    Diff {
        #[arg(long, help = "Show staged changes")]
        staged: bool,
        #[arg(help = "Show changes for specific commit")]
        commit: Option<String>,
    },

    #[command(about = "Merge a branch or commit into the current branch")]
    Merge {
        #[arg(help = "Branch or commit to merge")]
        target: String,
    },

    #[command(about = "Rebase the current branch onto an upstream")]
    Rebase {
        #[arg(help = "Upstream branch or revision")]
        upstream: Option<String>,
        #[arg(short = 'i', long, help = "Interactive rebase")]
        interactive: bool,
    },

    #[command(about = "Reset the current branch to a revision")]
    Reset {
        #[arg(long, help = "Move HEAD, index, and working tree (discard changes)")]
        hard: bool,
        #[arg(long, help = "Move HEAD only, keep changes")]
        soft: bool,
        #[arg(help = "Revision to reset to (defaults to HEAD)")]
        target: Option<String>,
    },

    #[command(about = "Cherry-pick a commit onto the current branch")]
    CherryPick {
        #[arg(help = "Commit to cherry-pick")]
        commit: String,
    },

    #[command(about = "Revert a commit")]
    Revert {
        #[arg(help = "Commit to revert")]
        commit: String,
    },

    #[command(about = "Stash uncommitted changes")]
    Stash {
        #[command(subcommand)]
        action: StashAction,
    },

    #[command(about = "Fetch from a remote")]
    Fetch {
        #[arg(help = "Remote to fetch from (defaults to origin)")]
        remote: Option<String>,
        #[arg(long, help = "Prune deleted remote branches")]
        prune: bool,
    },

    #[command(about = "Fetch and merge remote changes")]
    Pull {
        #[arg(long, help = "Rebase instead of merge")]
        rebase: bool,
    },

    #[command(about = "Push commits to a remote")]
    Push {
        #[arg(help = "Remote to push to (defaults to origin)")]
        remote: Option<String>,
        #[arg(help = "Branch to push (defaults to current)")]
        branch: Option<String>,
        #[arg(long, help = "Also push tags")]
        tags: bool,
        #[arg(long, help = "Set upstream tracking")]
        set_upstream: bool,
    },

    #[command(about = "Annotate a file with the commits that last touched each line")]
    Blame {
        #[arg(help = "Path to annotate")]
        path: String,
    },

    #[command(about = "Configure openisl settings")]
    Config {
        #[arg(long, help = "Show current configuration")]
        show: bool,
        #[arg(long, help = "Reset configuration to defaults")]
        reset: bool,
        #[arg(long, help = "Set theme (dark/light)")]
        theme: Option<String>,
        #[arg(long, help = "Set max commits")]
        max_commits: Option<usize>,
    },

    #[command(about = "Manage git remotes")]
    Remote {
        #[arg(long, help = "List all remotes")]
        list: bool,
        #[arg(help = "Remote name to add")]
        name: Option<String>,
        #[arg(help = "Remote URL (used with add)")]
        url: Option<String>,
        #[arg(long, help = "Remove a remote by name")]
        remove: Option<String>,
    },

    #[command(about = "Manage git tags")]
    Tag {
        #[arg(long, help = "List all tags")]
        list: bool,
        #[arg(help = "Create a tag")]
        create: Option<String>,
        #[arg(long, help = "Delete a tag")]
        delete: Option<String>,
        #[arg(short, long, help = "Tag message for annotated tag")]
        message: Option<String>,
    },

    #[command(about = "Print file contents at a revision")]
    Cat {
        #[arg(help = "Revision, e.g. HEAD or abc1234")]
        revision: String,
        #[arg(help = "Path to print")]
        path: String,
    },

    #[command(about = "Apply a patch file")]
    Apply {
        #[arg(help = "Patch file to apply")]
        patch: String,
        #[arg(long, help = "Apply to the index instead of the working tree")]
        cached: bool,
    },

    #[command(about = "Find the commit that introduced a bug")]
    Bisect {
        #[command(subcommand)]
        action: BisectAction,
    },

    #[command(about = "List or resolve merge conflicts")]
    Resolve {
        #[arg(long, help = "List conflicted files")]
        list: bool,
        #[arg(help = "Paths to mark as resolved")]
        paths: Vec<String>,
    },

    #[command(about = "Undo the last operation (destructive)")]
    Undo,

    #[command(about = "Squash commits up to a revision into one")]
    Squash {
        #[arg(help = "Commit to squash up to (inclusive)")]
        commit: String,
        #[arg(short, long, help = "Message for the squashed commit")]
        message: String,
    },
}

#[derive(Subcommand)]
enum BisectAction {
    #[command(about = "Start a bisect session")]
    Start {
        #[arg(help = "Known-bad revision")]
        bad: String,
        #[arg(help = "Known-good revision")]
        good: String,
    },
    #[command(about = "Mark current revision as good")]
    Good {
        #[arg(help = "Optional revision to mark")]
        revision: Option<String>,
    },
    #[command(about = "Mark current revision as bad")]
    Bad {
        #[arg(help = "Optional revision to mark")]
        revision: Option<String>,
    },
    #[command(about = "Skip the current revision")]
    Skip,
    #[command(about = "End the bisect session")]
    Reset,
}

#[derive(Subcommand)]
enum StashAction {
    #[command(about = "List stashes")]
    List,
    #[command(about = "Create a stash")]
    Push {
        #[arg(short = 'm', long, help = "Stash message")]
        message: Option<String>,
    },
    #[command(about = "Apply and remove the newest stash")]
    Pop {
        #[arg(help = "Stash reference, e.g. stash@{0}")]
        stash: Option<String>,
    },
    #[command(about = "Apply a stash without removing it")]
    Apply {
        #[arg(help = "Stash reference, e.g. stash@{0}")]
        stash: Option<String>,
    },
    #[command(about = "Drop a stash")]
    Drop {
        #[arg(help = "Stash reference, e.g. stash@{0}")]
        stash: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Log {
            simple,
            branch,
            remote,
            max_count,
        } => {
            cmd_log(*simple, branch.as_deref(), *remote, *max_count)?;
        }
        Commands::Tui => {
            cmd_tui()?;
        }
        Commands::Init => {
            cmd_init()?;
        }
        Commands::Clone { url, destination } => {
            cmd_clone(url, destination.as_deref())?;
        }
        Commands::Branch { name, remote, all } => {
            cmd_branch(name.as_deref(), *remote, *all)?;
        }
        Commands::Checkout { target } => {
            cmd_checkout(target)?;
        }
        Commands::Add { paths, all } => {
            cmd_add(paths, *all)?;
        }
        Commands::Rm { path } => {
            cmd_rm(path)?;
        }
        Commands::Mv { from, to } => {
            cmd_mv(from, to)?;
        }
        Commands::Commit { message, amend } => {
            cmd_commit(message.as_deref(), *amend)?;
        }
        Commands::Show { commit } => {
            cmd_show(commit)?;
        }
        Commands::Status => {
            cmd_status()?;
        }
        Commands::Diff { staged, commit } => {
            cmd_diff(*staged, commit.as_deref())?;
        }
        Commands::Merge { target } => {
            cmd_merge(target)?;
        }
        Commands::Rebase {
            upstream,
            interactive,
        } => {
            cmd_rebase(upstream.as_deref(), *interactive)?;
        }
        Commands::Reset { hard, soft, target } => {
            cmd_reset(*hard, *soft, target.as_deref())?;
        }
        Commands::CherryPick { commit } => {
            cmd_cherry_pick(commit)?;
        }
        Commands::Revert { commit } => {
            cmd_revert(commit)?;
        }
        Commands::Stash { action } => {
            cmd_stash(action)?;
        }
        Commands::Fetch { remote, prune } => {
            cmd_fetch(remote.as_deref(), *prune)?;
        }
        Commands::Pull { rebase } => {
            cmd_pull(*rebase)?;
        }
        Commands::Push {
            remote,
            branch,
            tags,
            set_upstream,
        } => {
            cmd_push(remote.as_deref(), branch.as_deref(), *tags, *set_upstream)?;
        }
        Commands::Blame { path } => {
            cmd_blame(path)?;
        }
        Commands::Config {
            show,
            reset,
            theme,
            max_commits,
        } => {
            cmd_config(*show, *reset, theme.as_deref(), *max_commits)?;
        }
        Commands::Remote {
            list,
            name,
            url,
            remove,
        } => {
            cmd_remote(*list, name.as_deref(), url.as_deref(), remove.as_deref())?;
        }
        Commands::Tag {
            list,
            create,
            delete,
            message,
        } => {
            cmd_tag(
                *list,
                create.as_deref(),
                delete.as_deref(),
                message.as_deref(),
            )?;
        }
        Commands::Cat { revision, path } => {
            cmd_cat(revision, path)?;
        }
        Commands::Apply { patch, cached } => {
            cmd_apply(patch, *cached)?;
        }
        Commands::Bisect { action } => {
            cmd_bisect(action)?;
        }
        Commands::Resolve { list, paths } => {
            cmd_resolve(*list, paths)?;
        }
        Commands::Undo => {
            cmd_undo()?;
        }
        Commands::Squash { commit, message } => {
            cmd_squash(commit, message)?;
        }
    }

    Ok(())
}

fn cmd_log(
    simple: bool,
    branch: Option<&str>,
    remote: bool,
    max_count: Option<usize>,
) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    let commits = get_commits_filtered(&repo_path, max_count, branch, remote)?;

    if simple {
        let formatter = SmartLogFormatter::new(commits, 80);
        print!("{}", formatter.format());
    } else {
        println!("Commit Log ({} commits):\n", commits.len());

        for commit in commits {
            println!("{} - {}", commit.short_hash, commit.summary);
            println!("  Author: {} <{}>", commit.author, commit.email);
            println!("  Date:   {}\n", commit.date);
        }
    }

    Ok(())
}

fn cmd_tui() -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    let config = Config::load().unwrap_or_default();
    let commits = get_commits(&repo_path, Some(config.general.max_commits))?;
    let current_branch = get_current_branch(&repo_path)?.unwrap_or_else(|| "main".to_string());
    openisl_tui::run_tui(commits, current_branch, Some(repo_path))
}

fn cmd_init() -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    init(&repo_path)?;
    println!(
        "Initialized empty Git repository in {}",
        repo_path.display()
    );
    Ok(())
}

fn cmd_clone(url: &str, destination: Option<&str>) -> Result<()> {
    let dest = destination.unwrap_or_else(|| {
        url.rsplit(['/', ':'])
            .next()
            .unwrap_or("repo")
            .trim_end_matches(".git")
    });
    clone(url, dest)?;
    println!("Cloned '{}' into '{}'", url, dest);
    Ok(())
}

fn cmd_branch(name: Option<&str>, remote: bool, all: bool) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    if let Some(branch_name) = name {
        create_branch(&repo_path, branch_name)?;
        println!("Created branch: {}", branch_name);
    } else {
        let branches = get_branches(&repo_path)?;
        let current = get_current_branch(&repo_path)?;

        let filtered_branches: Vec<_> = branches
            .iter()
            .filter(|b| {
                if remote && !all {
                    b.ref_type == openisl_git::RefType::Remote
                } else if all {
                    true
                } else {
                    b.ref_type != openisl_git::RefType::Remote
                }
            })
            .collect();

        println!("Branches:");
        for git_ref in &filtered_branches {
            let prefix = if current.as_ref() == Some(&git_ref.name) {
                "* "
            } else {
                "  "
            };
            println!("{}{}", prefix, git_ref.name);
        }
    }

    Ok(())
}

fn cmd_checkout(target: &str) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    checkout(&repo_path, target)?;
    println!("Checked out '{}'", target);
    Ok(())
}

fn cmd_add(paths: &[String], all: bool) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    if all {
        stage_all(&repo_path)?;
        println!("Staged all changes");
    } else if paths.is_empty() {
        anyhow::bail!("No paths given. Use `openisl add <paths...>` or `openisl add -A`.");
    } else {
        let path_refs: Vec<&str> = paths.iter().map(String::as_str).collect();
        add_paths(&repo_path, &path_refs)?;
        println!("Staged {} path(s)", paths.len());
    }

    Ok(())
}

fn cmd_rm(path: &str) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    remove_file(&repo_path, path)?;
    println!("Removed '{}'", path);
    Ok(())
}

fn cmd_mv(from: &str, to: &str) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    move_file(&repo_path, from, to)?;
    println!("Moved '{}' to '{}'", from, to);
    Ok(())
}

fn cmd_commit(message: Option<&str>, amend: bool) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    if amend {
        amend_commit(&repo_path, message)?;
        println!(
            "Amended commit{}",
            message.map(|_| " with new message").unwrap_or("")
        );
    } else {
        let message = message.context("A commit message is required: use -m <message>")?;
        commit(&repo_path, message)?;
        println!("Created commit");
    }

    Ok(())
}

fn cmd_show(commit: &str) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    print!("{}", get_commit_diff(&repo_path, commit)?);
    Ok(())
}

fn cmd_status() -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    let files = get_status(&repo_path)?;

    if files.is_empty() {
        println!("Working tree is clean");
    } else {
        println!("Changes:");
        for file in files {
            let status_str = match file.status {
                StatusType::Modified => "Modified",
                StatusType::Added => "Added",
                StatusType::Deleted => "Deleted",
                StatusType::Untracked => "Untracked",
                StatusType::ModifiedStaged => "Modified (staged)",
                StatusType::AddedStaged => "Added (staged)",
                StatusType::DeletedStaged => "Deleted (staged)",
                StatusType::Renamed => "Renamed",
                StatusType::Conflicted => "Conflicted",
            };
            println!("{}: {}", status_str, file.path);
        }
    }

    Ok(())
}

fn cmd_diff(staged: bool, commit: Option<&str>) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    let diff = get_diff(&repo_path, commit, staged)?;

    if diff.is_empty() {
        println!("No changes");
    } else {
        print!("{}", diff);
    }

    Ok(())
}

fn cmd_merge(target: &str) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    let output = merge(&repo_path, target, false)?;
    print!("{}", output);
    Ok(())
}

fn cmd_rebase(upstream: Option<&str>, interactive: bool) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    let output = rebase(&repo_path, upstream, interactive)?;
    print!("{}", output);
    Ok(())
}

fn cmd_reset(hard: bool, soft: bool, target: Option<&str>) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    let mode = if hard {
        ResetMode::Hard
    } else if soft {
        ResetMode::Soft
    } else {
        ResetMode::Mixed
    };
    let output = reset(&repo_path, mode, target)?;
    print!("{}", output);
    Ok(())
}

fn cmd_cherry_pick(commit: &str) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    cherry_pick_commit(&repo_path, commit)?;
    println!("Cherry-picked '{}'", commit);
    Ok(())
}

fn cmd_revert(commit: &str) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    revert_commit(&repo_path, commit)?;
    println!("Reverted '{}'", commit);
    Ok(())
}

fn cmd_stash(action: &StashAction) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    match action {
        StashAction::List => {
            let stashes = get_stash_list(&repo_path)?;
            if stashes.is_empty() {
                println!("No stashes found");
            } else {
                for stash in &stashes {
                    println!("{} {} ({})", stash.name, stash.message, stash.hash);
                }
            }
        }
        StashAction::Push { message } => {
            stash_push(&repo_path, message.as_deref())?;
            println!("Created stash");
        }
        StashAction::Pop { stash } => {
            stash_pop(&repo_path, stash.as_deref())?;
            println!("Popped stash");
        }
        StashAction::Apply { stash } => {
            stash_apply(&repo_path, stash.as_deref())?;
            println!("Applied stash");
        }
        StashAction::Drop { stash } => {
            stash_drop(&repo_path, stash.as_deref())?;
            println!("Dropped stash");
        }
    }

    Ok(())
}

fn cmd_fetch(remote: Option<&str>, prune: bool) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    fetch(&repo_path, remote, prune)?;
    println!("Fetched from '{}'", remote.unwrap_or("origin"));
    Ok(())
}

fn cmd_pull(rebase: bool) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    print!("{}", pull(&repo_path, rebase)?);
    Ok(())
}

fn cmd_push(
    remote: Option<&str>,
    branch: Option<&str>,
    tags: bool,
    set_upstream: bool,
) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    push(&repo_path, remote, branch, tags, set_upstream)?;
    println!(
        "Pushed to '{}'{}",
        remote.unwrap_or("origin"),
        branch.map(|b| format!(" ({})", b)).unwrap_or_default()
    );
    Ok(())
}

fn cmd_blame(path: &str) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    print!("{}", get_blame(&repo_path, path)?);
    Ok(())
}

fn cmd_config(
    show: bool,
    reset: bool,
    theme: Option<&str>,
    max_commits: Option<usize>,
) -> Result<()> {
    if reset {
        let config = Config::default();
        config.save()?;
        println!("Configuration reset to defaults.");
        return Ok(());
    }

    let mut config = Config::load().unwrap_or_default();

    if let Some(t) = theme {
        if t == "dark" || t == "light" {
            config.tui.theme = t.to_string();
            println!("Theme set to: {}", t);
        } else {
            println!("Invalid theme. Use 'dark' or 'light'.");
        }
    }

    if let Some(n) = max_commits {
        config.general.max_commits = n;
        println!("Max commits set to: {}", n);
    }

    if show || (theme.is_none() && max_commits.is_none()) {
        println!("Current Configuration:");
        println!("  Theme: {}", config.tui.theme);
        println!("  Max Commits: {}", config.general.max_commits);
        println!("  Date Format: {}", config.general.date_format);
        println!("  Auto Fetch: {}", config.git.auto_fetch);
    }

    config.save()?;
    Ok(())
}

fn cmd_remote(
    list: bool,
    name: Option<&str>,
    url: Option<&str>,
    remove: Option<&str>,
) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    if list {
        let remotes = remote_list(&repo_path)?;
        if remotes.is_empty() {
            println!("No remotes configured");
        } else {
            for remote in remotes {
                if remote.fetch_type.trim().is_empty() {
                    println!("{}  {}", remote.name, remote.url);
                } else {
                    println!(
                        "{}  {}  {}",
                        remote.name,
                        remote.url,
                        remote.fetch_type.trim()
                    );
                }
            }
        }
    } else if let (Some(name), Some(url)) = (name, url) {
        remote_add(&repo_path, name, url)?;
        println!("Added remote '{}' -> {}", name, url);
    } else if let Some(name) = remove {
        remote_remove(&repo_path, name)?;
        println!("Removed remote '{}'", name);
    }

    Ok(())
}

fn cmd_tag(
    list: bool,
    create: Option<&str>,
    delete: Option<&str>,
    message: Option<&str>,
) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    if list {
        let tags = tag_list(&repo_path)?;
        if tags.is_empty() {
            println!("No tags found");
        } else {
            for tag in tags {
                println!("{}", tag.name);
            }
        }
    } else if let Some(name) = create {
        create_tag(&repo_path, name, message, None)?;
        println!("Created tag '{}'", name);
    } else if let Some(name) = delete {
        delete_tag(&repo_path, name)?;
        println!("Deleted tag '{}'", name);
    }

    Ok(())
}

fn cmd_cat(revision: &str, path: &str) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    print!("{}", get_file_at_revision(&repo_path, revision, path)?);
    Ok(())
}

fn cmd_apply(patch: &str, cached: bool) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    apply_patch(&repo_path, patch, cached)?;
    println!(
        "Applied '{}'{}",
        patch,
        if cached { " to index" } else { "" }
    );
    Ok(())
}

fn cmd_bisect(action: &BisectAction) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    match action {
        BisectAction::Start { bad, good } => {
            let log = bisect_start(&repo_path, bad, good)?;
            print!("{}", log);
            println!("\nBisecting: check out the listed revision, test it, then mark it good/bad.");
        }
        BisectAction::Good { revision } => {
            print!("{}", bisect_good(&repo_path, revision.as_deref())?);
        }
        BisectAction::Bad { revision } => {
            print!("{}", bisect_bad(&repo_path, revision.as_deref())?);
        }
        BisectAction::Skip => {
            print!("{}", bisect_skip(&repo_path)?);
        }
        BisectAction::Reset => {
            bisect_reset(&repo_path)?;
            println!("Bisect session ended.");
        }
    }

    Ok(())
}

fn cmd_resolve(list: bool, paths: &[String]) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    if list {
        let conflicted = get_conflicted_files(&repo_path)?;
        if conflicted.is_empty() {
            println!("No conflicted files");
        } else {
            for path in conflicted {
                println!("{}", path);
            }
        }
    } else if paths.is_empty() {
        anyhow::bail!(
            "No paths given. Use `openisl resolve <paths...>` or `openisl resolve --list`."
        );
    } else {
        let path_refs: Vec<&str> = paths.iter().map(String::as_str).collect();
        mark_resolved(&repo_path, &path_refs)?;
        println!("Marked {} file(s) as resolved", paths.len());
    }

    Ok(())
}

fn cmd_undo() -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    undo_last(&repo_path)?;
    println!("Undid the last operation.");
    Ok(())
}

fn cmd_squash(commit: &str, message: &str) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;
    squash_commits(&repo_path, commit, message)?;
    println!("Squashed commits up to '{}' into one", commit);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_log() {
        let args = vec!["openisl", "log", "--max-count", "10"];
        let cli = Cli::parse_from(&args);
        match &cli.command {
            Commands::Log {
                simple: _,
                branch: _,
                remote: _,
                max_count,
            } => {
                assert_eq!(*max_count, Some(10));
            }
            _ => panic!("Expected Log command"),
        }
    }

    #[test]
    fn test_cli_parse_branch() {
        let args = vec!["openisl", "branch", "feature/new"];
        let cli = Cli::parse_from(&args);
        match &cli.command {
            Commands::Branch { name, .. } => {
                assert_eq!(name.as_ref().unwrap(), "feature/new");
            }
            _ => panic!("Expected Branch command"),
        }
    }

    #[test]
    fn test_cli_parse_remote() {
        let args = vec!["openisl", "remote", "--list"];
        let cli = Cli::parse_from(&args);
        match &cli.command {
            Commands::Remote { list, .. } => {
                assert!(*list);
            }
            _ => panic!("Expected Remote command"),
        }
    }

    #[test]
    fn test_cli_parse_remote_add() {
        let args = vec![
            "openisl",
            "remote",
            "origin",
            "https://github.com/bharat94/openISL.git",
        ];
        let cli = Cli::parse_from(&args);
        match &cli.command {
            Commands::Remote { name, url, .. } => {
                assert_eq!(name.as_deref(), Some("origin"));
                assert_eq!(
                    url.as_deref(),
                    Some("https://github.com/bharat94/openISL.git")
                );
            }
            _ => panic!("Expected Remote command"),
        }
    }

    #[test]
    fn test_cli_parse_tag() {
        let args = vec!["openisl", "tag", "--list"];
        let cli = Cli::parse_from(&args);
        match &cli.command {
            Commands::Tag { list, .. } => {
                assert!(*list);
            }
            _ => panic!("Expected Tag command"),
        }
    }

    #[test]
    fn test_cli_parse_commit() {
        let args = vec!["openisl", "commit", "-m", "fix: bug"];
        let cli = Cli::parse_from(&args);
        match &cli.command {
            Commands::Commit { message, amend } => {
                assert_eq!(message.as_deref(), Some("fix: bug"));
                assert!(!amend);
            }
            _ => panic!("Expected Commit command"),
        }
    }

    #[test]
    fn test_cli_parse_stash_push() {
        let args = vec!["openisl", "stash", "push", "-m", "wip"];
        let cli = Cli::parse_from(&args);
        match &cli.command {
            Commands::Stash { action } => match action {
                StashAction::Push { message } => {
                    assert_eq!(message.as_deref(), Some("wip"));
                }
                _ => panic!("Expected stash push"),
            },
            _ => panic!("Expected Stash command"),
        }
    }

    #[test]
    fn test_cli_parse_add() {
        let args = vec!["openisl", "add", "src/main.rs", "src/lib.rs"];
        let cli = Cli::parse_from(&args);
        match &cli.command {
            Commands::Add { paths, all } => {
                assert_eq!(paths.len(), 2);
                assert!(!all);
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_cli_parse_cat() {
        let args = vec!["openisl", "cat", "HEAD", "src/main.rs"];
        let cli = Cli::parse_from(&args);
        match &cli.command {
            Commands::Cat { revision, path } => {
                assert_eq!(revision, "HEAD");
                assert_eq!(path, "src/main.rs");
            }
            _ => panic!("Expected Cat command"),
        }
    }

    #[test]
    fn test_cli_parse_bisect_start() {
        let args = vec!["openisl", "bisect", "start", "badhash", "goodhash"];
        let cli = Cli::parse_from(&args);
        match &cli.command {
            Commands::Bisect { action } => match action {
                BisectAction::Start { bad, good } => {
                    assert_eq!(bad, "badhash");
                    assert_eq!(good, "goodhash");
                }
                _ => panic!("Expected bisect start"),
            },
            _ => panic!("Expected Bisect command"),
        }
    }

    #[test]
    fn test_cli_parse_resolve() {
        let args = vec!["openisl", "resolve", "a.txt"];
        let cli = Cli::parse_from(&args);
        match &cli.command {
            Commands::Resolve { list, paths } => {
                assert!(!list);
                assert_eq!(paths.len(), 1);
            }
            _ => panic!("Expected Resolve command"),
        }
    }
}
