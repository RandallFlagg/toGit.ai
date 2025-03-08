use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BranchInfo {
    pub name: String,
    pub commit_age: String,
    pub tracking_branch: Option<String>,
    pub last_commit_message: String,
    pub last_commit_author: String,
    pub is_merged: bool,
    // tags: Vec<String>, // Add tags field
}

#[derive(Debug, Serialize)]
pub struct TagInfo {
    pub name: String,
    pub     message: Option<String>,
    pub tagger: Option<String>,
    pub tag_date: Option<String>,
}