use tauri::{command, AppHandle};
use crate::database::{
    operations::get_staff_by_id as db_get_staff_by_id,
    operations::get_all_staff as db_get_all_staff,
    schema::Staff,
};
use std::fs;
use std::path::PathBuf;
use crate::utils::get_app_data_dir;

fn get_downloads_dir() -> Result<PathBuf, String> {
    if let Some(user_dirs) = directories::UserDirs::new() {
        if let Some(downloads) = user_dirs.download_dir() {
            return Ok(downloads.to_path_buf());
        }
    }

    // Fallback to home directory + Downloads
    if let Some(home) = std::env::var_os("HOME") {
        let mut path = PathBuf::from(home);
        path.push("Downloads");
        if path.exists() {
            return Ok(path);
        }
    }

    // Windows fallback
    if let Some(userprofile) = std::env::var_os("USERPROFILE") {
        let mut path = PathBuf::from(userprofile);
        path.push("Downloads");
        if path.exists() {
            return Ok(path);
        }
    }

    Err("Could not find Downloads directory".to_string())
}

#[command]
pub async fn generate_staff_pdf(
    app_handle: AppHandle,
    staff_id: String,
) -> Result<String, String> {
    let app_data_dir = get_app_data_dir(&app_handle)?;

    let staff = db_get_staff_by_id(&app_data_dir, &staff_id)
        .map_err(|e| format!("Failed to get staff: {}", e))?;

    // Generate HTML content that matches the preview exactly
    let html_content = generate_individual_staff_html(&staff)?;

    // Save HTML file for browser-based PDF generation
    let safe_name = staff.full_name
        .replace(" ", "_")
        .replace("/", "_")
        .replace("\\", "_")
        .replace(":", "_")
        .replace("*", "_")
        .replace("?", "_")
        .replace("\"", "_")
        .replace("<", "_")
        .replace(">", "_")
        .replace("|", "_");

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("Staff_Record_{}_{}_{}.html", safe_name, staff.appointment_number.replace("/", "_"), timestamp);

    let downloads_dir = get_downloads_dir()?;
    let file_path = downloads_dir.join(&filename);

    fs::write(&file_path, html_content)
        .map_err(|e| format!("Failed to write HTML file: {}", e))?;

    // Open the HTML file in the browser for PDF printing
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/c", "start", &file_path.to_string_lossy()])
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&file_path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&file_path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    Ok(format!("HTML file saved and opened for PDF printing: {}", filename))
}

#[command]
pub async fn generate_bulk_staff_pdf(
    app_handle: AppHandle,
    staff_ids: Vec<String>,
) -> Result<String, String> {
    let app_data_dir = get_app_data_dir(&app_handle)?;

    let mut staff_list = Vec::new();

    if staff_ids.is_empty() {
        staff_list = db_get_all_staff(&app_data_dir)
            .map_err(|e| format!("Failed to get all staff: {}", e))?;
    } else {
        for staff_id in staff_ids {
            let staff = db_get_staff_by_id(&app_data_dir, &staff_id)
                .map_err(|e| format!("Failed to get staff: {}", e))?;
            staff_list.push(staff);
        }
    }

    if staff_list.is_empty() {
        return Err("No staff data to export".to_string());
    }

    // Generate HTML content that matches the preview exactly
    let html_content = generate_bulk_staff_html(&staff_list)?;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("Staff_Directory_{}_Records_{}.html", staff_list.len(), timestamp);

    let downloads_dir = get_downloads_dir()?;
    let file_path = downloads_dir.join(&filename);

    fs::write(&file_path, html_content)
        .map_err(|e| format!("Failed to write HTML file: {}", e))?;

    // Open the HTML file in the browser for PDF printing
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/c", "start", &file_path.to_string_lossy()])
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&file_path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&file_path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    Ok(format!("HTML file saved and opened for PDF printing: {} ({} records)", filename, staff_list.len()))
}

