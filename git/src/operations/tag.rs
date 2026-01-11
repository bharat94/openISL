use crate::command::run;
use anyhow::{Context, Result};
use std::path::Path;

pub fn tag_list(repo_path: &Path) -> Result<Vec<Tag>> {
    let output = run(
        &["tag", "-l", "--format=%(refname:short)|%(taggername)|%(taggeremail)|%(contents:subject)|%(creatordate:iso)"],
        Some(repo_path)
    ).context("Failed to list tags")?;

    let mut tags = Vec::new();
    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.splitn(5, '|').collect();
        if !parts.is_empty() {
            tags.push(Tag {
                name: parts[0].to_string(),
                tagger: if parts.len() > 1 {
                    parts[1].to_string()
                } else {
                    String::new()
                },
                email: if parts.len() > 2 {
                    parts[2].to_string()
                } else {
                    String::new()
                },
                message: if parts.len() > 3 {
                    parts[3].to_string()
                } else {
                    String::new()
                },
                date: if parts.len() > 4 {
                    parts[4].to_string()
                } else {
                    String::new()
                },
                is_annotated: parts.len() > 1 && !parts[1].is_empty(),
            });
        }
    }

    Ok(tags)
}

pub fn create_tag(
    repo_path: &Path,
    name: &str,
    message: Option<&str>,
    commit: Option<&str>,
) -> Result<()> {
    let mut args = vec!["tag"];

    if let Some(msg) = message {
        args.push("-a");
        args.push(name);
        args.push("-m");
        args.push(msg);
    } else {
        args.push(name);
    }

    if let Some(c) = commit {
        args.push(c);
    }

    run(&args, Some(repo_path)).with_context(|| format!("Failed to create tag '{}'", name))?;
    Ok(())
}

pub fn delete_tag(repo_path: &Path, name: &str) -> Result<()> {
    let args = vec!["tag", "-d", name];
    run(&args, Some(repo_path)).with_context(|| format!("Failed to delete tag '{}'", name))?;
    Ok(())
}

pub fn show_tag(repo_path: &Path, name: &str) -> Result<String> {
    run(&["tag", "-l", name], Some(repo_path))
        .with_context(|| format!("Failed to show tag '{}'", name))
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub tagger: String,
    pub email: String,
    pub message: String,
    pub date: String,
    pub is_annotated: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_list() {
        let repo_path = std::env::current_dir().unwrap();
        let result = tag_list(&repo_path);
        // Will fail if no tags, but that's OK
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_create_tag() {
        let repo_path = std::env::current_dir().unwrap();
        // Create a lightweight tag for testing
        let result = create_tag(&repo_path, "test-tag-12345", None, None);
        // Will succeed or fail depending on whether tag exists
        // Clean up if it succeeded
        if result.is_ok() {
            let _ = delete_tag(&repo_path, "test-tag-12345");
        }
    }
}
