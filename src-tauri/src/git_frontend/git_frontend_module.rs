// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use git2::{BranchType, Oid, Repository, Status};
// use libgit2_sys::{git_repository, git_repository};
use log::debug;
use serde::Serialize;
use shellexpand;
use std::collections::{HashMap, HashSet};
use std::path::Path;

// fn get_modified_files(repo_path: &Path) -> Result<Vec<String>, git2::Error> {
//     // Open the repository
//     let repo = Repository::open(repo_path)?;

//     // Get the index (staging area)
//     let index = repo.index()?;

//     // Get the list of modified files
//     let mut modified_files = Vec::new();
//     for entry in index.iter() {
//         // let path = entry.path().ok_or(git2::Error::from_str("Invalid UTF-8 path"))?;
//         let path_str = std::str::from_utf8(&entry.path).map_err(|_| git2::Error::from_str("Invalid UTF-8 path"))?;
//         let path = Path::new(path_str);
//         let status = repo.status_file(path)?;

//         // Check if the file is modified
//         if status.is_index_modified() || status.is_wt_modified() {
//             modified_files.push(path.to_string_lossy().to_string());
//         }
//     }

//     Ok(modified_files)
// }

#[tauri::command]
fn get_repo_changes(repo_path: &Path) -> Result<Vec<(String, String)>, String> {
    get_repo_changes_internal(repo_path).map_err(|e| e.to_string())
}

fn get_repo_changes_internal(repo_path: &Path) -> Result<Vec<(String, String)>, git2::Error> {
    // Open the repository
    let repo = Repository::open(repo_path)?;

    // Get the status options and status entries
    let statuses = repo.statuses(None)?;

    // Get the list of changes
    let mut changes = Vec::new();
    for entry in statuses.iter() {
        let path = entry
            .path()
            .ok_or(git2::Error::from_str("Invalid UTF-8 path"))?;
        let status = entry.status();

        // Determine the type of change
        let change_type = if status.contains(Status::INDEX_NEW) || status.contains(Status::WT_NEW) {
            "Untracked"
        } else if status.contains(Status::INDEX_DELETED) || status.contains(Status::WT_DELETED) {
            "Deleted"
        } else if status.contains(Status::INDEX_MODIFIED) || status.contains(Status::WT_MODIFIED) {
            "Modified"
        } else if status.contains(Status::INDEX_RENAMED) || status.contains(Status::WT_RENAMED) {
            "Renamed"
        } else if status.contains(Status::INDEX_TYPECHANGE)
            || status.contains(Status::WT_TYPECHANGE)
        {
            "Type Changed"
        } else {
            continue; //"Unknown"
        };

        changes.push((
            Path::new(path).to_string_lossy().to_string(),
            change_type.to_string(),
        )); //TODO: change to path?
    }

    Ok(changes)
}

#[tauri::command]
fn get_git_data(repo_path: String) -> Result<String, String> {//TODO: Should be changed to the below line
// fn get_git_data(repo_path: String) -> Result<GitData, String> {
    // let config = AppConfig::default();
    // GitData::new(repo_path, &config)
    unimplemented!("This function is not yet implemented.");
}

#[tauri::command]
fn show_menu() -> Vec<&'static str> {
    // println!("Showing Menu");
    // vec!["Hello", "Hi", "Hey"]
    unimplemented!("This function is not yet implemented.");
}