#[command]
pub async fn export_staff_pdf(
    app_handle: AppHandle,
    staff_ids: Vec<String>,
    is_bulk: bool,
) -> Result<String, String> {
    if is_bulk || staff_ids.len() > 1 {
        generate_bulk_staff_pdf(app_handle, staff_ids).await
    } else {
        if staff_ids.is_empty() {
            return Err("No staff ID provided".to_string());
        }
        generate_staff_pdf(app_handle, staff_ids[0].clone()).await
    }
}

#[command]
pub async fn open_downloads_folder() -> Result<String, String> {
    let downloads_dir = get_downloads_dir()?;

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&downloads_dir)
            .spawn()
            .map_err(|e| format!("Failed to open Downloads folder: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&downloads_dir)
            .spawn()
            .map_err(|e| format!("Failed to open Downloads folder: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&downloads_dir)
            .spawn()
            .map_err(|e| format!("Failed to open Downloads folder: {}", e))?;
    }

    Ok(format!("Downloads folder opened: {}", downloads_dir.display()))
}

// New command to generate HTML preview (keeping existing for frontend)
#[command]
pub async fn generate_staff_preview(
    app_handle: AppHandle,
    staff_id: String,
) -> Result<String, String> {
    let app_data_dir = get_app_data_dir(&app_handle)?;

    let staff = db_get_staff_by_id(&app_data_dir, &staff_id)
        .map_err(|e| format!("Failed to get staff: {}", e))?;

    Ok(generate_staff_html_preview(&staff))
}

#[command]
pub async fn generate_bulk_staff_preview(
    app_handle: AppHandle,
    staff_ids: Vec<String>,
) -> Result<String, String> {
    let app_data_dir = get_app_data_dir(&app_handle)?;

    let mut staff_list = Vec::new();

    if staff_ids.is_empty() {
        staff_list = db_get_all_staff(&app_data_dir)
            .map_err(|e| format!("Failed to get all staff: {}", e))?;
    } else {
        for staff_id in staff_ids {
            let staff = db_get_staff_by_id(&app_data_dir, &staff_id)
                .map_err(|e| format!("Failed to get staff: {}", e))?;
            staff_list.push(staff);
        }
    }

    Ok(generate_bulk_html_preview(&staff_list))
}

fn format_currency(value: f64) -> String {
    let formatted = format!("{:.2}", value);
    let parts: Vec<&str> = formatted.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = if parts.len() > 1 { parts[1] } else { "00" };

    let mut result = String::new();
    let chars: Vec<char> = integer_part.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*c);
    }

    format!("Rs. {}.{}", result, decimal_part)
}

fn format_date(date_str: &str) -> String {
    if let Ok(date) = chrono::DateTime::parse_from_rfc3339(date_str) {
        date.format("%d-%m-%Y").to_string()
    } else if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        date.format("%d-%m-%Y").to_string()
    } else {
        date_str.to_string()
    }
}

