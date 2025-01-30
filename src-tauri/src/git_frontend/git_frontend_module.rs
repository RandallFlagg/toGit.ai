use anyhow::{Context, Result};
use binaryornot::is_binary;
// use chrono::{DateTime, Utc};
// use file_format::FileFormat;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use git2::{Commit, DiffFormat, DiffOptions, Error, IndexAddOption, Repository, Signature, Status};
// use libgit2_sys::{git_repository, git_repository};
use infer;
// use lazy_static::lazy_static;
use log::debug;
// use mime_guess::from_path;
// use serde::Serialize;
// use shellexpand;
// use std::cell::{OnceCell, RefCell};
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::Read;
// use std::ops::ControlFlow;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex, OnceLock};
// use std::time::SystemTime;
use std::{
    io,
    path::{Path, PathBuf},
};
use tauri::{path, App, AppHandle, Emitter};

use crate::components::file_metadata::FileMetadata;
use crate::components::git_frontend_error::GitFrontendError;
use crate::git_frontend::app_config::AppConfig;
// #[macro_use]
// extern crate lazy_static;

// static CONFIG: OnceLock<AppConfig> = OnceLock::new();
static CONFIG: OnceLock<Mutex<AppConfig>> = OnceLock::new();
// static CONFIG: OnceCell<RefCell<AppConfig>> = OnceCell::new();
static FILE_FORMAT_CACHE: LazyLock<Mutex<HashMap<PathBuf, String>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
// lazy_static! {
//     static ref FILE_FORMAT_CACHE: Mutex<HashMap<PathBuf, String>> = Mutex::new(HashMap::new());
// }

#[tauri::command]
pub fn commit(message: &str, signer_name: &str, signer_email: &str) -> Result<String, GitFrontendError> {
    let config = CONFIG.get().unwrap().lock().unwrap();
    let repo_path = config.repo_path_as_path();

    println!("AAA1");
    // Open the repository
    let repo = Repository::open(repo_path)?;

    
    // Get the index and add all files to it
    let mut index = repo.index()?;
    println!("AAA3");
    // Commit the changes
    
    println!("AAA2");
    //TODO: handle signature logic
    // Create a signature for the committer
    let signature = Signature::now(signer_name, signer_email)?;
    println!("AAA4");
    let signature = repo.signature()?;
    println!("AAA5");

    // Check if there is a HEAD commit
    let mut parent_commits: Vec<Commit> = Vec::new();
    if let Ok(head) = repo.head() {
        // Not the initial commit, get the parent commit
        let parent_commit = head.peel_to_commit()?;
        parent_commits.push(parent_commit);
    }
    let parent_refs: Vec<&Commit> = parent_commits.iter().collect();

    let signoff_option_enabled = true; //TODO: This should be a parameter
    let signed_off_message;
    if signoff_option_enabled {
        // Append the Signed-off-by line to the commit message
        signed_off_message = format!("{}\n\nSigned-off-by: {} <{}>", message, signer_name, signer_email);
    } else {
        signed_off_message = message.to_string();
    }

    println!("AAA6");
    // Create a tree from the index
    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;
    println!("AAA7");
    // Create the commit
    repo.commit(
        Some("HEAD"),        // Update the HEAD reference
        &signature,          // Author signature
        &signature,          // Committer signature
        &signed_off_message, // Commit message
        &tree,               // Tree object
        &parent_refs,        // Parent commit
    )?;

    println!("Changes committed.");

    Ok("Commit Success".to_string())
}

