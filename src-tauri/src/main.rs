#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
// https://confidence.sh/blog/rust-module-system-explained/
// use chrono::{DateTime, NaiveDate, NaiveDateTime, ParseError, Utc};
use log::{debug, info};
use notify::event::ModifyKind;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};
// use mime_guess::from_path;
// use std::cell::OnceCell;
use std::{env, thread};
// use file_format::{FileFormat, Kind};
// use std::fs::{File, metadata};
use std::path::{Path, PathBuf};
use std::process::exit;
// use std::string::ParseError;
// use std::sync::OnceLock;
// use std::time::SystemTime;
use tauri::{generate_context, Emitter, State};
use tauri::{
    menu::{AboutMetadata, Menu, MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    Manager,
};

mod components;
mod git_frontend;
mod logic;
// use crate::components::file_metadata::FileMetadata;
use crate::components::git_frontend_error::GitFrontendError;
use crate::git_frontend::app_config::AppConfig;
use crate::git_frontend::git_frontend_module::change_file_status;
use crate::git_frontend::git_frontend_module::commit;
use crate::git_frontend::git_frontend_module::get_file_content;
use crate::git_frontend::git_frontend_module::get_repo_status;
use crate::git_frontend::git_frontend_module::is_git_repo;

// use notify::{DebouncedEvent, RecursiveMode, Watcher};
// use std::sync::mpsc::{channel, Receiver};
// use std::thread;
// use std::time::Duration;
// use std::path::Path;

#[tauri::command]
fn get_launch_path() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_string()
}

fn main() -> Result<(), GitFrontendError> {
    // Enable backtrace support
    std::env::set_var("RUST_LIB_BACKTRACE", "1");
    env_logger::init();
    debug!("main start");
    //MISC. RUN
    // match generate_diff("../../TEST REPO", "a") {//TEST1/b.txt .a.kate-swp
    //     Ok(diff) => println!("{}", diff),
    //     Err(e) => eprintln!("Error generating diff: {}", e),
    // }

    // println!("{:?}",main_change_file_status());

    // let repo_path = PathBuf::from("../../TEST REPO");
    // let relative_file_path = PathBuf::from("/TEST1/b.txt");
    // // let full_file_path = PathBuf::from("TEST1/b.txt");
    // let content = get_file_content(repo_path, relative_file_path)?;
    // println!("{}", content);
    // let repo_path = PathBuf::from("../../TEST REPO");
    // let relative_file_path = PathBuf::from("a");
    // let content = get_file_content(repo_path, relative_file_path)?;
    // println!("{}", content);

    // let args: Vec<String> = env::args().collect();
    // let repo_path = &args[1];

    //main_filesystem RUN
    // if let Err(err) = main_filesystem(None) {
    //     eprintln!("Error: {:?}", err);

    //     // Print backtrace if available
    //     if let GitFrontendError::AnyhowError(e) = &err {
    //         eprintln!("Backtrace: {:?}", e.backtrace());
    //     } else {
    //         eprintln!("No backtrace available.");
    //     }
    //     // Check for and print backtrace if available
    //     // match err {
    //     //     GitFrontendError::AnyhowError(ref e) => {
    //     //         eprintln!("Backtrace: {:?}", e.backtrace());
    //     //     }
    //     //     _ => {
    //     //         eprintln!("No backtrace available.");
    //     //     }
    //     // }
    // }

    // println!("{:?}", main_filesystem(None).map_err(|e| e.to_string()));

    //main_tauri RUN
    // if let Err(err) = main_tauri() {
    //     eprintln!("Error: {:?}", err);

    //     // Print backtrace if available
    //     if let GitFrontendError::AnyhowError(e) = &err {
    //         eprintln!("Backtrace: {:?}", e.backtrace());
    //     } else {
    //         eprintln!("No backtrace available.");
    //     }
    // }
    main_tauri();

    //Ok("SUCCESS".to_string())
    Ok(())
}