// Generate individual staff HTML exactly matching the preview for PDF printing
fn generate_individual_staff_html(staff: &Staff) -> Result<String, String> {
    let address = format_address_html(staff);
    let current_date = chrono::Utc::now().format("%d-%m-%Y").to_string();

    let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Staff Record - {}</title>
    <style>
        @page {{
            size: A4;
            margin: 15mm;
        }}

        body {{
            font-family: 'Times New Roman', serif;
            margin: 0;
            padding: 0;
            background: white;
            color: #000;
            line-height: 1.2;
            font-size: 11px;
        }}

        .document {{
            width: 100%;
            max-width: 210mm;
            margin: 0 auto;
            background: white;
            min-height: 100vh;
            position: relative;
            padding: 10px;
        }}

        .header {{
            text-align: center;
            border-bottom: 3px double #000;
            padding-bottom: 10px;
            margin-bottom: 15px;
        }}

        .org-title {{
            font-size: 16px;
            font-weight: bold;
            text-transform: uppercase;
            margin: 3px 0;
        }}

        .org-subtitle {{
            font-size: 13px;
            margin: 3px 0;
        }}

        .org-location {{
            font-size: 11px;
            font-style: italic;
            margin: 3px 0;
        }}

        .document-title {{
            font-size: 15px;
            font-weight: bold;
            text-transform: uppercase;
            margin-top: 10px;
            text-decoration: underline;
        }}

        .content {{
            padding: 0 10px;
        }}

        .section {{
            margin-bottom: 15px;
        }}

        .section-title {{
            font-size: 13px;
            font-weight: bold;
            text-transform: uppercase;
            border-bottom: 2px solid #000;
            padding-bottom: 3px;
            margin-bottom: 8px;
        }}

        .field-row {{
            display: table;
            width: 100%;
            margin-bottom: 4px;
        }}

        .field-label {{
            display: table-cell;
            font-weight: bold;
            width: 140px;
            vertical-align: top;
            padding-right: 8px;
            font-size: 10px;
        }}

        .field-value {{
            display: table-cell;
            vertical-align: top;
            border-bottom: 1px dotted #333;
            padding-bottom: 1px;
            font-size: 10px;
        }}

        .signature-section {{
            margin-top: 25px;
            display: table;
            width: 100%;
        }}

        .signature-box {{
            display: table-cell;
            width: 50%;
            text-align: center;
            padding: 15px;
        }}

        .signature-line {{
            border-top: 1px solid #000;
            margin-top: 30px;
            padding-top: 3px;
            font-size: 9px;
        }}

        .footer {{
            position: absolute;
            bottom: 10px;
            left: 10px;
            right: 10px;
            border-top: 1px solid #000;
            padding-top: 5px;
            font-size: 8px;
            text-align: center;
        }}

        .clearfix {{
            clear: both;
        }}

        @media print {{
            body {{ margin: 0; padding: 0; }}
            .document {{ margin: 0; box-shadow: none; padding: 0; }}
            @page {{ margin: 15mm; }}
        }}
    </style>
</head>
<body>
    <div class="document">
        <div class="header">
            <div class="org-title">Government of Sri Lanka</div>
            <div class="org-subtitle">Ministry of Environment and Natural Resources</div>
            <div class="org-subtitle">Divisional Forest Office</div>
            <div class="org-location">Vavuniya, North Central Province</div>
            <div class="document-title">Official Staff Record</div>
        </div>

        <div class="content">
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
                    <div class="field-value">{} years</div>
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
                    <div class="field-value">{}</div>
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
                    <div class="field-value">{}</div>
                </div>

                <div class="field-row">
                    <div class="field-label">Increment Amount:</div>
                    <div class="field-value">{}</div>
                </div>
            </div>

            <div class="signature-section">
                <div class="signature-box">
                    <div class="signature-line">
                        Staff Member Signature
                    </div>
                </div>
                <div class="signature-box">
                    <div class="signature-line">
                        Authorized Officer Signature<br>
                        Divisional Forest Office
                    </div>
                </div>
            </div>
        </div>

        <div class="footer">
            <div>
                Document Generated: {} | Reference: {}<br>
                Divisional Forest Office, Vavuniya | Official Use Only
            </div>
        </div>
    </div>

    <script>
        // Auto-trigger print dialog when page loads
        window.onload = function() {{
            setTimeout(function() {{
                window.print();
            }}, 500);
        }};
    </script>
</body>
</html>
    "#,
        staff.full_name,
        staff.appointment_number,
        staff.full_name,
        staff.gender,
        format_date(&staff.date_of_birth),
        staff.age,
        staff.nic_number,
        staff.marital_status,
        address,
        staff.contact_number.as_deref().unwrap_or("Not provided"),
        staff.email.as_deref().unwrap_or("Not provided"),
        staff.designation,
        format_date(&staff.date_of_first_appointment),
        format_date(&staff.date_of_retirement),
        staff.increment_date.as_deref().unwrap_or("Not specified"),
        staff.salary_code,
        format_currency(staff.basic_salary),
        format_currency(staff.increment_amount),
        current_date,
        staff.appointment_number
    );

    Ok(html)
}

