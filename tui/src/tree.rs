use ratatui::prelude::{Line, Span, Style};
use crate::theme::Theme;
use openisl_git::Commit;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub commit: Commit,
    pub children: Vec<TreeNode>,
    pub is_main_branch: bool,
    pub branch_lanes: Vec<BranchLane>,
    pub lane_index: usize,
    pub commit_type: CommitType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitType {
    Initial,
    Branch,
    Merge,
    Tag,
    Revert,
    Squash,
    Regular,
}

#[derive(Debug, Clone)]
pub struct BranchLane {
    pub is_continuing: bool,
    pub is_branch_point: bool,
    pub is_merge: bool,
    pub is_active: bool,
    pub lane_color: Option<usize>,
}

pub struct CommitTree {
    nodes: Vec<TreeNode>,
    max_depth: usize,
}

impl CommitTree {
    pub fn new(commits: Vec<Commit>) -> Self {
        let mut tree = Self {
            nodes: Vec::new(),
            max_depth: 0,
        };
        tree.build_tree(commits);
        tree
    }

    fn build_tree(&mut self, commits: Vec<Commit>) {
        if commits.is_empty() {
            return;
        }

        let commit_map: HashMap<String, &Commit> =
            commits.iter().map(|c| (c.hash.clone(), c)).collect();

        let mut parent_map: HashMap<String, Vec<String>> = HashMap::new();
        for commit in &commits {
            for parent in &commit.parent_hashes {
                parent_map
                    .entry(parent.clone())
                    .or_default()
                    .push(commit.hash.clone());
            }
        }

        let mut processed = HashSet::new();
        let mut root_commits: Vec<&Commit> = Vec::new();

        for commit in &commits {
            if commit.parent_hashes.is_empty() || !parent_map.contains_key(&commit.hash) {
                root_commits.push(commit);
            }
        }

        for root in root_commits {
            self.build_branch(
                root,
                &commit_map,
                &parent_map,
                &mut processed,
                &mut Vec::new(),
                0,
                0,
            );
        }

        self.nodes.sort_by_key(|n| n.commit.date);
        self.nodes.reverse();
    }

    fn detect_commit_type(
        commit: &Commit,
        children: &[String],
        parents: &[String],
        is_merge: bool,
    ) -> CommitType {
        let summary_lower = commit.summary.to_lowercase();

        if commit
            .refs
            .iter()
            .any(|r| r.ref_type == openisl_git::RefType::Tag)
        {
            CommitType::Tag
        } else if summary_lower.starts_with("merge") || is_merge {
            CommitType::Merge
        } else if summary_lower.starts_with("revert ") || summary_lower.starts_with("revert:") {
            CommitType::Revert
        } else if summary_lower.starts_with("squash ") {
            CommitType::Squash
        } else if parents.is_empty() {
            CommitType::Initial
        } else if children.len() > 1 {
            CommitType::Branch
        } else {
            CommitType::Regular
        }
    }

