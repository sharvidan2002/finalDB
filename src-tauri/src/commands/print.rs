use tauri::{command, AppHandle}; // removed unused `Manager`
use crate::database::{
    operations::get_staff_by_id as db_get_staff_by_id, // only what's used
    schema::Staff,
};

#[command]
pub async fn print_staff_individual(
    app_handle: AppHandle,
    staff_id: String,
) -> Result<String, String> {
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or("Failed to resolve app data directory")?;

    let staff = db_get_staff_by_id(&app_data_dir, &staff_id)
        .map_err(|e| format!("Failed to get staff: {}", e))?;

    let html_template = generate_individual_print_template(&staff);
    Ok(html_template)
}

#[command]
pub async fn print_staff_bulk(
    app_handle: AppHandle,
    staff_ids: Vec<String>,
) -> Result<String, String> {
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or("Failed to resolve app data directory")?;

    let mut staff_list = Vec::new();

    for staff_id in staff_ids {
        let staff = db_get_staff_by_id(&app_data_dir, &staff_id)
            .map_err(|e| format!("Failed to get staff: {}", e))?;
        staff_list.push(staff);
    }

    let html_template = generate_bulk_print_template(&staff_list);
    Ok(html_template)
}

#[command]
pub async fn export_staff_pdf(
    app_handle: AppHandle,
    staff_ids: Vec<String>,
    is_bulk: bool,
) -> Result<String, String> {
    if is_bulk {
        print_staff_bulk(app_handle, staff_ids).await
    } else {
        if staff_ids.is_empty() {
            return Err("No staff ID provided".to_string());
        }
        print_staff_individual(app_handle, staff_ids[0].clone()).await
    }
}

