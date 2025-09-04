// src/database/schema.rs

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    pub id: String,

    // Identification & Personal Details
    pub appointment_number: String,
    pub full_name: String,
    pub gender: String,
    pub date_of_birth: String,
    pub age: i32,
    pub nic_number: String,
    pub nic_number_old: Option<String>,
    pub marital_status: String,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub address_line3: Option<String>,
    pub contact_number: Option<String>,
    pub email: Option<String>,

    // Employment Details
    pub designation: String,
    pub date_of_first_appointment: String,
    pub date_of_retirement: String,
    pub increment_date: Option<String>,

    // Salary Information
    pub salary_code: String,
    pub basic_salary: f64,
    pub increment_amount: f64,

    // Image (base64)
    pub image_data: Option<String>,

    // Timestamps
    // chrono::DateTime<Utc> serializes/deserializes (with chrono's serde feature) as RFC3339 strings by default.
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStaff {
    // Identification & Personal Details
    pub appointment_number: String,
    pub full_name: String,
    pub gender: String,
    pub date_of_birth: String,
    pub age: i32,
    pub nic_number: String,
    pub nic_number_old: Option<String>,
    pub marital_status: String,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub address_line3: Option<String>,
    pub contact_number: Option<String>,
    pub email: Option<String>,

    // Employment Details
    pub designation: String,
    pub date_of_first_appointment: String,
    pub date_of_retirement: String,
    pub increment_date: Option<String>,

    // Salary Information
    pub salary_code: String,
    pub basic_salary: f64,
    pub increment_amount: f64,

    // Image (optional base64)
    pub image_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStaff {
    pub id: String,

    // Identification & Personal Details
    pub appointment_number: String,
    pub full_name: String,
    pub gender: String,
    pub date_of_birth: String,
    pub age: i32,
    pub nic_number: String,
    pub nic_number_old: Option<String>,
    pub marital_status: String,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub address_line3: Option<String>,
    pub contact_number: Option<String>,
    pub email: Option<String>,

    // Employment Details
    pub designation: String,
    pub date_of_first_appointment: String,
    pub date_of_retirement: String,
    pub increment_date: Option<String>,

    // Salary Information
    pub salary_code: String,
    pub basic_salary: f64,
    pub increment_amount: f64,

    // Image (optional base64)
    pub image_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffSearchParams {
    pub search_term: Option<String>,
    pub designation: Option<String>,
    pub age_min: Option<i32>,
    pub age_max: Option<i32>,
    pub nic_number: Option<String>,
    pub salary_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintStaffBulkParams {
    pub staff_ids: Vec<String>,
    pub filters: Option<StaffSearchParams>,
}