    fn build_branch<'a>(
        &mut self,
        commit: &'a Commit,
        commit_map: &HashMap<String, &'a Commit>,
        parent_map: &HashMap<String, Vec<String>>,
        processed: &mut HashSet<String>,
        lanes: &mut [bool],
        depth: usize,
        lane_index: usize,
    ) {
        if processed.contains(&commit.hash) {
            return;
        }
        processed.insert(commit.hash.clone());

        let is_main = commit
            .refs
            .iter()
            .any(|r| r.ref_type == openisl_git::RefType::Head);

        let children_hashes = parent_map.get(&commit.hash).cloned().unwrap_or_default();
        let is_merge = children_hashes.len() > 1 || commit.parent_hashes.len() > 1;

        let commit_type =
            Self::detect_commit_type(commit, &children_hashes, &commit.parent_hashes, is_merge);

        let branch_lanes: Vec<BranchLane> = lanes
            .iter()
            .enumerate()
            .map(|(idx, &is_continuing)| BranchLane {
                is_continuing,
                is_branch_point: idx == lane_index && children_hashes.len() > 1,
                is_merge: is_merge && idx == lane_index,
                is_active: idx == lane_index,
                lane_color: Some(idx % 8),
            })
            .collect();

        self.max_depth = self.max_depth.max(depth);

        let node = TreeNode {
            commit: commit.clone(),
            children: Vec::new(),
            is_main_branch: is_main,
            branch_lanes,
            lane_index,
            commit_type,
        };

        self.nodes.push(node);

        if !children_hashes.is_empty() {
            for (i, child_hash) in children_hashes.iter().enumerate() {
                if let Some(child_commit) = commit_map.get(child_hash) {
                    let mut new_lanes = lanes.to_owned();
                    let child_lane_index = if i == 0 { lane_index } else { depth + i };

                    if i == 0 {
                        if !new_lanes.is_empty() && lane_index < new_lanes.len() {
                            new_lanes[lane_index] = true;
                        }
                    } else {
                        while new_lanes.len() <= child_lane_index {
                            new_lanes.push(false);
                        }
                        new_lanes[child_lane_index] = true;
                    }

                    self.build_branch(
                        child_commit,
                        commit_map,
                        parent_map,
                        processed,
                        &mut new_lanes,
                        depth + 1,
                        child_lane_index,
                    );
                }
            }
        }
    }

    pub fn nodes(&self) -> &[TreeNode] {
        &self.nodes
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }
}

pub fn format_tree_node<'a>(
    node: &'a TreeNode,
    _is_last: bool,
    selected: bool,
    theme: &'a Theme,
) -> Line<'a> {
    let mut spans = Vec::new();

    // Graph part
    let mut graph_str = String::new();
    for (idx, lane) in node.branch_lanes.iter().enumerate() {
        // Here we could use lane.lane_color if we wanted different colors for graph lines
        if lane.is_continuing {
            if lane.is_merge {
                graph_str.push('┤');
            } else if lane.is_active {
                graph_str.push('│');
            } else {
                graph_str.push(' ');
            }
        } else if lane.is_branch_point && idx == node.lane_index {
            graph_str.push('┬');
        } else {
            graph_str.push(' ');
        }
    }
    spans.push(Span::raw(graph_str));

    // Selection indicator
    if selected {
        spans.push(Span::raw(" >"));
    } else {
        spans.push(Span::raw(" "));
    }

    // Commit symbol
    let commit_symbol = match node.commit_type {
        CommitType::Initial if node.is_main_branch => "┌●",
        CommitType::Initial => "┌○",
        CommitType::Merge if node.is_main_branch => "┼●",
        CommitType::Merge => "┼○",
        CommitType::Tag if node.is_main_branch => "◆●",
        CommitType::Tag => "◆○",
        CommitType::Revert if node.is_main_branch => "↩●",
        CommitType::Revert => "↩○",
        CommitType::Squash if node.is_main_branch => "≡●",
        CommitType::Squash => "≡○",
        CommitType::Branch if node.is_main_branch => "┬●",
        CommitType::Branch => "┬○",
        CommitType::Regular if node.is_main_branch => "─●",
        CommitType::Regular => "─○",
    };
    spans.push(Span::raw(commit_symbol));
    spans.push(Span::raw(" "));

    // Hash
    let hash_part = if node.is_main_branch {
        format!("{}*", node.commit.short_hash)
    } else {
        node.commit.short_hash.clone()
    };
    spans.push(Span::styled(hash_part, Style::default().fg(theme.commit_hash)));
    spans.push(Span::raw(" "));

    // Summary
    spans.push(Span::raw(format!("- {}", node.commit.summary)));
    spans.push(Span::raw(" "));

    // Relative time
    let relative_time = format_relative_time(node.commit.date);
    spans.push(Span::styled(
        format!("({})", relative_time),
        Style::default().fg(theme.commit_date),
    ));

    // Branches
    let branch_names: Vec<String> = node
        .commit
        .refs
        .iter()
        .filter(|r| r.ref_type != openisl_git::RefType::Remote)
        .filter(|r| !r.name.starts_with("HEAD"))
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
        .filter(|n| !n.is_empty())
        .collect();

    if !branch_names.is_empty() {
        spans.push(Span::raw(" ["));
        let styled_branches: Vec<Span> = branch_names
            .into_iter()
            .map(|name| Span::styled(name, Style::default().fg(theme.branch_name)))
            .collect();
        for (i, branch_span) in styled_branches.into_iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw(", "));
            }
            spans.push(branch_span);
        }
        spans.push(Span::raw("]"));
    }

    // Tags
    let tags: Vec<String> = node
        .commit
        .refs
        .iter()
        .filter(|r| r.ref_type == openisl_git::RefType::Tag)
        .map(|r| {
            let name = if r.name.starts_with("refs/tags/") {
                &r.name[10..]
            } else {
                &r.name
            };
            name.to_string()
        })
        .collect();

    if !tags.is_empty() {
        spans.push(Span::raw(" (tags: "));
        let styled_tags: Vec<Span> = tags
            .into_iter()
            .map(|name| Span::styled(name, Style::default().fg(theme.accent))) // Use accent color for tags
            .collect();
        for (i, tag_span) in styled_tags.into_iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw(", "));
            }
            spans.push(tag_span);
        }
        spans.push(Span::raw(")"));
    }

    Line::from(spans)

}