// Generate bulk staff HTML exactly matching the preview for PDF printing
fn generate_bulk_staff_html(staff_list: &[Staff]) -> Result<String, String> {
    let current_date = chrono::Utc::now().format("%d-%m-%Y").to_string();

    let staff_rows = staff_list.iter().enumerate().map(|(index, staff)| {
        format!(r#"
            <tr>
                <td style="text-align: center;">{}</td>
                <td>{}</td>
                <td>{}</td>
                <td style="word-wrap: break-word; max-width: 150px;">{}</td>
                <td style="text-align: center;">{}</td>
                <td>{}</td>
                <td>{}</td>
                <td style="text-align: center;">{}</td>
                <td style="text-align: right;">{}</td>
            </tr>
        "#,
            index + 1,
            truncate_text(&staff.appointment_number, 15),
            truncate_text(&staff.full_name, 25),
            staff.designation, // No truncation for designation - display full text
            staff.age,
            truncate_text(&staff.nic_number, 12),
            truncate_text(&staff.contact_number.as_deref().unwrap_or("N/A"), 12),
            staff.salary_code,
            format_currency(staff.basic_salary)
        )
    }).collect::<Vec<_>>().join("");

    let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Staff Directory</title>
    <style>
        @page {{
            size: A4 landscape;
            margin: 10mm;
        }}

        body {{
            font-family: 'Times New Roman', serif;
            margin: 0;
            padding: 0;
            background: white;
            color: #000;
            line-height: 1.3;
            font-size: 9px;
        }}

        .document {{
            width: 100%;
            max-width: 297mm;
            margin: 0 auto;
            background: white;
            min-height: 100vh;
            padding: 5px;
        }}

        .header {{
            text-align: center;
            border-bottom: 3px double #000;
            padding-bottom: 8px;
            margin-bottom: 12px;
        }}

        .org-title {{
            font-size: 16px;
            font-weight: bold;
            text-transform: uppercase;
            margin: 2px 0;
        }}

        .org-subtitle {{
            font-size: 12px;
            margin: 2px 0;
        }}

        .document-title {{
            font-size: 14px;
            font-weight: bold;
            text-transform: uppercase;
            margin-top: 8px;
            text-decoration: underline;
        }}

        .summary {{
            margin: 10px 0;
            padding: 5px;
            background: #f5f5f5;
            border: 1px solid #000;
            text-align: center;
            font-size: 10px;
        }}

        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 10px 0;
            font-size: 8px;
        }}

        th, td {{
            border: 1px solid #000;
            padding: 4px;
            text-align: left;
            vertical-align: top;
        }}

        th {{
            background: #e0e0e0;
            font-weight: bold;
            text-align: center;
            font-size: 8px;
        }}

        .footer {{
            margin-top: 15px;
            border-top: 1px solid #000;
            padding-top: 5px;
            font-size: 7px;
            text-align: center;
        }}

        @media print {{
            body {{ margin: 0; padding: 0; }}
            .document {{ margin: 0; box-shadow: none; padding: 0; }}
            @page {{ margin: 10mm; }}
        }}
    </style>
