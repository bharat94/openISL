use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::Path;
use openisl_git::{get_commits, get_branches, get_current_branch, get_status, get_diff, StatusType, FileStatus, SmartLogFormatter};

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
        #[arg(long, help = "Show all branches")]
        all: bool,
        #[arg(long, help = "Hide remote branches")]
        no_remote: bool,
        #[arg(short, long, help = "Maximum number of commits to show")]
        max_count: Option<usize>,
    },

    #[command(about = "List, create, or delete branches")]
    Branch {
        #[arg(help = "Create a new branch with this name")]
        name: Option<String>,
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

    #[command(about = "Print help information")]
    Help,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Log { simple, all, no_remote, max_count } => {
            cmd_log(*simple, *all, *no_remote, *max_count)?;
        }
        Commands::Branch { name } => {
            cmd_branch(name.as_deref())?;
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
        Commands::Help => {
            println!("{}", Cli::command().render_help());
        }
    }

    Ok(())
}

fn cmd_log(simple: bool, _all: bool, _no_remote: bool, max_count: Option<usize>) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    let commits = get_commits(&repo_path, max_count)?;

    if simple {
        // Use simple ASCII format
        let formatter = SmartLogFormatter::new(commits, 80);
        print!("{}", formatter.format());
    } else {
        // Use detailed format
        println!("Commit Log ({} commits):\n", commits.len());

        for commit in commits {
            println!("{} - {}", commit.short_hash, commit.summary);
            println!("  Author: {} <{}>", commit.author, commit.email);
            println!("  Date:   {}\n", commit.date);
        }
    }

    Ok(())
}

fn cmd_branch(name: Option<&str>) -> Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    if let Some(branch_name) = name {
        // Create branch
        println!("Creating branch: {}", branch_name);
    } else {
        // List branches
        let branches = get_branches(&repo_path)?;
        let current = get_current_branch(&repo_path)?;

        println!("Branches:");
        for git_ref in &branches {
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
