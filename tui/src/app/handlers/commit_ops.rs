//! Commit operations
//!
//! Contains commit, file, hunk, and stash operations invoked by handlers.
use super::super::*;

impl App {
    pub(crate) fn stage_selected_hunks_or_lines(&mut self) {
        if let Some(repo_path) = &self.repo_path {
            if let Some(file) = self.files.get(self.selected_file_index) {
                let mut staged_any = false;
                // Only unstaged hunks can be staged. Staged hunks are skipped.
                for hunk in self.hunks.iter().filter(|h| !h.is_staged) {
                    if hunk.is_selected {
                        if let Err(e) = stage_hunk(repo_path, Path::new(&file.path), hunk) {
                            self.status_message = format!("Error staging hunk: {}", e);
                            return;
                        }
                        staged_any = true;
                    } else if hunk.lines.iter().any(|l| l.is_selected) {
                        if let Err(e) = stage_hunk_lines(repo_path, Path::new(&file.path), hunk) {
                            self.status_message = format!("Error staging selected lines: {}", e);
                            return;
                        }
                        staged_any = true;
                    }
                }
                if !staged_any {
                    self.status_message = "No unstaged hunks selected".to_string();
                } else {
                    self.status_message = "Staged selected changes".to_string();
                }
                self.fetch_diff(); // Refresh diff to show updated status
                self.refresh_files(); // Refresh files panel as well
            } else {
                self.status_message = "No file selected to stage hunks".to_string();
            }
        } else {
            self.status_message = "No repository path available".to_string();
        }
    }

    pub(crate) fn unstage_selected_hunks_or_lines(&mut self) {
        if let Some(repo_path) = &self.repo_path {
            if let Some(file) = self.files.get(self.selected_file_index) {
                let mut unstaged_any = false;
                // Only staged hunks can be unstaged.
                for hunk in self.hunks.iter().filter(|h| h.is_staged) {
                    if hunk.is_selected {
                        if let Err(e) = unstage_hunk(repo_path, Path::new(&file.path), hunk) {
                            self.status_message = format!("Error unstaging hunk: {}", e);
                            return;
                        }
                        unstaged_any = true;
                    } else if hunk.lines.iter().any(|l| l.is_selected) {
                        if let Err(e) = unstage_hunk_lines(repo_path, Path::new(&file.path), hunk) {
                            self.status_message = format!("Error unstaging selected lines: {}", e);
                            return;
                        }
                        unstaged_any = true;
                    }
                }
                if !unstaged_any {
                    self.status_message = "No staged hunks selected".to_string();
                } else {
                    self.status_message = "Unstaged selected changes".to_string();
                }
                self.fetch_diff(); // Refresh diff to show updated status
                self.refresh_files(); // Refresh files panel as well
            } else {
                self.status_message = "No file selected to unstage hunks".to_string();
            }
        } else {
            self.status_message = "No repository path available".to_string();
        }
    }

    pub(crate) fn fetch_diff(&mut self) {
        self.is_loading = true;
        self.hunks.clear(); // Clear previous hunks
        self.current_file_diff_output.clear(); // Clear previous diff output

        if let Some(file) = self.files.get(self.selected_file_index) {
            if let Some(ref repo_path) = self.repo_path {
                // Get diff for the selected file (either staged or unstaged)
                let staged_hunks_result =
                    get_file_diff_hunks(repo_path, Path::new(&file.path), true);
                let unstaged_hunks_result =
                    get_file_diff_hunks(repo_path, Path::new(&file.path), false);

                match (staged_hunks_result, unstaged_hunks_result) {
                    (Ok(staged_hunks), Ok(mut unstaged_hunks)) => {
                        // For simplicity, combine staged and unstaged hunks
                        // In a real scenario, you might want to show them separately
                        // or indicate their status more clearly.
                        self.hunks = staged_hunks;
                        self.hunks.append(&mut unstaged_hunks);
                        // Store the full diff output if needed, or re-fetch it when a hunk operation is performed
                        // For now, let's keep diff_content for display and hunks for interactive staging
                        // self.diff_content = ... (re-generate full diff from hunks or git show/diff)
                        self.status_message = format!("Fetched diff for {}", file.path);
                    }
                    (Err(e), _) | (_, Err(e)) => {
                        self.status_message =
                            format!("Error fetching hunks for {}: {}", file.path, e);
                    }
                }

                // Also update the general diff_content for the Diff View if still used
                let file_path_str = file.path.clone(); // Clone to own the String and avoid borrow issues
                match openisl_git::get_diff(repo_path, Some(&file_path_str), false) {
                    // Corrected: Path::new(&file.path) to &file.path
                    Ok(diff) => {
                        self.diff_content = diff;
                        self.current_file_diff_output = self.diff_content.clone(); // Store raw diff
                        self.parse_diff(); // Parse to get stats
                    }
                    Err(e) => {
                        self.diff_content = format!("Error fetching full diff: {}", e);
                        self.current_file_diff_output = self.diff_content.clone();
                        self.parse_diff();
                    }
                }
            } else {
                self.diff_content = "No repository path available".to_string();
                self.status_message = self.diff_content.clone();
                self.parse_diff();
            }
        } else if let Some(commit) = self.selected_commit() {
            let commit_hash = commit.hash.clone(); // Clone to end borrow of self
            let commit_short_hash = commit.short_hash.clone(); // Clone to end borrow of self
            if let Some(ref repo_path) = self.repo_path {
                match get_commit_diff(repo_path, &commit_hash) {
                    Ok(diff) => {
                        self.diff_content = diff;
                        self.current_file_diff_output = self.diff_content.clone();
                        self.parse_diff();
                        self.hunks.clear(); // No hunks for commit diff for now
                        self.status_message =
                            format!("Fetched diff for commit {}", commit_short_hash);
                        // Use cloned short_hash
                    }
                    Err(e) => {
                        self.diff_content = format!("Error fetching diff: {}", e);
                        self.current_file_diff_output = self.diff_content.clone();
                        self.parse_diff();
                    }
                }
            } else {
                self.diff_content = "No repository path available".to_string();
                self.status_message = self.diff_content.clone();
                self.parse_diff();
            }
        } else {
            self.diff_content = "No file or commit selected for diff".to_string();
            self.status_message = self.diff_content.clone();
            self.is_loading = false;
        }
        self.is_loading = false;
    }