</head>
<body>
    <div class="document">
        <div class="header">
            <div class="org-title">Government of Sri Lanka</div>
            <div class="org-subtitle">Ministry of Environment and Natural Resources</div>
            <div class="org-subtitle">Divisional Forest Office</div>
            <div class="org-subtitle">Vavuniya, North Central Province</div>
            <div class="document-title">Official Staff Directory</div>
        </div>

        <div class="summary">
            <strong>Total Staff: {}</strong> | Generated: {} | Status: Official Document
        </div>

        <table>
            <thead>
                <tr>
                    <th style="width: 4%;">#</th>
                    <th style="width: 12%;">Appointment No.</th>
                    <th style="width: 18%;">Full Name</th>
                    <th style="width: 20%;">Designation</th>
                    <th style="width: 5%;">Age</th>
                    <th style="width: 12%;">NIC Number</th>
                    <th style="width: 12%;">Contact</th>
                    <th style="width: 7%;">Salary Code</th>
                    <th style="width: 10%;">Basic Salary</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>

        <div class="footer">
            <div>
                Document Generated: {} | Total Records: {}<br>
                Divisional Forest Office, Vavuniya | Confidential - Official Use Only<br>
                This document contains sensitive information and should be handled according to government data protection policies.
            </div>
        </div>
    </div>

    <script>
        // Auto-trigger print dialog when page loads
        window.onload = function() {{
            setTimeout(function() {{
                window.print();
            }}, 500);
        }};
    </script>
</body>
</html>
    "#,
        staff_list.len(),
        current_date,
        staff_rows,
        current_date,
        staff_list.len()
    );

    Ok(html)
}

