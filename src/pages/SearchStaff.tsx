import { useState, useMemo } from 'react';
import { Search, Filter, Eye, Edit, Trash2, Users, Printer, Download, Plus } from 'lucide-react';
import { Button } from '../components/ui/button';
import { Input } from '../components/ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../components/ui/select';
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from '../components/ui/dialog';
import { Checkbox } from '../components/ui/checkbox';
import { useStaffList, useStaffSearch } from '../hooks/useStaff';
import { useDeleteStaff } from '../hooks/useStaffMutations';
import { usePrintBulk, useExportToPDF } from '../hooks/usePrint';
import { formatCurrency, debounce } from '../lib/utils';
import { DESIGNATIONS, SALARY_CODES } from '../types/staff';
import type { Staff, StaffSearchParams } from '../types/staff';

interface SearchStaffProps {
  onViewStaff: (staffId: string) => void;
  onEditStaff: (staffId: string) => void;
}

export function SearchStaff({ onViewStaff, onEditStaff }: SearchStaffProps) {
  const [searchParams, setSearchParams] = useState<StaffSearchParams>({});
  const [selectedStaff, setSelectedStaff] = useState<Set<string>>(new Set());
  const [showFilters, setShowFilters] = useState(false);
  const [deleteConfirm, setDeleteConfirm] = useState<{ open: boolean; staff?: Staff }>({ open: false });

  const { data: allStaff = [], isLoading: isLoadingAll } = useStaffList();
  const { data: searchResults = [], isLoading: isSearching } = useStaffSearch(searchParams);
  const deleteStaff = useDeleteStaff();
  const printBulk = usePrintBulk();
  const exportToPDF = useExportToPDF();

  // Debounced search function
  const debouncedSearch = useMemo(
    () => debounce((params: StaffSearchParams) => {
      setSearchParams(params);
    }, 300),
    []
  );

  const staffList = Object.values(searchParams).some(val => val !== undefined && val !== '')
    ? searchResults
    : allStaff;

  const isLoading = isLoadingAll || isSearching;

  const handleSearch = (field: keyof StaffSearchParams, value: string) => {
    const newParams = { ...searchParams, [field]: value || undefined };
    debouncedSearch(newParams);
  };

  const handleClearFilters = () => {
    setSearchParams({});
    setSelectedStaff(new Set());
  };

  const handleSelectAll = () => {
    if (selectedStaff.size === staffList.length) {
      setSelectedStaff(new Set());
    } else {
      setSelectedStaff(new Set(staffList.map(staff => staff.id)));
    }
  };

  const handleSelectStaff = (staffId: string) => {
    const newSelected = new Set(selectedStaff);
    if (newSelected.has(staffId)) {
      newSelected.delete(staffId);
    } else {
      newSelected.add(staffId);
    }
    setSelectedStaff(newSelected);
  };

  const handleDelete = async () => {
    if (deleteConfirm.staff) {
      try {
        await deleteStaff.mutateAsync(deleteConfirm.staff.id);
        setDeleteConfirm({ open: false });
        // Remove from selected if it was selected
        const newSelected = new Set(selectedStaff);
        newSelected.delete(deleteConfirm.staff.id);
        setSelectedStaff(newSelected);
      } catch (error) {
        console.error('Error deleting staff:', error);
      }
    }
  };

  const handleBulkPrint = () => {
    if (selectedStaff.size > 0) {
      printBulk.mutate(Array.from(selectedStaff));
    }
  };

  const handleBulkExport = () => {
    if (selectedStaff.size > 0) {
      exportToPDF.mutate({
        staffIds: Array.from(selectedStaff),
        isBulk: true
      });
    }
  };

  return (
    <div className="space-y-6">
      {/* Search and Filter Controls */}
      <div className="bg-white rounded-lg shadow-md border p-6 space-y-4">
        <div className="flex flex-col lg:flex-row gap-4">
          {/* Main search */}
          <div className="flex-1">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-slate-400" />
              <Input
                placeholder="Search by name, appointment number, or NIC..."
                className="pl-10"
                onChange={(e) => handleSearch('searchTerm', e.target.value)}
              />
            </div>
          </div>

          {/* Filter toggle */}
          <Button
            variant="outline"
            onClick={() => setShowFilters(!showFilters)}
            className="flex items-center space-x-2"
          >
            <Filter className="h-4 w-4" />
            <span>Filters</span>
          </Button>

          {/* Clear filters */}
          <Button
            variant="outline"
            onClick={handleClearFilters}
            disabled={Object.values(searchParams).every(val => !val)}
          >
            Clear
          </Button>
        </div>

        {/* Advanced Filters */}
        {showFilters && (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 pt-4 border-t">
            <div>
              <label className="block text-sm font-medium text-slate-700 mb-2">
                Designation
              </label>
              <Select onValueChange={(value) => handleSearch('designation', value)}>
                <SelectTrigger>
                  <SelectValue placeholder="Select designation" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="">All Designations</SelectItem>
                  {DESIGNATIONS.map((designation) => (
                    <SelectItem key={designation} value={designation}>
                      {designation}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div>
              <label className="block text-sm font-medium text-slate-700 mb-2">
                Age Range
              </label>
              <div className="flex space-x-2">
                <Input
                  type="number"
                  placeholder="Min"
                  onChange={(e) => handleSearch('ageMin', e.target.value)}
                />
                <Input
                  type="number"
                  placeholder="Max"
                  onChange={(e) => handleSearch('ageMax', e.target.value)}
                />
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-slate-700 mb-2">
                Salary Code
              </label>
              <Select onValueChange={(value) => handleSearch('salaryCode', value)}>
                <SelectTrigger>
                  <SelectValue placeholder="Select salary code" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="">All Codes</SelectItem>
                  {SALARY_CODES.map((code) => (
                    <SelectItem key={code} value={code}>
                      {code}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>
        )}
      </div>

      {/* Bulk Actions */}
      {selectedStaff.size > 0 && (
        <div className="bg-blue-50 rounded-lg border border-blue-200 p-4">
          <div className="flex items-center justify-between">
            <span className="text-sm text-blue-700">
              {selectedStaff.size} staff selected
            </span>
            <div className="flex space-x-2">
              <Button
                size="sm"
                variant="outline"
                onClick={handleBulkPrint}
                disabled={printBulk.isPending}
                className="flex items-center space-x-2"
              >
                <Printer className="h-4 w-4" />
                <span>Print Selected</span>
              </Button>
              <Button
                size="sm"
                variant="outline"
                onClick={handleBulkExport}
                disabled={exportToPDF.isPending}
                className="flex items-center space-x-2"
              >
                <Download className="h-4 w-4" />
                <span>Export Selected</span>
              </Button>
            </div>
          </div>
        </div>
      )}

      {/* Results */}
      <div className="bg-white rounded-lg shadow-md border">
        <div className="p-6 border-b border-slate-200">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-4">
              <h2 className="text-lg font-semibold text-slate-800">
                Staff Directory
              </h2>
              <span className="text-sm text-slate-600">
                {isLoading ? 'Loading...' : `${staffList.length} staff found`}
              </span>
            </div>

            {staffList.length > 0 && (
              <div className="flex items-center space-x-2">
                <Checkbox
                  checked={selectedStaff.size === staffList.length}
                  onCheckedChange={handleSelectAll}
                />
                <span className="text-sm text-slate-600">Select All</span>
              </div>
            )}
          </div>
        </div>

        <div className="p-6">
          {isLoading ? (
            <div className="space-y-4">
              {[...Array(3)].map((_, i) => (
                <div key={i} className="animate-pulse">
                  <div className="bg-slate-200 h-20 rounded-lg"></div>
                </div>
              ))}
            </div>
          ) : staffList.length === 0 ? (
            <div className="text-center py-12">
              <Users className="h-12 w-12 text-slate-400 mx-auto mb-4" />
              <h3 className="text-lg font-medium text-slate-700 mb-2">No staff found</h3>
              <p className="text-slate-500 mb-4">
                {Object.values(searchParams).some(val => val)
                  ? 'Try adjusting your search filters'
                  : 'No staff records in the database'
                }
              </p>
              {!Object.values(searchParams).some(val => val) && (
                <Button onClick={() => onEditStaff('')} className="flex items-center space-x-2">
                  <Plus className="h-4 w-4" />
                  <span>Add First Staff</span>
                </Button>
              )}
            </div>
          ) : (
            <div className="space-y-4">
              {staffList.map((staff) => (
                <div
                  key={staff.id}
                  className="border border-slate-200 rounded-lg p-4 hover:border-slate-300 hover:shadow-md transition-all"
                >
                  <div className="flex items-center space-x-4">
                    <Checkbox
                      checked={selectedStaff.has(staff.id)}
                      onCheckedChange={() => handleSelectStaff(staff.id)}
                    />

                    {staff.imageData ? (
                      <img
                        src={`data:image/jpeg;base64,${staff.imageData}`}
                        alt={staff.fullName}
                        className="w-16 h-20 object-cover rounded border"
                      />
                    ) : (
                      <div className="w-16 h-20 bg-slate-100 rounded border flex items-center justify-center">
                        <Users className="h-8 w-8 text-slate-400" />
                      </div>
                    )}

                    <div className="flex-1 min-w-0">
                      <div className="flex items-start justify-between">
                        <div>
                          <h3 className="text-lg font-semibold text-slate-800 truncate">
                            {staff.fullName}
                          </h3>
                          <p className="text-slate-600">{staff.designation}</p>
                          <p className="text-sm text-slate-500">
                            Appointment No: {staff.appointmentNumber} | Age: {staff.age} | NIC: {staff.nicNumber}
                          </p>
                          <p className="text-sm text-slate-500">
                            Salary: {formatCurrency(staff.basicSalary)} ({staff.salaryCode})
                          </p>
                        </div>

                        <div className="flex items-center space-x-2">
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={() => onViewStaff(staff.id)}
                            className="flex items-center space-x-1"
                          >
                            <Eye className="h-4 w-4" />
                            <span>View</span>
                          </Button>
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={() => onEditStaff(staff.id)}
                            className="flex items-center space-x-1"
                          >
                            <Edit className="h-4 w-4" />
                            <span>Edit</span>
                          </Button>
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={() => setDeleteConfirm({ open: true, staff })}
                            className="flex items-center space-x-1 text-red-600 hover:text-red-700"
                          >
                            <Trash2 className="h-4 w-4" />
                            <span>Delete</span>
                          </Button>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Delete Confirmation Dialog */}
      <Dialog open={deleteConfirm.open} onOpenChange={(open) => setDeleteConfirm({ open })}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Confirm Delete</DialogTitle>
            <DialogDescription>
              Are you sure you want to delete the staff record for{' '}
              <strong>{deleteConfirm.staff?.fullName}</strong>? This action cannot be undone.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => setDeleteConfirm({ open: false })}
              disabled={deleteStaff.isPending}
            >
              Cancel
            </Button>
            <Button
              variant="destructive"
              onClick={handleDelete}
              disabled={deleteStaff.isPending}
              className="flex items-center space-x-2"
            >
              {deleteStaff.isPending && (
                <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
              )}
              <span>Delete</span>
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}