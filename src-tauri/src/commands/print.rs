use tauri::{command, AppHandle};
use crate::database::{
    operations::get_staff_by_id as db_get_staff_by_id,
    operations::get_all_staff as db_get_all_staff,
    schema::Staff,
};
use std::fs;
use genpdf::elements::{Paragraph, TableLayout, LinearLayout};
use genpdf::fonts;
use genpdf::{Document, Element, SimplePageDecorator};
use std::path::PathBuf;

fn get_downloads_dir() -> Result<PathBuf, String> {
    // Try to get downloads directory using different methods
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

    // Get staff data
    let staff = db_get_staff_by_id(&app_data_dir, &staff_id)
        .map_err(|e| format!("Failed to get staff: {}", e))?;

    // Generate PDF
    let pdf_content = generate_individual_pdf(&staff)?;

    // Generate filename
    let filename = format!("staff-details-{}-{}.pdf",
                          staff.full_name.replace(" ", "_"),
                          chrono::Utc::now().format("%Y%m%d"));

    // Get downloads directory
    let downloads_dir = get_downloads_dir()?;
    let file_path = downloads_dir.join(&filename);

    // Write PDF file
    fs::write(&file_path, pdf_content)
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

    // Get staff data
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

    // Generate PDF
    let pdf_content = generate_bulk_pdf(&staff_list)?;

    // Generate filename
    let filename = format!("staff-directory-{}-{}.pdf",
                          staff_list.len(),
                          chrono::Utc::now().format("%Y%m%d"));

    // Get downloads directory
    let downloads_dir = get_downloads_dir()?;
    let file_path = downloads_dir.join(&filename);

    // Write PDF file
    fs::write(&file_path, pdf_content)
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

// Legacy functions for backward compatibility
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
    // Create a new PDF document
    let font_family = fonts::from_files("./fonts", "DejaVuSans", None)
        .unwrap_or_else(|_| fonts::builtin::HELVETICA);

    let mut doc = Document::new(font_family);
    doc.set_title("Staff Details");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);

    // Create page decorator
    let decorator = SimplePageDecorator::new(doc.get_font(), 10, "Divisional Forest Office - Vavuniya, Sri Lanka", None);
    doc.set_page_decorator(Box::new(decorator));

    // Header
    doc.push(
        Paragraph::new("Divisional Forest Office")
            .aligned(genpdf::Alignment::Center)
            .styled(genpdf::style::Style::new().bold().with_font_size(20))
    );

    doc.push(
        Paragraph::new("Vavuniya, Sri Lanka")
            .aligned(genpdf::Alignment::Center)
            .styled(genpdf::style::Style::new().with_font_size(14))
    );

    doc.push(
        Paragraph::new("Staff Information Sheet")
            .aligned(genpdf::Alignment::Center)
            .styled(genpdf::style::Style::new().bold().with_font_size(16))
    );

    doc.push(genpdf::elements::Break::new(2));

    // Personal Information Section
    doc.push(
        Paragraph::new("PERSONAL INFORMATION")
            .styled(genpdf::style::Style::new().bold().with_font_size(14))
    );

    let personal_info = vec![
        ("Appointment Number", &staff.appointment_number),
        ("Full Name", &staff.full_name),
        ("Gender", &staff.gender),
        ("Date of Birth", &staff.date_of_birth),
        ("Age", &format!("{} years", staff.age)),
        ("NIC Number", &staff.nic_number),
        ("Marital Status", &staff.marital_status),
        ("Contact Number", staff.contact_number.as_deref().unwrap_or("N/A")),
        ("Email", staff.email.as_deref().unwrap_or("N/A")),
    ];

    for (label, value) in personal_info {
        doc.push(
            Paragraph::new(format!("{}: {}", label, value))
                .styled(genpdf::style::Style::new().with_font_size(11))
        );
    }

    // Address
    let mut address = String::new();
    if let Some(line1) = &staff.address_line1 {
        if !line1.is_empty() { address.push_str(line1); }
    }
    if let Some(line2) = &staff.address_line2 {
        if !line2.is_empty() {
            if !address.is_empty() { address.push_str(", "); }
            address.push_str(line2);
        }
    }
    if let Some(line3) = &staff.address_line3 {
        if !line3.is_empty() {
            if !address.is_empty() { address.push_str(", "); }
            address.push_str(line3);
        }
    }
    if address.is_empty() { address = "Not provided".to_string(); }

    doc.push(
        Paragraph::new(format!("Address: {}", address))
            .styled(genpdf::style::Style::new().with_font_size(11))
    );

    doc.push(genpdf::elements::Break::new(1));

    // Employment Details Section
    doc.push(
        Paragraph::new("EMPLOYMENT DETAILS")
            .styled(genpdf::style::Style::new().bold().with_font_size(14))
    );

    let employment_info = vec![
        ("Designation", &staff.designation),
        ("Date of First Appointment", &staff.date_of_first_appointment),
        ("Date of Retirement", &staff.date_of_retirement),
        ("Increment Date", staff.increment_date.as_deref().unwrap_or("N/A")),
    ];

    for (label, value) in employment_info {
        doc.push(
            Paragraph::new(format!("{}: {}", label, value))
                .styled(genpdf::style::Style::new().with_font_size(11))
        );
    }

    doc.push(genpdf::elements::Break::new(1));

    // Salary Information Section
    doc.push(
        Paragraph::new("SALARY INFORMATION")
            .styled(genpdf::style::Style::new().bold().with_font_size(14))
    );

    doc.push(
        Paragraph::new(format!("Salary Code: {}", staff.salary_code))
            .styled(genpdf::style::Style::new().with_font_size(11))
    );

    doc.push(
        Paragraph::new(format!("Basic Salary: Rs. {:.2}", staff.basic_salary))
            .styled(genpdf::style::Style::new().with_font_size(11))
    );

    doc.push(
        Paragraph::new(format!("Increment Amount: Rs. {:.2}", staff.increment_amount))
            .styled(genpdf::style::Style::new().with_font_size(11))
    );

    doc.push(
        Paragraph::new(format!("Total Salary: Rs. {:.2}", staff.basic_salary + staff.increment_amount))
            .styled(genpdf::style::Style::new().bold().with_font_size(12))
    );

    // Generate PDF
    let mut buf = Vec::new();
    doc.render(&mut buf).map_err(|e| format!("Failed to generate PDF: {}", e))?;

    Ok(buf)
}

