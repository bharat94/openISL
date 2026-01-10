use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub hash: String,
    pub short_hash: String,
    pub message: String,
    pub summary: String,
    pub author: String,
    pub email: String,
    pub date: DateTime<Utc>,
    pub parent_hashes: Vec<String>,
    pub refs: Vec<GitRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitRef {
    pub name: String,
    pub ref_type: RefType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefType {
    Head,
    Branch,
    Tag,
    Remote,
}
