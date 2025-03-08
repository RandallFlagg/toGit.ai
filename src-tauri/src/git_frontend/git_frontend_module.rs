use anyhow::{Context, Result};
use binaryornot::is_binary;
// use chrono::{DateTime, Utc};
// use file_format::FileFormat;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use git2::{
    BranchType, Commit, Config, DiffFormat, DiffOptions, Error, FetchOptions, IndexAddOption, RemoteCallbacks, Repository, RepositoryInitOptions, Signature, Status,
    SubmoduleUpdateOptions, Time,
};
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
use std::time::{Duration, SystemTime};
// use std::time::SystemTime;
use std::{
    io,
    path::{Path, PathBuf},
};
use tauri::{path, App, AppHandle, Emitter};

use crate::components::file_metadata::FileMetadata;
use crate::components::git_frontend_error::GitFrontendError;
use crate::components::branch_info::BranchInfo;
use crate::components::repo_details::RemoteDetails;
use crate::components::repo_details::RepoDetails;
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

// #[tauri::command]
// fn show_menu() -> Vec<&'static str> {
//     // println!("Showing Menu");
//     // vec!["Hello", "Hi", "Hey"]
//     unimplemented!("This function is not yet implemented.");
// }

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
// #[tauri::command]
// fn trigger_event(app: AppHandle) {
//     app.emit("custom-event", &"Hello from Rust!").unwrap();
// }

#[tauri::command]
pub fn clone(url: &str, path: &Path, depth: usize, recursive: bool) -> Result<String, GitFrontendError> {
    println!("Cloning repository with Git: {}", url);
    println!("Cloning into: {}", path.to_string_lossy());
    println!("Recursive: {}", recursive);

    //TODO: This is in case we want to make sure we can clone a repo with a specific depth
    // Check if the repository path already exists
    // if path.exists() {
    //     println!("Path already exists. Removing it...");
    //     fs::remove_dir_all(path).expect("Failed to remove existing directory");
    // }

    // let repo;
    // // Clone the repository with the specified URL and into the specified path
    // if recursive {
    //     repo = Repository::clone_recurse(url, path);
    // } else {
    //     repo = Repository::clone(url, path);
    // }
    //TODO: add an option to handle a case where low depth is not enough
    let mut fetch_options = FetchOptions::new();
    let callbacks = RemoteCallbacks::new();

    fetch_options.remote_callbacks(callbacks);
    if depth > 0 {
        fetch_options.depth(i32::try_from(depth).unwrap()); // Set the clone depth here
    }

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);

    let repo = builder.clone(url, path)?;

    println!("AAAA");
    //TODO: Catch a case where cloning with low depth fails - OIV test case
    if recursive {
        // Create new FetchOptions for submodule updates
        let mut submodule_fetch_options = FetchOptions::new();
        if depth > 0 {
            submodule_fetch_options.depth(i32::try_from(depth).unwrap());
        }

        let mut submodule_options = SubmoduleUpdateOptions::new();
        submodule_options.fetch(submodule_fetch_options); // Set fetch options for submodules
        println!("BBBB");
        for mut submodule in repo.submodules()? {
            submodule.init(true)?;
            submodule.update(true, Some(&mut submodule_options))?;
        }
    }

    println!("Repository cloned successfully!");

    // Ok(repo)
    Ok("Repository cloned successfully".to_string())
    // repo
}

#[tauri::command]
pub(crate) fn set_repo_details(details: RepoDetails) -> Result<String, Error> {
    Ok("".to_string())
}

fn find_default_remote(config: &Config, key: &str) -> Option<String> {
    config.get_string(key).ok()
}

fn list_branches_for_remote(repo: &Repository, remote_name: &str, direction: &str) -> Result<Vec<String>, Error> {
    let mut results = Vec::new();
    let branches = repo.branches(Some(BranchType::Local))?;
    for branch in branches {
        let (branch, _) = branch?;
        let name = branch.name()?.unwrap_or_default().to_string();
        let branch_remote = repo.config()?.get_string(&format!("branch.{}.remote", name)).unwrap_or_default();
        debug!("branch_remote: {}", branch_remote);
        debug!("remote_name: {}", remote_name);
        if branch_remote == remote_name {
            debug!("1");
            let merge_or_push = match direction {
                "pull" => repo.config()?.get_string(&format!("branch.{}.merge", name)).unwrap_or_default(),
                "push" => repo.config()?.get_string(&format!("branch.{}.push", name)).unwrap_or_default(),
                _ => String::new(),
            };
            debug!("2");
            if direction == "pull" && !merge_or_push.is_empty() {
                debug!("3");
                let rebase = repo.config()?.get_bool(&format!("branch.{}.rebase", name)).unwrap_or(false);
                if rebase {
                    results.push(format!(
                        "{}      rebases onto remote {}",
                        name,
                        merge_or_push.strip_prefix("refs/heads/").unwrap_or(&merge_or_push)
                    ));
                } else {
                    results.push(format!(
                        "{}      merges with remote {}",
                        name,
                        merge_or_push.strip_prefix("refs/heads/").unwrap_or(&merge_or_push)
                    ));
                }
            } else if direction == "push" && !merge_or_push.is_empty() {
                debug!("3");
                let is_up_to_date = check_if_up_to_date(repo, &name, &merge_or_push)?;
                debug!("3.1");
                let status = if is_up_to_date { "up to date" } else { "needs update" };
                debug!("3.2");
                results.push(format!(
                    "{}      pushes to {}    ({})",
                    name,
                    merge_or_push.strip_prefix("refs/heads/").unwrap_or(&merge_or_push),
                    status
                ));
                debug!("3.3");
            }
        }
    }
    Ok(results)
}

