import { useMutation, useQueryClient } from '@tanstack/react-query';
import { staffDatabase } from '../lib/database';
import type { CreateStaffRequest, UpdateStaffRequest } from '../types/staff';

export function useCreateStaff() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (staffData: CreateStaffRequest) =>
      staffDatabase.create(staffData),
    onSuccess: () => {
      // Invalidate and refetch staff lists
      queryClient.invalidateQueries({ queryKey: ['staff'] });
    },
  });
}

export function useUpdateStaff() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (staffData: UpdateStaffRequest) =>
      staffDatabase.update(staffData),
    onSuccess: (updatedStaff) => {
      // Update the specific staff in cache
      queryClient.setQueryData(
        ['staff', 'detail', updatedStaff.id],
        updatedStaff
      );
      // Invalidate staff lists to reflect changes
      queryClient.invalidateQueries({ queryKey: ['staff', 'list'] });
      queryClient.invalidateQueries({ queryKey: ['staff', 'search'] });
    },
  });
}

export function useDeleteStaff() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => staffDatabase.delete(id),
    onSuccess: (_, deletedId) => {
      // Remove from cache
      queryClient.removeQueries({ queryKey: ['staff', 'detail', deletedId] });
      // Invalidate lists
      queryClient.invalidateQueries({ queryKey: ['staff', 'list'] });
      queryClient.invalidateQueries({ queryKey: ['staff', 'search'] });
    },
  });
}