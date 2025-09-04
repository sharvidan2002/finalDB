// import React from 'react';
import { Search, Filter } from 'lucide-react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../ui/select';
import { DESIGNATIONS, SALARY_CODES } from '../../types/staff';
import type { StaffSearchParams } from '../../types/staff';

interface StaffSearchProps {
  searchParams: StaffSearchParams;
  onSearch: (field: keyof StaffSearchParams, value: string) => void;
  onClearFilters: () => void;
  showFilters: boolean;
  onToggleFilters: () => void;
}

export function StaffSearch({
  searchParams,
  onSearch,
  onClearFilters,
  showFilters,
  onToggleFilters
}: StaffSearchProps) {
  const hasFilters = Object.values(searchParams).some(val => val !== undefined && val !== '');

  return (
    <div className="bg-white rounded-lg shadow-md border p-6 space-y-4">
      <div className="flex flex-col lg:flex-row gap-4">
        {/* Main search */}
        <div className="flex-1">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-slate-400" />
            <Input
              placeholder="Search by name, appointment number, or NIC..."
              className="pl-10"
              value={searchParams.searchTerm || ''}
              onChange={(e) => onSearch('searchTerm', e.target.value)}
            />
          </div>
        </div>

        {/* Filter controls */}
        <div className="flex space-x-2">
          <Button
            variant="outline"
            onClick={onToggleFilters}
            className="flex items-center space-x-2"
          >
            <Filter className="h-4 w-4" />
            <span>Filters</span>
          </Button>

          <Button
            variant="outline"
            onClick={onClearFilters}
            disabled={!hasFilters}
          >
            Clear
          </Button>
        </div>
      </div>

      {/* Advanced Filters */}
      {showFilters && (
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 pt-4 border-t">
          <div>
            <label className="block text-sm font-medium text-slate-700 mb-2">
              Designation
            </label>
            <Select
              value={searchParams.designation || ''}
              onValueChange={(value) => onSearch('designation', value)}
            >
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
                value={searchParams.ageMin || ''}
                onChange={(e) => onSearch('ageMin', e.target.value)}
              />
              <Input
                type="number"
                placeholder="Max"
                value={searchParams.ageMax || ''}
                onChange={(e) => onSearch('ageMax', e.target.value)}
              />
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium text-slate-700 mb-2">
              Salary Code
            </label>
            <Select
              value={searchParams.salaryCode || ''}
              onValueChange={(value) => onSearch('salaryCode', value)}
            >
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
  );
}