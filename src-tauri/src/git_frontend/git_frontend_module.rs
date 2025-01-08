use chrono::{DateTime, Utc};
use file_format::FileFormat;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use git2::{BranchType, Oid, Repository, Status};
// use libgit2_sys::{git_repository, git_repository};
use log::debug;
use mime_guess::from_path;
use serde::Serialize;
use shellexpand;
use std::collections::{HashMap, HashSet};
use std::fs::{self, metadata, File};
use std::io::Read;
use std::path::Path;
use std::time::SystemTime;
use std::{io, path};

use crate::components::file_metadata::FileMetadata;
use crate::components::git_frontend_error::GitFrontendError;

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
pub fn get_repo_status(repo_path: &Path) -> Result<Vec<FileMetadata>, String> {
    get_repo_status_internal(repo_path).map_err(|e| e.to_string())
}

fn get_repo_status_internal(repo_path: &Path) -> Result<Vec<FileMetadata>, GitFrontendError> {
    // Open the repository
    let repo = Repository::open(repo_path)?;

    // Get the status options and status entries
    let statuses = repo.statuses(None)?;

    // Get the list of changes
    let mut changes = Vec::new();

    for entry in statuses.iter() {
        let path = entry.path().ok_or(git2::Error::from_str("Invalid UTF-8 path"))?;
        let status = entry.status();

        // Determine the type of change
        let change_type = match status {
            _ if status.contains(Status::INDEX_NEW) || status.contains(Status::WT_NEW) => "Untracked",
            _ if status.contains(Status::INDEX_DELETED) || status.contains(Status::WT_DELETED) => "Deleted",
            _ if status.contains(Status::INDEX_MODIFIED) || status.contains(Status::WT_MODIFIED) => "Modified",
            _ if status.contains(Status::INDEX_RENAMED) || status.contains(Status::WT_RENAMED) => "Renamed",
            _ if status.contains(Status::INDEX_TYPECHANGE) || status.contains(Status::WT_TYPECHANGE) => "Type Changed",
            _ if status.contains(Status::IGNORED) => "Ignored",
            _ if status.contains(Status::CONFLICTED) => "Conflicted",
            _ => continue, //"Unknown"
        };

        let file = get_file_metadata(path, change_type)?;
        // changes.push((
        //     Path::new(path).to_string_lossy().to_string(),
        //     change_type.to_string(),
        // )); //TODO: change to path?
        changes.push(file);
    }

    Ok(changes)
}

#[tauri::command]
fn get_git_data(repo_path: String) -> Result<String, String> {
    //TODO: Should be changed to the below line
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

fn get_file_metadata(full_file_path: &str, change_type: &str) -> Result<FileMetadata, GitFrontendError> {
    let file_path = Path::new(full_file_path);
    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let file_format = determine_file_format(file_path)?;
    let metadata = metadata(file_path)?;
    let modified_time = metadata.modified().unwrap_or(SystemTime::now());
    let modified_at = system_time_to_naive_date_time(modified_time);
    let file_metadata = FileMetadata {
        id: 1,
        change_type: change_type.to_string(),
        file_name: file_name,
        file_extenstion: file_path.extension().and_then(|ext| ext.to_str()).unwrap_or("").to_string(),
        file_type: file_format,
        status: String::from("Added"),                         //TODO: Take from get_repo_changes
        size: format!("{} KB", file.metadata()?.len() / 1024), // Get size in KB //TODO: Format
        created_by: String::from("User A"),
        created_at: String::from("2024-01-01T00:00:00Z"),
        modified_by: String::from("User B"),
        modified_at: modified_at.to_string(),
        comments: String::from("No comments"),
        preview: contents, // Use file content as preview
        selected: false,
    };
    Ok(file_metadata)
}

fn determine_file_format(file_path: &Path) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let file_format;

    // First, try to determine the file format by its content
    let fmt = FileFormat::from_bytes(&buffer);
    let name = fmt.name().to_string();
    if name != "Empty" {
        file_format = name;
    } else {
        // Fallback to using mime_guess based on the file extension
        file_format = mime_guess::from_path(file_path)
            .first()
            .map(|mime| mime.to_string())
            .unwrap_or_else(|| "Unknown file type".to_string());
    }
    Ok(file_format)
}

fn system_time_to_naive_date_time(st: SystemTime) -> DateTime<Utc> {
    let datetime: DateTime<Utc> = st.into();
    DateTime::from_timestamp(datetime.timestamp(), datetime.timestamp_subsec_nanos()).unwrap()
}
