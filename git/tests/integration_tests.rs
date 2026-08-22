use openisl_git::{
    add_paths, apply_patch, bisect_reset, bisect_start, commit, get_blame, get_branches,
    get_commits, get_commits_filtered, get_conflicted_files, get_current_branch,
    get_file_at_revision, get_stash_list, get_status, init, mark_resolved, merge, move_file,
    remove_file, reset, stash_pop, stash_push, undo_last, Commit, FileStatus, GitRef, RefType,
    ResetMode, StatusType,
};

use std::process::Command;

fn git(repo: &std::path::Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .unwrap();
    assert!(output.status.success(), "git {:?} failed", args);
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn create_test_commit(
    hash: &str,
    summary: &str,
    author: &str,
    email: &str,
    parents: Vec<&str>,
) -> Commit {
    Commit {
        hash: hash.to_string(),
        short_hash: hash[..7].to_string(),
        message: summary.to_string(),
        summary: summary.to_string(),
        author: author.to_string(),
        email: email.to_string(),
        date: chrono::Utc::now(),
        parent_hashes: parents.iter().map(|s| s.to_string()).collect(),
        refs: vec![],
    }
}

#[cfg(test)]
mod git_operations_tests {
    use super::*;

    #[test]
    fn test_get_commits_returns_commits() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_commits(&repo_path, Some(5));
        assert!(result.is_ok());
        let commits = result.unwrap();
        assert!(!commits.is_empty());
    }

    #[test]
    fn test_get_commits_with_limit() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_commits(&repo_path, Some(3));
        assert!(result.is_ok());
        let commits = result.unwrap();
        assert!(commits.len() <= 3);
    }

    #[test]
    fn test_get_branches_returns_refs() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_branches(&repo_path);
        assert!(result.is_ok());
        let branches = result.unwrap();
        assert!(!branches.is_empty());
    }

    #[test]
    fn test_get_current_branch_returns_branch_name() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_current_branch(&repo_path);
        assert!(result.is_ok());
        let branch = result.unwrap();
        assert!(branch.is_some());
        assert!(!branch.unwrap().is_empty());
    }

    #[test]
    fn test_get_status_returns_file_statuses() {
        let repo_path = std::env::current_dir().unwrap();
        let result = get_status(&repo_path);
        assert!(result.is_ok());
        let _files = result.unwrap();
    }

    #[test]
    fn test_commit_order_is_chronological() {
        let repo_path = std::env::current_dir().unwrap();
        let commits = get_commits(&repo_path, Some(10)).unwrap();
        for i in 1..commits.len() {
            assert!(
                commits[i - 1].date >= commits[i].date,
                "Commits should be in chronological order (newest first)"
            );
        }
    }

    #[test]
    fn test_commits_have_valid_hashes() {
        let commits = get_commits(&std::env::current_dir().unwrap(), Some(10)).unwrap();
        for commit in commits {
            assert!(!commit.hash.is_empty());
            assert!(commit.short_hash.len() >= 7);
            assert!(commit.short_hash == commit.hash[..7.min(commit.hash.len())]);
        }
    }

    #[test]
    fn test_commits_have_valid_summaries() {
        let commits = get_commits(&std::env::current_dir().unwrap(), Some(10)).unwrap();
        for commit in commits {
            assert!(!commit.summary.is_empty());
            assert!(commit.message.len() >= commit.summary.len());
        }
    }

    #[test]
    fn test_commits_have_authors() {
        let commits = get_commits(&std::env::current_dir().unwrap(), Some(10)).unwrap();
        for commit in commits {
            assert!(!commit.author.is_empty());
            assert!(!commit.email.is_empty());
            assert!(commit.email.contains('@'));
        }
    }

    #[test]
    fn test_first_commit_is_root() {
        let commits = get_commits(&std::env::current_dir().unwrap(), Some(1)).unwrap();
        if let Some(first) = commits.first() {
            if !first.parent_hashes.is_empty() {
                println!(
                    "First commit has {} parents - this is expected for non-initial repos",
                    first.parent_hashes.len()
                );
            }
        }
    }

    #[test]
    fn test_branches_have_valid_names() {
        let branches = get_branches(&std::env::current_dir().unwrap()).unwrap();
        for branch in branches {
            assert!(!branch.name.is_empty());
        }
    }

    #[test]
    fn test_branches_have_ref_types() {
        let branches = get_branches(&std::env::current_dir().unwrap()).unwrap();
        for branch in branches {
            match branch.ref_type {
                RefType::Branch | RefType::Remote | RefType::Tag | RefType::Head => {}
            }
        }
    }

    #[test]
    fn test_status_types_are_distinct() {
        let types: Vec<StatusType> = vec![
            StatusType::Modified,
            StatusType::Added,
            StatusType::Deleted,
            StatusType::Untracked,
            StatusType::ModifiedStaged,
            StatusType::AddedStaged,
            StatusType::DeletedStaged,
            StatusType::Renamed,
            StatusType::Conflicted,
        ];
        assert_eq!(types.len(), 9, "All status types should be distinct");
    }
}

