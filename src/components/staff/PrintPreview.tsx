// import React from 'react';
import { Printer, Download, X } from 'lucide-react';
import { Button } from '../ui/button';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '../ui/dialog';
import type { Staff } from '../../types/staff';

interface PrintPreviewProps {
  isOpen: boolean;
  onClose: () => void;
  staff?: Staff;
  staffList?: Staff[];
  isBulk?: boolean;
  htmlContent: string;
  onPrint: () => void;
  onExport: () => void;
  isPrintPending?: boolean;
  isExportPending?: boolean;
}

export function PrintPreview({
  isOpen,
  onClose,
  staff,
  staffList,
  isBulk = false,
  htmlContent,
  onPrint,
  onExport,
  isPrintPending = false,
  isExportPending = false
}: PrintPreviewProps) {
  const title = isBulk
    ? `Staff Directory (${staffList?.length || 0} staff)`
    : `${staff?.fullName || 'Staff'} Details`;

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="max-w-4xl max-h-[90vh]">
        <DialogHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <DialogTitle className="text-lg font-semibold">
            Print Preview: {title}
          </DialogTitle>

          <div className="flex items-center space-x-2">
            <Button
              variant="outline"
              size="sm"
              onClick={onPrint}
              disabled={isPrintPending}
              className="flex items-center space-x-2"
            >
              {isPrintPending && (
                <div className="w-3 h-3 border border-slate-600 border-t-transparent rounded-full animate-spin" />
              )}
              <Printer className="h-4 w-4" />
              <span>Print</span>
            </Button>

            <Button
              variant="outline"
              size="sm"
              onClick={onExport}
              disabled={isExportPending}
              className="flex items-center space-x-2"
            >
              {isExportPending && (
                <div className="w-3 h-3 border border-slate-600 border-t-transparent rounded-full animate-spin" />
              )}
              <Download className="h-4 w-4" />
              <span>Export</span>
            </Button>

            <Button
              variant="ghost"
              size="sm"
              onClick={onClose}
              className="flex items-center space-x-1"
            >
              <X className="h-4 w-4" />
            </Button>
          </div>
        </DialogHeader>

        <div className="flex-1 overflow-hidden">
          <div className="h-full border border-slate-200 rounded-lg overflow-auto modal-content">
            <div
              className="p-4 bg-white min-h-full"
              dangerouslySetInnerHTML={{ __html: htmlContent }}
              style={{
                fontSize: '12px',
                lineHeight: '1.4'
              }}
            />
          </div>
        </div>

        <div className="flex items-center justify-between pt-2 border-t border-slate-200 text-xs text-slate-500">
          <div>
            Preview generated on {new Date().toLocaleString()}
          </div>
          <div>
            {isBulk ? `${staffList?.length || 0} staff records` : '1 staff record'}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}