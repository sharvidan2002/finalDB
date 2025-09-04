// src/database/operations.rs
use rusqlite::{Connection, Result, params};
use rusqlite::types::Type;
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::database::schema::{Staff, CreateStaff, UpdateStaff, StaffSearchParams};

pub fn get_database_path(app_data_dir: &PathBuf) -> PathBuf {
    app_data_dir.join("staff_database.db")
}

pub fn initialize_database(app_data_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = get_database_path(app_data_dir);
    let conn = Connection::open(&db_path)?;

    // Read and execute migration
    let migration_sql = include_str!("../../migrations/001_initial.sql");
    conn.execute_batch(migration_sql)?;

    Ok(())
}

pub fn get_connection(app_data_dir: &PathBuf) -> Result<Connection> {
    let db_path = get_database_path(app_data_dir);
    Connection::open(db_path)
}

pub fn create_staff(app_data_dir: &PathBuf, staff_data: CreateStaff) -> Result<Staff> {
    let conn = get_connection(app_data_dir)?;
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        INSERT INTO staff (
            id, appointment_number, full_name, gender, date_of_birth, age,
            nic_number, nic_number_old, marital_status, address_line1, address_line2, address_line3,
            contact_number, email, designation, date_of_first_appointment, date_of_retirement,
            increment_date, salary_code, basic_salary, increment_amount, image_data,
            created_at, updated_at
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24
        )
        "#,
        params![
            id, staff_data.appointment_number, staff_data.full_name, staff_data.gender,
            staff_data.date_of_birth, staff_data.age, staff_data.nic_number, staff_data.nic_number_old,
            staff_data.marital_status, staff_data.address_line1, staff_data.address_line2, staff_data.address_line3,
            staff_data.contact_number, staff_data.email, staff_data.designation, staff_data.date_of_first_appointment,
            staff_data.date_of_retirement, staff_data.increment_date, staff_data.salary_code, staff_data.basic_salary,
            staff_data.increment_amount, staff_data.image_data, now, now
        ],
    )?;

    get_staff_by_id(app_data_dir, &id)
}

/// Parse an RFC3339 timestamp (stored as TEXT) into DateTime<Utc>.
/// `col_index` should be the column index (usize) used for nicer error messages.
fn parse_datetime_from_row(col_index: usize, value: String) -> Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(&value)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
            col_index,
            Type::Text,
            Box::new(e),
        ))
}

pub fn get_all_staff(app_data_dir: &PathBuf) -> Result<Vec<Staff>> {
    let conn = get_connection(app_data_dir)?;
    let mut stmt = conn.prepare(
        r#"
        SELECT id, appointment_number, full_name, gender, date_of_birth, age,
               nic_number, nic_number_old, marital_status, address_line1, address_line2, address_line3,
               contact_number, email, designation, date_of_first_appointment, date_of_retirement,
               increment_date, salary_code, basic_salary, increment_amount, image_data,
               created_at, updated_at
        FROM staff
        ORDER BY full_name
        "#,
    )?;

    let staff_iter = stmt.query_map([], |row| {
        // read created/updated as String and parse
        let created_at_str: String = row.get(22)?;
        let created_at = parse_datetime_from_row(22, created_at_str)?;

        let updated_at_str: String = row.get(23)?;
        let updated_at = parse_datetime_from_row(23, updated_at_str)?;

        Ok(Staff {
            id: row.get(0)?,
            appointment_number: row.get(1)?,
            full_name: row.get(2)?,
            gender: row.get(3)?,
            date_of_birth: row.get(4)?,
            age: row.get(5)?,
            nic_number: row.get(6)?,
            nic_number_old: row.get(7)?,
            marital_status: row.get(8)?,
            address_line1: row.get(9)?,
            address_line2: row.get(10)?,
            address_line3: row.get(11)?,
            contact_number: row.get(12)?,
            email: row.get(13)?,
            designation: row.get(14)?,
            date_of_first_appointment: row.get(15)?,
            date_of_retirement: row.get(16)?,
            increment_date: row.get(17)?,
            salary_code: row.get(18)?,
            basic_salary: row.get(19)?,
            increment_amount: row.get(20)?,
            image_data: row.get(21)?,
            created_at,
            updated_at,
        })
    })?;

    let mut staff_list = Vec::new();
    for staff in staff_iter {
        staff_list.push(staff?);
    }

    Ok(staff_list)
}