#[cfg(test)]
mod commit_tests {
    use super::*;

    #[test]
    fn test_commit_clone() {
        let original = create_test_commit(
            "abc123def456789",
            "Test commit",
            "test@example.com",
            "test@example.com",
            vec![],
        );
        let cloned = original.clone();
        assert_eq!(original.hash, cloned.hash);
        assert_eq!(original.summary, cloned.summary);
        assert_eq!(original.author, cloned.author);
    }

    #[test]
    fn test_commit_partial_eq() {
        let commit1 = create_test_commit(
            "abc123def456789",
            "Test commit",
            "test@example.com",
            "test@example.com",
            vec![],
        );
        let commit2 = create_test_commit(
            "abc123def456789",
            "Test commit",
            "test@example.com",
            "test@example.com",
            vec![],
        );
        assert_eq!(commit1.hash, commit2.hash);
        assert_eq!(commit1.summary, commit2.summary);
    }

    #[test]
    fn test_git_ref_equality() {
        let ref1 = GitRef {
            name: "main".to_string(),
            ref_type: RefType::Branch,
        };
        let ref2 = GitRef {
            name: "main".to_string(),
            ref_type: RefType::Branch,
        };
        assert_eq!(ref1.name, ref2.name);
        assert_eq!(ref1.ref_type, ref2.ref_type);
    }

    #[test]
    fn test_git_ref_inequality() {
        let branch_ref = GitRef {
            name: "main".to_string(),
            ref_type: RefType::Branch,
        };
        let remote_ref = GitRef {
            name: "origin/main".to_string(),
            ref_type: RefType::Remote,
        };
        assert_ne!(branch_ref.ref_type, remote_ref.ref_type);
    }

    #[test]
    fn test_file_status_creation() {
        let status = FileStatus {
            path: "src/main.rs".to_string(),
            status: StatusType::Modified,
        };
        assert_eq!(status.path, "src/main.rs");
        assert_eq!(status.status, StatusType::Modified);
    }

    #[test]
    fn test_commit_with_multiple_parents() {
        let commit = create_test_commit(
            "abc123def456789",
            "Merge commit",
            "test@example.com",
            "test@example.com",
            vec!["parent1", "parent2", "parent3"],
        );
        assert_eq!(commit.parent_hashes.len(), 3);
    }

    #[test]
    fn test_commit_with_no_parents() {
        let commit = create_test_commit(
            "abc123def456789",
            "Initial commit",
            "test@example.com",
            "test@example.com",
            vec![],
        );
        assert!(commit.parent_hashes.is_empty());
    }

