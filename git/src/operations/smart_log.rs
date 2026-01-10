use crate::models::Commit;

pub struct SmartLogFormatter {
    commits: Vec<Commit>,
    width: usize,
}

impl SmartLogFormatter {
    pub fn new(commits: Vec<Commit>, width: usize) -> Self {
        Self { commits, width }
    }

    pub fn format(&self) -> String {
        if self.commits.is_empty() {
            return "No commits found".to_string();
        }

        let mut output = String::new();
        output.push_str(&format!("Smart Log ({} commits):\n\n", self.commits.len()));

        for (i, commit) in self.commits.iter().enumerate() {
            output.push_str(&self.format_commit(commit, i));
            output.push('\n');
        }

        output
    }

    fn format_commit(&self, commit: &Commit, index: usize) -> String {
        let mut line = String::new();

        // Add graph character based on position
        if index == 0 && self.commits.len() == 1 {
            line.push('o');
        } else if index == 0 {
            line.push('o');
        } else if index == self.commits.len() - 1 {
            line.push('~');
        } else {
            line.push('o');
        }

        line.push(' ');
        line.push_str(&commit.short_hash);
        line.push(' ');

        // Add branch info if available
        if !commit.refs.is_empty() {
            let branch_names: Vec<String> = commit.refs.iter()
                .map(|r| r.name.clone())
                .collect();
            line.push('[');
            line.push_str(&branch_names.join(", "));
            line.push_str("] ");
        }

        // Add commit summary (truncated to fit width)
        let max_summary_len = if self.width > 0 {
            self.width.saturating_sub(50)
        } else {
            50
        };
        let summary = if commit.summary.len() > max_summary_len {
            &commit.summary[..max_summary_len.saturating_sub(3)]
        } else {
            &commit.summary
        };
        line.push_str(summary);

        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_commit(hash: &str, summary: &str) -> Commit {
        Commit {
            hash: hash.to_string(),
            short_hash: hash[..7].to_string(),
            message: summary.to_string(),
            summary: summary.to_string(),
            author: "Test".to_string(),
            email: "test@example.com".to_string(),
            date: Utc::now(),
            parent_hashes: vec![],
            refs: vec![],
        }
    }

    #[test]
    fn test_empty_commits() {
        let formatter = SmartLogFormatter::new(vec![], 80);
        assert_eq!(formatter.format(), "No commits found");
    }

    #[test]
    fn test_single_commit() {
        let commits = vec![create_test_commit("abc123def456789", "Initial commit")];
        let formatter = SmartLogFormatter::new(commits, 80);
        let output = formatter.format();
        assert!(output.contains("abc123d"));
        assert!(output.contains("Initial commit"));
    }

    #[test]
    fn test_multiple_commits() {
        let commits = vec![
            create_test_commit("abc123def456789", "Latest commit"),
            create_test_commit("def456ghi789abc", "Middle commit"),
            create_test_commit("ghi789jkl012345", "Earliest commit"),
        ];
        let formatter = SmartLogFormatter::new(commits, 80);
        let output = formatter.format();

        assert!(output.contains("Latest commit"));
        assert!(output.contains("Middle commit"));
        assert!(output.contains("Earliest commit"));
    }

    #[test]
    fn test_commit_with_refs() {
        let commits = vec![Commit {
            hash: "abc123".to_string(),
            short_hash: "abc123d".to_string(),
            message: "Main commit".to_string(),
            summary: "Main commit".to_string(),
            author: "Test".to_string(),
            email: "test@example.com".to_string(),
            date: Utc::now(),
            parent_hashes: vec![],
            refs: vec![crate::models::GitRef {
                name: "main".to_string(),
                ref_type: crate::models::RefType::Branch,
            }],
        }];
        let formatter = SmartLogFormatter::new(commits, 80);
        let output = formatter.format();
        assert!(output.contains("[main]"));
    }
}
