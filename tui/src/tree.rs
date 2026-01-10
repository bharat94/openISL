use std::collections::{HashMap, HashSet};
use openisl_git::Commit;

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub commit: Commit,
    pub children: Vec<TreeNode>,
    pub is_main_branch: bool,
    pub branch_lanes: Vec<BranchLane>,
}

#[derive(Debug, Clone)]
pub struct BranchLane {
    pub is_continuing: bool,
    pub is_branch_point: bool,
    pub is_merge: bool,
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

        let commit_map: HashMap<String, &Commit> = commits.iter()
            .map(|c| (c.hash.clone(), c))
            .collect();

        let mut parent_map: HashMap<String, Vec<String>> = HashMap::new();
        for commit in &commits {
            for parent in &commit.parent_hashes {
                parent_map.entry(parent.clone())
                    .or_default()
                    .push(commit.hash.clone());
            }
        }

        let mut processed = HashSet::new();
        let mut root_commits: Vec<&Commit> = Vec::new();

        for commit in &commits {
            if commit.parent_hashes.is_empty() {
                root_commits.push(commit);
            } else if !parent_map.contains_key(&commit.hash) {
                root_commits.push(commit);
            }
        }

        for root in root_commits {
            self.build_branch(root, &commit_map, &parent_map, &mut processed, &mut Vec::new(), 0);
        }

        self.nodes.sort_by_key(|n| n.commit.date.clone());
        self.nodes.reverse();
    }

    fn build_branch<'a>(
        &mut self,
        commit: &'a Commit,
        commit_map: &HashMap<String, &'a Commit>,
        parent_map: &HashMap<String, Vec<String>>,
        processed: &mut HashSet<String>,
        lanes: &mut Vec<bool>,
        depth: usize,
    ) {
        if processed.contains(&commit.hash) {
            return;
        }
        processed.insert(commit.hash.clone());

        let is_main = commit.refs.iter().any(|r| r.ref_type == openisl_git::RefType::Head);

        let children_hashes = parent_map.get(&commit.hash).map(|v| v.clone()).unwrap_or_default();
        let is_merge = children_hashes.len() > 1 || commit.parent_hashes.len() > 1;

        let mut branch_lanes: Vec<BranchLane> = lanes.iter().map(|&is_continuing| {
            BranchLane {
                is_continuing,
                is_branch_point: false,
                is_merge: false,
            }
        }).collect();

        if is_merge && !branch_lanes.is_empty() {
            branch_lanes.last_mut().unwrap().is_merge = true;
        }

        self.max_depth = self.max_depth.max(depth);

        let node = TreeNode {
            commit: commit.clone(),
            children: Vec::new(),
            is_main_branch: is_main,
            branch_lanes,
        };

        self.nodes.push(node);

        if !children_hashes.is_empty() {
            for (i, child_hash) in children_hashes.iter().enumerate() {
                if let Some(child_commit) = commit_map.get(child_hash) {
                    let mut new_lanes = lanes.clone();
                    if i == children_hashes.len() - 1 {
                        if !new_lanes.is_empty() {
                            new_lanes.pop();
                        }
                    } else {
                        new_lanes.push(true);
                    }

                    self.build_branch(
                        child_commit,
                        commit_map,
                        parent_map,
                        processed,
                        &mut new_lanes,
                        depth + 1,
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

pub fn format_tree_node(node: &TreeNode, is_last: bool, _selected: bool) -> String {
    let mut line = String::new();

    for lane in &node.branch_lanes {
        if lane.is_continuing {
            if lane.is_merge {
                line.push('┤');
            } else {
                line.push('│');
            }
        } else {
            line.push(' ');
        }
    }

    if node.branch_lanes.is_empty() {
        if is_last {
            line.push('└');
        } else {
            line.push('├');
        }
    } else {
        if is_last {
            line.push('└');
        } else {
            line.push('├');
        }
    }

    if node.is_main_branch {
        line.push('●');
    } else {
        line.push('○');
    }
    line.push(' ');

    let hash_part = if node.is_main_branch {
        format!("{}*", node.commit.short_hash)
    } else {
        node.commit.short_hash.clone()
    };

    let branch_names: Vec<String> = node.commit.refs.iter()
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

    let mut content = format!("{}", hash_part);
    if !branch_names.is_empty() {
        content.push_str(&format!(" [{}]", branch_names.join(", ")));
    }
    content.push_str(&format!(" - {}", node.commit.summary));

    line.push_str(&content);

    line
}

pub fn format_tree_lines(commits: &[TreeNode], visible_start: usize, visible_count: usize) -> Vec<String> {
    commits.iter()
        .skip(visible_start)
        .take(visible_count)
        .enumerate()
        .map(|(i, node)| {
            let _global_index = visible_start + i;
            let is_last = _global_index == commits.len() - 1;
            format_tree_node(node, is_last, false)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_commit(hash: &str, summary: &str, parents: Vec<&str>) -> Commit {
        Commit {
            hash: hash.to_string(),
            short_hash: hash[..7].to_string(),
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
        let commits = vec![create_test_commit("abc123def456789", "Initial commit", vec![])];
        let tree = CommitTree::new(commits);
        assert_eq!(tree.nodes().len(), 1);
    }

    #[test]
    fn test_linear_commits() {
        let commits = vec![
            create_test_commit("c123456789abcde", "Third", vec!["b123456789abcde"]),
            create_test_commit("b123456789abcde", "Second", vec!["a123456789abcde"]),
            create_test_commit("a123456789abcde", "First", vec![]),
        ];
        let tree = CommitTree::new(commits);
        assert_eq!(tree.nodes().len(), 3);
    }

    #[test]
    fn test_branched_commits() {
        let commits = vec![
            create_test_commit("c123456789abcde", "Feature merge", vec!["b123456789abcde", "d123456789abcde"]),
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
        let line = format_tree_node(node, true, false);
        assert!(line.contains("abc123d"));
        assert!(line.contains("Test commit"));
    }

    #[test]
    fn test_format_tree_selected() {
        let commits = vec![create_test_commit("abc123def456789", "Test commit", vec![])];
        let tree = CommitTree::new(commits);
        let node = &tree.nodes()[0];
        let line = format_tree_node(node, false, false);
        assert!(line.contains("abc123d"), "Expected line to contain hash, got: {}", line);
        assert!(line.contains("Test commit"), "Expected line to contain summary, got: {}", line);
    }
}