pub fn get_staff_by_id(app_data_dir: &PathBuf, id: &str) -> Result<Staff> {
    let conn = get_connection(app_data_dir)?;
    let mut stmt = conn.prepare(
        r#"
        SELECT id, appointment_number, full_name, gender, date_of_birth, age,
               nic_number, nic_number_old, marital_status, address_line1, address_line2, address_line3,
               contact_number, email, designation, date_of_first_appointment, date_of_retirement,
               increment_date, salary_code, basic_salary, increment_amount, image_data,
               created_at, updated_at
        FROM staff
        WHERE id = ?1
        "#,
    )?;

    let staff = stmt.query_row([id], |row| {
        let created_at_str: String = row.get(22)?;
        let created_at = parse_datetime_from_row(22, created_at_str)?;

        let updated_at_str: String = row.get(23)?;
        let updated_at = parse_datetime_from_row(23, updated_at_str)?;

        Ok(Staff {
            id: row.get(0)?,
            appointment_number: row.get(1)?,
            full_name: row.get(2)?,
            gender: row.get(3)?,
            date_of_birth: row.get(4)?,
            age: row.get(5)?,
            nic_number: row.get(6)?,
            nic_number_old: row.get(7)?,
            marital_status: row.get(8)?,
            address_line1: row.get(9)?,
            address_line2: row.get(10)?,
            address_line3: row.get(11)?,
            contact_number: row.get(12)?,
            email: row.get(13)?,
            designation: row.get(14)?,
            date_of_first_appointment: row.get(15)?,
            date_of_retirement: row.get(16)?,
            increment_date: row.get(17)?,
            salary_code: row.get(18)?,
            basic_salary: row.get(19)?,
            increment_amount: row.get(20)?,
            image_data: row.get(21)?,
            created_at,
            updated_at,
        })
    })?;

    Ok(staff)
}

pub fn update_staff(app_data_dir: &PathBuf, staff_data: UpdateStaff) -> Result<Staff> {
    let conn = get_connection(app_data_dir)?;
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        r#"
        UPDATE staff SET
            appointment_number = ?2, full_name = ?3, gender = ?4, date_of_birth = ?5, age = ?6,
            nic_number = ?7, nic_number_old = ?8, marital_status = ?9, address_line1 = ?10, address_line2 = ?11, address_line3 = ?12,
            contact_number = ?13, email = ?14, designation = ?15, date_of_first_appointment = ?16, date_of_retirement = ?17,
            increment_date = ?18, salary_code = ?19, basic_salary = ?20, increment_amount = ?21, image_data = ?22,
            updated_at = ?23
        WHERE id = ?1
        "#,
        params![
            staff_data.id, staff_data.appointment_number, staff_data.full_name, staff_data.gender,
            staff_data.date_of_birth, staff_data.age, staff_data.nic_number, staff_data.nic_number_old,
            staff_data.marital_status, staff_data.address_line1, staff_data.address_line2, staff_data.address_line3,
            staff_data.contact_number, staff_data.email, staff_data.designation, staff_data.date_of_first_appointment,
            staff_data.date_of_retirement, staff_data.increment_date, staff_data.salary_code, staff_data.basic_salary,
            staff_data.increment_amount, staff_data.image_data, now
        ],
    )?;

    get_staff_by_id(app_data_dir, &staff_data.id)
}

pub fn delete_staff(app_data_dir: &PathBuf, id: &str) -> Result<()> {
    let conn = get_connection(app_data_dir)?;

    conn.execute("DELETE FROM staff WHERE id = ?1", params![id])?;

    Ok(())
}

