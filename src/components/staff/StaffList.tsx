// import React from 'react';
import { Eye, Edit, Trash2, Users, User, Phone, Mail } from 'lucide-react';
import { Button } from '../ui/button';
import { Checkbox } from '../ui/checkbox';
import { formatCurrency } from '../../lib/utils';
import type { Staff } from '../../types/staff';

interface StaffListProps {
  staff: Staff[];
  selectedStaff: Set<string>;
  onSelectStaff: (staffId: string) => void;
  onSelectAll: () => void;
  onView: (staffId: string) => void;
  onEdit: (staffId: string) => void;
  onDelete: (staff: Staff) => void;
  isLoading?: boolean;
}

export function StaffList({
  staff,
  selectedStaff,
  onSelectStaff,
  onSelectAll,
  onView,
  onEdit,
  onDelete,
  isLoading = false
}: StaffListProps) {
  if (isLoading) {
    return (
      <div className="space-y-4">
        {[...Array(3)].map((_, i) => (
          <div key={i} className="animate-pulse">
            <div className="bg-slate-200 h-20 rounded-lg"></div>
          </div>
        ))}
      </div>
    );
  }

  if (staff.length === 0) {
    return (
      <div className="text-center py-12">
        <Users className="h-12 w-12 text-slate-400 mx-auto mb-4" />
        <h3 className="text-lg font-medium text-slate-700 mb-2">No staff found</h3>
        <p className="text-slate-500">No staff records match your search criteria.</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex items-center justify-between pb-4 border-b border-slate-200">
        <div className="flex items-center space-x-4">
          <h3 className="text-lg font-semibold text-slate-800">Staff Directory</h3>
          <span className="text-sm text-slate-600">{staff.length} staff found</span>
        </div>

        <div className="flex items-center space-x-2">
          <Checkbox
            checked={selectedStaff.size === staff.length && staff.length > 0}
            onCheckedChange={onSelectAll}
          />
          <span className="text-sm text-slate-600">Select All</span>
        </div>
      </div>

      {/* Staff List */}
      <div className="space-y-3">
        {staff.map((member) => (
          <div
            key={member.id}
            className="border border-slate-200 rounded-lg p-4 hover:border-slate-300 hover:shadow-md transition-all bg-white"
          >
            <div className="flex items-center space-x-4">
              <Checkbox
                checked={selectedStaff.has(member.id)}
                onCheckedChange={() => onSelectStaff(member.id)}
              />

              {/* Photo */}
              <div className="flex-shrink-0">
                {member.imageData ? (
                  <img
                    src={`data:image/jpeg;base64,${member.imageData}`}
                    alt={member.fullName}
                    className="w-16 h-20 object-cover rounded border"
                  />
                ) : (
                  <div className="w-16 h-20 bg-slate-100 rounded border flex items-center justify-center">
                    <User className="h-8 w-8 text-slate-400" />
                  </div>
                )}
              </div>

              {/* Details */}
              <div className="flex-1 min-w-0">
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <h4 className="text-lg font-semibold text-slate-800 truncate">
                      {member.fullName}
                    </h4>
                    <p className="text-slate-600 font-medium">{member.designation}</p>

                    <div className="grid grid-cols-1 lg:grid-cols-2 gap-2 mt-2 text-sm text-slate-500">
                      <div>
                        <span className="font-medium">Appointment:</span> {member.appointmentNumber}
                      </div>
                      <div>
                        <span className="font-medium">Age:</span> {member.age} years
                      </div>
                      <div>
                        <span className="font-medium">NIC:</span> {member.nicNumber}
                      </div>
                      <div>
                        <span className="font-medium">Salary:</span> {formatCurrency(member.basicSalary)} ({member.salaryCode})
                      </div>
                    </div>

                    {/* Contact Info */}
                    <div className="flex items-center space-x-4 mt-2">
                      {member.contactNumber && (
                        <div className="flex items-center space-x-1 text-xs text-slate-500">
                          <Phone className="h-3 w-3" />
                          <span>{member.contactNumber}</span>
                        </div>
                      )}
                      {member.email && (
                        <div className="flex items-center space-x-1 text-xs text-slate-500">
                          <Mail className="h-3 w-3" />
                          <span>{member.email}</span>
                        </div>
                      )}
                    </div>
                  </div>

                  {/* Actions */}
                  <div className="flex items-center space-x-2 ml-4">
                    <Button
                      size="sm"
                      variant="outline"
                      onClick={() => onView(member.id)}
                      className="flex items-center space-x-1"
                    >
                      <Eye className="h-4 w-4" />
                      <span className="hidden sm:inline">View</span>
                    </Button>
                    <Button
                      size="sm"
                      variant="outline"
                      onClick={() => onEdit(member.id)}
                      className="flex items-center space-x-1"
                    >
                      <Edit className="h-4 w-4" />
                      <span className="hidden sm:inline">Edit</span>
                    </Button>
                    <Button
                      size="sm"
                      variant="outline"
                      onClick={() => onDelete(member)}
                      className="flex items-center space-x-1 text-red-600 hover:text-red-700 hover:border-red-300"
                    >
                      <Trash2 className="h-4 w-4" />
                      <span className="hidden sm:inline">Delete</span>
                    </Button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}