fn format_relative_time(date: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(date);

    let total_seconds = duration.num_seconds();
    let total_minutes = duration.num_minutes();
    let total_hours = duration.num_hours();
    let total_days = duration.num_days();

    if total_seconds < 60 {
        "just now".to_string()
    } else if total_minutes < 60 {
        format!("{}m ago", total_minutes)
    } else if total_hours < 24 {
        format!("{}h ago", total_hours)
    } else if total_days < 7 {
        format!("{}d ago", total_days)
    } else if total_days < 30 {
        format!("{}w ago", total_days / 7)
    } else if total_days < 365 {
        format!("{}mo ago", total_days / 30)
    } else {
        format!("{}y ago", total_days / 365)
    }
}

pub fn format_tree_lines<'a>(
    commits: &'a [TreeNode],
    visible_start: usize,
    visible_count: usize,
    theme: &'a Theme,
) -> Vec<Line<'a>> {
    commits
        .iter()
        .skip(visible_start)
        .take(visible_count)
        .enumerate()
        .map(|(i, node)| {
            let _global_index = visible_start + i;
            let is_last = _global_index == commits.len() - 1;
            format_tree_node(node, is_last, false, theme)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::theme::Theme;

    fn create_test_theme() -> Theme {
        Theme::dark()
    }

    fn create_test_commit(hash: &str, summary: &str, parents: Vec<&str>) -> Commit {
        let short_len = hash.len().min(7);
        Commit {
            hash: hash.to_string(),
            short_hash: hash[..short_len].to_string(),
            message: summary.to_string(),
            summary: summary.to_string(),
            author: "Test".to_string(),
            email: "test@example.com".to_string(),
            date: Utc::now(),
            parent_hashes: parents.iter().map(|s| s.to_string()).collect(),
            refs: vec![],
        }
    }

    #[test]
    fn test_empty_commits() {
        let tree = CommitTree::new(vec![]);
        assert_eq!(tree.nodes().len(), 0);
    }

    #[test]
    fn test_single_commit() {
        let commits = vec![create_test_commit(
            "abc123def456789",
            "Initial commit",
            vec![],
        )];
        let tree = CommitTree::new(commits);
        assert_eq!(tree.nodes().len(), 1);
    }

    #[test]
    fn test_linear_commits() {
        let commits = create_test_commits_with_dates();
        let tree = CommitTree::new(commits);
        assert_eq!(tree.nodes().len(), 3);
    }

    #[test]
    fn test_branched_commits() {
        let commits = vec![
            create_test_commit(
                "c123456789abcde",
                "Feature merge",
                vec!["b123456789abcde", "d123456789abcde"],
            ),
            create_test_commit("d123456789abcde", "Feature commit", vec!["a123456789abcde"]),
            create_test_commit("b123456789abcde", "Main commit", vec!["a123456789abcde"]),
            create_test_commit("a123456789abcde", "Initial", vec![]),
        ];
        let tree = CommitTree::new(commits);
        assert_eq!(tree.nodes().len(), 4);
    }

    #[test]
    fn test_format_tree_node() {
        let commits = vec![create_test_commit("abc123def456789", "Test commit", vec![])];
        let tree = CommitTree::new(commits);
        let node = &tree.nodes()[0];
        let theme = create_test_theme();
        let line = format_tree_node(node, true, false, &theme);
        let plain_text: String = line.iter().map(|s| s.content.to_string()).collect();
        assert!(plain_text.contains("abc123d"));
        assert!(plain_text.contains("Test commit"));
    }

    #[test]
    fn test_format_tree_selected() {
        let commits = vec![create_test_commit("abc123def456789", "Test commit", vec![])];
        let tree = CommitTree::new(commits);
        let node = &tree.nodes()[0];
        let theme = create_test_theme();
        let line = format_tree_node(node, false, false, &theme);
        let plain_text: String = line.iter().map(|s| s.content.to_string()).collect();
        assert!(
            plain_text.contains("abc123d"),
            "Expected line to contain hash, got: {}",
            plain_text
        );
        assert!(
            plain_text.contains("Test commit"),
            "Expected line to contain summary, got: {}",
            plain_text
        );
    }

    #[test]
    fn test_format_tree_node_contains_symbols() {
        let commits = vec![create_test_commit("abc123def456789", "Test commit", vec![])];
        let tree = CommitTree::new(commits);
        let node = &tree.nodes()[0];
        let theme = create_test_theme();
        let line = format_tree_node(node, true, false, &theme);
        let plain_text: String = line.iter().map(|s| s.content.to_string()).collect();
        assert!(plain_text.contains('●') || plain_text.contains('○'));
    }

    #[test]
    fn test_format_tree_lines() {
        let commits = create_test_commits_with_dates();
        let tree = CommitTree::new(commits);
        let theme = create_test_theme();
        let lines = format_tree_lines(tree.nodes(), 0, 10, &theme);
        assert_eq!(lines.len(), 3);
        let all_content: String = lines.into_iter().map(|line| {
            line.spans.iter().map(|s| s.content.to_string()).collect::<String>()
        }).collect::<Vec<String>>().join(" ");
        assert!(
            all_content.contains("c123456")
                || all_content.contains("b123456")
                || all_content.contains("a123456")
        );
    }

    #[test]
    fn test_format_tree_lines_with_offset() {
        let commits = create_test_commits_with_dates();
        let tree = CommitTree::new(commits);
        let theme = create_test_theme();
        let lines = format_tree_lines(tree.nodes(), 1, 10, &theme);
        assert_eq!(lines.len(), 2);
        let all_content: String = lines.into_iter().map(|line| {
            line.spans.iter().map(|s| s.content.to_string()).collect::<String>()
        }).collect::<Vec<String>>().join(" ");
        assert!(all_content.contains("b123456") || all_content.contains("a123456"));
    }
    #[test]
    fn test_format_tree_lines_with_limit() {
        let commits = create_test_commits_with_dates();
        let tree = CommitTree::new(commits);
        let theme = create_test_theme();
        let lines = format_tree_lines(tree.nodes(), 0, 2, &theme);
        assert_eq!(lines.len(), 2);
    }
    #[test]
    fn test_tree_max_depth() {
        let commits = create_test_commits_with_dates();
        let tree = CommitTree::new(commits);
        assert!(tree.max_depth() > 0);
    }

    #[test]
    fn test_tree_with_merge_commit() {
        let now = chrono::Utc::now();
        let commits = vec![
            Commit {
                hash: "e123456789abcde".to_string(),
                short_hash: "e123456".to_string(),
                message: "Merge commit".to_string(),
                summary: "Merge".to_string(),
                author: "test@example.com".to_string(),
                email: "test@example.com".to_string(),
                date: now - chrono::Duration::hours(1),
                parent_hashes: vec!["c123456789abcde".to_string(), "d123456789abcde".to_string()],
                refs: vec![],
            },
            Commit {
                hash: "d123456789abcde".to_string(),
                short_hash: "d123456".to_string(),
                message: "Feature B".to_string(),
                summary: "Feature B".to_string(),
                author: "test@example.com".to_string(),
                email: "test@example.com".to_string(),
                date: now - chrono::Duration::hours(2),
                parent_hashes: vec!["b123456789abcde".to_string()],
                refs: vec![],
            },
            Commit {
                hash: "c123456789abcde".to_string(),
                short_hash: "c123456".to_string(),
                message: "Feature A".to_string(),
                summary: "Feature A".to_string(),
                author: "test@example.com".to_string(),
                email: "test@example.com".to_string(),
                date: now - chrono::Duration::hours(3),
                parent_hashes: vec!["b123456789abcde".to_string()],
                refs: vec![],
            },
            Commit {
                hash: "b123456789abcde".to_string(),
                short_hash: "b123456".to_string(),
                message: "Second commit".to_string(),
                summary: "Second".to_string(),
                author: "test@example.com".to_string(),
                email: "test@example.com".to_string(),
                date: now - chrono::Duration::hours(4),
                parent_hashes: vec!["a123456789abcde".to_string()],
                refs: vec![],
            },
            Commit {
                hash: "a123456789abcde".to_string(),
                short_hash: "a123456".to_string(),
                message: "First commit".to_string(),
                summary: "First".to_string(),
                author: "test@example.com".to_string(),
                email: "test@example.com".to_string(),
                date: now - chrono::Duration::hours(5),
                parent_hashes: vec![],
                refs: vec![],
            },
        ];
        let tree = CommitTree::new(commits);
        assert_eq!(tree.nodes().len(), 5);
    }

    fn create_test_commits_with_dates() -> Vec<Commit> {
        let now = chrono::Utc::now();
        vec![
            Commit {
                hash: "c123456789abcde".to_string(),
                short_hash: "c123456".to_string(),
                message: "Third commit".to_string(),
                summary: "Third".to_string(),
                author: "test@example.com".to_string(),
                email: "test@example.com".to_string(),
                date: now - chrono::Duration::hours(1), // Most recent
                parent_hashes: vec!["b123456789abcde".to_string()],
                refs: vec![],
            },
            Commit {
                hash: "b123456789abcde".to_string(),
                short_hash: "b123456".to_string(),
                message: "Second commit".to_string(),
                summary: "Second".to_string(),
                author: "test@example.com".to_string(),
                email: "test@example.com".to_string(),
                date: now - chrono::Duration::hours(2), // Middle
                parent_hashes: vec!["a123456789abcde".to_string()],
                refs: vec![],
            },
            Commit {
                hash: "a123456789abcde".to_string(),
                short_hash: "a123456".to_string(),
                message: "First commit".to_string(),
                summary: "First".to_string(),
                author: "test@example.com".to_string(),
                email: "test@example.com".to_string(),
                date: now - chrono::Duration::hours(3), // Oldest
                parent_hashes: vec![],
                refs: vec![],
            },
        ]
    }

    #[test]
    fn test_tree_preserves_commit_order() {
        let commits = create_test_commits_with_dates();
        let tree = CommitTree::new(commits);
        let nodes = tree.nodes();
        assert_eq!(nodes.len(), 3);
        assert!(nodes[0].commit.summary.contains("Third"));
        assert!(nodes[1].commit.summary.contains("Second"));
        assert!(nodes[2].commit.summary.contains("First"));
    }

    #[test]
    fn test_tree_node_clone() {
        let commits = vec![create_test_commit("abc123def456789", "Test", vec![])];
        let tree = CommitTree::new(commits.clone());
        let node = tree.nodes()[0].clone();
        assert_eq!(node.commit.hash, "abc123def456789");
    }

    #[test]
    fn test_format_tree_node_with_branches() {
        use openisl_git::{GitRef, RefType};
        let mut commit = create_test_commit("abc123def456789", "Test commit", vec![]);
        commit.refs = vec![GitRef {
            name: "refs/heads/main".to_string(),
            ref_type: RefType::Branch,
        }];
        let tree = CommitTree::new(vec![commit]);
        let node = &tree.nodes()[0];
        let theme = create_test_theme();
        let line = format_tree_node(node, true, false, &theme);
        let plain_text: String = line.iter().map(|s| s.content.to_string()).collect();
        assert!(plain_text.contains("main"));
    }
    #[test]
    fn test_commit_type_initial() {
        let commits = vec![create_test_commit("abc123def456789", "Initial", vec![])];
        let tree = CommitTree::new(commits);
        let node = &tree.nodes()[0];
        assert_eq!(node.commit_type, CommitType::Initial);
    }

    #[test]
    fn test_commit_type_merge() {
        let commits = vec![create_test_commit(
            "merge123",
            "Merge branch 'feature'",
            vec!["parent1", "parent2"],
        )];
        let tree = CommitTree::new(commits);
        let node = &tree.nodes()[0];
        assert_eq!(node.commit_type, CommitType::Merge);
    }

    #[test]
    fn test_commit_type_revert() {
        let commits = vec![create_test_commit(
            "revert123",
            "Revert: Bad commit",
            vec!["parent1"],
        )];
        let tree = CommitTree::new(commits);
        let node = &tree.nodes()[0];
        assert_eq!(node.commit_type, CommitType::Revert);
    }

    #[test]
    fn test_commit_type_squash() {
        let commits = vec![create_test_commit(
            "squash123",
            "Squash changes",
            vec!["parent1"],
        )];
        let tree = CommitTree::new(commits);
        let node = &tree.nodes()[0];
        assert_eq!(node.commit_type, CommitType::Squash);
    }

    #[test]
    fn test_commit_type_tag() {
        use openisl_git::{GitRef, RefType};
        let mut commit = create_test_commit("tag123", "Tagged commit", vec![]);
        commit.refs = vec![GitRef {
            name: "refs/tags/v1.0.0".to_string(),
            ref_type: RefType::Tag,
        }];
        let tree = CommitTree::new(vec![commit]);
        let node = &tree.nodes()[0];
        assert_eq!(node.commit_type, CommitType::Tag);
    }

    #[test]
    fn test_format_tree_node_with_tags() {
        use openisl_git::{GitRef, RefType};
        let mut commit = create_test_commit("abc123def456789", "Test commit", vec![]);
        commit.refs = vec![GitRef {
            name: "refs/tags/v1.0.0".to_string(),
            ref_type: RefType::Tag,
        }];
        let tree = CommitTree::new(vec![commit]);
        let node = &tree.nodes()[0];
        let theme = create_test_theme();
        let line = format_tree_node(node, true, false, &theme);
        let plain_text: String = line.iter().map(|s| s.content.to_string()).collect();
        assert!(plain_text.contains("tags"));
        assert!(plain_text.contains("v1.0.0"));
    }
    #[test]
    fn test_format_tree_node_selected() {
        let commits = vec![create_test_commit("abc123def456789", "Test commit", vec![])];
        let tree = CommitTree::new(commits);
        let node = &tree.nodes()[0];
        let theme = create_test_theme();
        let line = format_tree_node(node, false, true, &theme);
        let plain_text: String = line.iter().map(|s| s.content.to_string()).collect();
        assert!(plain_text.contains('>'));
    }
    #[test]
    fn test_lane_index_assigned() {
        let commits = create_test_commits_with_dates();
        let tree = CommitTree::new(commits);
        let nodes = tree.nodes();
        assert!(!nodes.is_empty());
        for node in nodes {
            assert!(node.lane_index < 10);
        }
    }

    #[test]
    fn test_branch_lane_colors() {
        let commits = vec![create_test_commit("abc123def456789", "Test", vec![])];
        let tree = CommitTree::new(commits);
        let node = &tree.nodes()[0];
        for lane in &node.branch_lanes {
            if lane.is_active {
                assert!(lane.lane_color.is_some());
            }
        }
    }
}
