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

    format!("{}.{}", result, decimal_part)
}

fn generate_individual_pdf(staff: &Staff) -> Result<Vec<u8>, String> {
    let (doc, page1, layer1) = PdfDocument::new("Staff Information Sheet", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Failed to add font: {}", e))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Failed to add bold font: {}", e))?;

    let margin_left = Mm(20.0);
    let margin_right = Mm(190.0);
    let mut current_y = Mm(277.0);

    // Colors
    let primary_color = Color::Rgb(Rgb::new(0.067, 0.176, 0.314, None)); // Forest dark blue
    let accent_color = Color::Rgb(Rgb::new(0.064, 0.725, 0.404, None)); // Forest green
    let text_color = Color::Rgb(Rgb::new(0.2, 0.2, 0.2, None)); // Dark gray

    // Header
    current_layer.set_fill_color(primary_color.clone());
    current_layer.use_text("DIVISIONAL FOREST OFFICE", 20.0, margin_left + Mm(30.0), current_y, &font_bold);

    current_y -= Mm(10.0);
    current_layer.set_fill_color(accent_color.clone());
    current_layer.use_text("Vavuniya, Sri Lanka", 14.0, margin_left + Mm(50.0), current_y, &font_regular);

    current_y -= Mm(15.0);
    current_layer.set_fill_color(primary_color.clone());
    current_layer.use_text("STAFF INFORMATION SHEET", 16.0, margin_left + Mm(35.0), current_y, &font_bold);

    current_y -= Mm(30.0);

    // Personal Information Section
    current_layer.set_fill_color(accent_color.clone());
    current_layer.use_text("PERSONAL INFORMATION", 14.0, margin_left, current_y, &font_bold);
    draw_simple_line(&current_layer, margin_left, current_y - Mm(2.0), margin_right, current_y - Mm(2.0));
    current_y -= Mm(20.0);

    let col1_x = margin_left;
    let col2_x = margin_left + Mm(85.0);

    // Left column
    let mut left_y = current_y;
    add_field(&current_layer, "Appointment Number:", &staff.appointment_number, col1_x, &mut left_y, &font_bold, &font_regular, &text_color);
    add_field(&current_layer, "Full Name:", &staff.full_name, col1_x, &mut left_y, &font_bold, &font_regular, &text_color);
    add_field(&current_layer, "Gender:", &staff.gender, col1_x, &mut left_y, &font_bold, &font_regular, &text_color);
    add_field(&current_layer, "Date of Birth:", &staff.date_of_birth, col1_x, &mut left_y, &font_bold, &font_regular, &text_color);
    add_field(&current_layer, "Age:", &format!("{} years", staff.age), col1_x, &mut left_y, &font_bold, &font_regular, &text_color);

    // Right column
    let mut right_y = current_y;
    add_field(&current_layer, "NIC Number:", &staff.nic_number, col2_x, &mut right_y, &font_bold, &font_regular, &text_color);
    add_field(&current_layer, "Marital Status:", &staff.marital_status, col2_x, &mut right_y, &font_bold, &font_regular, &text_color);
    add_field(&current_layer, "Contact Number:", &staff.contact_number.clone().unwrap_or_else(|| "N/A".to_string()), col2_x, &mut right_y, &font_bold, &font_regular, &text_color);
    add_field(&current_layer, "Email:", &staff.email.clone().unwrap_or_else(|| "N/A".to_string()), col2_x, &mut right_y, &font_bold, &font_regular, &text_color);

    current_y = left_y.min(right_y) - Mm(10.0);

    // Address (full width)
    add_field_full_width(&current_layer, "Address:", &format_address(staff), col1_x, &mut current_y, &font_bold, &font_regular, &text_color);

    current_y -= Mm(25.0);

    // Employment Details Section
    current_layer.set_fill_color(accent_color.clone());
    current_layer.use_text("EMPLOYMENT DETAILS", 14.0, margin_left, current_y, &font_bold);
    draw_simple_line(&current_layer, margin_left, current_y - Mm(2.0), margin_right, current_y - Mm(2.0));
    current_y -= Mm(20.0);

    let mut emp_left_y = current_y;
    let mut emp_right_y = current_y;

    // Left column
    add_field(&current_layer, "Designation:", &staff.designation, col1_x, &mut emp_left_y, &font_bold, &font_regular, &text_color);
    add_field(&current_layer, "Date of First Appointment:", &staff.date_of_first_appointment, col1_x, &mut emp_left_y, &font_bold, &font_regular, &text_color);

    // Right column
    add_field(&current_layer, "Date of Retirement:", &staff.date_of_retirement, col2_x, &mut emp_right_y, &font_bold, &font_regular, &text_color);
    add_field(&current_layer, "Increment Date:", &staff.increment_date.clone().unwrap_or_else(|| "N/A".to_string()), col2_x, &mut emp_right_y, &font_bold, &font_regular, &text_color);

    current_y = emp_left_y.min(emp_right_y) - Mm(25.0);

    // Salary Information Section
    current_layer.set_fill_color(accent_color.clone());
    current_layer.use_text("SALARY INFORMATION", 14.0, margin_left, current_y, &font_bold);
    draw_simple_line(&current_layer, margin_left, current_y - Mm(2.0), margin_right, current_y - Mm(2.0));
    current_y -= Mm(20.0);

    let mut sal_left_y = current_y;
    let mut sal_right_y = current_y;

    // Left column
    add_field(&current_layer, "Salary Code:", &staff.salary_code, col1_x, &mut sal_left_y, &font_bold, &font_regular, &text_color);
    add_field(&current_layer, "Basic Salary:", &format!("Rs. {}", format_currency(staff.basic_salary)), col1_x, &mut sal_left_y, &font_bold, &font_regular, &text_color);

    // Right column
    add_field(&current_layer, "Increment Amount:", &format!("Rs. {}", format_currency(staff.increment_amount)), col2_x, &mut sal_right_y, &font_bold, &font_regular, &text_color);

    current_y = sal_left_y.min(sal_right_y) - Mm(15.0);

    // Total salary
    let total_salary = staff.basic_salary + staff.increment_amount;
    current_layer.set_fill_color(primary_color.clone());
    current_layer.use_text("TOTAL SALARY:", 12.0, margin_left, current_y, &font_bold);
    current_layer.set_fill_color(text_color.clone());
    current_layer.use_text(format!("Rs. {}", format_currency(total_salary)), 12.0, margin_left + Mm(40.0), current_y, &font_bold);

    // Footer
    current_y = Mm(25.0);
    draw_simple_line(&current_layer, margin_left, current_y + Mm(5.0), margin_right, current_y + Mm(5.0));

    current_layer.set_fill_color(text_color.clone());
    let footer_text = format!("Generated on: {} | Divisional Forest Office, Vavuniya, Sri Lanka",
                             chrono::Utc::now().format("%Y-%m-%d at %H:%M UTC"));
    current_layer.use_text(footer_text, 9.0, margin_left, current_y, &font_regular);

    doc.save_to_bytes().map_err(|e| format!("Failed to generate PDF: {}", e))
}

