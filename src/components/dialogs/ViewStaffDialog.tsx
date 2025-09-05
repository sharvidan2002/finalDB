import { useState } from 'react';
import { User, Briefcase, DollarSign, Calendar, Phone, Mail, MapPin, Hash, CreditCard, X, Eye } from 'lucide-react';
import { Button } from '../ui/button';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '../ui/dialog';
import { ProfessionalPrintPreview } from '../staff/ProfessionalPrintPreview';
import { useStaffById } from '../../hooks/useStaff';
import { formatDate, formatCurrency } from '../../lib/utils';

interface ViewStaffDialogProps {
  isOpen: boolean;
  onClose: () => void;
  staffId: string | null;
}

export function ViewStaffDialog({ isOpen, onClose, staffId }: ViewStaffDialogProps) {
  const { data: staff, isLoading } = useStaffById(staffId || undefined);
  const [showPrintPreview, setShowPrintPreview] = useState(false);

  const handleShowPreview = () => {
    if (staff) {
      setShowPrintPreview(true);
    }
  };

  const handleClosePrintPreview = () => {
    setShowPrintPreview(false);
  };

  if (!staffId) return null;

  return (
    <>
      <Dialog open={isOpen} onOpenChange={onClose}>
        <DialogContent className="max-w-4xl max-h-[90vh] overflow-auto">
          <DialogHeader className="flex flex-row items-center justify-between space-y-0 pb-4">
            <DialogTitle className="text-xl font-semibold">
              Staff Details
            </DialogTitle>
            <div className="flex items-center space-x-2">
              <Button
                variant="outline"
                size="sm"
                onClick={handleShowPreview}
                disabled={!staff}
                className="flex items-center space-x-2"
              >
                <Eye className="h-4 w-4" />
                <span>Print Preview</span>
              </Button>
              <Button
                variant="ghost"
                size="sm"
                onClick={onClose}
                className="p-2"
              >
                <X className="h-4 w-4" />
              </Button>
            </div>
          </DialogHeader>

          {isLoading ? (
            <div className="flex items-center justify-center py-12">
              <div className="loading-spinner w-8 h-8"></div>
            </div>
          ) : !staff ? (
            <div className="text-center py-8">
              <p className="text-slate-500">Staff not found</p>
            </div>
          ) : (
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 py-4">
              {/* Staff Photo and Basic Info */}
              <div className="lg:col-span-1">
                <div className="bg-slate-50 rounded-lg p-4 text-center">
                  <div className="mb-6">
                    {staff.imageData ? (
                      <img
                        src={`data:image/jpeg;base64,${staff.imageData}`}
                        alt={staff.fullName}
                        className="w-40 h-50 object-cover rounded-lg border mx-auto shadow-md"
                      />
                    ) : (
                      <div className="w-40 h-50 bg-slate-200 rounded-lg border mx-auto flex items-center justify-center shadow-md">
                        <User className="h-12 w-12 text-slate-400" />
                      </div>
                    )}
                  </div>

                  <h3 className="text-lg font-bold text-slate-800 mb-1">{staff.fullName}</h3>
                  <p className="text-slate-600 text-base mb-3">{staff.designation}</p>

                  <div className="space-y-1 text-sm">
                    <div className="flex items-center justify-center space-x-2">
                      <Hash className="h-3 w-3 text-slate-400" />
                      <span className="text-slate-600">{staff.appointmentNumber}</span>
                    </div>
                    <div className="flex items-center justify-center space-x-2">
                      <User className="h-3 w-3 text-slate-400" />
                      <span className="text-slate-600">{staff.gender}, {staff.age} years</span>
                    </div>
                    <div className="flex items-center justify-center space-x-2">
                      <CreditCard className="h-3 w-3 text-slate-400" />
                      <span className="text-slate-600">{staff.nicNumber}</span>
                    </div>
                  </div>
                </div>
              </div>

              {/* Detailed Information */}
              <div className="lg:col-span-2 space-y-4">
                {/* Personal Information */}
                <div className="bg-white border rounded-lg p-4">
                  <div className="flex items-center space-x-2 mb-3 pb-2 border-b">
                    <User className="h-4 w-4 text-blue-600" />
                    <span className="font-semibold text-slate-800">Personal Information</span>
                  </div>

                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
                    <div>
                      <label className="text-slate-600 font-medium">Date of Birth</label>
                      <div className="flex items-center space-x-2 mt-1">
                        <Calendar className="h-3 w-3 text-slate-400" />
                        <span className="text-slate-800">{formatDate(staff.dateOfBirth)}</span>
                      </div>
                    </div>

                    <div>
                      <label className="text-slate-600 font-medium">Marital Status</label>
                      <p className="text-slate-800 mt-1">{staff.maritalStatus}</p>
                    </div>

                    <div>
                      <label className="text-slate-600 font-medium">Contact</label>
                      <div className="space-y-1 mt-1">
                        {staff.contactNumber && (
                          <div className="flex items-center space-x-2">
                            <Phone className="h-3 w-3 text-slate-400" />
                            <span className="text-slate-800">{staff.contactNumber}</span>
                          </div>
                        )}
                        {staff.email && (
                          <div className="flex items-center space-x-2">
                            <Mail className="h-3 w-3 text-slate-400" />
                            <span className="text-slate-800">{staff.email}</span>
                          </div>
                        )}
                      </div>
                    </div>

                    <div>
                      <label className="text-slate-600 font-medium">Address</label>
                      <div className="flex items-start space-x-2 mt-1">
                        <MapPin className="h-3 w-3 text-slate-400 mt-0.5" />
                        <div className="text-slate-800 text-xs">
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
                <div className="bg-white border rounded-lg p-4">
                  <div className="flex items-center space-x-2 mb-3 pb-2 border-b">
                    <Briefcase className="h-4 w-4 text-green-600" />
                    <span className="font-semibold text-slate-800">Employment Details</span>
                  </div>

                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
                    <div>
                      <label className="text-slate-600 font-medium">Date of First Appointment</label>
                      <div className="flex items-center space-x-2 mt-1">
                        <Calendar className="h-3 w-3 text-slate-400" />
                        <span className="text-slate-800">{formatDate(staff.dateOfFirstAppointment)}</span>
                      </div>
                    </div>

                    <div>
                      <label className="text-slate-600 font-medium">Date of Retirement</label>
                      <div className="flex items-center space-x-2 mt-1">
                        <Calendar className="h-3 w-3 text-slate-400" />
                        <span className="text-slate-800">{formatDate(staff.dateOfRetirement)}</span>
                      </div>
                    </div>

                    <div>
                      <label className="text-slate-600 font-medium">Designation</label>
                      <p className="text-slate-800 mt-1 font-medium">{staff.designation}</p>
                    </div>

                    <div>
                      <label className="text-slate-600 font-medium">Increment Date</label>
                      <p className="text-slate-800 mt-1">
                        {staff.incrementDate || <span className="text-slate-500 italic">Not specified</span>}
                      </p>
                    </div>
                  </div>
                </div>

                {/* Salary Information */}
                <div className="bg-white border rounded-lg p-4">
                  <div className="flex items-center space-x-2 mb-3 pb-2 border-b">
                    <DollarSign className="h-4 w-4 text-green-600" />
                    <span className="font-semibold text-slate-800">Salary Information</span>
                  </div>

                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
                    <div>
                      <label className="text-slate-600 font-medium">Salary Code</label>
                      <div className="mt-1">
                        <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                          {staff.salaryCode}
                        </span>
                      </div>
                    </div>

                    <div>
                      <label className="text-slate-600 font-medium">Basic Salary</label>
                      <p className="text-base font-semibold text-green-600 mt-1">
                        {formatCurrency(staff.basicSalary)}
                      </p>
                    </div>

                    <div>
                      <label className="text-slate-600 font-medium">Increment Amount</label>
                      <p className="text-base font-semibold text-green-600 mt-1">
                        {formatCurrency(staff.incrementAmount)}
                      </p>
                    </div>
                  </div>

                  <div className="mt-4 pt-3 border-t border-slate-200">
                    <div className="flex items-center justify-between">
                      <span className="text-sm font-medium text-slate-600">Total Salary (Basic + Increment)</span>
                      <span className="text-lg font-bold text-green-700">
                        {formatCurrency(staff.basicSalary + staff.incrementAmount)}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          )}
        </DialogContent>
      </Dialog>

      {/* Print Preview Dialog */}
      {staff && (
        <ProfessionalPrintPreview
          isOpen={showPrintPreview}
          onClose={handleClosePrintPreview}
          staffIds={[staff.id]}
          isBulk={false}
          title={`${staff.fullName} - Official Record`}
        />
      )}
    </>
  );
}