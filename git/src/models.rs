use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.short_hash, self.summary)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitRef {
    pub name: String,
    pub ref_type: RefType,
}

impl fmt::Display for GitRef {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RefType {
    Head,
    Branch,
    Tag,
    Remote,
}