//TODO: Write a whole scenario to be able to test all the changes. Write it in TDD.
fn main_change_file_status() -> Result<(), GitFrontendError> {
    let repo_path = Path::new("../../TEST REPO"); //.exists();
    match get_repo_status() {
        Ok(it) => println!("{:?}", it),
        Err(err) => return Err(err),
    };

    // let file_path = Path::new("a");
    // if let Some(value) = git_add(repo_path, file_path) {
    //         return value;
    //     }

    let file_path = Path::new("a");
    if let Some(value) = git_remove(repo_path, file_path) {
        return value;
    }

    //    let file_path = Path::new("a");
    // if let Some(value) = git_add(repo_path, file_path) {
    //         return value;
    //     }
    Ok(())
}

fn git_remove(repo_path: &Path, file_path: &Path) -> Option<Result<(), GitFrontendError>> {
    let command = "Remove";
    let new_file_path = Some(Path::new("path/to/your/new_file"));
    if let Err(e) = change_file_status(file_path, command, new_file_path, None) {
        eprintln!("Error: {}", e);
    }
    match get_repo_status() {
        Ok(it) => println!("{:?}", it),
        Err(err) => return Some(Err(err)),
    };
    // Change this to "Remove", "Commit", "Delete", "Rename", etc.
    // Required for renaming

    None
}

fn git_add(repo_path: &Path, file_path: &Path) -> Option<Result<(), GitFrontendError>> {
    let command = "Add";
    let new_file_path = Some(Path::new("path/to/your/new_file"));
    if let Err(e) = change_file_status(file_path, command, new_file_path, None) {
        eprintln!("Error: {}", e);
    }
    match get_repo_status() {
        Ok(it) => println!("{:?}", it),
        Err(err) => return Some(Err(err)),
    };
    // Change this to "Remove", "Commit", "Delete", "Rename", etc.
    // Required for renaming

    None
}

fn main_filesystem(full_file_path: Option<&str>) -> Result<(), GitFrontendError> {
    // Get the command-line arguments
    #[cfg(not(debug_assertions))]
    let repo_path = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        eprintln!("No repository path provided.");
        None
    });

    // #[cfg(all(debug_assertions, feature = "testing"))]
    #[cfg(debug_assertions)]
    //TODO: Remove the following code and the mut from path. For develpment purpose only.
    let repo_path = PathBuf::from("../../TEST REPO");

    if is_git_repo(Some(repo_path)) {
        println!("It is a git repo");
    } else {
        println!("It is not a git repo");
        exit(0);
    }

    // match env::current_dir() {
    //     Ok(path) => println!("Current working directory: {}", path.display()),
    //     Err(e) => println!("Error retrieving current directory: {}", e),
    // }
    //let full_file_path = "../TEST REPO";
    // let repo_path = Path::new(full_file_path.unwrap());
    // let file_metadata = get_file_metadata(full_file_path)?;//TODO: If this is to be used it should be imported. currentlly private.
    // println!("RESULT: {:?}", file_metadata); // Print the struct
    let repo_changes = get_repo_status()?;
    println!("RESULT: {:?}", repo_changes);
    Ok(())
}

// fn main_changes() {
//     let repo_path = Path::new("..");
//     match get_repo_changes(repo_path) {
//         Ok(changes) => {
//             let mut i = 0;
//             for (path, change_type) in changes {
//                 i+=1;
//                 println!("{} - {}: {}", i, change_type, path);
//             }
//         }
//         Err(e) => println!("Error: {}", e),
//     }
// }

// fn main_modified() {
//     let repo_path = Path::new("..");
//     match get_modified_files(repo_path) {
//         Ok(files) => {
//             for file in files {
//                 println!("{}", file);
//             }
//         }
//         Err(e) => println!("Error: {}", e),
//     }
// }

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_shell::init())
//         .invoke_handler(tauri::generate_handler![get_repo_changes])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

