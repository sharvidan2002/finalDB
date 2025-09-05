export interface Staff {
  id: string;

  // Identification & Personal Details
  appointmentNumber: string;
  fullName: string;
  gender: 'Male' | 'Female';
  dateOfBirth: string;
  age: number;
  nicNumber: string;
  nicNumberOld?: string;
  maritalStatus: string;
  addressLine1?: string;
  addressLine2?: string;
  addressLine3?: string;
  contactNumber?: string;
  email?: string;

  // Employment Details
  designation: string;
  dateOfFirstAppointment: string;
  dateOfRetirement: string;
  incrementDate?: string;

  // Salary Information
  salaryCode: string;
  basicSalary: number;
  incrementAmount: number;

  // Image
  imageData?: string;

  // Timestamps
  createdAt: string;
  updatedAt: string;
}

export interface CreateStaffRequest {
  // Identification & Personal Details
  appointmentNumber: string;
  fullName: string;
  gender: 'Male' | 'Female';
  dateOfBirth: string;
  age: number;
  nicNumber: string;
  nicNumberOld?: string;
  maritalStatus: string;
  addressLine1?: string;
  addressLine2?: string;
  addressLine3?: string;
  contactNumber?: string;
  email?: string;

  // Employment Details
  designation: string;
  dateOfFirstAppointment: string;
  dateOfRetirement: string;
  incrementDate?: string;

  // Salary Information
  salaryCode: string;
  basicSalary: number;
  incrementAmount: number;

  // Image
  imageData?: string;
}

export interface UpdateStaffRequest extends CreateStaffRequest {
  id: string;
}

export interface StaffSearchParams {
  searchTerm?: string;
  designation?: string;
  gender?: 'Male' | 'Female';
  ageMin?: number;
  ageMax?: number;
  nicNumber?: string;
  salaryCode?: string;
}

export interface PrintStaffBulkParams {
  staffIds: string[];
  filters?: StaffSearchParams;
}

export const DESIGNATIONS = [
  'District Forest Officer',
  'Asst.District Forest Officer',
  'Management Service Officer',
  'Development Officer',
  'Range Forest officer',
  'Beat forest officer',
  'extension officer',
  'field forest assistant',
  'office employee service',
  'garden labour',
] as const;

export const SALARY_CODES = [
  'S1',
  'S2',
  'S3',
  'D1',
  'D2',
  'D3',
  'A1',
  'A2',
] as const;

export const MARITAL_STATUSES = [
  'Single',
  'Married',
  'Divorced',
  'Widowed',
] as const;

export const GENDERS = [
  'Male',
  'Female',
] as const;

export type Designation = typeof DESIGNATIONS[number];
export type SalaryCode = typeof SALARY_CODES[number];
export type MaritalStatus = typeof MARITAL_STATUSES[number];
export type Gender = typeof GENDERS[number];

export const FORM_DEFAULTS = {
  GENDER: 'Male' as const,
  MARITAL_STATUS: 'Single' as const,
  DESIGNATION: 'field forest assistant' as const,
  SALARY_CODE: 'S1' as const,
} as const;