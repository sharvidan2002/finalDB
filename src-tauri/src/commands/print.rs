use tauri::{command, AppHandle};
use crate::database::{
    operations::get_staff_by_id as db_get_staff_by_id,
    operations::get_all_staff as db_get_all_staff,
    schema::Staff,
};
use std::fs;
use printpdf::*;
use std::path::PathBuf;

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
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or("Failed to resolve app data directory")?;

    let staff = db_get_staff_by_id(&app_data_dir, &staff_id)
        .map_err(|e| format!("Failed to get staff: {}", e))?;

    let pdf_bytes = generate_professional_individual_pdf(&staff)?;

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

    let filename = format!("Staff_Record_{}_{}_{}.pdf",
                          safe_name,
                          staff.appointment_number.replace("/", "_"),
                          chrono::Utc::now().format("%Y%m%d_%H%M%S"));

    let downloads_dir = get_downloads_dir()?;
    let file_path = downloads_dir.join(&filename);

    fs::write(&file_path, pdf_bytes)
        .map_err(|e| format!("Failed to write PDF file: {}", e))?;

    Ok(format!("Staff record saved to Downloads: {}", filename))
}

#[command]
pub async fn generate_bulk_staff_pdf(
    app_handle: AppHandle,
    staff_ids: Vec<String>,
) -> Result<String, String> {
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or("Failed to resolve app data directory")?;

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

    let pdf_bytes = generate_professional_bulk_pdf(&staff_list)?;

    let filename = format!("Staff_Directory_{}_Records_{}.pdf",
                          staff_list.len(),
                          chrono::Utc::now().format("%Y%m%d_%H%M%S"));

    let downloads_dir = get_downloads_dir()?;
    let file_path = downloads_dir.join(&filename);

    fs::write(&file_path, pdf_bytes)
        .map_err(|e| format!("Failed to write PDF file: {}", e))?;

    Ok(format!("Staff directory saved to Downloads: {} ({} records)", filename, staff_list.len()))
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

// New command to generate HTML preview
#[command]
pub async fn generate_staff_preview(
    app_handle: AppHandle,
    staff_id: String,
) -> Result<String, String> {
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or("Failed to resolve app data directory")?;

    let staff = db_get_staff_by_id(&app_data_dir, &staff_id)
        .map_err(|e| format!("Failed to get staff: {}", e))?;

    Ok(generate_staff_html_preview(&staff))
}

#[command]
pub async fn generate_bulk_staff_preview(
    app_handle: AppHandle,
    staff_ids: Vec<String>,
) -> Result<String, String> {
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or("Failed to resolve app data directory")?;

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

fn generate_staff_html_preview(staff: &Staff) -> String {
    let address = format_address_html(staff);
    let current_date = chrono::Utc::now().format("%d-%m-%Y").to_string();

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
        .content {{
            padding: 0 20px;
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
                <td>{}</td>
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
            staff.designation,
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
                    <th style="width: 20%;">Full Name</th>
                    <th style="width: 18%;">Designation</th>
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

fn generate_professional_individual_pdf(staff: &Staff) -> Result<Vec<u8>, String> {
    let (doc, page1, layer1) = PdfDocument::new("Official Staff Record", Mm(210.0), Mm(297.0), "Layer 1");
    let mut current_layer = doc.get_page(page1).get_layer(layer1);
    let mut current_page = 1;

    let font_regular = doc.add_builtin_font(BuiltinFont::TimesRoman)
        .map_err(|e| format!("Failed to add font: {}", e))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::TimesBold)
        .map_err(|e| format!("Failed to add bold font: {}", e))?;

    let margin_left = Mm(15.0);
    let margin_right = Mm(195.0);
    let mut current_y = Mm(280.0); // Start with top margin

    // Official colors
    let black = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
    let dark_blue = Color::Rgb(Rgb::new(0.1, 0.2, 0.4, None));

    // Check if we need a new page
    let check_new_page = |y: &mut Mm, page: &mut i32, layer: &mut PdfLayerReference, doc: &PdfDocumentReference| -> Result<(), String> {
        if *y < Mm(30.0) {
            // Add new page
            let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), &format!("Layer {}", *page + 1));
            *layer = doc.get_page(new_page).get_layer(new_layer);
            *page += 1;
            *y = Mm(280.0); // Reset to top of new page

            // Re-add header on new page
            layer.set_fill_color(black.clone());
            layer.use_text("OFFICIAL STAFF RECORD (Continued)", 14.0, margin_left + Mm(25.0), *y, &font_bold);
            *y -= Mm(15.0);
        }
        Ok(())
    };

    // Official Header (without logo)
    current_layer.set_fill_color(black.clone());

    // Government header - centered
    let gov_text = "GOVERNMENT OF SRI LANKA";
    let gov_width = calculate_text_width(gov_text, 14.0);
    current_layer.use_text(gov_text, 14.0, (Mm(210.0) - gov_width) / 2.0, current_y, &font_bold);
    current_y -= Mm(6.0);

    let ministry_text = "Ministry of Environment and Natural Resources";
    let ministry_width = calculate_text_width(ministry_text, 12.0);
    current_layer.use_text(ministry_text, 12.0, (Mm(210.0) - ministry_width) / 2.0, current_y, &font_regular);
    current_y -= Mm(6.0);

    let office_text = "DIVISIONAL FOREST OFFICE";
    let office_width = calculate_text_width(office_text, 16.0);
    current_layer.use_text(office_text, 16.0, (Mm(210.0) - office_width) / 2.0, current_y, &font_bold);
    current_y -= Mm(6.0);

    let location_text = "Vavuniya, North Central Province";
    let location_width = calculate_text_width(location_text, 11.0);
    current_layer.use_text(location_text, 11.0, (Mm(210.0) - location_width) / 2.0, current_y, &font_regular);
    current_y -= Mm(12.0);

    // Draw official double border line
    draw_line(&current_layer, margin_left, current_y, margin_right, current_y, 1.5f32);
    current_y -= Mm(3.0);
    draw_line(&current_layer, margin_left, current_y, margin_right, current_y, 1.5f32);
    current_y -= Mm(12.0);

    // Document title
    current_layer.set_fill_color(dark_blue.clone());
    let title_text = "OFFICIAL STAFF RECORD";
    let title_width = calculate_text_width(title_text, 16.0);
    current_layer.use_text(title_text, 16.0, (Mm(210.0) - title_width) / 2.0, current_y, &font_bold);
    current_y -= Mm(20.0);

    current_layer.set_fill_color(black.clone());

    // Personal Information Section
    add_section_header(&current_layer, "PERSONAL INFORMATION", margin_left, &mut current_y, &font_bold);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Appointment Number:", &staff.appointment_number, margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Full Name:", &staff.full_name, margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Gender:", &staff.gender, margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Date of Birth:", &format_date(&staff.date_of_birth), margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Age:", &format!("{} years", staff.age), margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "NIC Number:", &staff.nic_number, margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Marital Status:", &staff.marital_status, margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Address:", &format_address_pdf(staff), margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Contact Number:", &staff.contact_number.clone().unwrap_or_else(|| "Not provided".to_string()), margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Email:", &staff.email.clone().unwrap_or_else(|| "Not provided".to_string()), margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    current_y -= Mm(10.0);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    // Employment Details Section
    add_section_header(&current_layer, "EMPLOYMENT DETAILS", margin_left, &mut current_y, &font_bold);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Designation:", &staff.designation, margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Date of First Appointment:", &format_date(&staff.date_of_first_appointment), margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Date of Retirement:", &format_date(&staff.date_of_retirement), margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Increment Date:", &staff.increment_date.clone().unwrap_or_else(|| "Not specified".to_string()), margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    current_y -= Mm(10.0);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    // Salary Information Section
    add_section_header(&current_layer, "SALARY INFORMATION", margin_left, &mut current_y, &font_bold);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Salary Code:", &staff.salary_code, margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Basic Salary:", &format_currency(staff.basic_salary), margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    add_official_field(&current_layer, "Increment Amount:", &format_currency(staff.increment_amount), margin_left, &mut current_y, &font_bold, &font_regular);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    current_y -= Mm(20.0);
    check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;

    // Add space for photo on the right side
    let photo_x = Mm(150.0);
    let photo_y = Mm(250.0);
    let photo_width = Mm(40.0);
    let photo_height = Mm(50.0);

    // Draw photo placeholder box
    draw_rect(&current_layer, photo_x, photo_y, photo_x + photo_width, photo_y - photo_height, 1.0);
    current_layer.use_text("PHOTO", 8.0, photo_x + Mm(10.0), photo_y - Mm(25.0), &font_regular);

    // Signature section
    let signature_y = if current_y < Mm(40.0) {
        check_new_page(&mut current_y, &mut current_page, &mut current_layer, &doc)?;
        Mm(280.0) - Mm(40.0)
    } else {
        current_y
    };

    current_layer.use_text("Staff Member Signature:", 10.0, margin_left, signature_y, &font_regular);
    draw_line(&current_layer, margin_left + Mm(45.0), signature_y - Mm(2.0), margin_left + Mm(85.0), signature_y - Mm(2.0), 1.0f32);

    current_layer.use_text("Authorized Officer Signature:", 10.0, margin_left + Mm(100.0), signature_y, &font_regular);
    draw_line(&current_layer, margin_left + Mm(145.0), signature_y - Mm(2.0), margin_right, signature_y - Mm(2.0), 1.0f32);

    current_y = signature_y - Mm(8.0);
    current_layer.use_text("Divisional Forest Office", 9.0, margin_left + Mm(145.0), current_y, &font_regular);

    // Footer - place at bottom of the last page
    let footer_y = Mm(15.0);

    draw_line(&current_layer, margin_left, footer_y + Mm(5.0), margin_right, footer_y + Mm(5.0), 1.0f32);

    let footer_text = format!("Document Generated: {} | Reference: {} | Divisional Forest Office, Vavuniya | Official Use Only | Page {} of {}",
                             chrono::Utc::now().format("%d-%m-%Y"),
                             staff.appointment_number,
                             current_page,
                             current_page);
    current_layer.use_text(&footer_text, 8.0, margin_left, footer_y, &font_regular);

    doc.save_to_bytes().map_err(|e| format!("Failed to generate PDF: {}", e))
}

fn generate_professional_bulk_pdf(staff_list: &[Staff]) -> Result<Vec<u8>, String> {
    let (doc, page1, layer1) = PdfDocument::new("Official Staff Directory", Mm(297.0), Mm(210.0), "Layer 1");
    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    let font_regular = doc.add_builtin_font(BuiltinFont::TimesRoman)
        .map_err(|e| format!("Failed to add font: {}", e))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::TimesBold)
        .map_err(|e| format!("Failed to add bold font: {}", e))?;

    let margin_left = Mm(15.0);
    let margin_right = Mm(282.0);
    let mut current_y = Mm(195.0);

    let black = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));

    // Official Header (without logo)
    current_layer.set_fill_color(black.clone());

    // Center the header text
    let gov_text = "GOVERNMENT OF SRI LANKA - DIVISIONAL FOREST OFFICE";
    let gov_width = calculate_text_width(gov_text, 16.0);
    current_layer.use_text(gov_text, 16.0, (Mm(297.0) - gov_width) / 2.0, current_y, &font_bold);
    current_y -= Mm(8.0);

    let title_text = "OFFICIAL STAFF DIRECTORY";
    let title_width = calculate_text_width(title_text, 14.0);
    current_layer.use_text(title_text, 14.0, (Mm(297.0) - title_width) / 2.0, current_y, &font_bold);
    current_y -= Mm(6.0);

    let location_text = "Vavuniya, North Central Province";
    let location_width = calculate_text_width(location_text, 11.0);
    current_layer.use_text(location_text, 11.0, (Mm(297.0) - location_width) / 2.0, current_y, &font_regular);
    current_y -= Mm(15.0);

    draw_line(&current_layer, margin_left, current_y, margin_right, current_y, 2.0f32);
    current_y -= Mm(15.0);

    let summary_text = format!("Total Staff: {} | Generated: {} | Status: Confidential - Official Use Only",
                              staff_list.len(),
                              chrono::Utc::now().format("%d-%m-%Y"));
    current_layer.use_text(&summary_text, 10.0, margin_left, current_y, &font_regular);
    current_y -= Mm(20.0);

    // Table headers with proper spacing
    let col_positions = [
        Mm(15.0),   // #
        Mm(25.0),   // Appointment No
        Mm(60.0),   // Name
        Mm(110.0),  // Designation
        Mm(150.0),  // Age
        Mm(165.0),  // NIC
        Mm(205.0),  // Contact
        Mm(235.0),  // Salary Code
        Mm(255.0),  // Basic Salary
    ];

    // Draw table header
    draw_table_header(&current_layer, &col_positions, current_y, &font_bold);
    current_y -= Mm(15.0);

    let mut page_num = 1;
    let rows_per_page = 25;
    let mut row_count = 0;

    for (index, staff) in staff_list.iter().enumerate() {
        if current_y < Mm(25.0) || (row_count > 0 && row_count % rows_per_page == 0) {
            // Add new page
            let (page_id, layer_id) = doc.add_page(Mm(297.0), Mm(210.0), &format!("Layer {}", page_num + 1));
            current_layer = doc.get_page(page_id).get_layer(layer_id);
            current_y = Mm(180.0);
            page_num += 1;

            // Re-add headers on new page
            current_layer.set_fill_color(black.clone());
            current_layer.use_text("STAFF DIRECTORY (Continued)", 14.0, Mm(100.0), current_y, &font_bold);
            current_y -= Mm(20.0);

            draw_table_header(&current_layer, &col_positions, current_y, &font_bold);
            current_y -= Mm(15.0);
        }

        // Add row data
        current_layer.use_text((index + 1).to_string(), 9.0, col_positions[0], current_y, &font_regular);
        current_layer.use_text(truncate_text(&staff.appointment_number, 15), 8.0, col_positions[1], current_y, &font_regular);
        current_layer.use_text(truncate_text(&staff.full_name, 25), 8.0, col_positions[2], current_y, &font_regular);
        current_layer.use_text(truncate_text(&staff.designation, 20), 8.0, col_positions[3], current_y, &font_regular);
        current_layer.use_text(staff.age.to_string(), 9.0, col_positions[4], current_y, &font_regular);
        current_layer.use_text(truncate_text(&staff.nic_number, 12), 8.0, col_positions[5], current_y, &font_regular);
        current_layer.use_text(truncate_text(&staff.contact_number.as_deref().unwrap_or("N/A"), 12), 8.0, col_positions[6], current_y, &font_regular);
        current_layer.use_text(staff.salary_code.clone(), 9.0, col_positions[7], current_y, &font_regular);
        current_layer.use_text(format_currency(staff.basic_salary), 8.0, col_positions[8], current_y, &font_regular);

        current_y -= Mm(10.0);
        row_count += 1;
    }

    // Footer
    let footer_text = format!("Document Generated: {} | Total Records: {} | Divisional Forest Office, Vavuniya | Confidential",
                             chrono::Utc::now().format("%d-%m-%Y"),
                             staff_list.len());
    current_layer.use_text(&footer_text, 8.0, margin_left, Mm(10.0), &font_regular);

    doc.save_to_bytes().map_err(|e| format!("Failed to generate PDF: {}", e))
}