//TODO: Add an option to send the repo path from the command line
//TODO: MultiInstance - Allowed? https://v2.tauri.app/plugin/single-instance/#setup
fn main_tauri() {
    // Get the command-line arguments
    #[cfg(not(debug_assertions))]
    let repo_path = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        eprintln!("No repository path provided.");
        None
    });

    // #[cfg(all(debug_assertions, feature = "testing"))]
    #[cfg(debug_assertions)]
    //TODO: Remove the following code and the mut from path. For develpment purpose only.
    let repo_path = PathBuf::from("../../TEST REPO");

    if is_git_repo(Some(repo_path)) {
        println!("It is a git repo");
    } else {
        println!("It is not a git repo");
        exit(0);
    }
    // TAURI() - Initialize Tauri app
    tauri::Builder::default()
        .setup(|app| {
            // Initialize the OnceLock to store the AppHandle
            static APP_HANDLE: OnceLock<Arc<Mutex<tauri::AppHandle>>> = OnceLock::new();
            let app_handle = Arc::new(Mutex::new(app.handle().clone()));
            APP_HANDLE.set(app_handle.clone()).unwrap();

            // Create a channel to receive the events
            let (tx, rx) = channel();

            println!("Setting up the watcher...");

            // Create and set up the watcher
            let watcher = setup_watcher("../../TEST REPO", tx).expect("Failed to setup watcher");

            // Handle the events in a separate thread
            std::thread::spawn(move || {
                handle_events(rx, app_handle);
            });

            // Ensure the watcher stays in scope
            std::mem::forget(watcher);

            setup_menu(app)?;

            Ok(())
        })
        // .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        // User Settings: Store user preferences or settings that can be accessed and modified throughout the application.
        // Database Connection Pool: Manage a pool of database connections that can be shared across different parts of the application.
        // Authentication State: Keep track of user authentication status and related information.
        .manage(AppConfig::default())
        .invoke_handler(tauri::generate_handler![
            get_repo_status,
            get_file_content,
            change_file_status,
            commit,
            get_launch_path
            //get_git_data,
            //show_menu
            // read_clipboard_text
            /* Add your Tauri commands here */
        ]) //TODO: Open
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_menu(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let settings = MenuItemBuilder::new("Settings...").id("settings").accelerator("CmdOrCtrl+,").build(app)?;

    let app_submenu = SubmenuBuilder::new(app, "App")
        .about(Some(AboutMetadata {
            name: Some("Git Revision Graph".to_string()),
            version: Some("1.0.0".to_string()),
            ..Default::default()
        }))
        .separator()
        .item(&settings)
        .separator()
        .services()
        .separator()
        .hide()
        .hide_others()
        .quit()
        .build()?;

    // ... any other submenus
    let menu = MenuBuilder::new(app)
        .items(&[
            &app_submenu, // ... include references to any other submenus
        ])
        .build()?;

    app.set_menu(menu)?;

    app.on_menu_event(move |app, event| {
        if event.id() == settings.id() {
            info!("Settings menu clicked");
            // emit a window event to the frontend
            // let _event = app.emit("custom-event", "/settings");//TODO: open and fix.
        }
    });

    // Ok("Success".to_string())
    Ok(())
}

fn setup_watcher(path: &str, tx: std::sync::mpsc::Sender<Event>) -> Result<RecommendedWatcher, Box<dyn std::error::Error>> {
    println!("Setting up the watcher...");

    // Create a watcher object using the recommended configuration
    let mut watcher = RecommendedWatcher::new(
        move |res: notify::Result<Event>| {
            match res {
                Ok(event) => {
                    if (event.kind.is_create() || event.kind.is_modify() || event.kind.is_remove()) && !event.paths[0].to_str().unwrap().contains(".kate-swp") {
                        let event_clone = event.clone(); // Clone the event for logging
                        println!("Received event: {:?}", event);
                        if tx.send(event).is_err() {
                            eprintln!("Error: Receiver dropped. Event: {:?}", event_clone);
                        }
                    } else {
                        // debug!("Skipping event: {:?}", event);
                    }
                }
                Err(e) => eprintln!("Error receiving event: {:?}", e),
            }
        },
        Config::default().with_poll_interval(Duration::from_millis(10)),
    )?;

    // Specify the directory to watch, with recursive mode to watch subdirectories
    let path_to_watch = Path::new(path);
    watcher.watch(path_to_watch, RecursiveMode::Recursive)?;

    println!("Watcher set up to watch for changes in {:?}", path_to_watch);

    // Return the watcher to keep it in scope
    Ok(watcher)
}

