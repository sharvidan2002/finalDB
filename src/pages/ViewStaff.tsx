// import React from 'react';
import { ArrowLeft, Edit, Printer, Download, User, Briefcase, DollarSign, Calendar, Phone, Mail, MapPin, Hash, CreditCard } from 'lucide-react';
import { Button } from '../components/ui/button';
import { useStaffById } from '../hooks/useStaff';
import { usePrintIndividual, useExportToPDF } from '../hooks/usePrint';
import { formatDate, formatCurrency } from '../lib/utils';

interface ViewStaffProps {
  staffId: string;
  onEdit: () => void;
  onBack: () => void;
}

export function ViewStaff({ staffId, onEdit, onBack }: ViewStaffProps) {
  const { data: staff, isLoading, error } = useStaffById(staffId);
  const printIndividual = usePrintIndividual();
  const exportToPDF = useExportToPDF();

  const handlePrint = () => {
    if (staff) {
      printIndividual.mutate(staff.id);
    }
  };

  const handleExport = () => {
    if (staff) {
      exportToPDF.mutate({
        staffIds: [staff.id],
        isBulk: false
      });
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="loading-spinner w-8 h-8"></div>
      </div>
    );
  }

  if (error || !staff) {
    return (
      <div className="text-center py-12">
        <h3 className="text-lg font-medium text-slate-700 mb-2">Staff not found</h3>
        <p className="text-slate-500 mb-4">The requested staff record could not be found.</p>
        <Button onClick={onBack} variant="outline">
          Back to Search
        </Button>
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
            onClick={onBack}
            className="flex items-center space-x-2"
          >
            <ArrowLeft className="h-4 w-4" />
            <span>Back</span>
          </Button>
          <h2 className="heading-2">Staff Details</h2>
        </div>

        <div className="flex space-x-2">
          <Button
            variant="outline"
            onClick={handlePrint}
            disabled={printIndividual.isPending}
            className="flex items-center space-x-2"
          >
            {printIndividual.isPending && (
              <div className="w-4 h-4 border-2 border-slate-600 border-t-transparent rounded-full animate-spin" />
            )}
            <Printer className="h-4 w-4" />
            <span>Print</span>
          </Button>
          <Button
            variant="outline"
            onClick={handleExport}
            disabled={exportToPDF.isPending}
            className="flex items-center space-x-2"
          >
            {exportToPDF.isPending && (
              <div className="w-4 h-4 border-2 border-slate-600 border-t-transparent rounded-full animate-spin" />
            )}
            <Download className="h-4 w-4" />
            <span>Export</span>
          </Button>
          <Button
            onClick={onEdit}
            className="flex items-center space-x-2"
          >
            <Edit className="h-4 w-4" />
            <span>Edit</span>
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Staff Photo and Basic Info */}
        <div className="lg:col-span-1">
          <div className="form-section text-center">
            <div className="mb-6">
              {staff.imageData ? (
                <img
                  src={`data:image/jpeg;base64,${staff.imageData}`}
                  alt={staff.fullName}
                  className="w-48 h-60 object-cover rounded-lg border mx-auto shadow-lg"
                />
              ) : (
                <div className="w-48 h-60 bg-slate-100 rounded-lg border mx-auto flex items-center justify-center shadow-lg">
                  <User className="h-16 w-16 text-slate-400" />
                </div>
              )}
            </div>

            <h3 className="text-xl font-bold text-slate-800 mb-2">{staff.fullName}</h3>
            <p className="text-slate-600 text-lg mb-4">{staff.designation}</p>

            <div className="space-y-2 text-sm">
              <div className="flex items-center justify-center space-x-2">
                <Hash className="h-4 w-4 text-slate-400" />
                <span className="text-slate-600">{staff.appointmentNumber}</span>
              </div>
              <div className="flex items-center justify-center space-x-2">
                <User className="h-4 w-4 text-slate-400" />
                <span className="text-slate-600">{staff.gender}, {staff.age} years</span>
              </div>
              <div className="flex items-center justify-center space-x-2">
                <CreditCard className="h-4 w-4 text-slate-400" />
                <span className="text-slate-600">{staff.nicNumber}</span>
              </div>
            </div>
          </div>
        </div>

        {/* Detailed Information */}
        <div className="lg:col-span-2 space-y-6">
          {/* Personal Information */}
          <div className="form-section">
            <div className="form-section-title flex items-center space-x-2">
              <User className="h-5 w-5" />
              <span>Personal Information</span>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div className="space-y-4">
                <div>
                  <label className="text-sm font-medium text-slate-600">Date of Birth</label>
                  <div className="flex items-center space-x-2 mt-1">
                    <Calendar className="h-4 w-4 text-slate-400" />
                    <span className="text-slate-800">{formatDate(staff.dateOfBirth)}</span>
                  </div>
                </div>

                <div>
                  <label className="text-sm font-medium text-slate-600">Marital Status</label>
                  <p className="text-slate-800 mt-1">{staff.maritalStatus}</p>
                </div>

                <div>
                  <label className="text-sm font-medium text-slate-600">Contact Information</label>
                  <div className="space-y-2 mt-1">
                    {staff.contactNumber && (
                      <div className="flex items-center space-x-2">
                        <Phone className="h-4 w-4 text-slate-400" />
                        <span className="text-slate-800">{staff.contactNumber}</span>
                      </div>
                    )}
                    {staff.email && (
                      <div className="flex items-center space-x-2">
                        <Mail className="h-4 w-4 text-slate-400" />
                        <span className="text-slate-800">{staff.email}</span>
                      </div>
                    )}
                  </div>
                </div>
              </div>

              <div>
                <label className="text-sm font-medium text-slate-600">Address</label>
                <div className="flex items-start space-x-2 mt-1">
                  <MapPin className="h-4 w-4 text-slate-400 mt-0.5" />
                  <div className="text-slate-800">
                    {staff.addressLine1 && <div>{staff.addressLine1}</div>}
                    {staff.addressLine2 && <div>{staff.addressLine2}</div>}
                    {staff.addressLine3 && <div>{staff.addressLine3}</div>}
                    {!staff.addressLine1 && !staff.addressLine2 && !staff.addressLine3 && (
                      <span className="text-slate-500 italic">No address provided</span>
                    )}
                  </div>
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

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div className="space-y-4">
                <div>
                  <label className="text-sm font-medium text-slate-600">Date of First Appointment</label>
                  <div className="flex items-center space-x-2 mt-1">
                    <Calendar className="h-4 w-4 text-slate-400" />
                    <span className="text-slate-800">{formatDate(staff.dateOfFirstAppointment)}</span>
                  </div>
                </div>

                <div>
                  <label className="text-sm font-medium text-slate-600">Date of Retirement</label>
                  <div className="flex items-center space-x-2 mt-1">
                    <Calendar className="h-4 w-4 text-slate-400" />
                    <span className="text-slate-800">{formatDate(staff.dateOfRetirement)}</span>
                  </div>
                </div>
              </div>

              <div className="space-y-4">
                <div>
                  <label className="text-sm font-medium text-slate-600">Designation</label>
                  <p className="text-slate-800 mt-1 font-medium">{staff.designation}</p>
                </div>

                <div>
                  <label className="text-sm font-medium text-slate-600">Increment Date</label>
                  <p className="text-slate-800 mt-1">
                    {staff.incrementDate || <span className="text-slate-500 italic">Not specified</span>}
                  </p>
                </div>
              </div>
            </div>
          </div>

          {/* Salary Information */}
          <div className="form-section">
            <div className="form-section-title flex items-center space-x-2">
              <DollarSign className="h-5 w-5" />
              <span>Salary Information</span>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div>
                <label className="text-sm font-medium text-slate-600">Salary Code</label>
                <div className="mt-1">
                  <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                    {staff.salaryCode}
                  </span>
                </div>
              </div>

              <div>
                <label className="text-sm font-medium text-slate-600">Basic Salary</label>
                <p className="text-lg font-semibold text-green-600 mt-1">
                  {formatCurrency(staff.basicSalary)}
                </p>
              </div>

              <div>
                <label className="text-sm font-medium text-slate-600">Increment Amount</label>
                <p className="text-lg font-semibold text-green-600 mt-1">
                  {formatCurrency(staff.incrementAmount)}
                </p>
              </div>
            </div>
          </div>

          {/* Additional Information */}
          <div className="form-section bg-slate-50">
            <div className="form-section-title">Record Information</div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6 text-sm">
              <div>
                <label className="text-slate-600">Created At</label>
                <p className="text-slate-800 mt-1">{formatDate(staff.createdAt, 'dd-MM-yyyy HH:mm')}</p>
              </div>

              <div>
                <label className="text-slate-600">Last Updated</label>
                <p className="text-slate-800 mt-1">{formatDate(staff.updatedAt, 'dd-MM-yyyy HH:mm')}</p>
              </div>

              {staff.nicNumberOld && (
                <div className="md:col-span-2">
                  <label className="text-slate-600">Previous NIC Format</label>
                  <p className="text-slate-800 mt-1 font-mono">{staff.nicNumberOld}</p>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}