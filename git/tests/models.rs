#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_display() {
        let commit = Commit {
            hash: "abc123def456789".to_string(),
            short_hash: "abc123d".to_string(),
            message: "Initial commit\n\nWith multiple lines".to_string(),
            summary: "Initial commit".to_string(),
            author: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            date: chrono::Utc::now(),
            parent_hashes: vec![],
            refs: vec![],
        };

        let display = format!("{}", commit);
        assert!(display.contains("abc123d"));
        assert!(display.contains("Initial commit"));
    }

    #[test]
    fn test_git_ref_display_branch() {
        let git_ref = GitRef {
            name: "main".to_string(),
            ref_type: RefType::Branch,
        };

        assert_eq!(format!("{}", git_ref), "branch: main");
    }

    #[test]
    fn test_git_ref_display_remote() {
        let git_ref = GitRef {
            name: "origin/main".to_string(),
            ref_type: RefType::Remote,
        };

        assert_eq!(format!("{}", git_ref), "remote: origin/main");
    }

    #[test]
    fn test_git_ref_display_tag() {
        let git_ref = GitRef {
            name: "v1.0.0".to_string(),
            ref_type: RefType::Tag,
        };

        assert_eq!(format!("{}", git_ref), "tag: v1.0.0");
    }

    #[test]
    fn test_git_ref_display_head() {
        let git_ref = GitRef {
            name: "HEAD".to_string(),
            ref_type: RefType::Head,
        };

        assert_eq!(format!("{}", git_ref), "HEAD");
    }

    #[test]
    fn test_ref_type_equality() {
        assert_eq!(RefType::Branch, RefType::Branch);
        assert_ne!(RefType::Branch, RefType::Remote);
    }

    #[test]
    fn test_commit_serialization() {
        let commit = Commit {
            hash: "abc123".to_string(),
            short_hash: "abc123".to_string(),
            message: "Test".to_string(),
            summary: "Test".to_string(),
            author: "Test".to_string(),
            email: "test@test.com".to_string(),
            date: chrono::Utc::now(),
            parent_hashes: vec![],
            refs: vec![],
        };

        let json = serde_json::to_string(&commit).unwrap();
        let deserialized: Commit = serde_json::from_str(&json).unwrap();

        assert_eq!(commit.hash, deserialized.hash);
        assert_eq!(commit.summary, deserialized.summary);
    }
}
