import { useMutation } from '@tanstack/react-query';
import { printService } from '../lib/database';
import { printContent, downloadContent } from '../lib/utils';

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
    onSuccess: (htmlContent, variables) => {
      const filename = variables.isBulk
        ? `staff-directory-${new Date().toISOString().split('T')[0]}.html`
        : `staff-details-${new Date().toISOString().split('T')[0]}.html`;

      downloadContent(htmlContent, filename);
    },
  });
}