use chrono::{DateTime, Utc};
use file_format::FileFormat;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use git2::{Repository, DiffOptions, Error, DiffFormat, DiffLineType, BranchType, Oid, Status};
// use libgit2_sys::{git_repository, git_repository};
use crate::components::file_metadata::FileMetadata;
use crate::components::git_frontend_error::GitFrontendError;
use binaryornot::is_binary;
use infer;
use log::debug;
use mime_guess::from_path;
use serde::Serialize;
use shellexpand;
use std::collections::{HashMap, HashSet};
use std::fs::{self, metadata, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::SystemTime;
use std::{io, path};
use std::sync::Mutex;
use lazy_static::lazy_static;

// #[macro_use]
// extern crate lazy_static;

lazy_static! {
    static ref FILE_FORMAT_CACHE: Mutex<HashMap<PathBuf, String>> = Mutex::new(HashMap::new());
}


#[tauri::command]
pub fn get_file_content(full_file_path: PathBuf) -> Result<String, GitFrontendError> {
    let mut contents=String::new();
    if is_binary(&full_file_path).expect("unable to read file") {
        contents = "Binary file".to_string();
    } else {
        // let mut file = fs::File::open(&full_file_path).map_err(|e| e.to_string())?;
        // file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
        generate_diff(repo_path, full_file_path)?;
    }

    Ok(contents)
}

/// Retrieves the status of a Git repository at the specified path.
///
/// # Arguments
///
/// * `repo_path` - A reference to a `Path` that holds the path to the Git repository.
///
/// # Returns
///
/// * `Result<Vec<FileMetadata>, GitFrontendError>` - A vector of `FileMetadata` on success, or a `GitFrontendError` on failure.
#[tauri::command]
pub fn get_repo_status(repo_path: &Path) -> Result<Vec<FileMetadata>, GitFrontendError> {
    if !repo_path.exists() || !repo_path.is_dir() {
        return Err("Invalid repository path".to_string());
    }

    // Call the internal function to get the repository status
    match get_repo_status_internal(repo_path) {
        Ok(status) => Ok(status),
        Err(e) => Err(e), // Propagate the error from the internal function
    }
    // get_repo_status_internal(repo_path).map_err(|e| e.to_string())
}

fn get_repo_status_internal(repo_path: &Path) -> Result<Vec<FileMetadata>, GitFrontendError> {
    // Open the repository
    let repo = Repository::open(repo_path)?;
    // Get the status options and status entries
    let statuses = repo.statuses(None)?;
    // Get the list of changes
    let mut changes = Vec::new();
    for entry in statuses.iter() {
        let mut full_file_path = PathBuf::from(repo_path);
        full_file_path.push(entry.path().ok_or(git2::Error::from_str("Invalid UTF-8 path"))?);
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
        let file = get_file_metadata(&full_file_path, change_type, repo_path)?;
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

fn get_file_metadata(full_file_path: &PathBuf, status: &str, repo_root: &Path) -> Result<FileMetadata, GitFrontendError> {
    static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
    let file_name = full_file_path
        .file_name()
        .expect("File not found")
        .to_str()
        .expect("Invalid UTF-8 in file name")
        .to_string();
    let file = fs::File::open(full_file_path)?;
    let file_format = determine_file_format2(full_file_path)?; //TODO: Cache the result of this call
    let metadata = metadata(full_file_path)?;
    let modified_time = metadata.modified().unwrap_or(SystemTime::now());
    let modified_at = system_time_to_naive_date_time(modified_time);
    let file_metadata = FileMetadata {
        id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
        // change_type: change_type.to_string(),
        file_name: file_name,
        full_file_path: full_file_path.to_string_lossy().to_string(), //TODO: Convert to base64 in order to handle non utf-8 path. Need to check on what OS this is valid to make sure we need to handle this.
        relative_file_path: full_file_path.strip_prefix(repo_root).unwrap_or(full_file_path).to_string_lossy().to_string(), //TODO: Make this path relative to the file and not as it is now, full. //TODO: Convert to base64 in order to handle non utf-8 path. Need to check on what OS this is valid to make sure we need to handle this.
        file_extension: full_file_path.extension().and_then(|ext| ext.to_str()).unwrap_or("").to_string(),
        file_type: file_format,
        file_status: status.to_string(),
        size: file.metadata()?.len().to_string(), // size: format!("{:.2} KB", file.metadata()?.len() as f64 / 1024.0), // Get size in KB //TODO: Format
                                                  // created_by: String::from("User A"),
                                                  // created_at: String::from("2024-01-01T00:00:00Z"),
                                                  // modified_by: String::from("User B"),
                                                  // modified_at: modified_at.to_string(),
                                                  // comments: String::from("No comments"),
                                                  // preview: "Loading...".to_string(), // Use file content as preview
                                                  // selected: false,
    };

    Ok(file_metadata)
}

fn determine_file_format(file_path: &Path) -> io::Result<String> {
    determine_file_format1(file_path)
}

//Using FileFormat and MimeGuess
fn determine_file_format1(file_path: &Path) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = vec![0; 512]; // Define a buffer with 512 bytes
    let bytes_read = file.read(&mut buffer)?;
    if bytes_read == 0 {
        //TODO: Decide how we want to handle this
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "File is empty or could not be read"));
    }
    buffer.truncate(bytes_read);

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

//Caching
fn determine_file_format2(file_path: &Path) -> Result<String, GitFrontendError> {
    let mut cache = FILE_FORMAT_CACHE.lock().unwrap();
    if let Some(format) = cache.get(file_path) {
        return Ok(format.clone());
    }

    let mut file = File::open(file_path).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let mut buffer = [0; 512];
    file.read(&mut buffer).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let format = if let Some(kind) = infer::get(&buffer) {
        kind.mime_type().to_string()
    } else {
        "unknown".to_string()
    };

    cache.insert(file_path.to_path_buf(), format.clone());
    Ok(format)
}

//Using infer
fn determine_file_format3(file_path: &Path) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = [0; 512]; // Read the first 512 bytes
    file.read(&mut buffer)?;

    if let Some(kind) = infer::get(&buffer) {
        return Ok(kind.mime_type().to_string());
    }

    Ok("unknown".to_string())
}

