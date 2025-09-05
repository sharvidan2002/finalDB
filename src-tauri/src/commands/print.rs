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

fn format_number(value: f64, precision: usize) -> String {
    let s = format!("{value:.precision$}");
    let negative = value < 0.0;
    let abs_s = if negative { &s[1..] } else { &s };
    let parts: Vec<&str> = abs_s.split('.').collect();
    let int_part = parts[0];
    let dec_part = if parts.len() > 1 { parts[1] } else { "" };

    let mut formatted_int = String::new();
    let int_chars: Vec<char> = int_part.chars().collect();
    let len = int_chars.len();
    for (i, &c) in int_chars.iter().enumerate() {
        formatted_int.push(c);
        if (len - i - 1) % 3 == 0 && i < len - 1 {
            formatted_int.push(',');
        }
    }

    let mut result = if negative { "-".to_string() } else { String::new() };
    result.push_str(&formatted_int);
    if precision > 0 {
        result.push('.');
        result.push_str(dec_part);
    }
    result
}

fn generate_individual_pdf(staff: &Staff) -> Result<Vec<u8>, String> {
    let (doc, page1, layer1) = PdfDocument::new("Staff Details", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Failed to add font: {}", e))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Failed to add bold font: {}", e))?;

    let mut current_y = Mm(280.0);
    let left_margin = Mm(15.0);
    let right_margin = Mm(195.0);

    // Draw header box
    let header_height = Mm(35.0);
    draw_box(&current_layer, left_margin, current_y - header_height, right_margin - left_margin, header_height);

    // Header section with better formatting
    current_y -= Mm(8.0);
    current_layer.use_text("DIVISIONAL FOREST OFFICE", 20.0, Mm(45.0), current_y, &font_bold);
    current_y -= Mm(10.0);
    current_layer.use_text("Vavuniya, Sri Lanka", 16.0, Mm(70.0), current_y, &font);
    current_y -= Mm(12.0);
    current_layer.use_text("STAFF INFORMATION SHEET", 18.0, Mm(55.0), current_y, &font_bold);
    current_y -= Mm(15.0);

    // Add some space after header
    current_y -= Mm(10.0);

    // Personal Information Section with better layout
    draw_section_header(&current_layer, "PERSONAL INFORMATION", left_margin, &mut current_y, &font_bold);

    let col1_x = left_margin + Mm(5.0);
    let col2_x = Mm(110.0);
    let field_spacing = Mm(10.0);

    // Left column fields
    let mut left_y = current_y;
    add_field_enhanced(&current_layer, "Appointment Number", &staff.appointment_number, col1_x, &mut left_y, &font, &font_bold, field_spacing);
    add_field_enhanced(&current_layer, "Full Name", &staff.full_name, col1_x, &mut left_y, &font, &font_bold, field_spacing);
    add_field_enhanced(&current_layer, "Gender", &staff.gender, col1_x, &mut left_y, &font, &font_bold, field_spacing);
    add_field_enhanced(&current_layer, "Date of Birth", &staff.date_of_birth, col1_x, &mut left_y, &font, &font_bold, field_spacing);
    add_field_enhanced(&current_layer, "Age", &format!("{} years", staff.age), col1_x, &mut left_y, &font, &font_bold, field_spacing);

    // Right column fields
    let mut right_y = current_y;
    add_field_enhanced(&current_layer, "NIC Number", &staff.nic_number, col2_x, &mut right_y, &font, &font_bold, field_spacing);
    add_field_enhanced(&current_layer, "Marital Status", &staff.marital_status, col2_x, &mut right_y, &font, &font_bold, field_spacing);
    add_field_enhanced(&current_layer, "Contact Number", &staff.contact_number.clone().unwrap_or_else(|| "N/A".to_string()), col2_x, &mut right_y, &font, &font_bold, field_spacing);
    add_field_enhanced(&current_layer, "Email", &staff.email.clone().unwrap_or_else(|| "N/A".to_string()), col2_x, &mut right_y, &font, &font_bold, field_spacing);

    // Use the lower y position for continuation
    current_y = left_y.min(right_y) - Mm(5.0);

    // Address field (full width)
    add_field_enhanced(&current_layer, "Address", &format_address_enhanced(staff), col1_x, &mut current_y, &font, &font_bold, field_spacing);

    current_y -= Mm(15.0);

    // Employment Details Section
    draw_section_header(&current_layer, "EMPLOYMENT DETAILS", left_margin, &mut current_y, &font_bold);

    let mut emp_left_y = current_y;
    let mut emp_right_y = current_y;

    // Left column
    add_field_enhanced(&current_layer, "Designation", &staff.designation, col1_x, &mut emp_left_y, &font, &font_bold, field_spacing);
    add_field_enhanced(&current_layer, "Date of First Appointment", &staff.date_of_first_appointment, col1_x, &mut emp_left_y, &font, &font_bold, field_spacing);

    // Right column
    add_field_enhanced(&current_layer, "Date of Retirement", &staff.date_of_retirement, col2_x, &mut emp_right_y, &font, &font_bold, field_spacing);
    add_field_enhanced(&current_layer, "Increment Date", &staff.increment_date.clone().unwrap_or_else(|| "N/A".to_string()), col2_x, &mut emp_right_y, &font, &font_bold, field_spacing);

    current_y = emp_left_y.min(emp_right_y) - Mm(15.0);

    // Salary Information Section
    draw_section_header(&current_layer, "SALARY INFORMATION", left_margin, &mut current_y, &font_bold);

    let mut sal_left_y = current_y;
    let mut sal_right_y = current_y;

    // Left column
    add_field_enhanced(&current_layer, "Salary Code", &staff.salary_code, col1_x, &mut sal_left_y, &font, &font_bold, field_spacing);
    add_field_enhanced(&current_layer, "Basic Salary", &format!("Rs. {}", format_number(staff.basic_salary, 2)), col1_x, &mut sal_left_y, &font, &font_bold, field_spacing);

    // Right column
    add_field_enhanced(&current_layer, "Increment Amount", &format!("Rs. {}", format_number(staff.increment_amount, 2)), col2_x, &mut sal_right_y, &font, &font_bold, field_spacing);

    current_y = sal_left_y.min(sal_right_y) - Mm(10.0);

    // Total salary highlight box
    let total_salary = staff.basic_salary + staff.increment_amount;
    let total_text = format!("TOTAL SALARY: Rs. {}", format_number(total_salary, 2));

    // Draw highlight box for total
    let box_height = Mm(12.0);
    let box_width = right_margin - left_margin - Mm(10.0);
    draw_highlight_box(&current_layer, left_margin + Mm(5.0), current_y - box_height, box_width, box_height);

    current_y -= Mm(8.0);
    current_layer.use_text(total_text, 14.0, left_margin + Mm(15.0), current_y, &font_bold);

    // Footer with better spacing
    current_y = Mm(30.0);
    draw_line(&current_layer, left_margin, current_y + Mm(5.0), right_margin, current_y + Mm(5.0));

    let footer_text = format!("Generated on: {} | Divisional Forest Office, Vavuniya, Sri Lanka",
                             chrono::Utc::now().format("%Y-%m-%d at %H:%M UTC"));
    current_layer.use_text(footer_text, 10.0, left_margin, current_y, &font);

    doc.save_to_bytes().map_err(|e| format!("Failed to generate PDF: {}", e))
}