// Keep existing HTML preview functions for the frontend preview
fn generate_staff_html_preview(staff: &Staff) -> String {
    let address = format_address_html(staff);
    let current_date = chrono::Utc::now().format("%d-%m-%Y").to_string();

    // Generate photo HTML if image data exists
    let photo_html = if let Some(image_data) = &staff.image_data {
        format!(r#"<img src="data:image/jpeg;base64,{}" alt="Staff Photo" class="staff-photo">"#, image_data)
    } else {
        r#"<div class="staff-photo-placeholder">
            <div class="photo-text">PHOTO</div>
        </div>"#.to_string()
    };

    format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Staff Record - {}</title>
    <style>
        body {{
            font-family: 'Times New Roman', serif;
            margin: 0;
            padding: 20px;
            background: white;
            color: #000;
            line-height: 1.4;
        }}
        .document {{
            max-width: 210mm;
            margin: 0 auto;
            background: white;
            min-height: 297mm;
            position: relative;
        }}
        .header {{
            text-align: center;
            border-bottom: 3px double #000;
            padding-bottom: 15px;
            margin-bottom: 25px;
            position: relative;
        }}
        .org-title {{
            font-size: 20px;
            font-weight: bold;
            text-transform: uppercase;
            margin: 5px 0;
        }}
        .org-subtitle {{
            font-size: 16px;
            margin: 5px 0;
        }}
        .org-location {{
            font-size: 14px;
            font-style: italic;
            margin: 5px 0;
        }}
        .document-title {{
            font-size: 18px;
            font-weight: bold;
            text-transform: uppercase;
            margin-top: 15px;
            text-decoration: underline;
        }}
        .staff-photo {{
            position: absolute;
            top: 15px;
            right: 15px;
            width: 120px;
            height: 150px;
            object-fit: cover;
            border: 2px solid #000;
            box-shadow: 0 2px 4px rgba(0,0,0,0.3);
        }}

        .staff-photo-placeholder {{
            position: absolute;
            top: 15px;
            right: 15px;
            width: 120px;
            height: 150px;
            border: 2px solid #000;
            background: #f5f5f5;
            display: flex;
            align-items: center;
            justify-content: center;
        }}

        .photo-text {{
            font-size: 12px;
            font-weight: bold;
            color: #666;
        }}
        .content {{
            padding: 0 20px;
            margin-right: 140px; /* Make space for photo */
        }}
        .section {{
            margin-bottom: 25px;
        }}
        .section-title {{
            font-size: 16px;
            font-weight: bold;
            text-transform: uppercase;
            border-bottom: 2px solid #000;
            padding-bottom: 5px;
            margin-bottom: 15px;
        }}
        .field-row {{
            display: table;
            width: 100%;
            margin-bottom: 8px;
        }}
        .field-label {{
            display: table-cell;
            font-weight: bold;
            width: 180px;
            vertical-align: top;
            padding-right: 10px;
        }}
        .field-value {{
            display: table-cell;
            vertical-align: top;
            border-bottom: 1px dotted #333;
            padding-bottom: 2px;
        }}
        .signature-section {{
            margin-top: 40px;
            display: table;
            width: 100%;
            margin-right: 0; /* Reset margin for signatures */
        }}
        .signature-box {{
            display: table-cell;
            width: 50%;
            text-align: center;
            padding: 20px;
        }}
        .signature-line {{
            border-top: 1px solid #000;
            margin-top: 40px;
            padding-top: 5px;
            font-size: 12px;
        }}
        .footer {{
            position: absolute;
            bottom: 20px;
            left: 20px;
            right: 20px;
            border-top: 1px solid #000;
            padding-top: 10px;
            font-size: 10px;
            text-align: center;
        }}
        .clearfix {{
            clear: both;
        }}
        .signature-full-width {{
            margin-right: 0 !important;
        }}
        @media print {{
            body {{ margin: 0; padding: 0; }}
            .document {{ margin: 0; box-shadow: none; }}
        }}
    </style>
</head>
<body>
    <div class="document">
        <div class="header">
            <div class="org-title">Government of Sri Lanka</div>
            <div class="org-subtitle">Ministry of Environment and Natural Resources</div>
            <div class="org-subtitle">Divisional Forest Office</div>
            <div class="org-location">Vavuniya, North Central Province</div>
            <div class="document-title">Official Staff Record</div>

            <!-- Staff Photo positioned in top right -->
            {}
        </div>

        <div class="content">
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
                    <div class="field-value">{} years</div>
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
                    <div class="field-value">{}</div>
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
                    <div class="field-value">{}</div>
                </div>

                <div class="field-row">
                    <div class="field-label">Increment Amount:</div>
                    <div class="field-value">{}</div>
                </div>
            </div>
        </div>

        <div class="signature-section signature-full-width">
            <div class="signature-box">
                <div class="signature-line">
                    Staff Member Signature
                </div>
            </div>
            <div class="signature-box">
                <div class="signature-line">
                    Authorized Officer Signature<br>
                    Divisional Forest Office
                </div>
            </div>
        </div>

        <div class="footer">
            <div>
                Document Generated: {} | Reference: {}<br>
                Divisional Forest Office, Vavuniya | Official Use Only
            </div>
        </div>
    </div>
</body>
</html>
    "#,
        staff.full_name,
        photo_html,
        staff.appointment_number,
        staff.full_name,
        staff.gender,
        format_date(&staff.date_of_birth),
        staff.age,
        staff.nic_number,
        staff.marital_status,
        address,
        staff.contact_number.as_deref().unwrap_or("Not provided"),
        staff.email.as_deref().unwrap_or("Not provided"),
        staff.designation,
        format_date(&staff.date_of_first_appointment),
        format_date(&staff.date_of_retirement),
        staff.increment_date.as_deref().unwrap_or("Not specified"),
        staff.salary_code,
        format_currency(staff.basic_salary),
        format_currency(staff.increment_amount),
        current_date,
        staff.appointment_number
    )
}

