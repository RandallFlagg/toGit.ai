use std::{cell::OnceCell, marker::PhantomData, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};
//TODO: Should this be here or in the js?
#[derive(Serialize, Deserialize)]
//TODO: Rename? Settings, UserSetting, UserConfig, Config, etc.
pub struct AppConfig<'a> {//TOOD: Change to private
    pub background_color: String,
    pub local_branch_color: String,
    pub active_branch_color: String,
    pub remote_branch_color: String,
    pub remote_tag_color: String,
    pub local_tag_color: String,
    pub type_order: Vec<String>, // Type priority orde>r
    repo_path_buf: Option<PathBuf>,
    pub repo_path_string: String,
    // repo_path_str: &'static str,
    _marker: PhantomData<&'a ()>,//TODO: Remove
}

impl <'a> Default for AppConfig<'a> {
    fn default() -> Self {
        Self {
            background_color: "#FFFFFF".to_string(),
            local_branch_color: "#00FF00".to_string(),
            active_branch_color: "#FF0000".to_string(),
            remote_branch_color: "#F5F5DC".to_string(),
            remote_tag_color: "#FFFF00".to_string(),
            local_tag_color: "#FFFFE0".to_string(),
            type_order: vec![
                "LocalTag".to_string(),
                "RemoteTag".to_string(),
                "LocalBranch".to_string(),
                "RemoteBranch".to_string(),
                "ActiveBranch".to_string(),
            ],
            repo_path_buf: None,
            // repo_path_path: todo!(),
            repo_path_string: "".to_string(),
            // repo_path_str: todo!(),
            _marker: PhantomData,
        }
    }
}

impl<'a> AppConfig<'a> {
    // Constructor to initialize with repo_path
    // pub fn with_repo_path(repo_path: PathBuf) -> Self {
    //     let mut config = Self::default();
    //     config.repo_path_buf = repo_path;
    //     config
    // }

    //TODO: should this be writeable or readonly?
    pub fn set_repo_path(&mut self, repo_path: &str) {
            self.repo_path_buf = Some(PathBuf::from(repo_path));
            self.repo_path_string = repo_path.to_string();
            // self.repo_path_str = repo_path;//Box::leak(self.repo_path_string.into_boxed_str());
    }

    pub fn repo_path_as_pathbuf(&self) -> &PathBuf {
        self.repo_path_buf.as_ref().unwrap()
    }
    pub fn repo_path_as_path(&self) -> &Path {
        self.repo_path_as_pathbuf().as_path()
    }
    // pub fn repo_path_string(&self) -> String {
    //     self.repo_path_string
    // }
    pub fn repo_path_as_str(&self) -> &str {
        &self.repo_path_string
    }
}
