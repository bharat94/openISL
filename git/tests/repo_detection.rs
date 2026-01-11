use openisl_git::{find_repo_root, is_git_repo};

#[test]
fn test_is_git_repo_in_repo() {
    let current = std::env::current_dir().unwrap();
    assert!(is_git_repo(&current));
}

#[test]
fn test_find_repo_root_in_repo() {
    let current = std::env::current_dir().unwrap();
    let result = find_repo_root(&current);
    assert!(result.is_ok());
}

#[test]
fn test_find_repo_root_from_subdirectory() {
    let current = std::env::current_dir().unwrap();
    let subdir = current.join("git").join("src");

    if subdir.exists() {
        let result = find_repo_root(&subdir);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), current);
    }
}

#[test]
fn test_is_git_repo_non_git_dir() {
    let tmp = std::env::temp_dir();
    assert!(!is_git_repo(&tmp));
}

#[test]
fn test_find_repo_root_non_git_dir() {
    let tmp = std::env::temp_dir();
    let result = find_repo_root(&tmp);
    assert!(result.is_err());
}
