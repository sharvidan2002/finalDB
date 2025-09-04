import { useMutation } from '@tanstack/react-query';
import { printService } from '../lib/database';
import { printContent } from '../lib/utils';

export function usePrintIndividual() {
  return useMutation({
    mutationFn: (staffId: string) => printService.printIndividual(staffId),
    onSuccess: (htmlContent) => {
      printContent(htmlContent);
    },
  });
}

export function usePrintBulk() {
  return useMutation({
    mutationFn: (staffIds: string[]) => printService.printBulk(staffIds),
    onSuccess: (htmlContent) => {
      printContent(htmlContent);
    },
  });
}

export function useExportToPDF() {
  return useMutation({
    mutationFn: ({ staffIds, isBulk }: { staffIds: string[]; isBulk: boolean }) =>
      printService.exportToPDF(staffIds, isBulk),
    onSuccess: (htmlContent) => {
      // Open in new window and trigger print dialog for PDF export
      printContent(htmlContent);
    },
  });
}