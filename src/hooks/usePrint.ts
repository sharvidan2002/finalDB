import { useMutation } from '@tanstack/react-query';
import { printService } from '../lib/database';
import { downloadContent } from '../lib/utils';

// Enhanced PDF export function
function exportToPDF(htmlContent: string, filename: string, staffCount: number = 1) {
  // 1. Download the HTML file
  downloadContent(htmlContent, filename, 'text/html');

  // 2. Open in new tab for immediate PDF saving
  const newWindow = window.open('', '_blank');
  if (newWindow) {
    newWindow.document.write(htmlContent);
    newWindow.document.close();

    // 3. Auto-trigger print dialog after content loads
    newWindow.onload = () => {
      setTimeout(() => {
        newWindow.print();
      }, 1000);
    };
  }

  // 4. Show user-friendly instructions
  setTimeout(() => {
    const message = staffCount > 1
      ? `${staffCount} staff records ready for PDF export!\n\n✓ File downloaded to your Downloads folder\n✓ Print dialog opened in new tab\n\nIn the print dialog:\n1. Select "Save as PDF" as destination\n2. Click "Save" to create your PDF`
      : `Staff details ready for PDF export!\n\n✓ File downloaded to your Downloads folder\n✓ Print dialog opened in new tab\n\nIn the print dialog:\n1. Select "Save as PDF" as destination\n2. Click "Save" to create your PDF`;

    alert(message);
  }, 800);
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
    onSuccess: (htmlContent) => {
      const filename = `staff-directory-${new Date().toISOString().split('T')[0]}.html`;
      exportToPDF(htmlContent, filename, Array.isArray(arguments[1]) ? arguments[1].length : 1);
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