fn generate_bulk_pdf(staff_list: &[Staff]) -> Result<Vec<u8>, String> {
    let (doc, page1, layer1) = PdfDocument::new("Staff Directory", Mm(297.0), Mm(210.0), "Layer 1");
    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Failed to add font: {}", e))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Failed to add bold font: {}", e))?;

    let mut current_y = Mm(195.0);
    let left_margin = Mm(10.0);

    // Header with better formatting
    current_layer.use_text("DIVISIONAL FOREST OFFICE - STAFF DIRECTORY", 18.0, Mm(60.0), current_y, &font_bold);
    current_y -= Mm(10.0);
    current_layer.use_text("Vavuniya, Sri Lanka", 14.0, Mm(110.0), current_y, &font);
    current_y -= Mm(8.0);

    let header_info = format!("Total Staff: {} | Generated: {}",
                             staff_list.len(),
                             chrono::Utc::now().format("%Y-%m-%d at %H:%M"));
    current_layer.use_text(header_info, 12.0, Mm(85.0), current_y, &font);
    current_y -= Mm(15.0);

    // Draw table header with background
    let header_height = Mm(8.0);
    draw_highlight_box(&current_layer, left_margin, current_y - header_height, Mm(277.0), header_height);

    // Table headers with better spacing
    current_layer.use_text("#", 10.0, Mm(15.0), current_y - Mm(5.0), &font_bold);
    current_layer.use_text("Appointment No.", 10.0, Mm(25.0), current_y - Mm(5.0), &font_bold);
    current_layer.use_text("Full Name", 10.0, Mm(60.0), current_y - Mm(5.0), &font_bold);
    current_layer.use_text("Designation", 10.0, Mm(105.0), current_y - Mm(5.0), &font_bold);
    current_layer.use_text("Age", 10.0, Mm(150.0), current_y - Mm(5.0), &font_bold);
    current_layer.use_text("NIC Number", 10.0, Mm(165.0), current_y - Mm(5.0), &font_bold);
    current_layer.use_text("Contact", 10.0, Mm(210.0), current_y - Mm(5.0), &font_bold);
    current_layer.use_text("Salary Code", 10.0, Mm(245.0), current_y - Mm(5.0), &font_bold);
    current_layer.use_text("Basic Salary", 10.0, Mm(270.0), current_y - Mm(5.0), &font_bold);

    current_y -= Mm(12.0);

    let mut page_num = 1;
    let rows_per_page = 16;
    let mut row_count = 0;

    for (index, staff) in staff_list.iter().enumerate() {
        if current_y < Mm(25.0) || (row_count > 0 && row_count % rows_per_page == 0) {
            // Add new page
            let (page_id, layer_id) = doc.add_page(Mm(297.0), Mm(210.0), &format!("Layer {}", page_num + 1));
            current_layer = doc.get_page(page_id).get_layer(layer_id);
            current_y = Mm(185.0);
            page_num += 1;

            // Re-add headers on new page
            current_layer.use_text("STAFF DIRECTORY (Continued)", 16.0, Mm(100.0), current_y, &font_bold);
            current_y -= Mm(15.0);

            // Re-draw table header
            draw_highlight_box(&current_layer, left_margin, current_y - header_height, Mm(277.0), header_height);

            current_layer.use_text("#", 10.0, Mm(15.0), current_y - Mm(5.0), &font_bold);
            current_layer.use_text("Appointment No.", 10.0, Mm(25.0), current_y - Mm(5.0), &font_bold);
            current_layer.use_text("Full Name", 10.0, Mm(60.0), current_y - Mm(5.0), &font_bold);
            current_layer.use_text("Designation", 10.0, Mm(105.0), current_y - Mm(5.0), &font_bold);
            current_layer.use_text("Age", 10.0, Mm(150.0), current_y - Mm(5.0), &font_bold);
            current_layer.use_text("NIC Number", 10.0, Mm(165.0), current_y - Mm(5.0), &font_bold);
            current_layer.use_text("Contact", 10.0, Mm(210.0), current_y - Mm(5.0), &font_bold);
            current_layer.use_text("Salary Code", 10.0, Mm(245.0), current_y - Mm(5.0), &font_bold);
            current_layer.use_text("Basic Salary", 10.0, Mm(270.0), current_y - Mm(5.0), &font_bold);

            current_y -= Mm(12.0);
        }

        // Add row data with better formatting
        let row_color = if index % 2 == 0 { Some(0.95f32) } else { None };
        if let Some(gray) = row_color {
            draw_gray_box(&current_layer, left_margin, current_y - Mm(6.0), Mm(277.0), Mm(6.0), gray);
        }

        current_layer.use_text((index + 1).to_string(), 9.0, Mm(15.0), current_y, &font);
        current_layer.use_text(truncate_text(&staff.appointment_number, 18), 9.0, Mm(25.0), current_y, &font);
        current_layer.use_text(truncate_text(&staff.full_name, 22), 9.0, Mm(60.0), current_y, &font);
        current_layer.use_text(truncate_text(&staff.designation, 20), 9.0, Mm(105.0), current_y, &font);
        current_layer.use_text(staff.age.to_string(), 9.0, Mm(150.0), current_y, &font);
        current_layer.use_text(truncate_text(&staff.nic_number, 15), 9.0, Mm(165.0), current_y, &font);
        current_layer.use_text(truncate_text(&staff.contact_number.as_deref().unwrap_or("N/A"), 12), 9.0, Mm(210.0), current_y, &font);
        current_layer.use_text(staff.salary_code.clone(), 9.0, Mm(245.0), current_y, &font);
        current_layer.use_text(format!("Rs. {}", format_number(staff.basic_salary, 0)), 8.0, Mm(265.0), current_y, &font);

        current_y -= Mm(8.0);
        row_count += 1;

        // Add separator line every 5 rows
        if row_count % 5 == 0 {
            draw_line(&current_layer, left_margin, current_y + Mm(1.0), left_margin + Mm(277.0), current_y + Mm(1.0));
        }
    }

    // Footer
    let footer_text = format!("Page {} of {} | Total Records: {} | Generated: {} | Divisional Forest Office, Vavuniya",
                             page_num, page_num, staff_list.len(), chrono::Utc::now().format("%Y-%m-%d at %H:%M UTC"));
    current_layer.use_text(footer_text, 8.0, left_margin, Mm(10.0), &font);

    doc.save_to_bytes().map_err(|e| format!("Failed to generate PDF: {}", e))
}

