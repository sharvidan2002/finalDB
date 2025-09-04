import { useState } from 'react';
import { Users, UserPlus, Search } from 'lucide-react';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../components/ui/tabs';
import { AddStaff } from './AddStaff';
import { SearchStaff } from './SearchStaff';
import { useStaffList } from '../hooks/useStaff';

export function Dashboard() {
  const [activeTab, setActiveTab] = useState('search');
  const { data: staffList = [], isLoading } = useStaffList();

  const handleStaffCreated = () => {
    setActiveTab('search');
  };

  const handleBackToSearch = () => {
    setActiveTab('search');
  };

  return (
    <div className="space-y-8 fade-in">
      {/* Dashboard Header */}
      <div className="text-center space-y-4">
        <h1 className="heading-1">Staff Management System</h1>
        <p className="text-muted text-lg">
          Manage staff records for Divisional Forest Office - Vavuniya
        </p>

        {/* Stats Cards */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mt-8">
          <div className="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
            <div className="flex items-center space-x-4">
              <div className="p-3 bg-blue-100 rounded-lg">
                <Users className="h-8 w-8 text-blue-600" />
              </div>
              <div>
                <p className="text-2xl font-bold text-slate-800">
                  {isLoading ? '...' : staffList.length}
                </p>
                <p className="text-slate-600">Total Staff</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
            <div className="flex items-center space-x-4">
              <div className="p-3 bg-green-100 rounded-lg">
                <UserPlus className="h-8 w-8 text-green-600" />
              </div>
              <div>
                <p className="text-2xl font-bold text-slate-800">
                  {isLoading ? '...' : staffList.filter(s => s.age < 55).length}
                </p>
                <p className="text-slate-600">Active Staff</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
            <div className="flex items-center space-x-4">
              <div className="p-3 bg-purple-100 rounded-lg">
                <Search className="h-8 w-8 text-purple-600" />
              </div>
              <div>
                <p className="text-2xl font-bold text-slate-800">
                  {isLoading ? '...' : new Set(staffList.map(s => s.designation)).size}
                </p>
                <p className="text-slate-600">Designations</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Main Tabs */}
      <div className="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
        <Tabs value={activeTab} onValueChange={setActiveTab} className="w-full">
          <TabsList className="grid w-full grid-cols-2 mb-8">
            <TabsTrigger value="search" className="flex items-center space-x-2">
              <Search className="h-4 w-4" />
              <span>Search Staff</span>
            </TabsTrigger>
            <TabsTrigger value="add" className="flex items-center space-x-2">
              <UserPlus className="h-4 w-4" />
              <span>Add Staff</span>
            </TabsTrigger>
          </TabsList>

          <TabsContent value="search" className="space-y-6">
            <SearchStaff />
          </TabsContent>

          <TabsContent value="add" className="space-y-6">
            <AddStaff
              onStaffCreated={handleStaffCreated}
              onCancel={handleBackToSearch}
            />
          </TabsContent>
        </Tabs>
      </div>
    </div>
  );
}