// Helper functions
fn add_section_header(
    layer: &PdfLayerReference,
    title: &str,
    x: Mm,
    y: &mut Mm,
    font_bold: &IndirectFontRef,
) {
    layer.use_text(title, 12.0, x, *y, font_bold);
    draw_line(layer, x, *y - Mm(2.0), x + Mm(135.0), *y - Mm(2.0), 1.5f32);
    *y -= Mm(15.0); // Increased from 12.0 for better spacing
}

fn add_official_field(
    layer: &PdfLayerReference,
    label: &str,
    value: &str,
    x: Mm,
    y: &mut Mm,
    font_bold: &IndirectFontRef,
    font_regular: &IndirectFontRef,
) {
    layer.use_text(label, 10.0, x, *y, font_bold);

    // Calculate text width and wrap if necessary
    let max_value_width = Mm(100.0);
    let wrapped_value = wrap_text(value, 10.0, max_value_width);

    let lines: Vec<&str> = wrapped_value.split('\n').collect();

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            layer.use_text(*line, 10.0, x + Mm(50.0), *y, font_regular);
        } else {
            *y -= Mm(5.0);
            layer.use_text(*line, 10.0, x + Mm(50.0), *y, font_regular);
        }

        // Draw dotted line only for the first line
        if i == 0 {
            draw_dotted_line(layer, x + Mm(48.0), *y - Mm(1.0), x + Mm(135.0), *y - Mm(1.0));
        }
    }

    *y -= Mm(6.0 + (lines.len() as f32 - 1.0) * 5.0);
}