/// Main event loop to handle incoming events from the watcher
fn handle_events(rx: Receiver<Event>, app_handle: Arc<Mutex<tauri::AppHandle>>) {
    let evnet_name = "file-watcher";
    // let mut last_event_time = Instant::now();
    loop {
        match rx.recv() {
            Ok(event) => {
                // Filter out frequent events
                // if let EventKind::Access(_) = event.kind {
                // let now = Instant::now();
                // if now.duration_since(last_event_time) < Duration::from_secs(1) {
                // continue;
                // }
                // last_event_time = now;
                // }
                // Log other events
                match &event.kind {
                    EventKind::Create(path) => {
                        //TODO: If using _ then instead of path we use event.paths
                        debug!("File created: {:?}", path);
                        // TAURI() - Emit event to the Tauri frontend
                        app_handle.lock().unwrap().emit(evnet_name, format!("File created: {:?}", path)).unwrap();
                        //TODO: Consider using this in production to prevent panic in the code in case of an error
                        // if let Err(e) = app_handle.lock().unwrap().emit("file_event", format!("File created: {:?}", path)) {
                        //     log::error!("Failed to emit event: {:?}", e);
                        // }
                    }
                    EventKind::Modify(modify_kind) => {
                        match modify_kind {
                            // ModifyKind::Any => println!("File modified (any): {:?}", event.paths),
                            ModifyKind::Data(_) => println!("File data modified: {:?}", event.paths), //Edit
                            ModifyKind::Metadata(_) => println!("File metadata modified: {:?}", event.paths), //TOUCH
                            ModifyKind::Name(_) => println!("File renamed: {:?}", event.paths),       //RENAME
                            _ => {
                                println!("Other modify event: {:?}", event);
                                panic!("How did we get here? {:?}", event);
                            }
                        }
                        // TAURI() - Emit event to the Tauri frontend
                        app_handle.lock().unwrap().emit(evnet_name, format!("File modified: {:?}", event.paths)).unwrap();
                    }
                    EventKind::Remove(path) => {
                        println!("File deleted: {:?}", path);
                        // TAURI() - Emit event to the Tauri frontend
                        app_handle.lock().unwrap().emit(evnet_name, format!("File deleted: {:?}", path)).unwrap();
                    }
                    // EventKind::Access(path) => {
                    //     println!("File accessed: {:?}", path);
                    //     // TAURI() - Emit event to the Tauri frontend
                    //     app_handle.lock().unwrap().emit(event_name, format!("File accessed: {:?}", path)).unwrap();
                    // }
                    // EventKind::Any => {
                    //     println!("Any requested: {:?}", event.paths);
                    //     // TAURI() - Emit event to the Tauri frontend
                    //     // app_handle.lock().unwrap().emit(event_name, "Rescan requested".to_string()).unwrap();
                    //     continue;
                    // }
                    // EventKind::Other => {
                    //     println!("Other requested: {:?}", event.paths);
                    //     // TAURI() - Emit event to the Tauri frontend
                    //     app_handle.lock().unwrap().emit(event_name, "Rescan requested".to_string()).unwrap();
                    // }
                    _ => {
                        eprintln!("How did we get here? {:?}", event);
                        panic!("How did we get here?");
                        // Err(String::from("Cannot divide by zero"))
                        // println!("Other event: {:?}", event), //TODO: Unreacable event - Del?
                    }
                }
            }
            Err(e) => {
                println!("Watch error: {:?}", e);
                eprintln!("Receiver end closed or other error encountered. Exiting event loop.");
                break;
            }
        }
    }
    println!("Event handling thread exiting.");
}
