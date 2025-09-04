import { useQuery } from '@tanstack/react-query';
import { staffDatabase } from '../lib/database';
import type { StaffSearchParams } from '../types/staff';

export function useStaffList() {
  return useQuery({
    queryKey: ['staff', 'list'],
    queryFn: () => staffDatabase.getAll(),
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

export function useStaffById(id: string | undefined) {
  return useQuery({
    queryKey: ['staff', 'detail', id],
    queryFn: () => staffDatabase.getById(id!),
    enabled: !!id,
    staleTime: 5 * 60 * 1000,
  });
}

export function useStaffSearch(params: StaffSearchParams) {
  return useQuery({
    queryKey: ['staff', 'search', params],
    queryFn: () => staffDatabase.search(params),
    enabled: Object.values(params).some(value =>
      value !== undefined && value !== null && value !== ''
    ),
    staleTime: 1 * 60 * 1000, // 1 minute for search results
  });
}

export function useStaffByNIC(nic: string | undefined) {
  return useQuery({
    queryKey: ['staff', 'nic', nic],
    queryFn: () => staffDatabase.getByNIC(nic!),
    enabled: !!nic && nic.length > 5,
    staleTime: 5 * 60 * 1000,
  });
}