pub fn search_staff(app_data_dir: &PathBuf, params: StaffSearchParams) -> Result<Vec<Staff>> {
    let conn = get_connection(app_data_dir)?;

    let mut query = String::from(
        r#"
        SELECT id, appointment_number, full_name, gender, date_of_birth, age,
               nic_number, nic_number_old, marital_status, address_line1, address_line2, address_line3,
               contact_number, email, designation, date_of_first_appointment, date_of_retirement,
               increment_date, salary_code, basic_salary, increment_amount, image_data,
               created_at, updated_at
        FROM staff
        WHERE 1=1
        "#,
    );

    let mut query_params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(search_term) = &params.search_term {
        if !search_term.is_empty() {
            query.push_str(" AND (full_name LIKE ?1 OR appointment_number LIKE ?1 OR nic_number LIKE ?1 OR nic_number_old LIKE ?1)");
            query_params.push(Box::new(format!("%{}%", search_term)));
        }
    }

    if let Some(designation) = &params.designation {
        if !designation.is_empty() {
            query.push_str(&format!(" AND designation = ?{}", query_params.len() + 1));
            query_params.push(Box::new(designation.clone()));
        }
    }

    if let Some(age_min) = params.age_min {
        query.push_str(&format!(" AND age >= ?{}", query_params.len() + 1));
        query_params.push(Box::new(age_min));
    }

    if let Some(age_max) = params.age_max {
        query.push_str(&format!(" AND age <= ?{}", query_params.len() + 1));
        query_params.push(Box::new(age_max));
    }

    if let Some(nic_number) = &params.nic_number {
        if !nic_number.is_empty() {
            query.push_str(&format!(" AND (nic_number = ?{} OR nic_number_old = ?{})", query_params.len() + 1, query_params.len() + 2));
            query_params.push(Box::new(nic_number.clone()));
            query_params.push(Box::new(nic_number.clone()));
        }
    }

    if let Some(salary_code) = &params.salary_code {
        if !salary_code.is_empty() {
            query.push_str(&format!(" AND salary_code = ?{}", query_params.len() + 1));
            query_params.push(Box::new(salary_code.clone()));
        }
    }

    query.push_str(" ORDER BY full_name");

    let mut stmt = conn.prepare(&query)?;
    let staff_iter = stmt.query_map(rusqlite::params_from_iter(query_params.iter()), |row| {
        let created_at_str: String = row.get(22)?;
        let created_at = parse_datetime_from_row(22, created_at_str)?;

        let updated_at_str: String = row.get(23)?;
        let updated_at = parse_datetime_from_row(23, updated_at_str)?;

        Ok(Staff {
            id: row.get(0)?,
            appointment_number: row.get(1)?,
            full_name: row.get(2)?,
            gender: row.get(3)?,
            date_of_birth: row.get(4)?,
            age: row.get(5)?,
            nic_number: row.get(6)?,
            nic_number_old: row.get(7)?,
            marital_status: row.get(8)?,
            address_line1: row.get(9)?,
            address_line2: row.get(10)?,
            address_line3: row.get(11)?,
            contact_number: row.get(12)?,
            email: row.get(13)?,
            designation: row.get(14)?,
            date_of_first_appointment: row.get(15)?,
            date_of_retirement: row.get(16)?,
            increment_date: row.get(17)?,
            salary_code: row.get(18)?,
            basic_salary: row.get(19)?,
            increment_amount: row.get(20)?,
            image_data: row.get(21)?,
            created_at,
            updated_at,
        })
    })?;

    let mut staff_list = Vec::new();
    for staff in staff_iter {
        staff_list.push(staff?);
    }

    Ok(staff_list)
}

pub fn get_staff_by_nic(app_data_dir: &PathBuf, nic: &str) -> Result<Option<Staff>> {
    let conn = get_connection(app_data_dir)?;
    let mut stmt = conn.prepare(
        r#"
        SELECT id, appointment_number, full_name, gender, date_of_birth, age,
               nic_number, nic_number_old, marital_status, address_line1, address_line2, address_line3,
               contact_number, email, designation, date_of_first_appointment, date_of_retirement,
               increment_date, salary_code, basic_salary, increment_amount, image_data,
               created_at, updated_at
        FROM staff
        WHERE nic_number = ?1 OR nic_number_old = ?1
        "#,
    )?;

    match stmt.query_row([nic], |row| {
        let created_at_str: String = row.get(22)?;
        let created_at = parse_datetime_from_row(22, created_at_str)?;

        let updated_at_str: String = row.get(23)?;
        let updated_at = parse_datetime_from_row(23, updated_at_str)?;

        Ok(Staff {
            id: row.get(0)?,
            appointment_number: row.get(1)?,
            full_name: row.get(2)?,
            gender: row.get(3)?,
            date_of_birth: row.get(4)?,
            age: row.get(5)?,
            nic_number: row.get(6)?,
            nic_number_old: row.get(7)?,
            marital_status: row.get(8)?,
            address_line1: row.get(9)?,
            address_line2: row.get(10)?,
            address_line3: row.get(11)?,
            contact_number: row.get(12)?,
            email: row.get(13)?,
            designation: row.get(14)?,
            date_of_first_appointment: row.get(15)?,
            date_of_retirement: row.get(16)?,
            increment_date: row.get(17)?,
            salary_code: row.get(18)?,
            basic_salary: row.get(19)?,
            increment_amount: row.get(20)?,
            image_data: row.get(21)?,
            created_at,
            updated_at,
        })
    }) {
        Ok(staff) => Ok(Some(staff)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}
