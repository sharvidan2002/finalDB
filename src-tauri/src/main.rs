// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod database;
mod utils;

use commands::{staff::*, print::*};
use database::operations::initialize_database;
use tauri_plugin_fs;
use tauri_plugin_dialog;
use tauri_plugin_shell;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
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

            // PDF generation commands
            generate_staff_pdf,
            generate_bulk_staff_pdf,
            export_staff_pdf,
            open_downloads_folder,

            // Preview commands
            generate_staff_preview,
            generate_bulk_staff_preview,

            // Legacy print commands (now generate PDFs)
            print_staff_individual,
            print_staff_bulk
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}