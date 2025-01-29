#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Context, Result};
use git2::{Error, Repository};
// https://confidence.sh/blog/rust-module-system-explained/
use log::{debug, info};
use mime_guess::from_path;
use std::cell::OnceCell;
use std::env;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::OnceLock;
use std::time::SystemTime;
use tauri::{generate_context, State};
use tauri::{
    menu::{AboutMetadata, Menu, MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    Manager,
};
// use std::string::ParseError;
use chrono::{DateTime, NaiveDate, NaiveDateTime, ParseError, Utc};
use file_format::{FileFormat, Kind};
use std::fs::File;
use std::fs::{self, metadata};
use std::io::{self, Read};
use thiserror::Error;

mod components;
mod git_frontend;
mod logic;
use crate::components::file_metadata::FileMetadata;
use crate::components::git_frontend_error::GitFrontendError;
use crate::git_frontend::app_config::AppConfig;
use crate::git_frontend::git_frontend_module::change_file_status;
use crate::git_frontend::git_frontend_module::commit;
use crate::git_frontend::git_frontend_module::get_file_content;
use crate::git_frontend::git_frontend_module::get_repo_status;
use crate::git_frontend::git_frontend_module::is_git_repo;

fn main() -> Result<(), GitFrontendError> {
    // Enable backtrace support
    std::env::set_var("RUST_LIB_BACKTRACE", "1");
    env_logger::init();

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

    tauri::Builder::default()
        // .plugin(tauri_plugin_opener::init())
        .setup(|app| {
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
        })
        // User Settings: Store user preferences or settings that can be accessed and modified throughout the application.
        // Database Connection Pool: Manage a pool of database connections that can be shared across different parts of the application.
        // Authentication State: Keep track of user authentication status and related information.
        .manage(AppConfig::default())
        .invoke_handler(tauri::generate_handler![
            get_repo_status,
            get_file_content,
            change_file_status,
            commit /*get_git_data, show_menu*/
        ]) //TODO: Open
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
