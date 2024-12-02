// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// https://confidence.sh/blog/rust-module-system-explained/
use log::{debug, info};
use mime_guess::from_path;
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

mod git_frontend;
mod logic;
use crate::logic::app_config::AppConfig;
// use crate::git_frontend::git_frontend_module::get_repo_changes;
// use crate::git_frontend::git_frontend_module::get_git_data;
// use crate::git_frontend::git_frontend_module::show_menu;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("IO error")]
    Io(#[from] io::Error),
    #[error("Parse error")]
    Parse(#[from] ParseError),
}

//TODO: Should all the String be changed to &str?
#[derive(Debug)]
struct FileMetadata {
    id: u32,
    file_name: String,
    file_extenstion: String,
    file_type: String,
    status: String,
    size: String,
    created_by: String,
    created_at: NaiveDate,
    modified_by: String,
    modified_at: DateTime<Utc>,
    comments: String,
    preview: String,
    selected: bool,
}

fn main() {
    println!("{:?}", main_filesystem().map_err(|e| e.to_string()));
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
        file_format = from_path(file_path)
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

fn main_filesystem() -> Result<(), FileError> {
    let full_file_path =
        "D:/Ohad/Projects/toGit.ai/TAURI/git-revision-graph/src-tauri/tauri.conf.json";
    let file_path = Path::new(full_file_path);
    //let file_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();
    let file_name = Path::new(file_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // Example to read file content (this is a placeholder, replace with actual file operations)
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    // Try to determine the file format by its content
    let file_format = determine_file_format(file_path)?;

    // Get file metadata
    let metadata = metadata(file_path)?;
    let modified_time = metadata.modified().unwrap_or(SystemTime::now());
    let modified_at = system_time_to_naive_date_time(modified_time);

    let file_metadata = FileMetadata {
        id: 1,
        file_name: file_name,
        file_extenstion: file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string(),
        file_type: file_format,
        status: String::from("Added"), //TODO: Take from get_repo_changes
        size: format!("{} KB", file.metadata()?.len() / 1024), // Get size in KB //TODO: Format
        created_by: String::from("User A"),
        created_at: NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d")?,
        modified_by: String::from("User B"),
        modified_at: modified_at,
        comments: String::from("No comments"),
        preview: contents, // Use file content as preview
        selected: false,
    };

    println!("RESULT: {:?}", file_metadata); // Print the struct

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
            let settings = MenuItemBuilder::new("Settings...")
                .id("settings")
                .accelerator("CmdOrCtrl+,")
                .build(app)?;

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
        //.invoke_handler(tauri::generate_handler![get_git_data, show_menu]) //TODO: Open
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
