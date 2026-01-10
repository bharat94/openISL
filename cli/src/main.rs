use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use openisl_git::{get_commits, get_branches, get_current_branch, get_status, get_diff, StatusType, SmartLogFormatter, remote_list, tag_list, remote_remove, create_tag, delete_tag};
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
        #[arg(short, long, help = "Maximum number of commits to show")]
        max_count: Option<usize>,
    },

    #[command(about = "Launch interactive TUI for commit history")]
    Tui,

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

    #[command(about = "Show working tree status")]
    Status,

    #[command(about = "Show changes between commits")]
    Diff {
        #[arg(long, help = "Show staged changes")]
        staged: bool,
        #[arg(help = "Show changes for specific commit")]
        commit: Option<String>,
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
        #[arg(help = "Add a remote")]
        add: Option<String>,
        #[arg(help = "Remove a remote")]
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Log { simple, branch, remote, max_count } => {
            cmd_log(*simple, branch.as_deref(), *remote, *max_count)?;
        }
        Commands::Tui => {
            println!("Launching TUI... (Run 'cargo run -p openisl-tui' to use TUI)");
        }
        Commands::Branch { name, remote, all } => {
            cmd_branch(name.as_deref(), *remote, *all)?;
        }
        Commands::Checkout { target } => {
            cmd_checkout(target)?;
        }
        Commands::Status => {
            cmd_status()?;
        }
        Commands::Diff { staged, commit } => {
            cmd_diff(*staged, commit.as_deref())?;
        }
        Commands::Config { show, reset, theme, max_commits } => {
            cmd_config(*show, *reset, theme.as_deref(), *max_commits)?;
        }
        Commands::Remote { list, add, remove } => {
            cmd_remote(*list, add.as_deref(), remove.as_deref())?;
        }
        Commands::Tag { list, create, delete, message } => {
            cmd_tag(*list, create.as_deref(), delete.as_deref(), message.as_deref())?;
        }
    }

    Ok(())
}

fn cmd_log(simple: bool, _branch: Option<&str>, _remote: bool, max_count: Option<usize>) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    let commits = get_commits(&repo_path, max_count)?;

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

fn cmd_branch(name: Option<&str>, remote: bool, all: bool) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    if let Some(branch_name) = name {
        println!("Creating branch: {}", branch_name);
    } else {
        let branches = get_branches(&repo_path)?;
        let current = get_current_branch(&repo_path)?;

        let filtered_branches: Vec<_> = branches.iter().filter(|b| {
            if remote && !all {
                b.ref_type == openisl_git::RefType::Remote
            } else if all {
                true
            } else {
                b.ref_type != openisl_git::RefType::Remote
            }
        }).collect();

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
    println!("Would checkout: {}", target);
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

fn cmd_diff(_staged: bool, _commit: Option<&str>) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    let diff = get_diff(&repo_path, None, false)?;

    if diff.is_empty() {
        println!("No changes");
    } else {
        print!("{}", diff);
    }

    Ok(())
}

fn cmd_config(show: bool, reset: bool, theme: Option<&str>, max_commits: Option<usize>) -> Result<()> {
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

    if show || (!theme.is_some() && max_commits.is_none()) {
        println!("Current Configuration:");
        println!("  Theme: {}", config.tui.theme);
        println!("  Max Commits: {}", config.general.max_commits);
        println!("  Date Format: {}", config.general.date_format);
        println!("  Auto Fetch: {}", config.git.auto_fetch);
    }

    config.save()?;
    Ok(())
}

fn cmd_remote(list: bool, add: Option<&str>, remove: Option<&str>) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    if list {
        let remotes = remote_list(&repo_path)?;
        if remotes.is_empty() {
            println!("No remotes configured");
        } else {
            for remote in remotes {
                println!("{}  {} ({})", remote.name, remote.url, remote.fetch_type.trim());
            }
        }
    } else if let Some(_name) = add {
        println!("Use 'openisl remote add <name> <url>' - URL argument needed");
    } else if let Some(name) = remove {
        remote_remove(&repo_path, name)?;
        println!("Removed remote '{}'", name);
    }

    Ok(())
}

fn cmd_tag(list: bool, create: Option<&str>, delete: Option<&str>, message: Option<&str>) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_log() {
        let args = vec!["openisl", "log", "--simple", "-n", "10"];
        let cli = Cli::parse_from(&args);
        match &cli.command {
            Commands::Log { simple, all: _, no_remote: _, max_count } => {
                assert!(*simple);
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
            Commands::Branch { name } => {
                assert_eq!(name.as_ref().unwrap(), "feature/new");
            }
            _ => panic!("Expected Branch command"),
        }
    }

    #[test]
    fn test_cli_parse_help() {
        let args = vec!["openisl", "help"];
        let cli = Cli::parse_from(&args);
        match &cli.command {
            Commands::Help => {}
            _ => panic!("Expected Help command"),
        }
    }
}
