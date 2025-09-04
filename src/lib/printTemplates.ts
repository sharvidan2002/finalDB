import type { Staff } from '../types/staff';
import { formatDate, formatCurrency } from './utils';

export function generateIndividualPrintTemplate(staff: Staff): string {
  return `
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="UTF-8">
        <title>Staff Details - ${staff.fullName}</title>
        <style>
            @page { margin: 20px; }
            body {
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                margin: 0;
                padding: 20px;
                color: #2c3e50;
                line-height: 1.6;
            }
            .header {
                text-align: center;
                margin-bottom: 30px;
                border-bottom: 3px solid #34495e;
                padding-bottom: 20px;
            }
            .header h1 {
                color: #2c3e50;
                margin: 0;
                font-size: 28px;
                font-weight: 700;
            }
            .header h2 {
                color: #7f8c8d;
                margin: 5px 0;
                font-size: 18px;
                font-weight: 400;
            }
            .staff-photo {
                float: right;
                width: 120px;
                height: 150px;
                border: 2px solid #bdc3c7;
                margin-left: 20px;
                border-radius: 8px;
                overflow: hidden;
                background-color: #ecf0f1;
                display: flex;
                align-items: center;
                justify-content: center;
                color: #95a5a6;
                font-size: 12px;
            }
            .staff-info { overflow: hidden; }
            .section {
                margin-bottom: 25px;
                background: white;
                border-radius: 8px;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                padding: 20px;
            }
            .section-title {
                background: linear-gradient(135deg, #3498db, #2980b9);
                color: white;
                padding: 12px 20px;
                margin: -20px -20px 20px -20px;
                font-weight: 600;
                font-size: 16px;
                border-radius: 8px 8px 0 0;
                text-transform: uppercase;
                letter-spacing: 0.5px;
            }
            .field-row {
                display: flex;
                margin-bottom: 12px;
                padding: 8px 0;
                border-bottom: 1px solid #ecf0f1;
            }
            .field-row:last-child { border-bottom: none; }
            .field-label {
                width: 200px;
                font-weight: 600;
                color: #34495e;
                font-size: 14px;
            }
            .field-value {
                flex: 1;
                color: #2c3e50;
                font-size: 14px;
            }
            .address { margin-top: 5px; }
            .footer {
                margin-top: 40px;
                text-align: center;
                color: #7f8c8d;
                font-size: 12px;
                border-top: 1px solid #ecf0f1;
                padding-top: 20px;
            }
            .salary-highlight {
                background: linear-gradient(135deg, #27ae60, #229954);
                color: white;
                padding: 2px 8px;
                border-radius: 4px;
                font-weight: 600;
            }
        </style>
    </head>
    <body>
        <div class="header">
            <h1>Divisional Forest Office</h1>
            <h2>Vavuniya, Sri Lanka</h2>
            <h2>Staff Information Sheet</h2>
        </div>

        <div class="staff-info">
            ${staff.imageData ?
                `<div class="staff-photo" style="background-image: url('data:image/jpeg;base64,${staff.imageData}'); background-size: cover; background-position: center;"></div>` :
                '<div class="staff-photo">No Photo Available</div>'
            }

            <div class="section">
                <div class="section-title">Personal Information</div>
                <div class="field-row">
                    <div class="field-label">Appointment Number:</div>
                    <div class="field-value">${staff.appointmentNumber}</div>
                </div>
                <div class="field-row">
                    <div class="field-label">Full Name:</div>
                    <div class="field-value">${staff.fullName}</div>
                </div>
                <div class="field-row">
                    <div class="field-label">Gender:</div>
                    <div class="field-value">${staff.gender}</div>
                </div>
                <div class="field-row">
                    <div class="field-label">Date of Birth:</div>
                    <div class="field-value">${formatDate(staff.dateOfBirth)}</div>
                </div>
                <div class="field-row">
                    <div class="field-label">Age:</div>
                    <div class="field-value">${staff.age} years</div>
                </div>
                <div class="field-row">
                    <div class="field-label">NIC Number:</div>
                    <div class="field-value">${staff.nicNumber}</div>
                </div>
                <div class="field-row">
                    <div class="field-label">Marital Status:</div>
                    <div class="field-value">${staff.maritalStatus}</div>
                </div>
                <div class="field-row">
                    <div class="field-label">Address:</div>
                    <div class="field-value address">
                        ${staff.addressLine1 || ''}<br/>
                        ${staff.addressLine2 || ''}<br/>
                        ${staff.addressLine3 || ''}
                    </div>
                </div>
                <div class="field-row">
                    <div class="field-label">Contact Number:</div>
                    <div class="field-value">${staff.contactNumber || 'N/A'}</div>
                </div>
                <div class="field-row">
                    <div class="field-label">Email:</div>
                    <div class="field-value">${staff.email || 'N/A'}</div>
                </div>
            </div>

            <div class="section">
                <div class="section-title">Employment Details</div>
                <div class="field-row">
                    <div class="field-label">Designation:</div>
                    <div class="field-value">${staff.designation}</div>
                </div>
                <div class="field-row">
                    <div class="field-label">Date of First Appointment:</div>
                    <div class="field-value">${formatDate(staff.dateOfFirstAppointment)}</div>
                </div>
                <div class="field-row">
                    <div class="field-label">Date of Retirement:</div>
                    <div class="field-value">${formatDate(staff.dateOfRetirement)}</div>
                </div>
                <div class="field-row">
                    <div class="field-label">Increment Date:</div>
                    <div class="field-value">${staff.incrementDate || 'N/A'}</div>
                </div>
            </div>

            <div class="section">
                <div class="section-title">Salary Information</div>
                <div class="field-row">
                    <div class="field-label">Salary Code:</div>
                    <div class="field-value">${staff.salaryCode}</div>
                </div>
                <div class="field-row">
                    <div class="field-label">Basic Salary:</div>
                    <div class="field-value">
                        <span class="salary-highlight">${formatCurrency(staff.basicSalary)}</span>
                    </div>
                </div>
                <div class="field-row">
                    <div class="field-label">Increment Amount:</div>
                    <div class="field-value">
                        <span class="salary-highlight">${formatCurrency(staff.incrementAmount)}</span>
                    </div>
                </div>
            </div>
        </div>

        <div class="footer">
            Generated on ${formatDate(new Date().toISOString(), 'dd-MM-yyyy HH:mm')} | Divisional Forest Office - Vavuniya, Sri Lanka
        </div>
    </body>
    </html>
  `;
}

