import React from 'react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../ui/select';
import { Checkbox } from '../ui/checkbox';
import { DatePicker } from '../forms/DatePicker';
import { NICInput } from '../forms/NICInput';
import { ImageUpload } from '../forms/ImageUpload';
import { DESIGNATIONS, SALARY_CODES, MARITAL_STATUSES } from '../../types/staff';
import type { CreateStaffRequest } from '../../types/staff';

interface StaffFormProps {
  formData: CreateStaffRequest;
  onChange: (field: keyof CreateStaffRequest, value: any) => void;
  errors: Record<string, string>;
  onSubmit: (e: React.FormEvent) => void;
  isPending: boolean;
  onCancel: () => void;
  isEditing?: boolean;
}

export function StaffForm({
  formData,
  onChange,
  errors,
  onSubmit,
  isPending,
  onCancel,
  isEditing = false
}: StaffFormProps) {
  const handleIncrementDateChange = (value: string) => {
    let formatted = value.replace(/\D/g, '');
    if (formatted.length >= 2) {
      formatted = formatted.slice(0, 2) + '-' + formatted.slice(2, 4);
    }
    if (formatted.length > 5) {
      formatted = formatted.slice(0, 5);
    }
    onChange('incrementDate', formatted);
  };

  const handleNICChange = (data: { newFormat: string; oldFormat?: string }) => {
    onChange('nicNumber', data.newFormat);
    onChange('nicNumberOld', data.oldFormat);
  };

  return (
    <form onSubmit={onSubmit} className="space-y-8">
      <div className="form-section">
        <div className="form-section-title">Staff Photo</div>
        <ImageUpload
          value={formData.imageData}
          onChange={(imageData) => onChange('imageData', imageData)}
          label="Upload Staff Photo"
        />
      </div>

      <div className="form-section">
        <div className="form-section-title">Personal Information</div>

        <div className="field-row">
          <div className="field-group">
            <label className="block text-sm font-medium text-slate-700 mb-2">
              Appointment Number <span className="text-red-500">*</span>
            </label>
            <Input
              value={formData.appointmentNumber}
              onChange={(e) => onChange('appointmentNumber', e.target.value)}
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
              onChange={(e) => onChange('fullName', e.target.value)}
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
                  onCheckedChange={() => onChange('gender', 'Male')}
                />
                <span className="text-sm">Male</span>
              </div>
              <div className="flex items-center space-x-2">
                <Checkbox
                  checked={formData.gender === 'Female'}
                  onCheckedChange={() => onChange('gender', 'Female')}
                />
                <span className="text-sm">Female</span>
              </div>
            </div>
          </div>

          <div className="field-group">
            <DatePicker
              label="Date of Birth"
              value={formData.dateOfBirth}
              onChange={(date) => onChange('dateOfBirth', date)}
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
          <Select value={formData.maritalStatus} onValueChange={(value) => onChange('maritalStatus', value)}>
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
              onChange={(e) => onChange('addressLine1', e.target.value)}
              placeholder="Address Line 1"
            />
            <Input
              value={formData.addressLine2}
              onChange={(e) => onChange('addressLine2', e.target.value)}
              placeholder="Address Line 2"
            />
            <Input
              value={formData.addressLine3}
              onChange={(e) => onChange('addressLine3', e.target.value)}
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
              onChange={(e) => onChange('contactNumber', e.target.value)}
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
              onChange={(e) => onChange('email', e.target.value)}
              placeholder="Enter email address"
              className={errors.email ? 'border-red-500' : ''}
            />
            {errors.email && (
              <p className="text-red-500 text-xs mt-1">{errors.email}</p>
            )}
          </div>
        </div>
      </div>

      <div className="form-section">
        <div className="form-section-title">Employment Details</div>

        <div className="field-group">
          <label className="block text-sm font-medium text-slate-700 mb-2">
            Designation <span className="text-red-500">*</span>
          </label>
          <Select value={formData.designation} onValueChange={(value) => onChange('designation', value)}>
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
              onChange={(date) => onChange('dateOfFirstAppointment', date)}
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

      <div className="form-section">
        <div className="form-section-title">Salary Information</div>

        <div className="field-group">
          <label className="block text-sm font-medium text-slate-700 mb-2">
            Salary Code <span className="text-red-500">*</span>
          </label>
          <Select value={formData.salaryCode} onValueChange={(value) => onChange('salaryCode', value)}>
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
              onChange={(e) => onChange('basicSalary', parseFloat(e.target.value) || 0)}
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
              onChange={(e) => onChange('incrementAmount', parseFloat(e.target.value) || 0)}
              placeholder="Enter increment amount"
              className={errors.incrementAmount ? 'border-red-500' : ''}
            />
            {errors.incrementAmount && (
              <p className="text-red-500 text-xs mt-1">{errors.incrementAmount}</p>
            )}
          </div>
        </div>
      </div>

      <div className="flex justify-end space-x-4">
        <Button
          type="button"
          variant="outline"
          onClick={onCancel}
          disabled={isPending}
        >
          Cancel
        </Button>
        <Button
          type="submit"
          disabled={isPending}
          className="flex items-center space-x-2"
        >
          {isPending && <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />}
          <span>{isEditing ? 'Update Staff' : 'Save Staff'}</span>
        </Button>
      </div>
    </form>
  );
}