fn list_remote_branches(repo: &Repository, remote_name: &str) -> Result<Vec<BranchInfo>, Error> {
    let mut results = Vec::new();
    let branches = repo.branches(Some(BranchType::Remote))?;
    for branch in branches {
        let (branch, _) = branch?;
        let name = branch.name()?.unwrap_or_default().to_string();

        if name.starts_with(&format!("{}/", remote_name)) {
            let commit = branch.get().peel_to_commit()?;
            let commit_time = commit.time();
            let commit_age = get_commit_age(commit_time);
            let tracking_branch = find_tracking_branch(repo, &name)?;
            let last_commit_message = commit.message().unwrap_or("No commit message").to_string();
            let last_commit_author = commit.author().name().unwrap_or("Unknown author").to_string();
            let is_merged = is_branch_merged(repo, &name)?;

            let branch_info = BranchInfo {
                name,
                commit_age,
                tracking_branch,
                last_commit_message,
                last_commit_author,
                is_merged,
            };

            results.push(branch_info);
        }
    }
    Ok(results)
}

fn find_tracking_branch(repo: &Repository, remote_branch_name: &str) -> Result<Option<String>, Error> {
    let branches = repo.branches(Some(BranchType::Local))?;
    for branch in branches {
        println!("####################################################################");
        let (branch, _) = branch?;
        let name = branch.name()?.unwrap_or_default().to_string();
        let upstream_ref = repo.config()?.get_string(&format!("branch.{}.merge", name)).unwrap_or_default();

        // Debug prints to understand the issue
        println!("Checking if local branch '{}' merges with remote branch '{}'", name, remote_branch_name);
        println!("Local branch merge ref: '{}'", upstream_ref);
        println!("Remote branch name: '{}'", remote_branch_name);

        if !upstream_ref.is_empty() && remote_branch_name.ends_with(upstream_ref.strip_prefix("refs/heads/").unwrap_or(&upstream_ref)) {
            return Ok(Some(name));
        }
    }
    Ok(None)
}

fn branch_remote(remote_branch_name: &str) -> Result<&str, Error> {
    let parts: Vec<&str> = remote_branch_name.split('/').collect();
    if parts.len() > 1 {
        Ok(parts[0])
    } else {
        Err(Error::from_str("Invalid remote branch name"))
    }
}

fn check_if_up_to_date(repo: &Repository, branch_name: &str, remote_branch_ref: &str) -> Result<bool, Error> {
    debug!("check_if_up_to_date enter");
    if remote_branch_ref.is_empty() {
        return Err(Error::from_str("Remote branch reference is empty."));
    }

    let branch = repo.find_branch(branch_name, BranchType::Local)?;
    debug!("0.1");
    let branch_commit = branch.get().peel_to_commit()?;
    debug!("0.2");
    let branch_oid = branch_commit.id();
    debug!("0.3");

    // Check if the remote branch reference is valid and exists
    let remote_branch = match repo.revparse_single(remote_branch_ref) {
        Ok(branch) => branch,
        Err(e) => {
            println!("Failed to resolve remote branch ref '{}': {}", remote_branch_ref, e);
            return Err(Error::from_str(&format!("Failed to resolve remote branch ref '{}': {}", remote_branch_ref, e)));
        }
    };
    debug!("0.4");
    let remote_commit = remote_branch.peel_to_commit()?;
    debug!("0.5");
    let remote_oid = remote_commit.id();
    debug!("check_if_up_to_date exit");
    Ok(branch_oid == remote_oid)
}