#[tauri::command]
//TODO: Change all types to be mandatory? Is there a scenario where it would be useful to have an optional path?
pub fn change_file_status(relative_file_path: &Path, command: &str, new_file_path: Option<&Path>, new_files_path: Option<Vec<&Path>>) -> Result<String, GitFrontendError> {
    println!("AAA1");
    let config = CONFIG.get().unwrap().lock().unwrap();
    let repo_path = config.repo_path_as_path();
    let repo = Repository::open(repo_path)?; // Open the repository
                                             // let repo = Repository::open(repo_path).expect("Failed to open repository"); //TODO: Check which is better here
    println!("AAA2");
    let mut index = repo.index()?; // Get the index (staging area)
                                   // let mut index = repo.index().expect("Failed to get index"); //TODO: Check which is better here
    match command {
        "Add" => {
            println!("Add");
            // Add file to the index (staging area)
            index.add_path(relative_file_path)?;
            index.write()?;
            println!("File added to the index(staged).");
        }
        "Add All" => {
            println!("Add All");
            // Add all changes in the working directory to the index
            index.add_all(["*"], IndexAddOption::DEFAULT, None)?;
            index.write()?;
            println!("All the files were added to the index.");
        }
        "Add Files" => {
            println!("Add All Selected");
            // Add all changes in the working directory to the index
            index.add_all(new_files_path.unwrap(), IndexAddOption::DEFAULT, None)?;
            index.write()?;
            println!("All the selected files were added to the index.");
        }
        "Remove" => {
            // Remove file from the index
            index.remove_path(relative_file_path)?; //TODO: Change on a conflicted file
            index.write()?;
            println!("File removed from the index.");
        }
        // "Remove All" => {
        //     // Clear the index
        //     index.clear()?;
        //     index.write()?;
        //     println!("All changes removed from the index.");
        // }
        "Delete" => {
            // Delete file from the working directory and index
            let path = Path::new(relative_file_path);

            if path.exists() {
                println!("File exists.");
                fs::remove_file(relative_file_path)?;
                println!("Deleted.");
            } else {
                println!("File does not exist.");
            }

            index.remove_path(relative_file_path)?;
            index.write()?;
            println!("File deleted from the working directory and index.");
        }
        "Rename" => {
            if let Some(new_path) = new_file_path {
                // Rename file in the working directory and update the index
                fs::rename(relative_file_path, new_path)?;
                index.remove_path(relative_file_path)?;
                index.add_path(new_path)?;
                index.write()?;
                println!("File renamed and index updated.");
            } else {
                println!("New file path required for renaming.");
            }
        }
        "Untrack" => {
            // Ensure the file is removed from the index
            index.remove_path(relative_file_path)?;
            index.write()?;
            println!("File is now untracked.");
        }
        "Unstage" => {
            // Check if there is a HEAD commit
            if let Ok(head) = repo.head() {
                println!("Unstage1");
                if let Ok(commit) = head.peel_to_commit() {
                    println!("Unstage2");
                    let tree = commit.tree()?;

                    // If the file exists in the HEAD commit, add it back to the index
                    if let Ok(entry) = tree.get_path(relative_file_path) {
                        println!("Unstage3");
                        let blob_id = entry.id();

                        // Create index entry from existing entry
                        let entry = git2::IndexEntry {
                            ctime: git2::IndexTime::new(0, 0),
                            mtime: git2::IndexTime::new(0, 0),
                            dev: 0,
                            ino: 0,
                            mode: entry.filemode() as u32,
                            uid: 0,
                            gid: 0,
                            file_size: 0,
                            id: blob_id,
                            flags: 0,
                            flags_extended: 0,
                            path: relative_file_path.to_str().unwrap().as_bytes().to_vec(),
                        };

                        // Add entry to index
                        index.add(&entry)?;
                    } else {
                        // Remove the file from the index
                        println!("Unstage4");
                        index.remove_path(relative_file_path)?;
                    }
                }
            }

            // Write the index to disk
            index.write()?;
            println!("File unstaged successfully!");
        }
        "Unstage All" => {
            // Get the HEAD commit
            if let Ok(head) = repo.head() {
                if let Ok(commit) = head.peel_to_commit() {
                    let tree = commit.tree()?;

                    // Collect paths of staged files
                    let staged_paths: Vec<_> = index.iter().map(|entry| Path::new(std::str::from_utf8(&entry.path).unwrap()).to_path_buf()).collect();

                    // Modify the index
                    for path in staged_paths {
                        // If the file exists in the HEAD commit, add it back to the index
                        if let Ok(tree_entry) = tree.get_path(&path) {
                            let blob_id = tree_entry.id();

                            // Create index entry from existing entry
                            let entry = git2::IndexEntry {
                                ctime: git2::IndexTime::new(0, 0),
                                mtime: git2::IndexTime::new(0, 0),
                                dev: 0,
                                ino: 0,
                                mode: tree_entry.filemode() as u32,
                                uid: 0,
                                gid: 0,
                                file_size: 0,
                                id: blob_id,
                                flags: 0,
                                flags_extended: 0,
                                path: path.to_str().unwrap().as_bytes().to_vec(),
                            };

                            // Add entry to index
                            index.add(&entry)?;
                        } else {
                            // File is newly added, remove from index
                            index.remove_path(&path)?;
                        }
                    }
                }
            }

            // Write the index to disk
            index.write()?;
            println!("All files unstaged successfully!");
        }
        "Unstage Files" => {
            println!("Add All Selected");
            // Add all changes in the working directory to the index
            index.remove_all(new_files_path.unwrap(), None)?;
            index.write()?;
            println!("All the selected files were added to the index.");
        }
        _ => {
            println!("Unknown command."); //TODO: throw an error here
        }
    }

    Ok("Changed file status successfully!".to_string()) //TODO: Change return type
}

