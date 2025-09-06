use tauri::{command, AppHandle};
use crate::database::{
  operations::{
    create_staff as db_create_staff,
    get_all_staff as db_get_all_staff,
    get_staff_by_id as db_get_staff_by_id,
    update_staff as db_update_staff,
    delete_staff as db_delete_staff,
    search_staff as db_search_staff,
    get_staff_by_nic as db_get_staff_by_nic,
  },
  schema::{Staff, CreateStaff, UpdateStaff, StaffSearchParams},
};
use crate::utils::get_app_data_dir;

#[command]
pub async fn create_staff(
    app_handle: AppHandle,
    staff_data: CreateStaff,
) -> Result<Staff, String> {
    let app_data_dir = get_app_data_dir(&app_handle)?;

    db_create_staff(&app_data_dir, staff_data)
        .map_err(|e| format!("Failed to create staff: {}", e))
}

#[command]
pub async fn get_all_staff(app_handle: AppHandle) -> Result<Vec<Staff>, String> {
    let app_data_dir = get_app_data_dir(&app_handle)?;

    db_get_all_staff(&app_data_dir)
        .map_err(|e| format!("Failed to get staff: {}", e))
}

#[command]
pub async fn get_staff_by_id(app_handle: AppHandle, id: String) -> Result<Staff, String> {
    let app_data_dir = get_app_data_dir(&app_handle)?;

    db_get_staff_by_id(&app_data_dir, &id)
        .map_err(|e| format!("Failed to get staff by ID: {}", e))
}

#[command]
pub async fn update_staff(
    app_handle: AppHandle,
    staff_data: UpdateStaff,
) -> Result<Staff, String> {
    let app_data_dir = get_app_data_dir(&app_handle)?;

    db_update_staff(&app_data_dir, staff_data)
        .map_err(|e| format!("Failed to update staff: {}", e))
}

#[command]
pub async fn delete_staff(app_handle: AppHandle, id: String) -> Result<(), String> {
    let app_data_dir = get_app_data_dir(&app_handle)?;

    db_delete_staff(&app_data_dir, &id)
        .map_err(|e| format!("Failed to delete staff: {}", e))
}

#[command]
pub async fn search_staff(
    app_handle: AppHandle,
    params: StaffSearchParams,
) -> Result<Vec<Staff>, String> {
    let app_data_dir = get_app_data_dir(&app_handle)?;

    db_search_staff(&app_data_dir, params)
        .map_err(|e| format!("Failed to search staff: {}", e))
}

#[command]
pub async fn get_staff_by_nic(
    app_handle: AppHandle,
    nic: String,
) -> Result<Option<Staff>, String> {
    let app_data_dir = get_app_data_dir(&app_handle)?;

    db_get_staff_by_nic(&app_data_dir, &nic)
        .map_err(|e| format!("Failed to get staff by NIC: {}", e))
}