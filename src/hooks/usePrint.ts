import { useMutation } from '@tanstack/react-query';
import { printService } from '../lib/database';
import { save } from '@tauri-apps/api/dialog';
import { writeBinaryFile, writeFile } from '@tauri-apps/api/fs';
import { downloadDir } from '@tauri-apps/api/path';

// Import html2pdf
declare global {
  interface Window {
    html2pdf: any;
  }
}

// Load html2pdf dynamically
const loadHtml2Pdf = async () => {
  if (!window.html2pdf) {
    const script = document.createElement('script');
    script.src = 'https://cdnjs.cloudflare.com/ajax/libs/html2pdf.js/0.10.1/html2pdf.bundle.min.js';
    script.async = true;
    document.head.appendChild(script);

    return new Promise((resolve, reject) => {
      script.onload = resolve;
      script.onerror = reject;
    });
  }
};

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
      max-width: 300px;
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

// Enhanced PDF export function with Tauri file system support
async function exportToPDF(htmlContent: string, filename: string, staffCount: number = 1) {
  const loadingNotification = showNotification(
    `Generating PDF for ${staffCount > 1 ? `${staffCount} staff records` : 'staff record'}...`,
    'loading'
  );

  try {
    // Ensure html2pdf is loaded
    await loadHtml2Pdf();

    // Wait a moment for the script to be ready
    await new Promise(resolve => setTimeout(resolve, 500));

    // Create a temporary container
    const container = document.createElement('div');
    container.innerHTML = htmlContent;
    container.style.position = 'absolute';
    container.style.left = '-9999px';
    container.style.top = '-9999px';
    container.style.width = '210mm'; // A4 width
    document.body.appendChild(container);

    // PDF generation options
    const options = {
      margin: [10, 10, 10, 10], // top, right, bottom, left in mm
      filename: filename.replace('.html', '.pdf'),
      image: {
        type: 'jpeg',
        quality: 0.95
      },
      html2canvas: {
        scale: 2,
        useCORS: true,
        letterRendering: true,
        allowTaint: true,
        backgroundColor: '#ffffff'
      },
      jsPDF: {
        unit: 'mm',
        format: 'a4',
        orientation: staffCount > 1 ? 'landscape' : 'portrait'
      },
      pagebreak: {
        mode: ['avoid-all', 'css', 'legacy']
      }
    };

    // Generate PDF blob
    const pdfBlob = await window.html2pdf()
      .set(options)
      .from(container)
      .outputPdf('blob');

    // Clean up container
    document.body.removeChild(container);

    // Remove loading notification
    if (document.body.contains(loadingNotification)) {
      document.body.removeChild(loadingNotification);
    }

    // Use Tauri's save dialog to let user choose location
    try {
      const filePath = await save({
        filters: [{
          name: 'PDF',
          extensions: ['pdf']
        }],
        defaultPath: options.filename
      });

      if (filePath) {
        // Convert blob to array buffer
        const arrayBuffer = await pdfBlob.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        // Write the PDF file
        await writeBinaryFile(filePath, uint8Array);

        showNotification('PDF saved successfully!', 'success');
        return true;
      } else {
        // User cancelled the save dialog
        showNotification('PDF export cancelled', 'error');
        return false;
      }
    } catch (saveError) {
      console.error('Save dialog error:', saveError);

      // Fallback: Save to downloads directory
      try {
        const downloadsPath = await downloadDir();
        const fallbackPath = `${downloadsPath}/${options.filename}`;

        const arrayBuffer = await pdfBlob.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        await writeBinaryFile(fallbackPath, uint8Array);
        showNotification(`PDF saved to Downloads folder as ${options.filename}`, 'success');
        return true;
      } catch (fallbackError) {
        console.error('Fallback save failed:', fallbackError);
        throw fallbackError;
      }
    }

  } catch (error) {
    console.error('PDF generation failed:', error);

    // Remove loading notification
    if (document.body.contains(loadingNotification)) {
      document.body.removeChild(loadingNotification);
    }

    const errorMessage = error instanceof Error ? error.message : String(error);
    showNotification(`PDF generation failed: ${errorMessage}`, 'error');

    // Fallback: Save HTML file instead
    try {
      const downloadsPath = await downloadDir();
      const htmlFilePath = `${downloadsPath}/${filename}`;
      const htmlBytes = new TextEncoder().encode(htmlContent);
      await writeFile(htmlFilePath, htmlBytes);

      showNotification('Saved as HTML file instead', 'success');
      console.log('Saved HTML fallback to:', htmlFilePath);
    } catch (fallbackError) {
      console.error('Fallback HTML save also failed:', fallbackError);
      showNotification('All export methods failed', 'error');
    }

    return false;
  }
}

export function usePrintIndividual() {
  return useMutation({
    mutationFn: (staffId: string) => printService.printIndividual(staffId),
    onSuccess: async (htmlContent) => {
      const filename = `staff-details-${new Date().toISOString().split('T')[0]}.pdf`;
      await exportToPDF(htmlContent, filename, 1);
    },
    onError: (error) => {
      console.error('Print individual failed:', error);
      showNotification('Failed to generate staff details', 'error');
    }
  });
}

export function usePrintBulk() {
  return useMutation({
    mutationFn: (staffIds: string[]) => printService.printBulk(staffIds),
    onSuccess: async (htmlContent, variables) => {
      const filename = `staff-directory-${new Date().toISOString().split('T')[0]}.pdf`;
      await exportToPDF(htmlContent, filename, variables.length);
    },
    onError: (error) => {
      console.error('Print bulk failed:', error);
      showNotification('Failed to generate staff directory', 'error');
    }
  });
}

export function useExportToPDF() {
  return useMutation({
    mutationFn: ({ staffIds, isBulk }: { staffIds: string[]; isBulk: boolean }) =>
      printService.exportToPDF(staffIds, isBulk),
    onSuccess: async (htmlContent, variables) => {
      const filename = variables.isBulk
        ? `staff-directory-${new Date().toISOString().split('T')[0]}.pdf`
        : `staff-details-${new Date().toISOString().split('T')[0]}.pdf`;

      await exportToPDF(htmlContent, filename, variables.staffIds.length);
    },
    onError: (error) => {
      console.error('Export to PDF failed:', error);
      showNotification('Failed to export PDF', 'error');
    }
  });
}