fn generate_bulk_pdf(staff_list: &[Staff]) -> Result<Vec<u8>, String> {
    let (doc, page1, layer1) = PdfDocument::new("Staff Directory", Mm(297.0), Mm(210.0), "Layer 1");
    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Failed to add font: {}", e))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Failed to add bold font: {}", e))?;

    let margin_left = Mm(15.0);
    let mut current_y = Mm(195.0);

    let primary_color = Color::Rgb(Rgb::new(0.067, 0.176, 0.314, None)); // Forest dark blue
    let accent_color = Color::Rgb(Rgb::new(0.064, 0.725, 0.404, None)); // Forest green
    let text_color = Color::Rgb(Rgb::new(0.2, 0.2, 0.2, None)); // Dark gray

    // Header
    current_layer.set_fill_color(primary_color.clone());
    current_layer.use_text("DIVISIONAL FOREST OFFICE - STAFF DIRECTORY", 18.0, Mm(60.0), current_y, &font_bold);
    current_y -= Mm(10.0);
    current_layer.set_fill_color(accent_color.clone());
    current_layer.use_text("Vavuniya, Sri Lanka", 12.0, Mm(110.0), current_y, &font_regular);
    current_y -= Mm(8.0);

    let header_info = format!("Total Staff: {} | Generated: {}",
                             staff_list.len(),
                             chrono::Utc::now().format("%Y-%m-%d at %H:%M"));
    current_layer.set_fill_color(text_color.clone());
    current_layer.use_text(header_info, 10.0, Mm(90.0), current_y, &font_regular);
    current_y -= Mm(25.0);

    // Table headers
    current_layer.set_fill_color(accent_color.clone());
    current_layer.use_text(" # ", 10.0, Mm(15.0), current_y, &font_bold);
    current_layer.use_text("Appointment No.", 10.0, Mm(25.0), current_y, &font_bold);
    current_layer.use_text("Full Name", 10.0, Mm(60.0), current_y, &font_bold);
    current_layer.use_text("Designation", 10.0, Mm(100.0), current_y, &font_bold);
    current_layer.use_text("Age", 10.0, Mm(140.0), current_y, &font_bold);
    current_layer.use_text("NIC Number", 10.0, Mm(155.0), current_y, &font_bold);
    current_layer.use_text("Contact", 10.0, Mm(195.0), current_y, &font_bold);
    current_layer.use_text("Salary Code", 10.0, Mm(225.0), current_y, &font_bold);
    current_layer.use_text("Basic Salary", 10.0, Mm(250.0), current_y, &font_bold);

    current_y -= Mm(15.0);

    let mut page_num = 1;
    let rows_per_page = 20;
    let mut row_count = 0;

    current_layer.set_fill_color(text_color.clone());

    for (index, staff) in staff_list.iter().enumerate() {
        if current_y < Mm(25.0) || (row_count > 0 && row_count % rows_per_page == 0) {
            // Add new page
            let (page_id, layer_id) = doc.add_page(Mm(297.0), Mm(210.0), &format!("Layer {}", page_num + 1));
            current_layer = doc.get_page(page_id).get_layer(layer_id);
            current_y = Mm(185.0);
            page_num += 1;

            // Re-add headers on new page
            current_layer.set_fill_color(primary_color.clone());
            current_layer.use_text("STAFF DIRECTORY (Continued)", 16.0, Mm(100.0), current_y, &font_bold);
            current_y -= Mm(15.0);

            // Re-draw table header
            current_layer.set_fill_color(accent_color.clone());
            current_layer.use_text(" # ", 10.0, Mm(15.0), current_y, &font_bold);
            current_layer.use_text("Appointment No.", 10.0, Mm(25.0), current_y, &font_bold);
            current_layer.use_text("Full Name", 10.0, Mm(60.0), current_y, &font_bold);
            current_layer.use_text("Designation", 10.0, Mm(100.0), current_y, &font_bold);
            current_layer.use_text("Age", 10.0, Mm(140.0), current_y, &font_bold);
            current_layer.use_text("NIC Number", 10.0, Mm(155.0), current_y, &font_bold);
            current_layer.use_text("Contact", 10.0, Mm(195.0), current_y, &font_bold);
            current_layer.use_text("Salary Code", 10.0, Mm(225.0), current_y, &font_bold);
            current_layer.use_text("Basic Salary", 10.0, Mm(250.0), current_y, &font_bold);

            current_y -= Mm(15.0);
            current_layer.set_fill_color(text_color.clone());
        }

        // Add row data
        current_layer.use_text((index + 1).to_string(), 9.0, Mm(15.0), current_y, &font_regular);
        current_layer.use_text(truncate_text(&staff.appointment_number, 18), 9.0, Mm(25.0), current_y, &font_regular);
        current_layer.use_text(truncate_text(&staff.full_name, 22), 9.0, Mm(60.0), current_y, &font_regular);
        current_layer.use_text(truncate_text(&staff.designation, 20), 9.0, Mm(100.0), current_y, &font_regular);
        current_layer.use_text(staff.age.to_string(), 9.0, Mm(140.0), current_y, &font_regular);
        current_layer.use_text(truncate_text(&staff.nic_number, 15), 9.0, Mm(155.0), current_y, &font_regular);
        current_layer.use_text(truncate_text(&staff.contact_number.as_deref().unwrap_or("N/A"), 12), 9.0, Mm(195.0), current_y, &font_regular);
        current_layer.use_text(staff.salary_code.clone(), 9.0, Mm(225.0), current_y, &font_regular);
        current_layer.use_text(format!("Rs. {}", format_currency(staff.basic_salary)), 8.0, Mm(245.0), current_y, &font_regular);

        current_y -= Mm(12.0);
        row_count += 1;
    }

    // Footer
    let footer_text = format!("Total Records: {} | Generated: {} | Divisional Forest Office, Vavuniya",
                             staff_list.len(), chrono::Utc::now().format("%Y-%m-%d at %H:%M UTC"));
    current_layer.use_text(footer_text, 8.0, margin_left, Mm(10.0), &font_regular);

    doc.save_to_bytes().map_err(|e| format!("Failed to generate PDF: {}", e))
}