fn generate_bulk_pdf(staff_list: &[Staff]) -> Result<Vec<u8>, String> {
    // Create a new PDF document
    let font_family = fonts::from_files("./fonts", "DejaVuSans", None)
        .unwrap_or_else(|_| fonts::builtin::HELVETICA);

    let mut doc = Document::new(font_family);
    doc.set_title("Staff Directory");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);

    // Header
    doc.push(
        Paragraph::new("Divisional Forest Office")
            .aligned(genpdf::Alignment::Center)
            .styled(genpdf::style::Style::new().bold().with_font_size(18))
    );

    doc.push(
        Paragraph::new("Vavuniya, Sri Lanka")
            .aligned(genpdf::Alignment::Center)
            .styled(genpdf::style::Style::new().with_font_size(12))
    );

    doc.push(
        Paragraph::new("Staff Directory")
            .aligned(genpdf::Alignment::Center)
            .styled(genpdf::style::Style::new().bold().with_font_size(14))
    );

    doc.push(
        Paragraph::new(format!("Total Staff: {} | Generated: {}",
                              staff_list.len(),
                              chrono::Utc::now().format("%Y-%m-%d")))
            .aligned(genpdf::Alignment::Right)
            .styled(genpdf::style::Style::new().with_font_size(10))
    );

    doc.push(genpdf::elements::Break::new(2));

    // Create table
    let mut table = TableLayout::new(vec![1, 2, 3, 2, 1, 2, 2, 1, 2]);
    table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new(true, true, false));

    // Table headers
    table.row().element(Paragraph::new("#").styled(genpdf::style::Style::new().bold()));
    table.row().element(Paragraph::new("Appointment No.").styled(genpdf::style::Style::new().bold()));
    table.row().element(Paragraph::new("Full Name").styled(genpdf::style::Style::new().bold()));
    table.row().element(Paragraph::new("Designation").styled(genpdf::style::Style::new().bold()));
    table.row().element(Paragraph::new("Age").styled(genpdf::style::Style::new().bold()));
    table.row().element(Paragraph::new("NIC Number").styled(genpdf::style::Style::new().bold()));
    table.row().element(Paragraph::new("Contact").styled(genpdf::style::Style::new().bold()));
    table.row().element(Paragraph::new("Salary Code").styled(genpdf::style::Style::new().bold()));
    table.row().element(Paragraph::new("Basic Salary").styled(genpdf::style::Style::new().bold()));

    // Add staff data
    for (index, staff) in staff_list.iter().enumerate() {
        table.row().element(Paragraph::new((index + 1).to_string()));
        table.row().element(Paragraph::new(&staff.appointment_number));
        table.row().element(Paragraph::new(&staff.full_name));
        table.row().element(Paragraph::new(&staff.designation));
        table.row().element(Paragraph::new(staff.age.to_string()));
        table.row().element(Paragraph::new(&staff.nic_number));
        table.row().element(Paragraph::new(staff.contact_number.as_deref().unwrap_or("N/A")));
        table.row().element(Paragraph::new(&staff.salary_code));
        table.row().element(Paragraph::new(format!("Rs. {:.0}", staff.basic_salary)));
    }

    doc.push(table);

    // Generate PDF
    let mut buf = Vec::new();
    doc.render(&mut buf).map_err(|e| format!("Failed to generate PDF: {}", e))?;

    Ok(buf)
}