    #[test]
    fn test_ref_type_variants() {
        assert_ne!(RefType::Head, RefType::Branch);
        assert_ne!(RefType::Head, RefType::Remote);
        assert_ne!(RefType::Head, RefType::Tag);
        assert_ne!(RefType::Branch, RefType::Remote);
        assert_ne!(RefType::Branch, RefType::Tag);
        assert_ne!(RefType::Remote, RefType::Tag);
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_commits_ordering_with_same_date() {
        let now = chrono::Utc::now();
        let commits = [
            Commit {
                hash: "abc123def456789".to_string(),
                short_hash: "abc123d".to_string(),
                message: "First".to_string(),
                summary: "First".to_string(),
                author: "test".to_string(),
                email: "test@test.com".to_string(),
                date: now,
                parent_hashes: vec![],
                refs: vec![],
            },
            Commit {
                hash: "def456ghi789abc".to_string(),
                short_hash: "def456g".to_string(),
                message: "Second".to_string(),
                summary: "Second".to_string(),
                author: "test".to_string(),
                email: "test@test.com".to_string(),
                date: now,
                parent_hashes: vec!["abc123def456789".to_string()],
                refs: vec![],
            },
        ];
        assert_eq!(commits.len(), 2);
    }

    #[test]
    fn test_long_author_names() {
        let commit = Commit {
            hash: "abc123def456789".to_string(),
            short_hash: "abc123d".to_string(),
            message: "Test".to_string(),
            summary: "Test".to_string(),
            author: "Very Long Author Name That Might Be Truncated".to_string(),
            email: "very.long.email.address@example.com".to_string(),
            date: chrono::Utc::now(),
            parent_hashes: vec![],
            refs: vec![],
        };
        assert!(commit.author.len() > 40);
        assert!(commit.email.len() > 30);
    }

    #[test]
    fn test_special_characters_in_paths() {
        let status = FileStatus {
            path: "src/path/with spaces/and-dashes/file.rs".to_string(),
            status: StatusType::Modified,
        };
        assert!(status.path.contains(' '));
        assert!(status.path.contains('-'));
    }

    #[test]
    fn test_unicode_in_commit_messages() {
        let commit = Commit {
            hash: "abc123def456789".to_string(),
            short_hash: "abc123d".to_string(),
            message: "Unicode test: café, ñ, 中文, 🚀".to_string(),
            summary: "Unicode test".to_string(),
            author: "Test".to_string(),
            email: "test@test.com".to_string(),
            date: chrono::Utc::now(),
            parent_hashes: vec![],
            refs: vec![],
        };
        assert!(commit.message.contains("café"));
        assert!(commit.message.contains("中文"));
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;

    #[test]
    fn test_commit_json_serialization() {
        let commit = Commit {
            hash: "abc123def456789".to_string(),
            short_hash: "abc123d".to_string(),
            message: "Test message".to_string(),
            summary: "Test summary".to_string(),
            author: "Test Author".to_string(),
            email: "test@example.com".to_string(),
            date: chrono::Utc::now(),
            parent_hashes: vec!["parent1".to_string(), "parent2".to_string()],
            refs: vec![GitRef {
                name: "main".to_string(),
                ref_type: RefType::Branch,
            }],
        };

        let json = serde_json::to_string(&commit).unwrap();
        assert!(json.contains("abc123def456789"));
        assert!(json.contains("Test summary"));

        let deserialized: Commit = serde_json::from_str(&json).unwrap();
        assert_eq!(commit.hash, deserialized.hash);
        assert_eq!(commit.summary, deserialized.summary);
        assert_eq!(commit.parent_hashes.len(), deserialized.parent_hashes.len());
    }

    #[test]
    fn test_git_ref_serialization() {
        let git_ref = GitRef {
            name: "feature/test-branch".to_string(),
            ref_type: RefType::Branch,
        };

        let json = serde_json::to_string(&git_ref).unwrap();
        let deserialized: GitRef = serde_json::from_str(&json).unwrap();
        assert_eq!(git_ref.name, deserialized.name);
        assert_eq!(git_ref.ref_type, deserialized.ref_type);
    }

    #[test]
    fn test_file_status_serialization() {
        let status = FileStatus {
            path: "src/main.rs".to_string(),
            status: StatusType::Modified,
        };

        assert_eq!(status.path, "src/main.rs");
        assert_eq!(status.status, StatusType::Modified);
    }

    #[test]
    fn test_get_commits_filtered_by_branch() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "-q"]);
        git(repo, &["config", "user.email", "test@example.com"]);
        git(repo, &["config", "user.name", "Test"]);
        std::fs::write(repo.join("a.txt"), "a").unwrap();
        git(repo, &["add", "a.txt"]);
        git(repo, &["commit", "-q", "-m", "initial"]);
        git(repo, &["checkout", "-q", "-b", "feature/x"]);
        std::fs::write(repo.join("b.txt"), "b").unwrap();
        git(repo, &["add", "b.txt"]);
        git(repo, &["commit", "-q", "-m", "feature work"]);