/// Retrieves the content of a file in the specified repository and relative path.
///
/// # Arguments
///
/// * `repo_path` - A reference to a `Path` that holds the path to the Git repository.
/// * `relative_file_path` - A reference to a `Path` that holds the relative path to the file within the repository. The file path should not be prefixed with a '/'.
///
/// # Returns
///
/// * `Result<String, GitFrontendError>` - The content of the file as a `String` on success, or a `GitFrontendError` on failure.
///
/// # Example
///
/// ```rust
/// fn example() -> Result<(), GitFrontendError> {
///     let repo_path = PathBuf::from("../../TEST REPO");
///     let relative_file_path = PathBuf::from("TEST1/b.txt");
///     let content = get_file_content(repo_path, relative_file_path)?;
///     println!("{}", content);
///
///     let repo_path = PathBuf::from("../../TEST REPO");
///     let relative_file_path = PathBuf::from("a");
///     let content = get_file_content(repo_path, relative_file_path)?;
///     println!("{}", content);
///
///     Ok(())
/// }
/// ```
#[tauri::command]
pub fn get_file_content(repo_path: PathBuf, relative_file_path: PathBuf) -> Result<String, GitFrontendError> {
    println!("Relative Repo Path: {}", repo_path.to_string_lossy());
    let absolute_repo_path = fs::canonicalize(&repo_path).map_err(|e| GitFrontendError::OtherError(e.to_string()))?; //TODO: Do we need todo this?
    println!("Absolute Repo Path: {}", absolute_repo_path.to_string_lossy());
    println!("Relative File Path: {}", relative_file_path.to_string_lossy());
    let full_file_path = absolute_repo_path.join(&relative_file_path.strip_prefix("/").unwrap_or(&relative_file_path));
    println!("Full File Path: {}", full_file_path.to_string_lossy());
    let contents;
    if is_binary(full_file_path).expect("unable to read file") {
        contents = "Binary file".to_string();
    } else {
        // let mut file = fs::File::open(&full_file_path).map_err(|e| e.to_string())?;
        // file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

        // Resolve the full path from the relative path
        // let absolute_file_path = fs::canonicalize(&full_file_path).map_err(|e| GitFrontendError::Other(e.to_string()))?; //TODO: Do we need todo this?
        // println!("Absolute File Path: {}", absolute_file_path.to_string_lossy());
        // let relative_file_path;
        // match absolute_file_path.strip_prefix(&absolute_repo_path) {
        //     Ok(prefix) => {
        //         relative_file_path = prefix;
        //         println!("Prefix: {}", prefix.to_string_lossy())
        //     }
        //     Err(e) => {
        //         eprintln!("Error getting file content: {}", e);
        //         return Err(GitFrontendError::from(e.to_string()));
        //     }
        // };
        println!("Relative File Path: {}", relative_file_path.to_string_lossy());
        contents = generate_file_diff_with_git2(absolute_repo_path, relative_file_path.to_path_buf())?;
        println!("Content: {}", contents);
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
//TODO: Chang to anyhow erro handling to see if it is better than the current soution.
// #[log_macro::log]
//pub fn get_repo_status() -> Result<Vec<FileMetadata>, GitFrontendError> {
//pub fn get_repo_status() -> Result<> {
pub fn get_repo_status() -> Result<Vec<FileMetadata>, GitFrontendError> {
    println!("Entering get_repo_status");
    let config = CONFIG.get().unwrap().lock().unwrap();
    let repo_path = config.repo_path_as_pathbuf();
    debug!("Repo Path: {}", repo_path.to_string_lossy());

    // Open the repository with context for error
    let repo = Repository::open(repo_path).context("Failed to open Git repository")?;

    // Get the status options and status entries
    let statuses = repo.statuses(None).context("Failed to get repository statuses")?;

    // Get the list of changes
    let mut changes = Vec::new();
    for entry in statuses.iter() {
        let mut full_file_path = PathBuf::from(repo_path);
        full_file_path.push(
            entry
                .path()
                .ok_or_else(|| git2::Error::from_str("Invalid UTF-8 path"))
                .context("Failed to get entry path")?,
        );

        if full_file_path.is_dir() {
            //TODO: What should happen in case of a dir?
            continue;
        }

        let status = entry.status();
        // Determine the type of change
        let change_type = match to_status_string(&status) {
            Some(value) => value,
            None => continue,
        };

        let file = get_file_metadata(&full_file_path, change_type, repo_path)
            .with_context(|| format!("Failed to get file metadata for {:?} with status: {}", full_file_path, change_type))?;
        changes.push(file);
    }
    println!("Exiting get_repo_status");
    Ok(changes)
}

fn to_status_string(status: &Status) -> Option<&str> {
    let change_type = match status {
        _ if status.contains(Status::INDEX_NEW) => "INDEX_NEW",
        _ if status.contains(Status::WT_NEW) => "WT_NEW",
        _ if status.contains(Status::INDEX_DELETED) => "INDEX_DELETED",
        _ if status.contains(Status::WT_DELETED) => "WT_DELETED",
        _ if status.contains(Status::INDEX_MODIFIED) => "INDEX_MODIFIED",
        _ if status.contains(Status::WT_MODIFIED) => "WT_MODIFIED",
        _ if status.contains(Status::INDEX_RENAMED) => "INDEX_RENAMED",
        _ if status.contains(Status::WT_RENAMED) => "WT_RENAMED",
        _ if status.contains(Status::INDEX_TYPECHANGE) => "INDEX_TYPECHANGE",
        _ if status.contains(Status::WT_TYPECHANGE) => "WT_TYPECHANGE",
        _ if status.contains(Status::IGNORED) => "IgnIGNORED",
        _ if status.contains(Status::CONFLICTED) => "CONFLICTED",
        _ => return None, //"Unknown" //TODO: This should be changed to an error?
    };
    Some(change_type)
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

fn get_file_metadata<P: AsRef<Path>>(full_file_path: P, status: &str, repo_root: &Path) -> Result<FileMetadata, GitFrontendError> {
    let path_ref: &Path = full_file_path.as_ref();
    debug!("file0a");
    static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
    debug!("file0b");
    let file_name = path_ref.file_name().expect("File not found").to_str().expect("Invalid UTF-8 in file name").to_string();
    debug!("file0c");
    let mut file_format = "".to_string();
    let mut file_size = "".to_string();
    if status != "INDEX_DELETED" && status != "WT_DELETED" {
        debug!("file0d");
        // Process file
        file_size = fs::File::open(path_ref)?.metadata()?.len().to_string(); // size: format!("{:.2} KB", file.metadata()?.len() as f64 / 1024.0), // Get size in KB //TODO: Format
        file_format = determine_file_format(path_ref)?; //TODO: Cache the result of this call
                                                        // let metadata = metadata(path_ref)?;
                                                        // let modified_time = metadata.modified().unwrap_or(SystemTime::now());
                                                        // let modified_at = system_time_to_naive_date_time(modified_time);
    }
    debug!("file0e");
    let file_metadata = FileMetadata {
        id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
        // change_type: change_type.to_string(),
        file_name: file_name,
        full_file_path: path_ref.to_string_lossy().to_string(), //TODO: Convert to base64 in order to handle non utf-8 path. Need to check on what OS this is valid to make sure we need to handle this.
        relative_file_path: path_ref.strip_prefix(repo_root).unwrap_or(path_ref).to_string_lossy().to_string(), //TODO: Make this path relative to the file and not as it is now, full. //TODO: Convert to base64 in order to handle non utf-8 path. Need to check on what OS this is valid to make sure we need to handle this.
        file_extension: path_ref.extension().and_then(|ext| ext.to_str()).unwrap_or("").to_string(),
        file_type: file_format,
        file_status: status.to_string(),
        size: file_size,
        // created_by: String::from("User A"),
        // created_at: String::from("2024-01-01T00:00:00Z"),
        // modified_by: String::from("User B"),
        // modified_at: modified_at.to_string(),
        // comments: String::from("No comments"),
        // preview: "Loading...".to_string(), // Use file content as preview
        // selected: false,
    };
    debug!("file0f");
    Ok(file_metadata)
}

fn determine_file_format(file_path: &Path) -> Result<String, GitFrontendError> {
    determine_file_format2(file_path)
}

//Using FileFormat and MimeGuess
fn determine_file_format1(file_path: &Path) { //-> io::Result<String> {
                                              // let mut file = File::open(file_path)?;
                                              // let mut buffer = vec![0; 512]; // Define a buffer with 512 bytes
                                              // let bytes_read = file.read(&mut buffer)?;
                                              // if bytes_read == 0 {
                                              //     //TODO: Decide how we want to handle this
                                              //     return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "File is empty or could not be read"));
                                              // }
                                              // buffer.truncate(bytes_read);

    // let file_format;

    // // First, try to determine the file format by its content
    // let fmt = FileFormat::from_bytes(&buffer);
    // let name = fmt.name().to_string();
    // if name != "Empty" {
    // file_format = name;
    // } else {
    //     // Fallback to using mime_guess based on the file extension
    //     file_format = mime_guess::from_path(file_path)
    //         .first()
    //         .map(|mime| mime.to_string())
    //         .unwrap_or_else(|| "Unknown file type".to_string());
    // }
    // Ok(file_format)
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

// fn system_time_to_naive_date_time(st: SystemTime) -> DateTime<Utc> {
//     let datetime: DateTime<Utc> = st.into();
//     DateTime::from_timestamp(datetime.timestamp(), datetime.timestamp_subsec_nanos()).unwrap()
// }

/// Generates a diff string between a file in the working tree and the index.
/// If the file is not in the working tree, it returns the diff against an empty file.
/// The file name should be relative to the repo path
fn generate_file_diff_with_git2(repo_path: PathBuf, relative_file_name: PathBuf) -> Result<String, Error> {
    println!("Working with File: {}", relative_file_name.to_string_lossy());
    println!("Working with Repo: {}", repo_path.to_string_lossy());
    // Open the repository
    let repo = Repository::open(&repo_path)?;
    println!("{}", repo.path().to_string_lossy());
    println!("work tree: {}", repo.is_worktree());
    // Prepare diff options
    let mut diff_options = DiffOptions::new();
    // let full_file_path = repo_path.join(file_name.clone());
    // println!("Full File Path: {}",full_file_path.to_string_lossy());
    diff_options.pathspec(&relative_file_name); //TODO: Remove the clone
    println!("Diff options set for file: {}", relative_file_name.to_string_lossy());

    // Generate the diff
    let diff = repo.diff_index_to_workdir(None, Some(&mut diff_options))?;
    println!("diff stats: {:?}", diff.stats());
    // Collect the diff output
    let mut diff_output: String = String::new();
    diff.print(DiffFormat::Patch, |_, _, line| {
        println!("{}", "c1");
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
        println!("Line processed: {}", std::str::from_utf8(line.content()).unwrap());
        true
    })?;
    println!("{}", "D");
    println!("DIFF: {}", diff_output);
    Ok(diff_output)
}

// #[log_macro::log]
//TODO: Change to init?
pub fn is_git_repo(path: Option<PathBuf>) -> bool {
    // // Initialize CONFIG if not already initialized
    CONFIG.set(Mutex::new(AppConfig::default())).unwrap_or_else(|_| {
        eprintln!("CONFIG is already initialized");
    });

    // if !repo_path.exists() || !repo_path.is_dir() {
    //     return Err(GitFrontendError::InvalidPath("Invalid repository path".to_string()));
    // }

    // Determine the path based on whether a parameter was passed
    let repo_path = match path {
        Some(path) => {
            println!("The CWD is: {}", env::current_dir().expect("Failed to get current directory").display());
            // Use the first parameter as the path and convert it to an absolute path
            path.canonicalize().expect("Failed to get absolute path")
        }
        None => {
            // Use the current directory and convert it to an absolute path
            debug!("Using the current directory.");
            env::current_dir().expect("Failed to get current directory")
        }
    };

    // Print the absolute path
    debug!("The repo path is: {}", repo_path.display());

    // Try to open the repository
    match Repository::discover(&repo_path) {
        Ok(repo) => {
            println!("This is a Git repository.");
            // Initialize the global variable if not already initialized
            let mut config = CONFIG.get().unwrap().lock().unwrap();
            debug!("AAA1");
            config.set_repo_path(repo_path.to_str().expect("Failed to set repo_path global variable"));
            debug!("AAA2");
            println!("Repository path: {}", config.repo_path_as_str());
            debug!("DONE is_git_repo");
            return true;
        }
        Err(_) => {
            eprintln!("Error: The specified path is not a Git repository.");
            // exit(1)
            return false;
        }
    }
}

//TODO: This is for file watcher. In logic moduile we have more code
#[tauri::command]
fn trigger_event(app: AppHandle) {
    app.emit("custom-event", &"Hello from Rust!").unwrap();
}

//TODO: Functions from here can be deleted?
/*
fn get_repo_tracked() -> Result<Vec<FileMetadata>, GitFrontendError> {
    let config = CONFIG.get().unwrap().lock().unwrap();
    let repo_path = config.repo_path_as_path();
    // Open the repository
    let repo = Repository::open(repo_path)?;
    // Get the index (staging area)
    let index = repo.index().expect("Failed to get index");
    // Collect all tracked files
    // let mut tracked_files = Vec::new();
    // let repo_root = CONFIG.get().unwrap().lock().unwrap().repo_path_as_path();

    let tracked_files = index
        .iter()
        .map(|entry| {
            let path = String::from_utf8(entry.path.clone()).expect("Invalid UTF-8 sequence");
            get_file_metadata(PathBuf::from(path), "status11", repo_path)
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Print the tracked files
    for file in &tracked_files {
        println!("{:?}", file);
    }

    Ok(tracked_files)
}


//fn use_library(settings: State<'_, Mutex<Settings>>) {
fn use_library() {
    // let config = CONFIG.get().unwrap().lock().unwrap();
    // match *config {
    //     Some(ref config) => {
    //         println!("Using library with parameter: {}", config.parameter);
    //     }
    //     None => {
    //         println!("Error: Library is not initialized. Please call init_library first.");
    //     }
    // }
    todo!("What do we want to do here?")
}
*/
