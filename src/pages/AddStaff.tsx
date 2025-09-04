import React, { useState, useEffect } from 'react';
import { Save, ArrowLeft, User, Briefcase, DollarSign } from 'lucide-react';
import { Button } from '../components/ui/button';
import { Input } from '../components/ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../components/ui/select';
import { Checkbox } from '../components/ui/checkbox';
import { DatePicker } from '../components/forms/DatePicker';
import { NICInput } from '../components/forms/NICInput';
import { ImageUpload } from '../components/forms/ImageUpload';
import { useStaffById } from '../hooks/useStaff';
import { useCreateStaff, useUpdateStaff } from '../hooks/useStaffMutations';
import { calculateAge, calculateRetirementDate } from '../lib/nicConverter';
import { DESIGNATIONS, SALARY_CODES, MARITAL_STATUSES, FORM_DEFAULTS } from '../types/staff';
import type { CreateStaffRequest, UpdateStaffRequest } from '../types/staff';

interface AddStaffProps {
  staffId?: string;
  onStaffCreated: () => void;
  onStaffUpdated: () => void;
  onCancel: () => void;
}

export function AddStaff({ staffId, onStaffCreated, onStaffUpdated, onCancel }: AddStaffProps) {
  const isEditing = !!staffId;
  const { data: existingStaff, isLoading: isLoadingStaff } = useStaffById(staffId);
  const createStaff = useCreateStaff();
  const updateStaff = useUpdateStaff();

  const [formData, setFormData] = useState<CreateStaffRequest>({
    appointmentNumber: '',
    fullName: '',
    gender: FORM_DEFAULTS.GENDER,
    dateOfBirth: '',
    age: 0,
    nicNumber: '',
    nicNumberOld: undefined,
    maritalStatus: FORM_DEFAULTS.MARITAL_STATUS,
    addressLine1: '',
    addressLine2: '',
    addressLine3: '',
    contactNumber: '',
    email: '',
    designation: FORM_DEFAULTS.DESIGNATION,
    dateOfFirstAppointment: '',
    dateOfRetirement: '',
    incrementDate: '',
    salaryCode: FORM_DEFAULTS.SALARY_CODE,
    basicSalary: 0,
    incrementAmount: 0,
    imageData: undefined,
  });

  const [errors, setErrors] = useState<Record<string, string>>({});
  const [, setNicData] = useState<{ newFormat: string; oldFormat?: string }>({ newFormat: '' });

  // Load existing staff data when editing
  useEffect(() => {
    if (existingStaff && isEditing) {
      setFormData({
        appointmentNumber: existingStaff.appointmentNumber,
        fullName: existingStaff.fullName,
        gender: existingStaff.gender,
        dateOfBirth: existingStaff.dateOfBirth,
        age: existingStaff.age,
        nicNumber: existingStaff.nicNumber,
        nicNumberOld: existingStaff.nicNumberOld,
        maritalStatus: existingStaff.maritalStatus,
        addressLine1: existingStaff.addressLine1 || '',
        addressLine2: existingStaff.addressLine2 || '',
        addressLine3: existingStaff.addressLine3 || '',
        contactNumber: existingStaff.contactNumber || '',
        email: existingStaff.email || '',
        designation: existingStaff.designation,
        dateOfFirstAppointment: existingStaff.dateOfFirstAppointment,
        dateOfRetirement: existingStaff.dateOfRetirement,
        incrementDate: existingStaff.incrementDate || '',
        salaryCode: existingStaff.salaryCode,
        basicSalary: existingStaff.basicSalary,
        incrementAmount: existingStaff.incrementAmount,
        imageData: existingStaff.imageData,
      });
    }
  }, [existingStaff, isEditing]);

  // Auto-calculate age and retirement date when date of birth changes
  useEffect(() => {
    if (formData.dateOfBirth) {
      const age = calculateAge(formData.dateOfBirth);
      const retirementDate = calculateRetirementDate(formData.dateOfBirth);

      setFormData(prev => ({
        ...prev,
        age,
        dateOfRetirement: retirementDate,
      }));
    }
  }, [formData.dateOfBirth]);

  const handleInputChange = (field: keyof CreateStaffRequest, value: any) => {
    setFormData(prev => ({ ...prev, [field]: value }));

    // Clear error when user starts typing
    if (errors[field]) {
      setErrors(prev => ({ ...prev, [field]: '' }));
    }
  };

  const handleNICChange = (data: { newFormat: string; oldFormat?: string }) => {
    setNicData(data);
    handleInputChange('nicNumber', data.newFormat);
    handleInputChange('nicNumberOld', data.oldFormat);
  };

  const handleIncrementDateChange = (value: string) => {
    // Format increment date as DD-MM automatically
    let formatted = value.replace(/\D/g, ''); // Remove non-digits
    if (formatted.length >= 2) {
      formatted = formatted.slice(0, 2) + '-' + formatted.slice(2, 4);
    }
    if (formatted.length > 5) {
      formatted = formatted.slice(0, 5);
    }
    handleInputChange('incrementDate', formatted);
  };

  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {};

    // Required fields
    if (!formData.appointmentNumber.trim()) newErrors.appointmentNumber = 'Appointment number is required';
    if (!formData.fullName.trim()) newErrors.fullName = 'Full name is required';
    if (!formData.dateOfBirth) newErrors.dateOfBirth = 'Date of birth is required';
    if (!formData.nicNumber.trim()) newErrors.nicNumber = 'NIC number is required';
    if (!formData.maritalStatus) newErrors.maritalStatus = 'Marital status is required';
    if (!formData.designation) newErrors.designation = 'Designation is required';
    if (!formData.dateOfFirstAppointment) newErrors.dateOfFirstAppointment = 'Date of first appointment is required';
    if (!formData.salaryCode) newErrors.salaryCode = 'Salary code is required';
    if (formData.basicSalary <= 0) newErrors.basicSalary = 'Basic salary must be greater than 0';
    if (formData.incrementAmount < 0) newErrors.incrementAmount = 'Increment amount cannot be negative';

    // Email validation
    if (formData.email && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
      newErrors.email = 'Invalid email format';
    }

    // Phone number validation
    if (formData.contactNumber && !/^[\d\s\-\+\(\)]+$/.test(formData.contactNumber)) {
      newErrors.contactNumber = 'Invalid contact number format';
    }

    // Increment date validation (DD-MM format)
    if (formData.incrementDate && !/^\d{2}-\d{2}$/.test(formData.incrementDate)) {
      newErrors.incrementDate = 'Increment date must be in DD-MM format';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!validateForm()) {
      return;
    }

    try {
      if (isEditing && staffId) {
        const updateData: UpdateStaffRequest = { ...formData, id: staffId };
        await updateStaff.mutateAsync(updateData);
        onStaffUpdated();
      } else {
        await createStaff.mutateAsync(formData);
        onStaffCreated();
      }
    } catch (error) {
      console.error('Error saving staff:', error);
      setErrors({ submit: 'Failed to save staff record. Please try again.' });
    }
  };

  const isPending = createStaff.isPending || updateStaff.isPending;

  if (isEditing && isLoadingStaff) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="loading-spinner w-8 h-8"></div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-4">
          <Button
            type="button"
            variant="outline"
            size="sm"
            onClick={onCancel}
            className="flex items-center space-x-2"
          >
            <ArrowLeft className="h-4 w-4" />
            <span>Back</span>
          </Button>
          <h2 className="heading-2">
            {isEditing ? 'Edit Staff' : 'Add New Staff'}
          </h2>
        </div>

        <Button
          type="submit"
          form="staff-form"
          disabled={isPending}
          className="flex items-center space-x-2"
        >
          {isPending && <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />}
          <Save className="h-4 w-4" />
          <span>{isEditing ? 'Update Staff' : 'Save Staff'}</span>
        </Button>
      </div>

      {/* Form */}
      <form id="staff-form" onSubmit={handleSubmit} className="space-y-8">
        {/* Image Upload */}
        <div className="form-section">
          <div className="form-section-title">Staff Photo</div>
          <ImageUpload
            value={formData.imageData}
            onChange={(imageData) => handleInputChange('imageData', imageData)}
            label="Upload Staff Photo"
          />
        </div>

        {/* Personal Information */}
        <div className="form-section">
          <div className="form-section-title flex items-center space-x-2">
            <User className="h-5 w-5" />
            <span>Personal Information</span>
          </div>

          <div className="space-y-4">
            <div className="field-row">
              <div className="field-group">
                <label className="block text-sm font-medium text-slate-700 mb-2">
                  Appointment Number <span className="text-red-500">*</span>
                </label>
                <Input
                  value={formData.appointmentNumber}
                  onChange={(e) => handleInputChange('appointmentNumber', e.target.value)}
                  placeholder="Enter appointment number"
                  className={errors.appointmentNumber ? 'border-red-500' : ''}
                />
                {errors.appointmentNumber && (
                  <p className="text-red-500 text-xs mt-1">{errors.appointmentNumber}</p>
                )}
              </div>

              <div className="field-group">
                <label className="block text-sm font-medium text-slate-700 mb-2">
                  Full Name <span className="text-red-500">*</span>
                </label>
                <Input
                  value={formData.fullName}
                  onChange={(e) => handleInputChange('fullName', e.target.value)}
                  placeholder="Enter full name"
                  className={errors.fullName ? 'border-red-500' : ''}
                />
                {errors.fullName && (
                  <p className="text-red-500 text-xs mt-1">{errors.fullName}</p>
                )}
              </div>
            </div>

            <div className="field-row">
              <div className="field-group">
                <label className="block text-sm font-medium text-slate-700 mb-2">
                  Gender <span className="text-red-500">*</span>
                </label>
                <div className="flex items-center space-x-6">
                  <div className="flex items-center space-x-2">
                    <Checkbox
                      checked={formData.gender === 'Male'}
                      onCheckedChange={() => handleInputChange('gender', 'Male')}
                    />
                    <span className="text-sm">Male</span>
                  </div>
                  <div className="flex items-center space-x-2">
                    <Checkbox
                      checked={formData.gender === 'Female'}
                      onCheckedChange={() => handleInputChange('gender', 'Female')}
                    />
                    <span className="text-sm">Female</span>
                  </div>
                </div>
              </div>

              <div className="field-group">
                <DatePicker
                  label="Date of Birth"
                  value={formData.dateOfBirth}
                  onChange={(date) => handleInputChange('dateOfBirth', date)}
                  placeholder="Select date of birth"
                  required
                  className={errors.dateOfBirth ? 'border-red-500' : ''}
                />
                {errors.dateOfBirth && (
                  <p className="text-red-500 text-xs mt-1">{errors.dateOfBirth}</p>
                )}
              </div>
            </div>

            <div className="field-row">
              <div className="field-group">
                <label className="block text-sm font-medium text-slate-700 mb-2">
                  Age (Calculated)
                </label>
                <Input
                  value={formData.age}
                  readOnly
                  className="bg-slate-50"
                />
              </div>

              <div className="field-group">
                <NICInput
                  label="NIC Number"
                  value={formData.nicNumber}
                  onChange={handleNICChange}
                  placeholder="Enter NIC number"
                  required
                  error={errors.nicNumber}
                />
              </div>
            </div>

            <div className="field-group">
              <label className="block text-sm font-medium text-slate-700 mb-2">
                Marital Status <span className="text-red-500">*</span>
              </label>
              <Select value={formData.maritalStatus} onValueChange={(value) => handleInputChange('maritalStatus', value)}>
                <SelectTrigger className={errors.maritalStatus ? 'border-red-500' : ''}>
                  <SelectValue placeholder="Select marital status" />
                </SelectTrigger>
                <SelectContent>
                  {MARITAL_STATUSES.map((status) => (
                    <SelectItem key={status} value={status}>
                      {status}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              {errors.maritalStatus && (
                <p className="text-red-500 text-xs mt-1">{errors.maritalStatus}</p>
              )}
            </div>

            <div className="space-y-4">
              <label className="block text-sm font-medium text-slate-700">Address</label>
              <div className="space-y-2">
                <Input
                  value={formData.addressLine1}
                  onChange={(e) => handleInputChange('addressLine1', e.target.value)}
                  placeholder="Address Line 1"
                />
                <Input
                  value={formData.addressLine2}
                  onChange={(e) => handleInputChange('addressLine2', e.target.value)}
                  placeholder="Address Line 2"
                />
                <Input
                  value={formData.addressLine3}
                  onChange={(e) => handleInputChange('addressLine3', e.target.value)}
                  placeholder="Address Line 3"
                />
              </div>
            </div>

            <div className="field-row">
              <div className="field-group">
                <label className="block text-sm font-medium text-slate-700 mb-2">
                  Contact Number
                </label>
                <Input
                  value={formData.contactNumber}
                  onChange={(e) => handleInputChange('contactNumber', e.target.value)}
                  placeholder="Enter contact number"
                  className={errors.contactNumber ? 'border-red-500' : ''}
                />
                {errors.contactNumber && (
                  <p className="text-red-500 text-xs mt-1">{errors.contactNumber}</p>
                )}
              </div>

              <div className="field-group">
                <label className="block text-sm font-medium text-slate-700 mb-2">
                  Email
                </label>
                <Input
                  type="email"
                  value={formData.email}
                  onChange={(e) => handleInputChange('email', e.target.value)}
                  placeholder="Enter email address"
                  className={errors.email ? 'border-red-500' : ''}
                />
                {errors.email && (
                  <p className="text-red-500 text-xs mt-1">{errors.email}</p>
                )}
              </div>
            </div>
          </div>
        </div>

        {/* Employment Details */}
        <div className="form-section">
          <div className="form-section-title flex items-center space-x-2">
            <Briefcase className="h-5 w-5" />
            <span>Employment Details</span>
          </div>

          <div className="space-y-4">
            <div className="field-group">
              <label className="block text-sm font-medium text-slate-700 mb-2">
                Designation <span className="text-red-500">*</span>
              </label>
              <Select value={formData.designation} onValueChange={(value) => handleInputChange('designation', value)}>
                <SelectTrigger className={errors.designation ? 'border-red-500' : ''}>
                  <SelectValue placeholder="Select designation" />
                </SelectTrigger>
                <SelectContent>
                  {DESIGNATIONS.map((designation) => (
                    <SelectItem key={designation} value={designation}>
                      {designation}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              {errors.designation && (
                <p className="text-red-500 text-xs mt-1">{errors.designation}</p>
              )}
            </div>

            <div className="field-row">
              <div className="field-group">
                <DatePicker
                  label="Date of First Appointment"
                  value={formData.dateOfFirstAppointment}
                  onChange={(date) => handleInputChange('dateOfFirstAppointment', date)}
                  placeholder="Select appointment date"
                  required
                  className={errors.dateOfFirstAppointment ? 'border-red-500' : ''}
                />
                {errors.dateOfFirstAppointment && (
                  <p className="text-red-500 text-xs mt-1">{errors.dateOfFirstAppointment}</p>
                )}
              </div>

              <div className="field-group">
                <label className="block text-sm font-medium text-slate-700 mb-2">
                  Date of Retirement (Age 60)
                </label>
                <Input
                  value={formData.dateOfRetirement}
                  readOnly
                  className="bg-slate-50"
                />
              </div>
            </div>

            <div className="field-group">
              <label className="block text-sm font-medium text-slate-700 mb-2">
                Increment Date (DD-MM)
              </label>
              <Input
                value={formData.incrementDate}
                onChange={(e) => handleIncrementDateChange(e.target.value)}
                placeholder="DD-MM"
                maxLength={5}
                className={errors.incrementDate ? 'border-red-500' : ''}
              />
              {errors.incrementDate && (
                <p className="text-red-500 text-xs mt-1">{errors.incrementDate}</p>
              )}
              <p className="text-xs text-slate-500 mt-1">
                Enter day and month only (e.g., 15-03)
              </p>
            </div>
          </div>
        </div>

        {/* Salary Information */}
        <div className="form-section">
          <div className="form-section-title flex items-center space-x-2">
            <DollarSign className="h-5 w-5" />
            <span>Salary Information</span>
          </div>

          <div className="space-y-4">
            <div className="field-group">
              <label className="block text-sm font-medium text-slate-700 mb-2">
                Salary Code <span className="text-red-500">*</span>
              </label>
              <Select value={formData.salaryCode} onValueChange={(value) => handleInputChange('salaryCode', value)}>
                <SelectTrigger className={errors.salaryCode ? 'border-red-500' : ''}>
                  <SelectValue placeholder="Select salary code" />
                </SelectTrigger>
                <SelectContent>
                  {SALARY_CODES.map((code) => (
                    <SelectItem key={code} value={code}>
                      {code}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              {errors.salaryCode && (
                <p className="text-red-500 text-xs mt-1">{errors.salaryCode}</p>
              )}
            </div>

            <div className="field-row">
              <div className="field-group">
                <label className="block text-sm font-medium text-slate-700 mb-2">
                  Basic Salary (Rs.) <span className="text-red-500">*</span>
                </label>
                <Input
                  type="number"
                  min="0"
                  step="0.01"
                  value={formData.basicSalary}
                  onChange={(e) => handleInputChange('basicSalary', parseFloat(e.target.value) || 0)}
                  placeholder="Enter basic salary"
                  className={errors.basicSalary ? 'border-red-500' : ''}
                />
                {errors.basicSalary && (
                  <p className="text-red-500 text-xs mt-1">{errors.basicSalary}</p>
                )}
              </div>

              <div className="field-group">
                <label className="block text-sm font-medium text-slate-700 mb-2">
                  Increment Amount (Rs.) <span className="text-red-500">*</span>
                </label>
                <Input
                  type="number"
                  min="0"
                  step="0.01"
                  value={formData.incrementAmount}
                  onChange={(e) => handleInputChange('incrementAmount', parseFloat(e.target.value) || 0)}
                  placeholder="Enter increment amount"
                  className={errors.incrementAmount ? 'border-red-500' : ''}
                />
                {errors.incrementAmount && (
                  <p className="text-red-500 text-xs mt-1">{errors.incrementAmount}</p>
                )}
              </div>
            </div>
          </div>
        </div>

        {/* Submit Error */}
        {errors.submit && (
          <div className="bg-red-50 border border-red-200 rounded-lg p-4">
            <p className="text-red-600 text-sm">{errors.submit}</p>
          </div>
        )}
      </form>
    </div>
  );
}