// Helper functions for better PDF formatting

fn draw_box(layer: &PdfLayerReference, x: Mm, y: Mm, width: Mm, height: Mm) {
    layer.set_outline_color(Color::Rgb(Rgb::new(0.0f32, 0.0f32, 0.0f32, None)));
    layer.set_outline_thickness(1.0);

    let points = vec![
        (Point::new(x, y + height), false),
        (Point::new(x + width, y + height), false),
        (Point::new(x + width, y), false),
        (Point::new(x, y), false),
    ];

    let line = Line {
        points,
        is_closed: true,
    };

    layer.add_line(line);
}

fn draw_highlight_box(layer: &PdfLayerReference, x: Mm, y: Mm, width: Mm, height: Mm) {
    layer.set_fill_color(Color::Rgb(Rgb::new(0.9f32, 0.9f32, 0.9f32, None)));
    layer.set_outline_color(Color::Rgb(Rgb::new(0.0f32, 0.0f32, 0.0f32, None)));
    layer.set_outline_thickness(1.0);

    let points = vec![
        (Point::new(x, y + height), false),
        (Point::new(x + width, y + height), false),
        (Point::new(x + width, y), false),
        (Point::new(x, y), false),
    ];

    let line = Line {
        points,
        is_closed: true,
    };

    layer.add_line(line);
}

