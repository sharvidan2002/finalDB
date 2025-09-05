import { useMutation } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/tauri';

// Show notification function
function showNotification(message: string, type: 'loading' | 'success' | 'error' = 'loading') {
  const notification = document.createElement('div');

  const bgColor = type === 'success' ? '#10b981' : type === 'error' ? '#ef4444' : '#3b82f6';
  const icon = type === 'success' ? '✓' : type === 'error' ? '✗' : '';

  notification.innerHTML = `
    <div style="
      position: fixed;
      top: 20px;
      right: 20px;
      background: ${bgColor};
      color: white;
      padding: 12px 24px;
      border-radius: 8px;
      z-index: 10000;
      box-shadow: 0 4px 12px rgba(0,0,0,0.15);
      font-family: system-ui;
      max-width: 400px;
    ">
      <div style="display: flex; align-items: center; gap: 8px;">
        ${type === 'loading' ? `
          <div style="
            width: 16px;
            height: 16px;
            border: 2px solid white;
            border-top-color: transparent;
            border-radius: 50%;
            animation: spin 1s linear infinite;
          "></div>
        ` : icon}
        ${message}
      </div>
    </div>
    ${type === 'loading' ? `
      <style>
        @keyframes spin {
          to { transform: rotate(360deg); }
        }
      </style>
    ` : ''}
  `;

  document.body.appendChild(notification);

  // Auto-remove after delay (except for loading notifications)
  if (type !== 'loading') {
    const delay = type === 'error' ? 5000 : 3000;
    setTimeout(() => {
      if (document.body.contains(notification)) {
        document.body.removeChild(notification);
      }
    }, delay);
  }

  return notification;
}

// Remove notification function
function removeNotification(notification: HTMLElement) {
  if (document.body.contains(notification)) {
    document.body.removeChild(notification);
  }
}

// Generate preview HTML for individual staff
export function useGenerateStaffPreview() {
  return useMutation({
    mutationFn: async (staffId: string) => {
      try {
        const result = await invoke<string>('generate_staff_preview', { staffId });
        return result;
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        showNotification(`Failed to generate preview: ${errorMessage}`, 'error');
        throw error;
      }
    },
    onError: (error) => {
      console.error('Generate staff preview failed:', error);
    }
  });
}

// Generate preview HTML for bulk staff
export function useGenerateBulkStaffPreview() {
  return useMutation({
    mutationFn: async (staffIds: string[]) => {
      try {
        const result = await invoke<string>('generate_bulk_staff_preview', { staffIds });
        return result;
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        showNotification(`Failed to generate preview: ${errorMessage}`, 'error');
        throw error;
      }
    },
    onError: (error) => {
      console.error('Generate bulk staff preview failed:', error);
    }
  });
}

// Print individual staff record
export function usePrintIndividual() {
  return useMutation({
    mutationFn: async (staffId: string) => {
      const loadingNotification = showNotification('Generating PDF for staff record...', 'loading');

      try {
        const result = await invoke<string>('generate_staff_pdf', { staffId });
        removeNotification(loadingNotification);

        // Show success message with option to open Downloads folder
        showNotification('PDF file opened in browser for printing! Click "Open Downloads" to view files.', 'success');

        return result;
      } catch (error) {
        removeNotification(loadingNotification);
        const errorMessage = error instanceof Error ? error.message : String(error);
        showNotification(`Failed to generate PDF: ${errorMessage}`, 'error');
        throw error;
      }
    },
    onError: (error) => {
      console.error('Print individual failed:', error);
    }
  });
}

// Print bulk staff records
export function usePrintBulk() {
  return useMutation({
    mutationFn: async (staffIds: string[]) => {
      const loadingNotification = showNotification(
        `Generating PDF for ${staffIds.length} staff records...`,
        'loading'
      );

      try {
        const result = await invoke<string>('generate_bulk_staff_pdf', { staffIds });
        removeNotification(loadingNotification);

        // Show success message with option to open Downloads folder
        showNotification('Staff directory PDF opened in browser for printing! Click "Open Downloads" to view files.', 'success');

        return result;
      } catch (error) {
        removeNotification(loadingNotification);
        const errorMessage = error instanceof Error ? error.message : String(error);
        showNotification(`Failed to generate PDF: ${errorMessage}`, 'error');
        throw error;
      }
    },
    onError: (error) => {
      console.error('Print bulk failed:', error);
    }
  });
}

// Export to PDF with professional template
export function useExportToPDF() {
  return useMutation({
    mutationFn: async ({ staffIds, isBulk }: { staffIds: string[]; isBulk: boolean }) => {
      const isMultiple = isBulk || staffIds.length > 1;
      const loadingNotification = showNotification(
        `Generating professional PDF for ${isMultiple ? `${staffIds.length} staff records` : 'staff record'}...`,
        'loading'
      );

      try {
        const result = await invoke<string>('export_staff_pdf', {
          staffIds,
          isBulk: isMultiple
        });
        removeNotification(loadingNotification);

        // Show success message
        const message = isMultiple
          ? 'Staff directory PDF opened in browser for printing! Click "Open Downloads" to view files.'
          : 'Staff details PDF opened in browser for printing! Click "Open Downloads" to view files.';

        showNotification(message, 'success');

        return result;
      } catch (error) {
        removeNotification(loadingNotification);
        const errorMessage = error instanceof Error ? error.message : String(error);
        showNotification(`Failed to generate PDF: ${errorMessage}`, 'error');
        throw error;
      }
    },
    onError: (error) => {
      console.error('Export to PDF failed:', error);
    }
  });
}

// Open Downloads folder
export function useOpenDownloadsFolder() {
  return useMutation({
    mutationFn: async () => {
      try {
        const result = await invoke<string>('open_downloads_folder');
        showNotification('Downloads folder opened!', 'success');
        return result;
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        showNotification(`Could not open Downloads folder: ${errorMessage}`, 'error');
        throw error;
      }
    }
  });
}

// Clean up temporary image files
export function useCleanupTempFiles() {
  return useMutation({
    mutationFn: async () => {
      try {
        const result = await invoke<string>('cleanup_temp_files');
        showNotification('Temporary files cleaned up successfully!', 'success');
        return result;
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        showNotification(`Failed to cleanup files: ${errorMessage}`, 'error');
        throw error;
      }
    },
    onError: (error) => {
      console.error('Cleanup temp files failed:', error);
    }
  });
}

// Legacy function names for backward compatibility
export function usePrintDirect() {
  return useExportToPDF();
}

// New hook for showing print preview dialog
export function usePrintWithPreview() {
  return {
    generatePreview: useGenerateStaffPreview(),
    generateBulkPreview: useGenerateBulkStaffPreview(),
    exportToPDF: useExportToPDF(),
    openDownloads: useOpenDownloadsFolder(),
    cleanup: useCleanupTempFiles(),
  };
}