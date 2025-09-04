import { invoke } from '@tauri-apps/api/tauri';
import type {
  Staff,
  CreateStaffRequest,
  UpdateStaffRequest,
  StaffSearchParams
} from '../types/staff';

// Convert between frontend (camelCase) and backend (snake_case) formats
function toBackendFormat<T extends Record<string, any>>(obj: T): any {
  const converted: any = {};

  for (const [key, value] of Object.entries(obj)) {
    const snakeKey = key.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
    converted[snakeKey] = value;
  }

  return converted;
}

function fromBackendFormat<T extends Record<string, any>>(obj: T): any {
  const converted: any = {};

  for (const [key, value] of Object.entries(obj)) {
    const camelKey = key.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
    converted[camelKey] = value;
  }

  return converted;
}

// Staff database operations
export const staffDatabase = {
  async create(staffData: CreateStaffRequest): Promise<Staff> {
    const backendData = toBackendFormat(staffData);
    const result = await invoke<any>('create_staff', { staffData: backendData });
    return fromBackendFormat(result);
  },

  async getAll(): Promise<Staff[]> {
    const result = await invoke<any[]>('get_all_staff');
    return result.map(fromBackendFormat);
  },

  async getById(id: string): Promise<Staff> {
    const result = await invoke<any>('get_staff_by_id', { id });
    return fromBackendFormat(result);
  },

  async update(staffData: UpdateStaffRequest): Promise<Staff> {
    const backendData = toBackendFormat(staffData);
    const result = await invoke<any>('update_staff', { staffData: backendData });
    return fromBackendFormat(result);
  },

  async delete(id: string): Promise<void> {
    await invoke('delete_staff', { id });
  },

  async search(params: StaffSearchParams): Promise<Staff[]> {
    const backendParams = toBackendFormat(params);
    const result = await invoke<any[]>('search_staff', { params: backendParams });
    return result.map(fromBackendFormat);
  },

  async getByNIC(nic: string): Promise<Staff | null> {
    const result = await invoke<any | null>('get_staff_by_nic', { nic });
    return result ? fromBackendFormat(result) : null;
  },
};

// Print operations
export const printService = {
  async printIndividual(staffId: string): Promise<string> {
    return await invoke<string>('print_staff_individual', { staffId });
  },

  async printBulk(staffIds: string[]): Promise<string> {
    return await invoke<string>('print_staff_bulk', { staffIds });
  },

  async exportToPDF(staffIds: string[], isBulk: boolean = false): Promise<string> {
    return await invoke<string>('export_staff_pdf', { staffIds, isBulk });
  },
};