fn is_branch_merged(repo: &Repository, branch_name: &str) -> Result<bool, Error> {
    let branch = repo.find_branch(branch_name, BranchType::Remote)?;
    let branch_commit = branch.get().peel_to_commit()?;
    let merge_base = repo.merge_base(repo.head()?.target().unwrap(), branch_commit.id())?;
    Ok(merge_base == branch_commit.id())
}

fn get_commit_age(commit_time: Time) -> String {
    let commit_timestamp = commit_time.seconds();
    let commit_duration = SystemTime::UNIX_EPOCH + Duration::from_secs(commit_timestamp as u64);
    let now = SystemTime::now();
    let age = now.duration_since(commit_duration).unwrap_or_else(|_| Duration::from_secs(0));

    let days = age.as_secs() / 86400;
    let hours = (age.as_secs() % 86400) / 3600;
    let minutes = (age.as_secs() % 3600) / 60;

    if days > 0 {
        format!("{} days ago", days)
    } else if hours > 0 {
        format!("{} hours ago", hours)
    } else {
        format!("{} minutes ago", minutes)
    }
}

fn get_push_status(repo: &Repository, branch_name: &str, remote_name: &str) -> Result<Option<String>, Error> {
    println!("5");
    let local_branch = repo.find_branch(branch_name, BranchType::Local)?;
    println!("6");
    let local_commit = local_branch.get().peel_to_commit()?;
    println!("7");
    let local_oid = local_commit.id();
    println!("8");

    let remote_branch_ref = format!("refs/remotes/{}/{}", remote_name, branch_name);
    println!("9");

    // Check if the remote branch reference is valid and exists
    let remote_branch = match repo.find_reference(&remote_branch_ref) {
        Ok(branch) => branch,
        Err(e) => {
            println!("Failed to resolve remote branch ref '{}': {}", remote_branch_ref, e);
            return Ok(None);// Ok("remote branch not found".to_string()); // Gracefully handle the error
        },
    };
    
    println!("10");
    let remote_commit = remote_branch.peel_to_commit()?;
    println!("1");
    let remote_oid = remote_commit.id();
    println!("12");

    let (ahead, behind) = repo.graph_ahead_behind(local_oid, remote_oid)?;
    println!("13");

    let status = if ahead == 0 && behind == 0 {
        "up to date".to_string()
    } else if ahead > 0 && behind == 0 {
        format!("{} commits ahead (remote out of date)", ahead)
    } else if ahead == 0 && behind > 0 {
        format!("{} commits behind (local out of date)", behind)
    } else {
        format!("{} commits ahead, {} commits behind (local and remote are out of date - divergent)", ahead, behind)
    };

    println!("14");
    Ok(Some(status))
}

fn list_branches_for_remote_push_status(repo: &Repository, remote_name: &str) -> Result<Vec<String>, Error> {
    println!("1");
    let mut results = Vec::new();
    println!("2");
    let branches = repo.branches(Some(BranchType::Local))?;
    println!("3");
    for branch in branches {
        let (branch, _) = branch?;
        let name = branch.name()?.unwrap_or_default().to_string();

        // Get push status
        match get_push_status(repo, &name, remote_name)? {
            Some(status) => results.push(format!("{} pushes to {} ({})", name, name, status)),
            None => continue,//results.push(format!("{} does not have a corresponding remote branch", name)),
        }
    }
    println!("4");
    Ok(results)
}

