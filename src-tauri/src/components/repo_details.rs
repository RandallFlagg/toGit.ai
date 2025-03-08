use serde::Serialize;
use crate::components::branch_info::BranchInfo;

//TODO: Take this out to a seperate file?
#[derive(Debug, Serialize)]
pub struct RemoteDetails {
    pub name: String,
    pub fetch_url: String,
    pub push_url: String,
    pub push_refspecs: Vec<String>,
    pub fetch_refspecs: Vec<String>,
    pub local_branches_configured_for_git_pull: Vec<String>,
    pub local_branches_configured_for_git_push: Vec<String>,
    pub remote_branches: Vec<BranchInfo>,
}

#[derive(Debug, Serialize)]
pub(crate) struct RepoDetails {
    //TODO: Fix visability
    // pub name: String,
    pub description: Option<String>,
    // pub url: String,
    pub remotes: Vec<RemoteDetails>,
    // pub branches: Vec<String>,
    pub tags: Vec<String>,
    pub default_branch_name: String,
    pub default_full_branch_name: String,
    pub default_push_remote: String,
    pub default_pull_remote: String,
    pub contributors: Vec<String>, // git2 does not provide contributor information directly
    pub forks: usize,              // git2 does not provide fork information
    pub stars: usize,              // git2 does not provide star information
    pub language: String,          // git2 does not provide repository language information
    pub size: usize,               // git2 does not provide repository size information
    pub created_at: String,        // git2 does not provide creation date information
    pub updated_at: String,        // git2 does not provide update date information
    pub git_settings: Vec<String>, // Placeholder, git2 does not provide detailed settings
}
