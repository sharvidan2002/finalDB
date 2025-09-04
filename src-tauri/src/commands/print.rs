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

    let pdf_bytes = generate_individual_pdf(&staff)?;

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

    let filename = format!("staff-details-{}-{}.pdf",
                          safe_name,
                          chrono::Utc::now().format("%Y%m%d_%H%M%S"));

    let downloads_dir = get_downloads_dir()?;
    let file_path = downloads_dir.join(&filename);

    fs::write(&file_path, pdf_bytes)
        .map_err(|e| format!("Failed to write PDF file: {}", e))?;

    Ok(format!("PDF saved to Downloads: {}", filename))
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

    let pdf_bytes = generate_bulk_pdf(&staff_list)?;

    let filename = format!("staff-directory-{}-records-{}.pdf",
                          staff_list.len(),
                          chrono::Utc::now().format("%Y%m%d_%H%M%S"));

    let downloads_dir = get_downloads_dir()?;
    let file_path = downloads_dir.join(&filename);

    fs::write(&file_path, pdf_bytes)
        .map_err(|e| format!("Failed to write PDF file: {}", e))?;

    Ok(format!("PDF saved to Downloads: {} ({} staff records)", filename, staff_list.len()))
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

fn generate_individual_pdf(staff: &Staff) -> Result<Vec<u8>, String> {
    let (doc, page1, layer1) = PdfDocument::new("Staff Details", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Failed to add font: {}", e))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Failed to add bold font: {}", e))?;

    let mut current_y = Mm(270.0);
    let left_margin = Mm(20.0);

    // Header section
    current_layer.use_text("DIVISIONAL FOREST OFFICE", 18.0, Mm(50.0), current_y, &font_bold);
    current_y -= Mm(8.0);
    current_layer.use_text("Vavuniya, Sri Lanka", 14.0, Mm(75.0), current_y, &font);
    current_y -= Mm(8.0);
    current_layer.use_text("STAFF INFORMATION SHEET", 16.0, Mm(60.0), current_y, &font_bold);
    current_y -= Mm(20.0);

    // Personal Information Section
    current_layer.use_text("PERSONAL INFORMATION", 14.0, left_margin, current_y, &font_bold);
    current_layer.use_text(&"=".repeat(50), 12.0, left_margin, current_y - Mm(3.0), &font);
    current_y -= Mm(12.0);

    let add_field = |label: &str, value: String, current_y: &mut Mm| {
        let text = format!("{}: {}", label, value);
        current_layer.use_text(text, 11.0, left_margin, *current_y, &font);
        *current_y -= Mm(6.0);
    };

    add_field("Appointment Number", staff.appointment_number.clone(), &mut current_y);
    add_field("Full Name", staff.full_name.clone(), &mut current_y);
    add_field("Gender", staff.gender.clone(), &mut current_y);
    add_field("Date of Birth", staff.date_of_birth.clone(), &mut current_y);
    add_field("Age", format!("{} years", staff.age), &mut current_y);
    add_field("NIC Number", staff.nic_number.clone(), &mut current_y);
    add_field("Marital Status", staff.marital_status.clone(), &mut current_y);
    add_field("Contact Number", staff.contact_number.clone().unwrap_or_else(|| "N/A".to_string()), &mut current_y);
    add_field("Email", staff.email.clone().unwrap_or_else(|| "N/A".to_string()), &mut current_y);
    add_field("Address", format_address(staff), &mut current_y);

    current_y -= Mm(10.0);

    // Employment Details Section
    current_layer.use_text("EMPLOYMENT DETAILS", 14.0, left_margin, current_y, &font_bold);
    current_layer.use_text(&"=".repeat(50), 12.0, left_margin, current_y - Mm(3.0), &font);
    current_y -= Mm(12.0);

    add_field("Designation", staff.designation.clone(), &mut current_y);
    add_field("Date of First Appointment", staff.date_of_first_appointment.clone(), &mut current_y);
    add_field("Date of Retirement", staff.date_of_retirement.clone(), &mut current_y);
    add_field("Increment Date", staff.increment_date.clone().unwrap_or_else(|| "N/A".to_string()), &mut current_y);

    current_y -= Mm(10.0);

    // Salary Information Section
    current_layer.use_text("SALARY INFORMATION", 14.0, left_margin, current_y, &font_bold);
    current_layer.use_text(&"=".repeat(50), 12.0, left_margin, current_y - Mm(3.0), &font);
    current_y -= Mm(12.0);

    add_field("Salary Code", staff.salary_code.clone(), &mut current_y);
    add_field("Basic Salary", format!("Rs. {:.2}", staff.basic_salary), &mut current_y);
    add_field("Increment Amount", format!("Rs. {:.2}", staff.increment_amount), &mut current_y);

    current_y -= Mm(3.0);
    let total_text = format!("TOTAL SALARY: Rs. {:.2}", staff.basic_salary + staff.increment_amount);
    current_layer.use_text(total_text, 12.0, left_margin, current_y, &font_bold);

    // Footer
    current_y = Mm(25.0);
    let footer_text = format!("Generated on: {} | Divisional Forest Office, Vavuniya",
                             chrono::Utc::now().format("%Y-%m-%d %H:%M UTC"));
    current_layer.use_text(footer_text, 8.0, left_margin, current_y, &font);

    doc.save_to_bytes().map_err(|e| format!("Failed to generate PDF: {}", e))
}

