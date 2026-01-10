use anyhow::Context;
use openisl_git::get_commits;
use openisl_tui::run_tui;

fn main() -> anyhow::Result<()> {
    let repo_path = std::env::current_dir().context("Not in a directory")?;

    let commits = get_commits(&repo_path, Some(100))?;
    let current_branch = "main".to_string();

    run_tui(commits, current_branch, Some(repo_path))?;

    Ok(())
}