fn generate_individual_print_template(staff: &Staff) -> String {
    // prepare image HTML (insert base64 if present)
    let image_html = if let Some(ref data) = staff.image_data {
        // insert the base64 payload into the image div
        format!(
            r#"<div class="staff-photo" style="background-image: url('data:image/jpeg;base64,{}'); background-size: cover; background-position: center;"></div>"#,
            data
        )
    } else {
        r#"<div class="staff-photo" style="background-color: #ecf0f1; display: flex; align-items: center; justify-content: center; color: #95a5a6;">No Photo</div>"#.to_string()
    };

    format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <title>Staff Details - {}</title>
            <style>
                @page {{ margin: 20px; }}
                body {{ font-family: 'Arial', sans-serif; margin: 0; padding: 20px; color: #333; }}
                .header {{ text-align: center; margin-bottom: 30px; border-bottom: 3px solid #2c3e50; padding-bottom: 20px; }}
                .header h1 {{ color: #2c3e50; margin: 0; font-size: 28px; }}
                .header h2 {{ color: #34495e; margin: 5px 0; font-size: 18px; font-weight: normal; }}
                .staff-photo {{ float: right; width: 120px; height: 150px; border: 2px solid #bdc3c7; margin-left: 20px; }}
                .staff-info {{ overflow: hidden; }}
                .section {{ margin-bottom: 25px; }}
                .section-title {{ background: #ecf0f1; padding: 10px; margin-bottom: 15px; font-weight: bold; color: #2c3e50; border-left: 4px solid #3498db; }}
                .field-row {{ display: flex; margin-bottom: 8px; }}
                .field-label {{ width: 200px; font-weight: bold; color: #34495e; }}
                .field-value {{ flex: 1; color: #2c3e50; }}
                .address {{ margin-top: 5px; }}
                .footer {{ margin-top: 40px; text-align: center; color: #7f8c8d; font-size: 12px; }}
            </style>
        </head>
        <body>
            <div class="header">
                <h1>Divisional Forest Office</h1>
                <h2>Vavuniya, Sri Lanka</h2>
                <h2>Staff Information Sheet</h2>
            </div>

            <div class="staff-info">
                {}

                <div class="section">
                    <div class="section-title">Personal Information</div>
                    <div class="field-row">
                        <div class="field-label">Appointment Number:</div>
                        <div class="field-value">{}</div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">Full Name:</div>
                        <div class="field-value">{}</div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">Gender:</div>
                        <div class="field-value">{}</div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">Date of Birth:</div>
                        <div class="field-value">{}</div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">Age:</div>
                        <div class="field-value">{}</div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">NIC Number:</div>
                        <div class="field-value">{}</div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">Marital Status:</div>
                        <div class="field-value">{}</div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">Address:</div>
                        <div class="field-value address">
                            {}<br/>
                            {}<br/>
                            {}
                        </div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">Contact Number:</div>
                        <div class="field-value">{}</div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">Email:</div>
                        <div class="field-value">{}</div>
                    </div>
                </div>

                <div class="section">
                    <div class="section-title">Employment Details</div>
                    <div class="field-row">
                        <div class="field-label">Designation:</div>
                        <div class="field-value">{}</div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">Date of First Appointment:</div>
                        <div class="field-value">{}</div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">Date of Retirement:</div>
                        <div class="field-value">{}</div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">Increment Date:</div>
                        <div class="field-value">{}</div>
                    </div>
                </div>

                <div class="section">
                    <div class="section-title">Salary Information</div>
                    <div class="field-row">
                        <div class="field-label">Salary Code:</div>
                        <div class="field-value">{}</div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">Basic Salary:</div>
                        <div class="field-value">Rs. {:.2}</div>
                    </div>
                    <div class="field-row">
                        <div class="field-label">Increment Amount:</div>
                        <div class="field-value">Rs. {:.2}</div>
                    </div>
                </div>
            </div>

            <div class="footer">
                Generated on {} | Divisional Forest Office - Vavuniya, Sri Lanka
            </div>
        </body>
        </html>
        "#,
        // 1: title -> use full name (or change to appointment number if you prefer)
        staff.full_name,
        // 2: image html (already contains base64 when present)
        image_html,
        // 3..21: the rest in order
        staff.appointment_number,
        staff.full_name,
        staff.gender,
        staff.date_of_birth,
        staff.age,
        staff.nic_number,
        staff.marital_status,
        staff.address_line1.as_deref().unwrap_or(""),
        staff.address_line2.as_deref().unwrap_or(""),
        staff.address_line3.as_deref().unwrap_or(""),
        staff.contact_number.as_deref().unwrap_or("N/A"),
        staff.email.as_deref().unwrap_or("N/A"),
        staff.designation,
        staff.date_of_first_appointment,
        staff.date_of_retirement,
        staff.increment_date.as_deref().unwrap_or("N/A"),
        staff.salary_code,
        staff.basic_salary,
        staff.increment_amount,
        // 22: timestamp
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    )
}

fn generate_bulk_print_template(staff_list: &[Staff]) -> String {
    let mut table_rows = String::new();

    for (index, staff) in staff_list.iter().enumerate() {
        table_rows.push_str(&format!(
            r#"
            <tr class="{}">
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>Rs. {:.2}</td>
            </tr>
            "#,
            if index % 2 == 0 { "even-row" } else { "odd-row" },
            index + 1,
            staff.appointment_number,
            staff.full_name,
            staff.designation,
            staff.age,
            staff.nic_number,
            staff.contact_number.as_deref().unwrap_or("N/A"),
            staff.salary_code,
            staff.basic_salary
        ));
    }

    format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <title>Staff Directory</title>
            <style>
                @page {{ margin: 15px; size: A4 landscape; }}
                body {{ font-family: 'Arial', sans-serif; margin: 0; padding: 10px; color: #333; font-size: 11px; }}
                .header {{ text-align: center; margin-bottom: 20px; }}
                .header h1 {{ color: #2c3e50; margin: 0; font-size: 24px; }}
                .header h2 {{ color: #34495e; margin: 5px 0; font-size: 16px; font-weight: normal; }}
                table {{ width: 100%; border-collapse: collapse; margin-top: 15px; }}
                th {{ background: #34495e; color: white; padding: 8px; text-align: left; font-weight: bold; border: 1px solid #2c3e50; }}
                td {{ padding: 6px 8px; border: 1px solid #bdc3c7; }}
                .even-row {{ background-color: #f8f9fa; }}
                .odd-row {{ background-color: white; }}
                .footer {{ margin-top: 20px; text-align: center; color: #7f8c8d; font-size: 10px; }}
                .summary {{ margin-bottom: 15px; text-align: right; color: #2c3e50; font-weight: bold; }}
            </style>
        </head>
        <body>
            <div class="header">
                <h1>Divisional Forest Office</h1>
                <h2>Vavuniya, Sri Lanka</h2>
                <h2>Staff Directory</h2>
            </div>

            <div class="summary">
                Total Staff: {} | Generated on {}
            </div>

            <table>
                <thead>
                    <tr>
                        <th>#</th>
                        <th>Appointment No.</th>
                        <th>Full Name</th>
                        <th>Designation</th>
                        <th>Age</th>
                        <th>NIC Number</th>
                        <th>Contact</th>
                        <th>Salary Code</th>
                        <th>Basic Salary</th>
                    </tr>
                </thead>
                <tbody>
                    {}
                </tbody>
            </table>

            <div class="footer">
                Divisional Forest Office - Vavuniya, Sri Lanka
            </div>
        </body>
        </html>
        "#,
        staff_list.len(),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        table_rows
    )
}