fn generate_bulk_pdf(staff_list: &[Staff]) -> Result<Vec<u8>, String> {
    let (doc, page1, layer1) = PdfDocument::new("Staff Directory", Mm(297.0), Mm(210.0), "Layer 1");
    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Failed to add font: {}", e))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Failed to add bold font: {}", e))?;

    let mut current_y = Mm(190.0);
    let left_margin = Mm(15.0);

    // Header
    current_layer.use_text("DIVISIONAL FOREST OFFICE - STAFF DIRECTORY", 16.0, Mm(70.0), current_y, &font_bold);
    current_y -= Mm(8.0);
    current_layer.use_text("Vavuniya, Sri Lanka", 12.0, Mm(115.0), current_y, &font);
    current_y -= Mm(12.0);

    let header_info = format!("Total Staff: {} | Generated: {}",
                             staff_list.len(),
                             chrono::Utc::now().format("%Y-%m-%d %H:%M"));
    current_layer.use_text(header_info, 10.0, Mm(90.0), current_y, &font);
    current_y -= Mm(15.0);

    // Table headers
    current_layer.use_text("#", 9.0, Mm(15.0), current_y, &font_bold);
    current_layer.use_text("Appointment No.", 9.0, Mm(25.0), current_y, &font_bold);
    current_layer.use_text("Full Name", 9.0, Mm(60.0), current_y, &font_bold);
    current_layer.use_text("Designation", 9.0, Mm(105.0), current_y, &font_bold);
    current_layer.use_text("Age", 9.0, Mm(145.0), current_y, &font_bold);
    current_layer.use_text("NIC Number", 9.0, Mm(160.0), current_y, &font_bold);
    current_layer.use_text("Contact", 9.0, Mm(200.0), current_y, &font_bold);
    current_layer.use_text("Salary Code", 9.0, Mm(235.0), current_y, &font_bold);
    current_layer.use_text("Basic Salary", 9.0, Mm(260.0), current_y, &font_bold);

    current_y -= Mm(4.0);
    current_layer.use_text(&"=".repeat(100), 8.0, left_margin, current_y, &font);
    current_y -= Mm(8.0);

    let mut page_num = 1;
    let rows_per_page = 18;
    let mut row_count = 0;

    for (index, staff) in staff_list.iter().enumerate() {
        if current_y < Mm(30.0) || (row_count > 0 && row_count % rows_per_page == 0) {
            // Add new page
            let (page_id, layer_id) = doc.add_page(Mm(297.0), Mm(210.0), &format!("Layer {}", page_num + 1));
            current_layer = doc.get_page(page_id).get_layer(layer_id);
            current_y = Mm(180.0);
            page_num += 1;

            // Re-add headers on new page
            current_layer.use_text("STAFF DIRECTORY (Continued)", 14.0, Mm(90.0), current_y, &font_bold);
            current_y -= Mm(12.0);

            current_layer.use_text("#", 9.0, Mm(15.0), current_y, &font_bold);
            current_layer.use_text("Appointment No.", 9.0, Mm(25.0), current_y, &font_bold);
            current_layer.use_text("Full Name", 9.0, Mm(60.0), current_y, &font_bold);
            current_layer.use_text("Designation", 9.0, Mm(105.0), current_y, &font_bold);
            current_layer.use_text("Age", 9.0, Mm(145.0), current_y, &font_bold);
            current_layer.use_text("NIC Number", 9.0, Mm(160.0), current_y, &font_bold);
            current_layer.use_text("Contact", 9.0, Mm(200.0), current_y, &font_bold);
            current_layer.use_text("Salary Code", 9.0, Mm(235.0), current_y, &font_bold);
            current_layer.use_text("Basic Salary", 9.0, Mm(260.0), current_y, &font_bold);

            current_y -= Mm(4.0);
            current_layer.use_text(&"=".repeat(100), 8.0, left_margin, current_y, &font);
            current_y -= Mm(8.0);
        }

        // Add row data
        current_layer.use_text((index + 1).to_string(), 8.0, Mm(15.0), current_y, &font);
        current_layer.use_text(truncate_text(&staff.appointment_number, 15), 8.0, Mm(25.0), current_y, &font);
        current_layer.use_text(truncate_text(&staff.full_name, 20), 8.0, Mm(60.0), current_y, &font);
        current_layer.use_text(truncate_text(&staff.designation, 18), 8.0, Mm(105.0), current_y, &font);
        current_layer.use_text(staff.age.to_string(), 8.0, Mm(145.0), current_y, &font);
        current_layer.use_text(truncate_text(&staff.nic_number, 15), 8.0, Mm(160.0), current_y, &font);
        current_layer.use_text(truncate_text(&staff.contact_number.as_deref().unwrap_or("N/A"), 12), 8.0, Mm(200.0), current_y, &font);
        current_layer.use_text(staff.salary_code.clone(), 8.0, Mm(235.0), current_y, &font);
        current_layer.use_text(format!("Rs. {:.0}", staff.basic_salary), 8.0, Mm(260.0), current_y, &font);

        current_y -= Mm(6.0);
        row_count += 1;

        // Add separator every 5 rows
        if row_count % 5 == 0 {
            current_layer.use_text(&"-".repeat(100), 6.0, left_margin, current_y, &font);
            current_y -= Mm(2.0);
        }
    }

    // Footer
    let footer_text = format!("Page {} | Total Records: {} | Generated: {} | Divisional Forest Office, Vavuniya",
                             page_num, staff_list.len(), chrono::Utc::now().format("%Y-%m-%d %H:%M UTC"));
    current_layer.use_text(footer_text, 7.0, left_margin, Mm(15.0), &font);

    doc.save_to_bytes().map_err(|e| format!("Failed to generate PDF: {}", e))
}

fn format_address(staff: &Staff) -> String {
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