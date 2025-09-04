// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod database;
mod utils;

use commands::{staff::*, print::*};
use database::operations::initialize_database;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path_resolver()
                .app_data_dir()
                .expect("failed to resolve app data directory");

            // Create app data directory if it doesn't exist
            std::fs::create_dir_all(&app_data_dir)?;

            // Initialize database
            initialize_database(&app_data_dir)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Staff commands
            create_staff,
            get_all_staff,
            get_staff_by_id,
            update_staff,
            delete_staff,
            search_staff,
            get_staff_by_nic,

            // Print commands
            print_staff_individual,
            print_staff_bulk,
            export_staff_pdf
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}