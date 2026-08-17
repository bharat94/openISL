//! Rendering module for the TUI application
//!
//! Contains rendering logic for commits, diff, panels, and status bar.

pub mod commits;
pub mod diff;
pub mod panels;
pub mod status_bar;

pub(crate) use commits::*;
pub(crate) use diff::*;
pub(crate) use panels::*;
pub(crate) use status_bar::*;
