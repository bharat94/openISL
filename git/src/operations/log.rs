use crate::command::run;
use crate::models::Commit;
use anyhow::{Context, Result};
use chrono::DateTime;
use std::path::Path;

const GIT_LOG_FORMAT: &str = "%H|%P|%an|%ae|%ad|%s";

pub fn get_commits(repo_path: &Path, max_count: Option<usize>) -> Result<Vec<Commit>> {
    let format_arg = format!("--format={}", GIT_LOG_FORMAT);
    let n_arg = max_count.map(|n| format!("-n{}", n));

    let mut args: Vec<&str> = vec!["log", "--all", "--date=iso", &format_arg];
    if let Some(ref n) = n_arg {
        args.push(n);
    }

    let output = run(&args, Some(repo_path))
        .with_context(|| format!("Failed to get git log from {}", repo_path.display()))?;

    parse_commits(&output)
}

fn parse_commits(output: &str) -> Result<Vec<Commit>> {
    let mut commits = Vec::new();
    let records: Vec<&str> = output.trim_end().split("\n").collect();

    for record in records {
        if record.trim().is_empty() {
            continue;
        }

        if let Some(commit) = parse_commit(record) {
            commits.push(commit);
        }
    }

    Ok(commits)
}

fn parse_commit(record: &str) -> Option<Commit> {
    let parts: Vec<&str> = record.splitn(7, '|').collect();

    if parts.len() < 6 {
        return None;
    }

    let hash = parts[0].to_string();
    let parent_hashes: Vec<String> = if !parts[1].is_empty() {
        parts[1].split(' ').map(|s| s.to_string()).collect()
    } else {
        vec![]
    };

    let author = parts[2].to_string();
    let email = parts[3].to_string();

    let date_str = parts[4]
        .replace(" +0000", "+00:00")
        .replace(" +0100", "+01:00")
        .replace(" +0200", "+02:00")
        .replace(" +0300", "+03:00")
        .replace(" +0400", "+04:00")
        .replace(" +0500", "+05:00")
        .replace(" +0530", "+05:30")
        .replace(" +0600", "+06:00")
        .replace(" +0700", "+07:00")
        .replace(" +0800", "+08:00")
        .replace(" +0900", "+09:00")
        .replace(" +1000", "+10:00")
        .replace(" -0000", "-00:00")
        .replace(" -0100", "-01:00")
        .replace(" -0200", "-02:00")
        .replace(" -0300", "-03:00")
        .replace(" -0400", "-04:00")
        .replace(" -0500", "-05:00")
        .replace(" -0530", "-05:30")
        .replace(" -0600", "-06:00")
        .replace(" -0700", "-07:00")
        .replace(" -0800", "-08:00")
        .replace(" -0900", "-09:00")
        .replace(" -1000", "-10:00")
        .replace(' ', "T");

    let date = DateTime::parse_from_rfc3339(&date_str)
        .map(|d| d.with_timezone(&chrono::Utc))
        .ok()?;

    let summary = parts[5].to_string();
    let message = if parts.len() > 6 && !parts[6].is_empty() {
        format!("{}\n\n{}", summary, parts[6])
    } else {
        summary.clone()
    };

    let short_hash = hash.chars().take(7).collect();

    Some(Commit {
        hash,
        short_hash,
        message,
        summary,
        author,
        email,
        date,
        parent_hashes,
        refs: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_commit() {
        let input = "abc123def456789|def456ghi789abc|john@example.com|john@example.com|2024-01-10T12:00:00+00:00|Initial commit|This is the body";
        let commits = parse_commits(input).unwrap();
        assert_eq!(commits.len(), 1);

        let commit = &commits[0];
        assert_eq!(commit.hash, "abc123def456789");
        assert_eq!(commit.short_hash, "abc123d");
        assert_eq!(commit.author, "john@example.com");
        assert_eq!(commit.email, "john@example.com");
        assert_eq!(commit.summary, "Initial commit");
        assert!(commit.message.contains("This is the body"));
    }

    #[test]
    fn test_parse_multiple_commits() {
        let input = "abc123|def456|jane@example.com|jane@example.com|2024-01-10T12:00:00+00:00|Second commit|\ndef456|abc123|john@example.com|john@example.com|2024-01-09T12:00:00+00:00|First commit|";
        let commits = parse_commits(input).unwrap();
        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].summary, "Second commit");
        assert_eq!(commits[1].summary, "First commit");
    }

    #[test]
    fn test_parse_commit_with_merge_parents() {
        let input = "abc123|def456 ghi789|jane@example.com|jane@example.com|2024-01-10T12:00:00+00:00|Merge branch|";
        let commits = parse_commits(input).unwrap();
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].parent_hashes.len(), 2);
    }

    #[test]
    fn test_parse_commit_with_no_body() {
        let input =
            "abc123||john@example.com|john@example.com|2024-01-10T12:00:00+00:00|Simple commit|";
        let commits = parse_commits(input).unwrap();
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].summary, "Simple commit");
        assert_eq!(commits[0].message, "Simple commit");
    }

    #[test]
    fn test_parse_commit_with_empty_parents() {
        let input =
            "abc123||john@example.com|john@example.com|2024-01-10T12:00:00+00:00|Root commit|";
        let commits = parse_commits(input).unwrap();
        assert_eq!(commits.len(), 1);
        assert!(commits[0].parent_hashes.is_empty());
    }

    #[test]
    fn test_parse_empty_output() {
        let commits = parse_commits("").unwrap();
        assert!(commits.is_empty());
    }

    #[test]
    fn test_parse_whitespace_only_output() {
        let commits = parse_commits("   \n\n   ").unwrap();
        assert!(commits.is_empty());
    }
}