// Helper functions

fn add_field(
    layer: &PdfLayerReference,
    label: &str,
    value: &str,
    x: Mm,
    y: &mut Mm,
    font_bold: &IndirectFontRef,
    font_regular: &IndirectFontRef,
    text_color: &Color,
) {
    layer.set_fill_color(text_color.clone());

    // Label
    layer.use_text(label, 10.0, x, *y, font_bold);

    let value_x = x + Mm(50.0); // Adjusted for longer labels

    let wrapped_value = wrap_text(value, 30);
    let lines: Vec<&str> = wrapped_value.lines().collect();

    let num_lines = lines.len() as f32;

    if !lines.is_empty() {
        layer.use_text(lines[0], 10.0, value_x, *y, font_regular);
        for (i, line) in lines.iter().enumerate().skip(1) {
            let line_y = *y - Mm((i as f32) * 4.0f32);
            layer.use_text(*line, 10.0, value_x, line_y, font_regular);
        }
    }

    let field_height = if num_lines > 1.0f32 { (num_lines - 1.0f32) * 4.0f32 + 12.0f32 } else { 12.0f32 };
    *y -= Mm(field_height);
}

fn add_field_full_width(
    layer: &PdfLayerReference,
    label: &str,
    value: &str,
    x: Mm,
    y: &mut Mm,
    font_bold: &IndirectFontRef,
    font_regular: &IndirectFontRef,
    text_color: &Color,
) {
    layer.set_fill_color(text_color.clone());

    // Label
    layer.use_text(label, 10.0, x, *y, font_bold);

    let value_x = x + Mm(50.0); // Adjusted for longer labels

    let wrapped_value = wrap_text(value, 60);
    let lines: Vec<&str> = wrapped_value.lines().collect();

    let num_lines = lines.len() as f32;

    if !lines.is_empty() {
        layer.use_text(lines[0], 10.0, value_x, *y, font_regular);
        for (i, line) in lines.iter().enumerate().skip(1) {
            let line_y = *y - Mm((i as f32) * 4.0f32);
            layer.use_text(*line, 10.0, value_x, line_y, font_regular);
        }
    }

    let field_height = if num_lines > 1.0f32 { (num_lines - 1.0f32) * 4.0f32 + 12.0f32 } else { 12.0f32 };
    *y -= Mm(field_height);
}

fn draw_simple_line(layer: &PdfLayerReference, x1: Mm, y1: Mm, x2: Mm, y2: Mm) {
    layer.set_outline_color(Color::Rgb(Rgb::new(0.5, 0.5, 0.5, None)));
    layer.set_outline_thickness(0.5);

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