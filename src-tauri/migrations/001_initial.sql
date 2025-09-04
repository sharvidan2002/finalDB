CREATE TABLE IF NOT EXISTS staff (
    id TEXT PRIMARY KEY NOT NULL,

    -- Identification & Personal Details
    appointment_number TEXT NOT NULL UNIQUE,
    full_name TEXT NOT NULL,
    gender TEXT NOT NULL CHECK(gender IN ('Male', 'Female')),
    date_of_birth TEXT NOT NULL,
    age INTEGER NOT NULL,
    nic_number TEXT NOT NULL UNIQUE,
    nic_number_old TEXT,
    marital_status TEXT NOT NULL,
    address_line1 TEXT,
    address_line2 TEXT,
    address_line3 TEXT,
    contact_number TEXT,
    email TEXT,

    -- Employment Details
    designation TEXT NOT NULL,
    date_of_first_appointment TEXT NOT NULL,
    date_of_retirement TEXT NOT NULL,
    increment_date TEXT,

    -- Salary Information
    salary_code TEXT NOT NULL,
    basic_salary REAL NOT NULL,
    increment_amount REAL NOT NULL,

    -- Image
    image_data TEXT,

    -- Timestamps
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_staff_nic ON staff(nic_number);
CREATE INDEX IF NOT EXISTS idx_staff_nic_old ON staff(nic_number_old);
CREATE INDEX IF NOT EXISTS idx_staff_appointment ON staff(appointment_number);
CREATE INDEX IF NOT EXISTS idx_staff_designation ON staff(designation);
CREATE INDEX IF NOT EXISTS idx_staff_name ON staff(full_name);

-- Create trigger to update updated_at timestamp
CREATE TRIGGER IF NOT EXISTS update_staff_timestamp
    AFTER UPDATE ON staff
    FOR EACH ROW
    BEGIN
        UPDATE staff SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;