fn draw_gray_box(layer: &PdfLayerReference, x: Mm, y: Mm, width: Mm, height: Mm, gray_level: f32) {
    layer.set_fill_color(Color::Rgb(Rgb::new(gray_level, gray_level, gray_level, None)));
    layer.set_outline_color(Color::Rgb(Rgb::new(0.0f32, 0.0f32, 0.0f32, None)));
    layer.set_outline_thickness(0.0);

    let points = vec![
        (Point::new(x, y + height), false),
        (Point::new(x + width, y + height), false),
        (Point::new(x + width, y), false),
        (Point::new(x, y), false),
    ];

    let line = Line {
        points,
        is_closed: true,
    };

    layer.add_line(line);
}

fn draw_line(layer: &PdfLayerReference, x1: Mm, y1: Mm, x2: Mm, y2: Mm) {
    layer.set_outline_color(Color::Rgb(Rgb::new(0.0f32, 0.0f32, 0.0f32, None)));
    layer.set_outline_thickness(1.0);

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

fn draw_section_header(layer: &PdfLayerReference, title: &str, left_margin: Mm, current_y: &mut Mm, font_bold: &IndirectFontRef) {
    // Draw section background
    let section_height = Mm(10.0);
    draw_highlight_box(layer, left_margin, *current_y - section_height, Mm(180.0), section_height);

    *current_y -= Mm(7.0);
    layer.use_text(title, 14.0, left_margin + Mm(5.0), *current_y, font_bold);
    *current_y -= Mm(8.0);
}

fn add_field_enhanced(
    layer: &PdfLayerReference,
    label: &str,
    value: &str,
    x: Mm,
    y: &mut Mm,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    spacing: Mm,
) {
    layer.use_text(format!("{}:", label), 12.0, x, *y, font_bold);
    *y -= Mm(4.0);

    // Wrap long text if needed
    let wrapped_value = wrap_text(value, 45);
    for line in wrapped_value.lines() {
        layer.use_text(line, 11.0, x + Mm(3.0), *y, font);
        *y -= Mm(4.0);
    }

    *y -= spacing - Mm(4.0);
}

fn wrap_text(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        return text.to_string();
    }

    let mut result = String::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 <= max_chars {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(&current_line);
    }

    result
}

fn format_address_enhanced(staff: &Staff) -> String {
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