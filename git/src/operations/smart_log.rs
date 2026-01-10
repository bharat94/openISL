use crate::models::Commit;

pub struct SmartLogFormatter {
    commits: Vec<Commit>,
    width: usize,
}

#[derive(Debug, Clone)]
struct GraphNode {
    commit: Commit,
    position: usize,
    is_main_branch: bool,
    has_children: bool,
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

        let graph = self.build_graph();
        for (i, node) in graph.iter().enumerate() {
            output.push_str(&self.format_graph_node(node, i, graph.len()));
            output.push('\n');
        }

        output
    }

    fn build_graph(&self) -> Vec<GraphNode> {
        let main_branch = self.find_main_branch();

        self.commits.iter().enumerate().map(|(i, commit)| {
            let is_main = commit.refs.iter().any(|r| {
                r.name == main_branch || r.name == "main" || r.name == "master"
            });
            let has_children = self.commits.iter().any(|c| {
                c.parent_hashes.contains(&commit.hash)
            });

            GraphNode {
                commit: commit.clone(),
                position: i,
                is_main_branch: is_main,
                has_children,
            }
        }).collect()
    }

    fn find_main_branch(&self) -> String {
        for commit in &self.commits {
            for r in &commit.refs {
                if r.ref_type == crate::models::RefType::Head {
                    return r.name.clone();
                }
            }
        }
        "main".to_string()
    }

    fn format_graph_node(&self, node: &GraphNode, index: usize, total: usize) -> String {
        let mut line = String::new();

        let is_last = index == total - 1;
        let is_first = index == 0;

        if is_first && total == 1 {
            line.push('●');
        } else if is_last {
            line.push('○');
        } else if node.has_children {
            line.push('│');
        } else {
            line.push(' ');
        }

        line.push(' ');
        line.push_str(&node.commit.short_hash);

        if node.is_main_branch {
            line.push('*');
        } else {
            line.push(' ');
        }
        line.push(' ');

        if !node.commit.refs.is_empty() {
            let branch_names: Vec<String> = node.commit.refs.iter()
                .filter(|r| r.ref_type != crate::models::RefType::Remote)
                .map(|r| {
                    let name = if r.name.starts_with("refs/heads/") {
                        &r.name[11..]
                    } else if r.name.starts_with("refs/remotes/") {
                        &r.name[13..]
                    } else {
                        &r.name
                    };
                    name.to_string()
                })
                .collect();
            if !branch_names.is_empty() {
                line.push('[');
                line.push_str(&branch_names.join(", "));
                line.push_str("] ");
            }
        }

        let max_summary_len = if self.width > 0 {
            self.width.saturating_sub(50)
        } else {
            50
        };
        let summary = if node.commit.summary.len() > max_summary_len {
            format!("{}...", &node.commit.summary[..max_summary_len.saturating_sub(3)])
        } else {
            node.commit.summary.clone()
        };
        line.push_str(&summary);

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
