use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitError {
    #[error("not a git repository: {0}")]
    NotAGitRepository(String),

    #[error("git repository not found")]
    RepositoryNotFound,

    #[error("git command failed: {0}")]
    CommandFailed(String),

    #[error("failed to parse git output: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("unknown error: {0}")]
    Unknown(String),
}