fn draw_line(layer: &PdfLayerReference, x1: Mm, y1: Mm, x2: Mm, y2: Mm, thickness: f32) {
    layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
    layer.set_outline_thickness(thickness);

    let points = vec![
        (Point::new(x1, y1), false),
        (Point::new(x2, y2), false),
    ];

    let line = Line {
        points,
        is_closed: false,
    };

    layer.add_line(line);
}

fn draw_dotted_line(layer: &PdfLayerReference, x1: Mm, y1: Mm, x2: Mm, y2: Mm) {
    layer.set_outline_color(Color::Rgb(Rgb::new(0.5, 0.5, 0.5, None)));
    layer.set_outline_thickness(0.5f32);

    let points = vec![
        (Point::new(x1, y1), false),
        (Point::new(x2, y2), false),
    ];

    let line = Line {
        points,
        is_closed: false,
    };

    layer.add_line(line);
}

fn draw_rect(layer: &PdfLayerReference, x1: Mm, y1: Mm, x2: Mm, y2: Mm, thickness: f32) {
    layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
    layer.set_outline_thickness(thickness);

    let points = vec![
        (Point::new(x1, y1), false),
        (Point::new(x2, y1), false),
        (Point::new(x2, y2), false),
        (Point::new(x1, y2), false),
        (Point::new(x1, y1), false),
    ];

    let rect = Line {
        points,
        is_closed: true,
    };

    layer.add_line(rect);
}

fn draw_table_header(layer: &PdfLayerReference, col_positions: &[Mm], y: Mm, font_bold: &IndirectFontRef) {
    let headers = ["#", "Appointment No.", "Full Name", "Designation", "Age", "NIC Number", "Contact", "Code", "Basic Salary"];

    for (i, header) in headers.iter().enumerate() {
        layer.use_text(*header, 9.0, col_positions[i], y, font_bold);
    }

    // Draw line under headers
    draw_line(layer, col_positions[0], y - Mm(2.0), col_positions[8] + Mm(30.0), y - Mm(2.0), 1.0f32);
}

fn format_address_pdf(staff: &Staff) -> String {
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

fn calculate_text_width(text: &str, font_size: f32) -> Mm {
    // Approximate calculation - each character is roughly 0.5 times font size in points
    let approx_width = text.len() as f32 * font_size * 0.5;
    Mm(approx_width * 0.3528) // Convert from points to mm
}

fn wrap_text(text: &str, font_size: f32, max_width: Mm) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        let test_line = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", current_line, word)
        };

        // Estimate text width (this is approximate)
        let estimated_width = calculate_text_width(&test_line, font_size);

        if estimated_width > max_width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = word.to_string();
        } else {
            current_line = test_line;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines.join("\n")
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