    pub(crate) fn refresh_files(&mut self) {
        self.is_loading = true;
        if let Some(ref repo_path) = self.repo_path {
            match openisl_git::get_status(repo_path) {
                Ok(files) => {
                    self.files = files;
                    self.is_loading = false;
                }
                Err(e) => {
                    self.status_message = format!("Error loading files: {}", e);
                    self.is_loading = false;
                }
            }
        } else {
            self.is_loading = false;
        }
    }

    pub(crate) fn stage_selected_file(&mut self) {
        if self.active_panel != PanelType::Files {
            return;
        }

        if self.files.is_empty() {
            self.status_message = "No files to stage".to_string();
            return;
        }

        if let Some(file) = self.files.get(self.selected_file_index) {
            if let Some(ref repo_path) = self.repo_path {
                match openisl_git::stage_file(repo_path, &file.path) {
                    Ok(_) => {
                        self.status_message = format!("Staged: {}", file.path);
                        self.refresh_files();
                    }
                    Err(e) => {
                        self.status_message = format!("Error staging file: {}", e);
                    }
                }
            } else {
                self.status_message = "No repository path available".to_string();
            }
        }
    }

    pub(crate) fn unstage_selected_file(&mut self) {
        if self.active_panel != PanelType::Files {
            return;
        }

        if self.files.is_empty() {
            self.status_message = "No files to unstage".to_string();
            return;
        }

        if let Some(file) = self.files.get(self.selected_file_index) {
            if let Some(ref repo_path) = self.repo_path {
                match openisl_git::unstage_file(repo_path, &file.path) {
                    Ok(_) => {
                        self.status_message = format!("Unstaged: {}", file.path);
                        self.refresh_files();
                    }
                    Err(e) => {
                        self.status_message = format!("Error unstaging file: {}", e);
                    }
                }
            } else {
                self.status_message = "No repository path available".to_string();
            }
        }
    }

    pub(crate) fn toggle_file_stage(&mut self) {
        if self.active_panel != PanelType::Files {
            return;
        }

        if self.files.is_empty() {
            self.status_message = "No files".to_string();
            return;
        }

        if let Some(file) = self.files.get(self.selected_file_index) {
            let is_staged = matches!(
                file.status,
                openisl_git::StatusType::ModifiedStaged
                    | openisl_git::StatusType::AddedStaged
                    | openisl_git::StatusType::DeletedStaged
            );

            if is_staged {
                self.unstage_selected_file();
            } else {
                self.stage_selected_file();
            }
        }
    }

    pub(crate) fn stage_all_files(&mut self) {
        if let Some(ref repo_path) = self.repo_path {
            match openisl_git::stage_all(repo_path) {
                Ok(_) => {
                    self.status_message = "Staged all files".to_string();
                    self.refresh_files();
                }
                Err(e) => {
                    self.status_message = format!("Error staging all files: {}", e);
                }
            }
        } else {
            self.status_message = "No repository path available".to_string();
        }
    }

