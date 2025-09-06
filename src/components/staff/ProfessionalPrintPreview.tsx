import { useState, useEffect } from 'react';
import { Printer, Download, X, FileText, Eye, ZoomIn, ZoomOut } from 'lucide-react';
import { Button } from '../ui/button';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '../ui/dialog';
import { useExportToPDF, useOpenDownloadsFolder } from '../../hooks/usePrint';
import { invoke } from '@tauri-apps/api/core';

interface ProfessionalPrintPreviewProps {
  isOpen: boolean;
  onClose: () => void;
  staffIds: string[];
  isBulk?: boolean;
  title?: string;
}

export function ProfessionalPrintPreview({
  isOpen,
  onClose,
  staffIds,
  isBulk = false,
  title
}: ProfessionalPrintPreviewProps) {
  const [htmlContent, setHtmlContent] = useState<string>('');
  const [isLoading, setIsLoading] = useState(false);
  const [zoom, setZoom] = useState(1);
  const [error, setError] = useState<string>('');

  const exportToPDF = useExportToPDF();
  const openDownloads = useOpenDownloadsFolder();

  // Load preview content when dialog opens
  useEffect(() => {
    if (isOpen && staffIds.length > 0) {
      loadPreviewContent();
    }
  }, [isOpen, staffIds, isBulk]);

  const loadPreviewContent = async () => {
    setIsLoading(true);
    setError('');

    try {
      let content: string;

      if (isBulk || staffIds.length > 1) {
        content = await invoke<string>('generate_bulk_staff_preview', {
          staffIds: staffIds
        });
      } else {
        content = await invoke<string>('generate_staff_preview', {
          staffId: staffIds[0]
        });
      }

      setHtmlContent(content);
    } catch (err) {
      console.error('Error loading preview:', err);
      setError(err instanceof Error ? err.message : 'Failed to load preview');
    } finally {
      setIsLoading(false);
    }
  };

  const handleExportPDF = async () => {
    try {
      await exportToPDF.mutateAsync({
        staffIds,
        isBulk: isBulk || staffIds.length > 1
      });

      // Show option to open downloads folder
      setTimeout(() => {
        if (window.confirm('PDF saved successfully! Would you like to open the Downloads folder?')) {
          openDownloads.mutate();
        }
      }, 1000);
    } catch (error) {
      console.error('Export failed:', error);
    }
  };

  const handlePrint = () => {
    if (htmlContent) {
      const printWindow = window.open('', '_blank');
      if (printWindow) {
        printWindow.document.write(htmlContent);
        printWindow.document.close();
        printWindow.onload = () => {
          setTimeout(() => {
            printWindow.print();
          }, 500);
        };
      }
    }
  };

  const handleZoomIn = () => {
    setZoom(prev => Math.min(prev + 0.1, 2));
  };

  const handleZoomOut = () => {
    setZoom(prev => Math.max(prev - 0.1, 0.5));
  };

  const getDialogTitle = () => {
    if (title) return title;
    if (isBulk || staffIds.length > 1) {
      return `Staff Directory Preview (${staffIds.length} records)`;
    }
    return 'Staff Record Preview';
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="max-w-6xl max-h-[95vh] overflow-hidden">
        <DialogHeader className="flex flex-row items-center justify-between space-y-0 pb-4">
          <div className="flex items-center space-x-3">
            <FileText className="h-5 w-5 text-blue-600" />
            <DialogTitle className="text-lg font-semibold">
              {getDialogTitle()}
            </DialogTitle>
          </div>

          {/* Action Buttons */}
          <div className="flex items-center space-x-2">
            {/* Zoom Controls */}
            <div className="flex items-center space-x-1 border rounded-md p-1">
              <Button
                variant="ghost"
                size="sm"
                onClick={handleZoomOut}
                disabled={zoom <= 0.5}
                className="p-1 h-6 w-6"
              >
                <ZoomOut className="h-3 w-3" />
              </Button>
              <span className="text-xs px-2 min-w-[3rem] text-center">
                {Math.round(zoom * 100)}%
              </span>
              <Button
                variant="ghost"
                size="sm"
                onClick={handleZoomIn}
                disabled={zoom >= 2}
                className="p-1 h-6 w-6"
              >
                <ZoomIn className="h-3 w-3" />
              </Button>
            </div>

            {/* Print Button */}
            <Button
              variant="outline"
              size="sm"
              onClick={handlePrint}
              disabled={isLoading || !htmlContent}
              className="flex items-center space-x-2"
            >
              <Printer className="h-4 w-4" />
              <span>Print</span>
            </Button>

            {/* Export PDF Button */}
            <Button
              variant="default"
              size="sm"
              onClick={handleExportPDF}
              disabled={exportToPDF.isPending || isLoading || !htmlContent}
              className="flex items-center space-x-2"
            >
              {exportToPDF.isPending && (
                <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
              )}
              <Download className="h-4 w-4" />
              <span>Export PDF</span>
            </Button>

            {/* Close Button */}
            <Button
              variant="ghost"
              size="sm"
              onClick={onClose}
              className="p-2"
            >
              <X className="h-4 w-4" />
            </Button>
          </div>
        </DialogHeader>

        {/* Preview Content */}
        <div className="flex-1 overflow-hidden bg-gray-100 rounded-lg border">
          {isLoading ? (
            <div className="flex items-center justify-center h-96">
              <div className="text-center">
                <div className="loading-spinner w-8 h-8 mx-auto mb-4"></div>
                <p className="text-slate-600">Loading preview...</p>
              </div>
            </div>
          ) : error ? (
            <div className="flex items-center justify-center h-96">
              <div className="text-center">
                <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                  <p className="font-bold">Error Loading Preview</p>
                  <p className="text-sm">{error}</p>
                </div>
                <Button onClick={loadPreviewContent} variant="outline">
                  Try Again
                </Button>
              </div>
            </div>
          ) : htmlContent ? (
            <div className="h-full overflow-auto p-4">
              <div
                className="bg-white shadow-lg mx-auto min-h-full"
                style={{
                  transform: `scale(${zoom})`,
                  transformOrigin: 'top center',
                  width: `${100 / zoom}%`,
                  maxWidth: isBulk ? '297mm' : '210mm',
                  minHeight: isBulk ? 'auto' : '297mm'
                }}
              >
                <div
                  dangerouslySetInnerHTML={{ __html: htmlContent }}
                  className="w-full h-full"
                />
              </div>
            </div>
          ) : (
            <div className="flex items-center justify-center h-96">
              <div className="text-center">
                <Eye className="h-12 w-12 text-slate-400 mx-auto mb-4" />
                <p className="text-slate-600">No preview available</p>
              </div>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="flex items-center justify-between pt-3 border-t border-slate-200 text-xs text-slate-500">
          <div className="flex items-center space-x-4">
            <span>
              Preview generated on {new Date().toLocaleString()}
            </span>
            <span>•</span>
            <span>
              {isBulk || staffIds.length > 1
                ? `${staffIds.length} staff records`
                : '1 staff record'}
            </span>
          </div>

          {htmlContent && (
            <div className="flex items-center space-x-2">
              <div className="w-2 h-2 bg-green-500 rounded-full"></div>
              <span>Ready to print/export</span>
            </div>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}