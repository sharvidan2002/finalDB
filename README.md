# Forest Office Staff Management System

A modern desktop application for managing staff records at the Divisional Forest Office - Vavuniya, Sri Lanka. Built with Tauri, React 18, TypeScript, and SQLite.

## Features

### Core Features
- ✅ **No Authentication Required** - Direct access to dashboard
- ✅ **Staff Management** - Add, edit, delete, and view staff records
- ✅ **Search & Filtering** - Advanced search with multiple filter options
- ✅ **NIC Conversion** - Automatic conversion between old and new NIC formats
- ✅ **Photo Management** - Upload and crop staff photos
- ✅ **Age Calculation** - Automatic age calculation from date of birth
- ✅ **Retirement Calculation** - Auto-calculated retirement date (age 60)

### Staff Information Fields
- **Personal Details**: Name, Gender, DOB, Age, NIC, Address, Contact, Email
- **Employment**: Designation, Appointment Date, Retirement Date, Increment Date
- **Salary**: Salary Code, Basic Salary, Increment Amount

### Print & Export
- ✅ **Individual Reports** - Professional single-staff printouts
- ✅ **Bulk Reports** - Multiple staff in tabular format
- ✅ **PDF Export** - Export reports as HTML files
- ✅ **Filter-based Printing** - Print selected staff based on filters

### Modern UI
- ✅ **Professional Design** - Clean, modern interface with shadows and animations
- ✅ **Responsive Layout** - Works on different screen sizes
- ✅ **Date Pickers** - Professional calendar components
- ✅ **Form Validation** - Real-time validation with helpful error messages

## Technology Stack

| Component | Technology | Version |
|-----------|------------|---------|
| **Desktop Framework** | Tauri | 1.5.x |
| **Frontend** | React 18 + TypeScript | 18.2.x |
| **Database** | SQLite with rusqlite | Latest |
| **State Management** | TanStack Query | 5.x |
| **UI Components** | shadcn/ui | Latest |
| **Styling** | Tailwind CSS | 3.4.x |
| **Build System** | Vite | 5.x |
| **Backend** | Rust | Latest |

## Prerequisites

Before you begin, ensure you have the following installed on your system:

### Required Software
1. **Node.js** (v18 or later) - [Download here](https://nodejs.org/)
2. **Rust** (latest stable) - [Install from here](https://rustup.rs/)
3. **Tauri CLI** - Will be installed via npm

### System Requirements
- **Windows**: Windows 10/11 (64-bit)
- **macOS**: macOS 10.15 or later
- **Linux**: Ubuntu 18.04+ or equivalent
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 500MB for installation

## Installation & Setup

### 1. Clone or Create Project Directory
```bash
mkdir forest-office-staff-app
cd forest-office-staff-app
```

### 2. Install Dependencies
```bash
# Install Node.js dependencies
npm install

# The Tauri CLI will be installed as part of devDependencies
```

### 3. Install Rust Dependencies
```bash
# Ensure Rust is up to date
rustup update

# Install additional targets if needed
rustup target add x86_64-pc-windows-msvc  # Windows
rustup target add x86_64-apple-darwin     # macOS
rustup target add x86_64-unknown-linux-gnu # Linux
```

### 4. Development Setup
```bash
# Start development server
npm run tauri:dev

# This will:
# 1. Start Vite dev server on http://localhost:1420
# 2. Compile Rust backend
# 3. Launch the desktop application
```

## Build for Production

### 1. Development Build
```bash
npm run tauri:dev
```

### 2. Production Build
```bash
# Build for current platform
npm run tauri:build

# Built files will be in src-tauri/target/release/
```

### 3. Cross-Platform Builds
```bash
# Windows (from any platform)
npm run tauri:build -- --target x86_64-pc-windows-msvc

# macOS (from macOS)
npm run tauri:build -- --target x86_64-apple-darwin

# Linux (from Linux)
npm run tauri:build -- --target x86_64-unknown-linux-gnu
```

## Project Structure

```
forest-office-staff-app/
├── src/                              # React frontend source
│   ├── components/                   # Reusable components
│   │   ├── ui/                      # shadcn/ui base components
│   │   ├── layout/                  # Layout components
│   │   ├── staff/                   # Staff-specific components
│   │   └── forms/                   # Form components
│   ├── pages/                       # Main page components
│   ├── hooks/                       # Custom React hooks
│   ├── lib/                         # Utilities and configurations
│   ├── types/                       # TypeScript definitions
│   └── styles/                      # Styling files
├── src-tauri/                       # Tauri backend source
│   ├── src/                        # Rust source code
│   │   ├── database/               # Database operations
│   │   ├── commands/               # Tauri commands
│   │   └── utils/                  # Rust utilities
│   ├── migrations/                 # Database migrations
│   └── icons/                      # App icons
└── Configuration files
```

## Database

The application uses **SQLite** as the database, which is:
- ✅ **Lightweight** - Single file database
- ✅ **Fast** - Optimized for desktop applications
- ✅ **Reliable** - ACID compliant
- ✅ **Portable** - Database file can be easily backed up

### Database Location
- **Windows**: `%APPDATA%/forest-office-staff-app/staff_database.db`
- **macOS**: `~/Library/Application Support/forest-office-staff-app/staff_database.db`
- **Linux**: `~/.local/share/forest-office-staff-app/staff_database.db`

## Features Guide

### Adding Staff
1. Click on "Add Staff" tab
2. Upload staff photo (optional)
3. Fill in personal information
4. Enter NIC number (supports both old and new formats)
5. Add employment details
6. Set salary information
7. Click "Save Staff"

### Searching Staff
1. Use the search bar for quick text search
2. Click "Filters" for advanced filtering:
   - Filter by designation
   - Filter by age range
   - Filter by salary code
3. Select multiple staff for bulk operations

### Printing & Export
1. **Individual Print**: View a staff member → Click "Print"
2. **Bulk Print**: Select multiple staff → Click "Print Selected"
3. **Export**: Click "Export" to download HTML file

### NIC Number Handling
- **Old Format**: `741922757V` → Automatically converts to `197419202757`
- **New Format**: `200012345678` → Recognized as new format
- **Gender Detection**: Automatically detects gender from NIC

## Troubleshooting

### Common Issues

#### 1. Rust Compilation Errors
```bash
# Update Rust and dependencies
rustup update
cargo clean
cargo build
```

#### 2. Node.js Dependencies
```bash
# Clear cache and reinstall
npm cache clean --force
rm -rf node_modules package-lock.json
npm install
```

#### 3. Tauri Build Issues
```bash
# Check Tauri prerequisites
npx @tauri-apps/cli info

# Update Tauri
npm update @tauri-apps/cli
```

#### 4. Database Issues
- Database file permissions
- Corrupted database (delete file to reset)
- Check app data directory exists

### Performance Tips
1. **Database**: Regularly backup the SQLite file
2. **Images**: Keep staff photos under 1MB for better performance
3. **Memory**: Close application completely between sessions

## Contributing

### Code Style
- **TypeScript**: Strict mode enabled
- **React**: Functional components with hooks
- **Rust**: Standard formatting with `cargo fmt`
- **CSS**: Tailwind utility classes

### Adding New Features
1. Update database schema in `migrations/`
2. Add Rust commands in `src-tauri/src/commands/`
3. Update TypeScript types in `src/types/`
4. Create React components and hooks
5. Test thoroughly before submitting

## Support

### Getting Help
1. **Documentation**: Check this README and inline code comments
2. **Issues**: Common problems and solutions listed above
3. **Tauri Docs**: [https://tauri.app/](https://tauri.app/)
4. **React Query Docs**: [https://tanstack.com/query/](https://tanstack.com/query/)

### System Information
To get system info for troubleshooting:
```bash
npx @tauri-apps/cli info
```

## License

This project is developed for the Divisional Forest Office - Vavuniya, Sri Lanka.

## Deployment Notes

### For End Users
1. Download the built application file (.exe, .dmg, or .AppImage)
2. Install the application
3. Launch and start adding staff records
4. No additional setup required - database is created automatically

### For IT Departments
- **Backup**: Regularly backup the database file
- **Updates**: Replace application file for updates (data is preserved)
- **Network**: Application works offline, no internet required
- **Security**: All data stored locally, no external connections

## Version History

- **v1.0.0** - Initial release with all core features
  - Staff management (CRUD operations)
  - Search and filtering
  - Print and export functionality
  - Modern responsive UI
  - SQLite database integration