    pub(crate) fn unstage_all_files(&mut self) {
        if let Some(ref repo_path) = self.repo_path {
            match openisl_git::unstage_all(repo_path) {
                Ok(_) => {
                    self.status_message = "Unstaged all files".to_string();
                    self.refresh_files();
                }
                Err(e) => {
                    self.status_message = format!("Error unstaging all files: {}", e);
                }
            }
        } else {
            self.status_message = "No repository path available".to_string();
        }
    }

    pub(crate) fn amend_commit(&mut self) {
        if let Some(ref repo_path) = self.repo_path {
            match openisl_git::amend_commit(repo_path, None) {
                Ok(_) => {
                    self.status_message = "Commit amended successfully".to_string();
                    self.refresh_commits();
                }
                Err(e) => {
                    self.status_message = format!("Error amending commit: {}", e);
                }
            }
        } else {
            self.status_message = "No repository path available".to_string();
        }
    }

    pub(crate) fn drop_commit(&mut self) {
        if let Some(commit) = self.selected_commit() {
            if let Some(ref repo_path) = self.repo_path {
                match openisl_git::drop_commit(repo_path, &commit.hash) {
                    Ok(_) => {
                        self.status_message = format!("Dropped commit {}", commit.short_hash);
                        self.refresh_commits();
                    }
                    Err(e) => {
                        self.status_message = format!("Error dropping commit: {}", e);
                    }
                }
            } else {
                self.status_message = "No repository path available".to_string();
            }
        } else {
            self.status_message = "No commit selected".to_string();
        }
    }

    pub(crate) fn squash_commits(&mut self) {
        if let Some(commit) = self.selected_commit() {
            if let Some(ref repo_path) = self.repo_path {
                match openisl_git::squash_commits(repo_path, &commit.hash, "Squashed commit") {
                    Ok(_) => {
                        self.status_message =
                            format!("Squashed commits into {}", commit.short_hash);
                        self.refresh_commits();
                    }
                    Err(e) => {
                        self.status_message = format!("Error squashing commits: {}", e);
                    }
                }
            } else {
                self.status_message = "No repository path available".to_string();
            }
        } else {
            self.status_message = "No commit selected".to_string();
        }
    }

    pub(crate) fn cherry_pick_commit(&mut self) {
        if let Some(commit) = self.selected_commit() {
            if let Some(ref repo_path) = self.repo_path {
                match openisl_git::cherry_pick_commit(repo_path, &commit.hash) {
                    Ok(_) => {
                        self.status_message = format!("Cherry-picked {}", commit.short_hash);
                        self.refresh_commits();
                    }
                    Err(e) => {
                        self.status_message = format!("Error cherry-picking: {}", e);
                    }
                }
            } else {
                self.status_message = "No repository path available".to_string();
            }
        } else {
            self.status_message = "No commit selected".to_string();
        }
    }

    pub(crate) fn revert_commit(&mut self) {
        if let Some(commit) = self.selected_commit() {
            if let Some(ref repo_path) = self.repo_path {
                match openisl_git::revert_commit(repo_path, &commit.hash) {
                    Ok(_) => {
                        self.status_message = format!("Reverted {}", commit.short_hash);
                        self.refresh_commits();
                    }
                    Err(e) => {
                        self.status_message = format!("Error reverting: {}", e);
                    }
                }
            } else {
                self.status_message = "No repository path available".to_string();
            }
        } else {
            self.status_message = "No commit selected".to_string();
        }
    }

    fn refresh_commits(&mut self) {
        self.is_loading = true;
        if let Some(ref repo_path) = self.repo_path {
            match openisl_git::get_commits(repo_path, Some(100)) {
                Ok(commits) => {
                    self.commits = commits.clone();
                    self.filtered_commits = commits.clone();
                    self.tree = crate::tree::CommitTree::new(commits);
                    self.selected_index = 0;
                    self.scroll_offset = 0;
                    self.is_loading = false;
                }
                Err(e) => {
                    self.status_message = format!("Error refreshing commits: {}", e);
                    self.is_loading = false;
                }
            }
        } else {
            self.is_loading = false;
        }
        self.refresh_sync_state();
    }

    pub(crate) fn refresh_sync_state(&mut self) {
        self.is_loading = true;
        if let Some(ref repo_path) = self.repo_path {
            match openisl_git::get_sync_state(repo_path) {
                Ok(sync_state) => {
                    self.repo_ahead = sync_state.local_unpushed;
                    self.repo_behind = sync_state.remote_unpulled;
                    self.has_conflicts = sync_state.has_conflicts;
                    self.is_loading = false;
                }
                Err(e) => {
                    self.status_message = format!("Error getting sync state: {}", e);
                    self.is_loading = false;
                }
            }
        } else {
            self.is_loading = false;
        }
    }

