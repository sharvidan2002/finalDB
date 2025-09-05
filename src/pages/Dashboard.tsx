import { useState } from 'react';
import { Search, UserPlus } from 'lucide-react';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../components/ui/tabs';
import { AddStaff } from './AddStaff';
import { SearchStaff } from './SearchStaff';

export function Dashboard() {
  const [activeTab, setActiveTab] = useState('search');

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