export function generateBulkPrintTemplate(staffList: Staff[]): string {
  const tableRows = staffList.map((staff, index) => `
    <tr class="${index % 2 === 0 ? 'even-row' : 'odd-row'}">
        <td>${index + 1}</td>
        <td>${staff.appointmentNumber}</td>
        <td>${staff.fullName}</td>
        <td>${staff.designation}</td>
        <td>${staff.age}</td>
        <td>${staff.nicNumber}</td>
        <td>${staff.contactNumber || 'N/A'}</td>
        <td>${staff.salaryCode}</td>
        <td>${formatCurrency(staff.basicSalary)}</td>
    </tr>
  `).join('');

  return `
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="UTF-8">
        <title>Staff Directory</title>
        <style>
            @page { margin: 15px; size: A4 landscape; }
            body {
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                margin: 0;
                padding: 10px;
                color: #2c3e50;
                font-size: 11px;
                background: #f8f9fa;
            }
            .header {
                text-align: center;
                margin-bottom: 20px;
                background: white;
                padding: 20px;
                border-radius: 8px;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            }
            .header h1 {
                color: #2c3e50;
                margin: 0;
                font-size: 24px;
                font-weight: 700;
            }
            .header h2 {
                color: #7f8c8d;
                margin: 5px 0;
                font-size: 16px;
                font-weight: 400;
            }
            .container {
                background: white;
                border-radius: 8px;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                padding: 20px;
            }
            table {
                width: 100%;
                border-collapse: collapse;
                margin-top: 15px;
                font-size: 10px;
            }
            th {
                background: linear-gradient(135deg, #34495e, #2c3e50);
                color: white;
                padding: 12px 8px;
                text-align: left;
                font-weight: 600;
                border: none;
                font-size: 11px;
                text-transform: uppercase;
                letter-spacing: 0.5px;
            }
            td {
                padding: 8px;
                border: 1px solid #e5e5e5;
                vertical-align: middle;
            }
            .even-row { background-color: #f8f9fa; }
            .odd-row { background-color: white; }
            .even-row:hover, .odd-row:hover {
                background-color: #e8f4fd;
            }
            .footer {
                margin-top: 20px;
                text-align: center;
                color: #7f8c8d;
                font-size: 10px;
                background: white;
                padding: 15px;
                border-radius: 8px;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            }
            .summary {
                margin-bottom: 15px;
                text-align: right;
                color: #2c3e50;
                font-weight: 600;
                font-size: 12px;
                background: linear-gradient(135deg, #3498db, #2980b9);
                color: white;
                padding: 10px 15px;
                border-radius: 6px;
                display: inline-block;
                float: right;
                clear: both;
            }
            .clearfix::after {
                content: "";
                display: table;
                clear: both;
            }
        </style>
    </head>
    <body>
        <div class="header">
            <h1>Divisional Forest Office</h1>
            <h2>Vavuniya, Sri Lanka</h2>
            <h2>Staff Directory</h2>
        </div>

        <div class="container">
            <div class="clearfix">
                <div class="summary">
                    Total Staff: ${staffList.length} | Generated: ${formatDate(new Date().toISOString(), 'dd-MM-yyyy HH:mm')}
                </div>
            </div>

            <table>
                <thead>
                    <tr>
                        <th style="width: 40px;">#</th>
                        <th style="width: 120px;">Appointment No.</th>
                        <th style="width: 180px;">Full Name</th>
                        <th style="width: 150px;">Designation</th>
                        <th style="width: 50px;">Age</th>
                        <th style="width: 120px;">NIC Number</th>
                        <th style="width: 100px;">Contact</th>
                        <th style="width: 80px;">Salary Code</th>
                        <th style="width: 100px;">Basic Salary</th>
                    </tr>
                </thead>
                <tbody>
                    ${tableRows}
                </tbody>
            </table>
        </div>

        <div class="footer">
            Divisional Forest Office - Vavuniya, Sri Lanka<br/>
            This document is computer generated and does not require a signature.
        </div>
    </body>
    </html>
  `;
}