    pub(crate) fn refresh_stashes(&mut self) {
        self.is_loading = true;
        if let Some(ref repo_path) = self.repo_path {
            match get_stash_list(repo_path) {
                Ok(stashes) => {
                    self.stashes = stashes;
                    self.selected_stash_index = 0;
                    self.stash_scroll_offset = 0;
                    self.is_loading = false;
                }
                Err(e) => {
                    self.status_message = format!("Error refreshing stashes: {}", e);
                    self.is_loading = false;
                }
            }
        } else {
            self.status_message = "No repository path available".to_string();
            self.is_loading = false;
        }
    }

    pub(crate) fn fetch_stash_diff(&mut self, stash_index_str: &str) {
        self.is_loading = true;
        if let Some(ref repo_path) = self.repo_path {
            match stash_show(repo_path, stash_index_str) {
                Ok(diff) => {
                    self.stash_diff_content = diff;
                    self.is_loading = false;
                }
                Err(e) => {
                    self.stash_diff_content = format!("Error fetching stash diff: {}", e);
                    self.is_loading = false;
                }
            }
        } else {
            self.stash_diff_content = "No repository path available".to_string();
            self.is_loading = false;
        }
    }

    pub(crate) fn apply_stash(&mut self, stash_index_str: Option<&str>) {
        self.is_loading = true;
        if let Some(ref repo_path) = self.repo_path {
            match stash_apply(repo_path, stash_index_str) {
                Ok(_) => {
                    self.status_message =
                        format!("Stash {} applied", stash_index_str.unwrap_or("0"));
                    self.refresh_stashes();
                    self.is_loading = false;
                }
                Err(e) => {
                    self.status_message = format!("Error applying stash: {}", e);
                    self.is_loading = false;
                }
            }
        } else {
            self.status_message = "No repository path available".to_string();
            self.is_loading = false;
        }
    }

    pub(crate) fn drop_stash(&mut self, stash_index_str: Option<&str>) {
        self.is_loading = true;
        if let Some(ref repo_path) = self.repo_path {
            match stash_drop(repo_path, stash_index_str) {
                Ok(_) => {
                    self.status_message =
                        format!("Stash {} dropped", stash_index_str.unwrap_or("0"));
                    self.refresh_stashes();
                    self.is_loading = false;
                }
                Err(e) => {
                    self.status_message = format!("Error dropping stash: {}", e);
                    self.is_loading = false;
                }
            }
        } else {
            self.status_message = "No repository path available".to_string();
            self.is_loading = false;
        }
    }

    pub(crate) fn pop_stash(&mut self, stash_index_str: Option<&str>) {
        self.is_loading = true;
        if let Some(ref repo_path) = self.repo_path {
            match stash_pop(repo_path, stash_index_str) {
                Ok(_) => {
                    self.status_message =
                        format!("Stash {} popped", stash_index_str.unwrap_or("0"));
                    self.refresh_stashes();
                    self.is_loading = false;
                }
                Err(e) => {
                    self.status_message = format!("Error popping stash: {}", e);
                    self.is_loading = false;
                }
            }
        } else {
            self.status_message = "No repository path available".to_string();
            self.is_loading = false;
        }
    }

    pub(crate) fn checkout_commit(&mut self) {
        if let Some(commit) = self.selected_commit() {
            if let Some(ref repo_path) = self.repo_path {
                match openisl_git::checkout_commit(repo_path, &commit.hash) {
                    Ok(_) => {
                        self.status_message = format!("Checked out {}", commit.short_hash);
                        self.refresh_commits();
                        self.view_mode = ViewMode::List;
                    }
                    Err(e) => {
                        self.status_message = format!("Error checking out commit: {}", e);
                    }
                }
            } else {
                self.status_message = "No repository path available".to_string();
            }
        } else {
            self.status_message = "No commit selected".to_string();
        }
    }

    pub(crate) fn create_branch_at_commit(&mut self) {
        if let Some(commit) = self.selected_commit() {
            if !self.branch_input.is_empty() {
                if let Some(ref repo_path) = self.repo_path {
                    match openisl_git::create_branch_from_commit(repo_path, &self.branch_input, &commit.hash) {
                        Ok(_) => {
                            self.status_message =
                                format!("Created branch '{}' from {}", self.branch_input, commit.short_hash);
                            self.refresh_commits();
                        }
                        Err(e) => {
                            self.status_message = format!("Error creating branch: {}", e);
                        }
                    }
                } else {
                    self.status_message = "No repository path available".to_string();
                }
            }
        } else {
            self.status_message = "No commit selected".to_string();
        }
    }
}
