// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// https://confidence.sh/blog/rust-module-system-explained/
use log::{debug, info};
use mime_guess::from_path;
use std::env;
use std::path::Path;
use std::time::SystemTime;
use tauri::State;
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
use crate::git_frontend::git_frontend_module::get_repo_status;
use crate::logic::app_config::AppConfig;
fn main() {
    let args: Vec<String> = env::args().collect();
    // let path = &args[1];
    let path = "../../TEST REPO";
    println!("{:?}", main_filesystem(path).map_err(|e| e.to_string()));
    //main_tauri();
}

fn main_filesystem(full_file_path: &str) -> Result<(), GitFrontendError> {
    match env::current_dir() {
        Ok(path) => println!("Current working directory: {}", path.display()),
        Err(e) => println!("Error retrieving current directory: {}", e),
    }
    //let full_file_path = "../TEST REPO";
    let repo_path = Path::new(full_file_path);
    // let file_metadata = get_file_metadata(full_file_path)?;//TODO: If this is to be used it should be imported. currentlly private.
    // println!("RESULT: {:?}", file_metadata); // Print the struct
    let repo_changes = get_repo_status(repo_path)?;
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

fn main_tauri() {
    tauri::Builder::default()
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

            Ok(())
        })
        .manage(AppConfig::default())
        .invoke_handler(tauri::generate_handler![get_repo_status /*get_git_data, show_menu*/]) //TODO: Open
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
