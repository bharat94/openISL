use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A version control commit/revision - VCS-agnostic representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    /// Unique identifier (git hash, mercurial changeset ID, etc.)
    pub id: String,
    /// Short identifier for display (e.g., 7-char git hash, 12-char sapling ID)
    pub short_id: String,
    /// Full commit message
    pub message: String,
    /// First line of message (subject)
    pub summary: String,
    /// Author name
    pub author: String,
    /// Author email
    pub email: String,
    /// Timestamp
    pub date: DateTime<Utc>,
    /// Parent change IDs
    pub parent_ids: Vec<String>,
    /// Associated references (branches, tags, etc.)
    pub refs: Vec<Ref>,
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.short_id, self.summary)
    }
}

impl From<Change> for crate::Commit {
    fn from(change: Change) -> Self {
        Self {
            hash: change.id,
            short_hash: change.short_id,
            message: change.message,
            summary: change.summary,
            author: change.author,
            email: change.email,
            date: change.date,
            parent_hashes: change.parent_ids,
            refs: change.refs.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<crate::Commit> for Change {
    fn from(commit: crate::Commit) -> Self {
        Self {
            id: commit.hash,
            short_id: commit.short_hash,
            message: commit.message,
            summary: commit.summary,
            author: commit.author,
            email: commit.email,
            date: commit.date,
            parent_ids: commit.parent_hashes,
            refs: commit.refs.into_iter().map(Into::into).collect(),
        }
    }
}

/// VCS-agnostic reference (branch, tag, bookmark, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ref {
    pub name: String,
    pub ref_type: RefType,
}

impl fmt::Display for Ref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self.ref_type {
            RefType::Head => "",
            RefType::Branch => "branch: ",
            RefType::Tag => "tag: ",
            RefType::Remote => "remote: ",
        };
        write!(f, "{}{}", prefix, self.name)
    }
}

impl From<crate::GitRef> for Ref {
    fn from(git_ref: crate::GitRef) -> Self {
        Self {
            name: git_ref.name,
            ref_type: git_ref.ref_type.into(),
        }
    }
}

impl From<Ref> for crate::GitRef {
    fn from(ref_: Ref) -> Self {
        Self {
            name: ref_.name,
            ref_type: ref_.ref_type.into(),
        }
    }
}

/// Type of reference - VCS-agnostic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefType {
    Head,
    Branch,
    Tag,
    Remote,
}

impl From<crate::RefType> for RefType {
    fn from(ref_type: crate::RefType) -> Self {
        match ref_type {
            crate::RefType::Head => Self::Head,
            crate::RefType::Branch => Self::Branch,
            crate::RefType::Tag => Self::Tag,
            crate::RefType::Remote => Self::Remote,
        }
    }
}

impl From<RefType> for crate::RefType {
    fn from(ref_type: RefType) -> Self {
        match ref_type {
            RefType::Head => Self::Head,
            RefType::Branch => Self::Branch,
            RefType::Tag => Self::Tag,
            RefType::Remote => Self::Remote,
        }
    }
}

/// Remote synchronization state - shows local/remote divergence
#[derive(Debug, Clone, Default)]
pub struct SyncState {
    /// Name of the remote being tracked
    pub remote_name: Option<String>,
    /// Number of local changes not pushed to remote (git: ahead, hg: outgoing)
    pub local_unpushed: Option<usize>,
    /// Number of remote changes not pulled to local (git: behind, hg: incoming)
    pub remote_unpulled: Option<usize>,
    /// Whether there are merge conflicts
    pub has_conflicts: bool,
}

/// Saved work entry (git: stash, hg: shelve)
#[derive(Debug, Clone)]
pub struct SavedWork {
    pub id: String,
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub files_affected: Vec<String>,
    pub change_count: ChangeCount,
}

/// Change count statistics
#[derive(Debug, Clone, Default)]
pub struct ChangeCount {
    pub additions: usize,
    pub deletions: usize,
}

/// History timeline point (git: reflog, hg: journal)
#[derive(Debug, Clone)]
pub struct HistoryPoint {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub description: String,
    pub refs: Vec<Ref>,
}

/// History edit action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryEditAction {
    Keep,
    Revise,
    Combine,
    Remove,
    Edit,
}

/// History edit plan
#[derive(Debug, Clone)]
pub struct HistoryEditPlan {
    pub changes: Vec<HistoryEditPlanEntry>,
}

/// History edit plan entry
#[derive(Debug, Clone)]
pub struct HistoryEditPlanEntry {
    pub change_id: String,
    pub action: HistoryEditAction,
    pub message: Option<String>,
}

/// Change segment for selective staging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeSegment {
    Include,
    Exclude,
}

/// Change patch for selective staging
#[derive(Debug, Clone)]
pub struct ChangePatch {
    pub file_path: String,
    pub segments: Vec<ChangeSegment>,
}