#[tauri::command]
pub(crate) fn get_repo_details() -> Result<RepoDetails, GitFrontendError> {
    let config = CONFIG.get().unwrap().lock().unwrap();
    let repo_path = config.repo_path_as_path();
    let repo = Repository::open(repo_path)?;

    // Repository name (assuming the directory name as repo name)
    // let repo_name = repo_path.to_str().unwrap().split('/').last().unwrap_or("").to_string();

    // Repository description (read from .git/description if available)
    let description = repo.path().parent().and_then(|path| std::fs::read_to_string(path.join("description")).ok());

    // Repository URL (read from .git/config)
    // let config = repo.config()?;
    // let remote_url = config.get_string("remote.origin.url").unwrap_or_else(|_| "No URL found".to_string());

    let mut remotes_details = Vec::new();
    // Get the list of all remotes
    let remotes = repo.remotes()?;

    // Iterate over each remote
    for remote_name in remotes.iter().filter_map(|r| r) {
        //.flatten() {
        // Get the remote by name
        let remote = repo.find_remote(remote_name)?;

        // Fetch URL and Push URL
        let fetch_url = remote.url().unwrap_or("No fetch URL found");
        let push_url = remote.pushurl().unwrap_or("No push URL found");

        debug!("* remote {}", remote_name);
        debug!("  Fetch URL: {}", fetch_url);
        debug!("  Push URL: {}", push_url);

        // Fetch Refspecs
        debug!("  Fetch Refspecs:");
        let fetch_urls = {
            let fetch_refspecs = remote.fetch_refspecs().unwrap();
            let mut temp_urls = Vec::new();
            for fetch_refspec in fetch_refspecs.iter() {
                let refspec = fetch_refspec.ok_or("Invalid refspec").unwrap();
                debug!("{}", refspec);
                temp_urls.push(refspec.to_string());
            }
            temp_urls
        };

        // Push Refspecs
        debug!("  Push Refspecs:");
        let push_urls = {
            let push_refspecs = remote.push_refspecs().unwrap();
            let mut temp_urls = Vec::new();
            for push_refspec in push_refspecs.iter() {
                let refspec = push_refspec.ok_or("Invalid refspec").unwrap();
                debug!("{}", refspec);
                temp_urls.push(refspec.to_string());
            }
            temp_urls
        };
        debug!("");
        let pull_local_branches = list_branches_for_remote(&repo, remote_name, "pull")?;
        let push_status = list_branches_for_remote_push_status(&repo, remote_name)?;
        let push_local_branches = push_status; //list_branches_for_remote(&repo, remote_name, "push")?;
        let remote_branches = list_remote_branches(&repo, remote_name)?;
        let remote_details: RemoteDetails = RemoteDetails {
            name: remote_name.to_string(),
            fetch_url: fetch_url.to_string(),
            push_url: push_url.to_string(),
            fetch_refspecs: fetch_urls,
            push_refspecs: push_urls,
            local_branches_configured_for_git_pull: pull_local_branches,
            local_branches_configured_for_git_push: push_local_branches,
            remote_branches: remote_branches,
        };
        debug!("remote_details end");
        remotes_details.push(remote_details);
    }

    // let branches = repo
    //     .branches(None)?
    //     .filter_map(|branch| branch.ok())
    //     .filter_map(|(branch, _)| branch.name().ok().flatten().map(|name| name.to_string()))
    //     .collect::<Vec<_>>();

    let head = repo.head()?;

    // Get shorthand name
    let default_branch_name = head.shorthand().unwrap_or("No branch found").to_string();

    // Get full reference name
    let default_full_branch_name = head.name().unwrap_or("No branch found").to_string();

    let tags = repo.tag_names(None)?.iter().filter_map(|name| name.map(String::from)).collect::<Vec<_>>();

    //Get the default pull and push remotes
    let config = repo.config()?;

    let default_push_remote =
        match find_default_remote(&config, &format!("remote.pushDefault")).or_else(|| find_default_remote(&config, &format!("branch.{}.remote", default_branch_name))) {
            Some(push) => push,
            None => "No default push remote found".to_string(),
        };
    let default_pull_remote = match find_default_remote(&config, &format!("branch.{}.remote", default_branch_name)) {
        Some(pull) => pull,
        None => "No current pull remote found(this means the active branch is not asociated to a remote branch)".to_string(),
    };

    Ok(RepoDetails {
        // name: repo_name,
        description,
        // url: remote_url,
        remotes: remotes_details,
        // branches,
        tags,
        contributors: vec![],              // git2 does not provide contributor information directly
        forks: 0,                          // git2 does not provide fork information
        stars: 0,                          // git2 does not provide star information
        language: "Unknown".to_string(),   // git2 does not provide repository language information
        size: 0,                           // git2 does not provide repository size information
        created_at: "Unknown".to_string(), // git2 does not provide creation date information
        updated_at: "Unknown".to_string(), // git2 does not provide update date information
        default_branch_name,
        default_full_branch_name,
        default_push_remote: default_push_remote,
        default_pull_remote: default_pull_remote,
        git_settings: vec![], // Placeholder, git2 does not provide detailed settings
    })
}

//TODO: Functions from here can be deleted?
/*
fn clone_repo_with_git(url: &str, path: &Path, ref_format: RepositoryInitFlag) -> Result<Repository, git2::Error> {
    let a  = Repository::clone(url, into);
    let ref_format = if use_reftable_format {
        RepositoryInitFlag::REFTABLE
    } else {
        RepositoryInitFlag::REFS
    };

    let mut fetch_options = FetchOptions::new();
    let mut callbacks = RemoteCallbacks::new();

    fetch_options.remote_callbacks(callbacks);

    let mut init_options = RepositoryInitOptions::new();
    init_options.flags(ref_format);

    let repo = Repository::init_opts(path, &init_options)?;
    let remote = repo.remote("origin", url)?;
*/
//remote.fetch(&["refs/heads/*:refs/remotes/origin/*"], Some(&mut fetch_options), None)?;
/* repo.find_reference("refs/remotes/origin/main")?.set_target(repo.head()?.target().unwrap(), "clone")?;

    Ok(repo)
}

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