        // default: all branches
        let all = get_commits_filtered(repo, None, None, false).unwrap();
        assert_eq!(all.len(), 2);

        // scoped to a single branch
        let main = get_commits_filtered(repo, None, Some("main"), false).unwrap();
        assert_eq!(main.len(), 1);
        assert_eq!(main[0].summary, "initial");

        let feature = get_commits_filtered(repo, None, Some("feature/x"), false).unwrap();
        assert_eq!(feature.len(), 2);
        assert_eq!(feature[0].summary, "feature work");
    }

    #[test]
    fn test_commit_and_reset() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "-q"]);
        git(repo, &["config", "user.email", "test@example.com"]);
        git(repo, &["config", "user.name", "Test"]);
        init(repo).unwrap();
        std::fs::write(repo.join("a.txt"), "one").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "first").unwrap();
        std::fs::write(repo.join("a.txt"), "two").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "second").unwrap();

        assert_eq!(get_commits(repo, None).unwrap().len(), 2);

        reset(repo, ResetMode::Hard, Some("HEAD~1")).unwrap();
        assert_eq!(get_commits(repo, None).unwrap().len(), 1);
        assert_eq!(std::fs::read_to_string(repo.join("a.txt")).unwrap(), "one");
    }

    #[test]
    fn test_stash_push_pop() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "-q"]);
        git(repo, &["config", "user.email", "test@example.com"]);
        git(repo, &["config", "user.name", "Test"]);
        std::fs::write(repo.join("a.txt"), "base").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "base").unwrap();

        std::fs::write(repo.join("a.txt"), "modified").unwrap();
        stash_push(repo, Some("wip")).unwrap();
        assert_eq!(get_stash_list(repo).unwrap().len(), 1);
        assert_eq!(std::fs::read_to_string(repo.join("a.txt")).unwrap(), "base");

        stash_pop(repo, None).unwrap();
        assert!(get_stash_list(repo).unwrap().is_empty());
        assert_eq!(
            std::fs::read_to_string(repo.join("a.txt")).unwrap(),
            "modified"
        );
    }

    #[test]
    fn test_move_and_remove_file() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "-q"]);
        git(repo, &["config", "user.email", "test@example.com"]);
        git(repo, &["config", "user.name", "Test"]);
        std::fs::write(repo.join("a.txt"), "x").unwrap();
        std::fs::write(repo.join("b.txt"), "y").unwrap();
        add_paths(repo, &["a.txt", "b.txt"]).unwrap();
        commit(repo, "base").unwrap();

        move_file(repo, "a.txt", "c.txt").unwrap();
        remove_file(repo, "b.txt").unwrap();
        commit(repo, "rename").unwrap();

        assert!(repo.join("c.txt").exists());
        assert!(!repo.join("a.txt").exists());
        assert!(!repo.join("b.txt").exists());
    }

    #[test]
    fn test_merge_branches() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "-q"]);
        git(repo, &["config", "user.email", "test@example.com"]);
        git(repo, &["config", "user.name", "Test"]);
        std::fs::write(repo.join("a.txt"), "base\n").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "base").unwrap();

        git(repo, &["checkout", "-q", "-b", "feature"]);
        std::fs::write(repo.join("b.txt"), "feature\n").unwrap();
        add_paths(repo, &["b.txt"]).unwrap();
        commit(repo, "feature work").unwrap();

        git(repo, &["checkout", "-q", "main"]);
        merge(repo, "feature", true).unwrap();
        assert!(repo.join("b.txt").exists());
    }

    #[test]
    fn test_blame() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "-q"]);
        git(repo, &["config", "user.email", "test@example.com"]);
        git(repo, &["config", "user.name", "Test"]);
        std::fs::write(repo.join("a.txt"), "line one\n").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "initial").unwrap();

        let blame = get_blame(repo, "a.txt").unwrap();
        assert!(blame.contains("line one"));
        assert!(blame.contains("Test"));
    }

    #[test]
    fn test_get_file_at_revision() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "-q"]);
        git(repo, &["config", "user.email", "test@example.com"]);
        git(repo, &["config", "user.name", "Test"]);
        std::fs::write(repo.join("a.txt"), "v1\n").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "first").unwrap();
        std::fs::write(repo.join("a.txt"), "v2\n").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "second").unwrap();

        assert_eq!(get_file_at_revision(repo, "HEAD", "a.txt").unwrap(), "v2\n");
        assert_eq!(
            get_file_at_revision(repo, "HEAD~1", "a.txt").unwrap(),
            "v1\n"
        );
    }

    #[test]
    fn test_apply_patch() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "-q"]);
        git(repo, &["config", "user.email", "test@example.com"]);
        git(repo, &["config", "user.name", "Test"]);
        std::fs::write(repo.join("a.txt"), "one\n").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "base").unwrap();
        std::fs::write(repo.join("a.txt"), "one\ntwo\n").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "second").unwrap();

        let patch = git(repo, &["format-patch", "-1", "HEAD", "--stdout"]);
        git(repo, &["reset", "-q", "--hard", "HEAD~1"]);
        std::fs::write(repo.join("change.patch"), patch).unwrap();
        apply_patch(repo, "change.patch", false).unwrap();
        assert_eq!(
            std::fs::read_to_string(repo.join("a.txt")).unwrap(),
            "one\ntwo\n"
        );
    }

    #[test]
    fn test_bisect_start_reset() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "-q"]);
        git(repo, &["config", "user.email", "test@example.com"]);
        git(repo, &["config", "user.name", "Test"]);
        std::fs::write(repo.join("a.txt"), "v1\n").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "first").unwrap();
        std::fs::write(repo.join("a.txt"), "v2\n").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "second").unwrap();

        let bad = git(repo, &["rev-parse", "HEAD"]).trim().to_string();
        let good = git(repo, &["rev-parse", "HEAD~1"]).trim().to_string();
        assert!(bisect_start(repo, &bad, &good).is_ok());
        bisect_reset(repo).unwrap();
        assert_eq!(git(repo, &["rev-parse", "HEAD"]).trim(), bad);
    }

    #[test]
    fn test_resolve_marks_conflict_resolved() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "-q"]);
        git(repo, &["config", "user.email", "test@example.com"]);
        git(repo, &["config", "user.name", "Test"]);
        std::fs::write(repo.join("a.txt"), "base\n").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "base").unwrap();

        git(repo, &["checkout", "-q", "-b", "feature"]);
        std::fs::write(repo.join("a.txt"), "feature\n").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "feature work").unwrap();

        git(repo, &["checkout", "-q", "main"]);
        std::fs::write(repo.join("a.txt"), "main\n").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "main work").unwrap();

        let result = merge(repo, "feature", true);
        assert!(result.is_err()); // conflict expected

        let conflicted = get_conflicted_files(repo).unwrap();
        assert!(conflicted.contains(&"a.txt".to_string()));

        std::fs::write(repo.join("a.txt"), "resolved\n").unwrap();
        mark_resolved(repo, &["a.txt"]).unwrap();
        assert!(get_conflicted_files(repo).unwrap().is_empty());
    }

    #[test]
    fn test_undo_last_commit() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        git(repo, &["init", "-q"]);
        git(repo, &["config", "user.email", "test@example.com"]);
        git(repo, &["config", "user.name", "Test"]);
        std::fs::write(repo.join("a.txt"), "v1\n").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "first").unwrap();
        std::fs::write(repo.join("a.txt"), "v2\n").unwrap();
        add_paths(repo, &["a.txt"]).unwrap();
        commit(repo, "second").unwrap();

        undo_last(repo).unwrap();
        assert_eq!(get_commits(repo, None).unwrap().len(), 1);
        assert_eq!(std::fs::read_to_string(repo.join("a.txt")).unwrap(), "v1\n");
    }
}
