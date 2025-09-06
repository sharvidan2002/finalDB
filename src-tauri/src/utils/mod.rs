use tauri::{AppHandle, Manager};
use std::path::PathBuf;

/// Resolve the app data directory and return a friendly String error on failure.
pub fn get_app_data_dir(app_handle: &AppHandle) -> Result<PathBuf, String> {
    app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to resolve app data directory: {}", e))
}