fn system_time_to_naive_date_time(st: SystemTime) -> DateTime<Utc> {
    let datetime: DateTime<Utc> = st.into();
    DateTime::from_timestamp(datetime.timestamp(), datetime.timestamp_subsec_nanos()).unwrap()
}

/// Generates a diff string between a file in the working tree and the index.
/// If the file is not in the working tree, it returns the diff against an empty file.
/// The file name should be relative to the repo path
fn generate_diff(repo_path: &str, file_name: &str) -> Result<String, Error> {
    // Open the repository
    let repo = Repository::open(repo_path)?;

    // Get the index
    let index = repo.index()?;

    // Get the OID of the file in the index
    let oid = index.get_path(Path::new(file_name), 0)
        .ok_or_else(|| Error::from_str("File not found in index"))?
        .id;

    // Read the file content from the index
    let blob = repo.find_blob(oid)?;
    let index_content = String::from_utf8_lossy(blob.content());

    // Read the file content from the working tree
    let working_tree_content = fs::read_to_string(Path::new(repo_path).join(file_name))
        .unwrap_or_else(|_| String::new());

    // Generate the diff
    let diff =generate_diff_with_git2(repo_path, file_name)?;

    Ok(diff)
}

fn generate_diff_with_git2(repo_path: &str, file_name: &str) -> Result<String, Error> {
    // Open the repository
    let repo = Repository::open(repo_path)?;

    // Prepare diff options
    let mut diff_options = DiffOptions::new();
    diff_options.pathspec(file_name);

    // Generate the diff
    let diff = repo.diff_index_to_workdir(None, Some(&mut diff_options))?;

    // Collect the diff output
    let mut diff_output = String::new();
    diff.print(DiffFormat::Patch, |_, _, line| {
        let prefix = match line.origin() {
            '+' => '+',
            '-' => '-',
            ' ' => ' ',
            '?' => '?',
            '!' => '!',
            '~' => '~',
            '/' => '/',
            '\\' => '\\',
            '`' => '`',
            'R' => 'R',
            // ' ' => ' ',
             // Add more cases as needed
            _ => ' ',
        };
        diff_output.push_str(&format!("{}{}", prefix, std::str::from_utf8(line.content()).unwrap()));
        true
    })?;

    Ok(diff_output)
}