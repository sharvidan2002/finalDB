import { useMutation } from '@tanstack/react-query';
import { printService } from '../lib/database';

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

// Enhanced PDF export function with automatic PDF generation
async function exportToPDF(htmlContent: string, filename: string, staffCount: number = 1) {
  try {
    // Show loading message
    const loadingMessage = `Generating PDF for ${staffCount > 1 ? `${staffCount} staff records` : 'staff record'}...`;

    // Create a temporary notification (you can style this better)
    const notification = document.createElement('div');
    notification.innerHTML = `
      <div style="
        position: fixed;
        top: 20px;
        right: 20px;
        background: #3b82f6;
        color: white;
        padding: 12px 24px;
        border-radius: 8px;
        z-index: 10000;
        box-shadow: 0 4px 12px rgba(0,0,0,0.15);
        font-family: system-ui;
      ">
        <div style="display: flex; align-items: center; gap: 8px;">
          <div style="
            width: 16px;
            height: 16px;
            border: 2px solid white;
            border-top-color: transparent;
            border-radius: 50%;
            animation: spin 1s linear infinite;
          "></div>
          ${loadingMessage}
        </div>
      </div>
      <style>
        @keyframes spin {
          to { transform: rotate(360deg); }
        }
      </style>
    `;
    document.body.appendChild(notification);

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
        format: staffCount > 1 ? 'a4' : 'a4',
        orientation: staffCount > 1 ? 'landscape' : 'portrait'
      },
      pagebreak: {
        mode: ['avoid-all', 'css', 'legacy']
      }
    };

    // Generate PDF (but don't auto-save)
    const pdfBlob = await window.html2pdf()
      .set(options)
      .from(container)
      .outputPdf('blob');

    // For Tauri app, you can use file dialog to let user choose location
    // Import at top of file: import { save } from '@tauri-apps/api/dialog';
    // const filePath = await save({
    //   filters: [{
    //     name: 'PDF',
    //     extensions: ['pdf']
    //   }],
    //   defaultPath: options.filename
    // });

    // For now, use browser default download
    const url = URL.createObjectURL(pdfBlob);
    const link = document.createElement('a');
    link.href = url;
    link.download = options.filename;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);

    // Clean up
    document.body.removeChild(container);
    document.body.removeChild(notification);

    // Show success message
    const successNotification = document.createElement('div');
    successNotification.innerHTML = `
      <div style="
        position: fixed;
        top: 20px;
        right: 20px;
        background: #10b981;
        color: white;
        padding: 12px 24px;
        border-radius: 8px;
        z-index: 10000;
        box-shadow: 0 4px 12px rgba(0,0,0,0.15);
        font-family: system-ui;
      ">
        ✓ PDF downloaded successfully!
      </div>
    `;
    document.body.appendChild(successNotification);

    // Remove success notification after 3 seconds
    setTimeout(() => {
      if (document.body.contains(successNotification)) {
        document.body.removeChild(successNotification);
      }
    }, 3000);

  } catch (error) {
    console.error('PDF generation failed:', error);

    // Properly handle error type
    const errorMessage = error instanceof Error ? error.message : String(error);

    // Show error message
    const errorNotification = document.createElement('div');
    errorNotification.innerHTML = `
      <div style="
        position: fixed;
        top: 20px;
        right: 20px;
        background: #ef4444;
        color: white;
        padding: 12px 24px;
        border-radius: 8px;
        z-index: 10000;
        box-shadow: 0 4px 12px rgba(0,0,0,0.15);
        font-family: system-ui;
        max-width: 300px;
      ">
        ✗ PDF generation failed<br>
        <small style="opacity: 0.9;">Error: ${errorMessage}</small>
      </div>
    `;
    document.body.appendChild(errorNotification);

    // Remove error notification after 5 seconds
    setTimeout(() => {
      if (document.body.contains(errorNotification)) {
        document.body.removeChild(errorNotification);
      }
    }, 5000);

    // Fallback: Save HTML file instead
    try {
      const { writeBinaryFile } = await import('@tauri-apps/api/fs');
      const { downloadDir } = await import('@tauri-apps/api/path');

      const downloadsPath = await downloadDir();
      const htmlFilePath = `${downloadsPath}${filename}`;
      const htmlBytes = new TextEncoder().encode(htmlContent);
      await writeBinaryFile(htmlFilePath, htmlBytes);

      console.log('Saved HTML fallback to:', htmlFilePath);
    } catch (fallbackError) {
      console.error('Fallback HTML save also failed:', fallbackError);
    }
  }
}

export function usePrintIndividual() {
  return useMutation({
    mutationFn: (staffId: string) => printService.printIndividual(staffId),
    onSuccess: (htmlContent) => {
      const filename = `staff-details-${new Date().toISOString().split('T')[0]}.html`;
      exportToPDF(htmlContent, filename, 1);
    },
  });
}

export function usePrintBulk() {
  return useMutation({
    mutationFn: (staffIds: string[]) => printService.printBulk(staffIds),
    onSuccess: (htmlContent, variables) => {
      const filename = `staff-directory-${new Date().toISOString().split('T')[0]}.html`;
      exportToPDF(htmlContent, filename, variables.length);
    },
  });
}

export function useExportToPDF() {
  return useMutation({
    mutationFn: ({ staffIds, isBulk }: { staffIds: string[]; isBulk: boolean }) =>
      printService.exportToPDF(staffIds, isBulk),
    onSuccess: (htmlContent, variables) => {
      const filename = variables.isBulk
        ? `staff-directory-${new Date().toISOString().split('T')[0]}.html`
        : `staff-details-${new Date().toISOString().split('T')[0]}.html`;

      exportToPDF(htmlContent, filename, variables.staffIds.length);
    },
  });
}