fn generate_bulk_html_preview(staff_list: &[Staff]) -> String {
    let current_date = chrono::Utc::now().format("%d-%m-%Y").to_string();

    let staff_rows = staff_list.iter().enumerate().map(|(index, staff)| {
        format!(r#"
            <tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td style="word-wrap: break-word;">{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>
        "#,
            index + 1,
            staff.appointment_number,
            staff.full_name,
            staff.designation, // No truncation for designation in preview
            staff.age,
            staff.nic_number,
            staff.contact_number.as_deref().unwrap_or("N/A"),
            staff.salary_code,
            format_currency(staff.basic_salary)
        )
    }).collect::<Vec<_>>().join("");

    format!(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Staff Directory</title>
    <style>
        body {{
            font-family: 'Times New Roman', serif;
            margin: 0;
            padding: 20px;
            background: white;
            color: #000;
            line-height: 1.4;
        }}
        .document {{
            max-width: 297mm;
            margin: 0 auto;
            background: white;
        }}
        .header {{
            text-align: center;
            border-bottom: 3px double #000;
            padding-bottom: 15px;
            margin-bottom: 25px;
        }}
        .org-title {{
            font-size: 20px;
            font-weight: bold;
            text-transform: uppercase;
            margin: 5px 0;
        }}
        .org-subtitle {{
            font-size: 16px;
            margin: 5px 0;
        }}
        .document-title {{
            font-size: 18px;
            font-weight: bold;
            text-transform: uppercase;
            margin-top: 15px;
            text-decoration: underline;
        }}
        .summary {{
            margin: 20px 0;
            padding: 10px;
            background: #f5f5f5;
            border: 1px solid #000;
            text-align: center;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 20px 0;
            font-size: 10px;
        }}
        th, td {{
            border: 1px solid #000;
            padding: 6px;
            text-align: left;
            vertical-align: top;
        }}
        th {{
            background: #e0e0e0;
            font-weight: bold;
            text-align: center;
        }}
        .footer {{
            margin-top: 30px;
            border-top: 1px solid #000;
            padding-top: 10px;
            font-size: 10px;
            text-align: center;
        }}
    </style>
</head>
<body>
    <div class="document">
        <div class="header">
            <div class="org-title">Government of Sri Lanka</div>
            <div class="org-subtitle">Ministry of Environment and Natural Resources</div>
            <div class="org-subtitle">Divisional Forest Office</div>
            <div class="org-subtitle">Vavuniya, North Central Province</div>
            <div class="document-title">Official Staff Directory</div>
        </div>

        <div class="summary">
            <strong>Total Staff: {}</strong> | Generated: {} | Status: Official Document
        </div>

        <table>
            <thead>
                <tr>
                    <th style="width: 5%;">#</th>
                    <th style="width: 12%;">Appointment No.</th>
                    <th style="width: 18%;">Full Name</th>
                    <th style="width: 20%;">Designation</th>
                    <th style="width: 6%;">Age</th>
                    <th style="width: 12%;">NIC Number</th>
                    <th style="width: 12%;">Contact</th>
                    <th style="width: 8%;">Salary Code</th>
                    <th style="width: 12%;">Basic Salary</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>

        <div class="footer">
            <div>
                Document Generated: {} | Total Records: {}<br>
                Divisional Forest Office, Vavuniya | Confidential - Official Use Only<br>
                This document contains sensitive information and should be handled according to government data protection policies.
            </div>
        </div>
    </div>
</body>
</html>
    "#,
        staff_list.len(),
        current_date,
        staff_rows,
        current_date,
        staff_list.len()
    )
}

fn format_address_html(staff: &Staff) -> String {
    let mut address_parts = Vec::new();

    if let Some(line1) = &staff.address_line1 {
        if !line1.trim().is_empty() {
            address_parts.push(line1.trim());
        }
    }

    if let Some(line2) = &staff.address_line2 {
        if !line2.trim().is_empty() {
            address_parts.push(line2.trim());
        }
    }

    if let Some(line3) = &staff.address_line3 {
        if !line3.trim().is_empty() {
            address_parts.push(line3.trim());
        }
    }

    if address_parts.is_empty() {
        "Not provided".to_string()
    } else {
        address_parts.join(", ")
    }
}

fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        format!("{}...", &text[..max_length.saturating_sub(3)])
    }
}

// Legacy function names for backward compatibility
#[command]
pub async fn print_staff_individual(
    app_handle: AppHandle,
    staff_id: String,
) -> Result<String, String> {
    generate_staff_pdf(app_handle, staff_id).await
}

#[command]
pub async fn print_staff_bulk(
    app_handle: AppHandle,
    staff_ids: Vec<String>,
) -> Result<String, String> {
    generate_bulk_staff_pdf(app_handle, staff_ids).await
}