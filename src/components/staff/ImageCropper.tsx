import { useRef, useState } from 'react';
import { Crop, RotateCcw } from 'lucide-react';
import { Button } from '../ui/button';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from '../ui/dialog';
import { IMAGE_CONSTRAINTS } from '../../lib/constants';

interface ImageCropperProps {
  isOpen: boolean;
  onClose: () => void;
  originalImage: string;
  onCrop: (croppedImage: string) => void;
}

export function ImageCropper({ isOpen, onClose, originalImage, onCrop }: ImageCropperProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isProcessing, setIsProcessing] = useState(false);

  const handleCrop = async () => {
    if (!canvasRef.current || !originalImage) return;

    setIsProcessing(true);

    try {
      const canvas = canvasRef.current;
      const ctx = canvas.getContext('2d');
      if (!ctx) return;

      const img = new Image();
      img.onload = () => {
        canvas.width = IMAGE_CONSTRAINTS.OUTPUT_WIDTH;
        canvas.height = IMAGE_CONSTRAINTS.OUTPUT_HEIGHT;

        const srcAspectRatio = img.width / img.height;
        const targetAspectRatio = IMAGE_CONSTRAINTS.CROP_ASPECT_RATIO;

        let srcX = 0, srcY = 0, srcWidth = img.width, srcHeight = img.height;

        if (srcAspectRatio > targetAspectRatio) {
          srcWidth = img.height * targetAspectRatio;
          srcX = (img.width - srcWidth) / 2;
        } else {
          srcHeight = img.width / targetAspectRatio;
          srcY = (img.height - srcHeight) / 2;
        }

        ctx.drawImage(
          img,
          srcX, srcY, srcWidth, srcHeight,
          0, 0, canvas.width, canvas.height
        );

        const croppedBase64 = canvas.toDataURL('image/jpeg', 0.8);
        const base64Data = croppedBase64.split(',')[1];

        onCrop(base64Data);
        onClose();
      };

      img.onerror = () => {
        console.error('Failed to load image for cropping');
        setIsProcessing(false);
      };

      img.src = originalImage;
    } catch (error) {
      console.error('Error cropping image:', error);
      setIsProcessing(false);
    }
  };

  const handleCancel = () => {
    setIsProcessing(false);
    onClose();
  };

  return (
    <Dialog open={isOpen} onOpenChange={handleCancel}>
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center space-x-2">
            <Crop className="h-5 w-5" />
            <span>Crop Image</span>
          </DialogTitle>
        </DialogHeader>

        <div className="space-y-4">
          {originalImage && (
            <div className="flex justify-center">
              <div className="max-w-full max-h-64 overflow-hidden rounded-lg border">
                <img
                  src={originalImage}
                  alt="Original"
                  className="max-w-full max-h-full object-contain"
                />
              </div>
            </div>
          )}

          <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
            <div className="flex items-start space-x-2">
              <Crop className="h-4 w-4 text-blue-600 mt-0.5" />
              <div className="text-sm text-blue-800">
                <p className="font-medium mb-1">Auto-Crop Information:</p>
                <ul className="space-y-1 text-xs">
                  <li>• Image will be cropped to 3:4 aspect ratio</li>
                  <li>• Final size: {IMAGE_CONSTRAINTS.OUTPUT_WIDTH}×{IMAGE_CONSTRAINTS.OUTPUT_HEIGHT} pixels</li>
                  <li>• Centered crop for best composition</li>
                </ul>
              </div>
            </div>
          </div>

          <div className="text-center">
            <div className="inline-flex items-center space-x-2 text-sm text-slate-600">
              <RotateCcw className="h-4 w-4" />
              <span>Preview will be generated after cropping</span>
            </div>
          </div>
        </div>

        <DialogFooter>
          <Button
            type="button"
            variant="outline"
            onClick={handleCancel}
            disabled={isProcessing}
          >
            Cancel
          </Button>
          <Button
            type="button"
            onClick={handleCrop}
            disabled={isProcessing}
            className="flex items-center space-x-2"
          >
            {isProcessing && (
              <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
            )}
            <Crop className="h-4 w-4" />
            <span>Crop & Save</span>
          </Button>
        </DialogFooter>

        <canvas
          ref={canvasRef}
          className="hidden"
          width={IMAGE_CONSTRAINTS.OUTPUT_WIDTH}
          height={IMAGE_CONSTRAINTS.OUTPUT_HEIGHT}
        />
      </DialogContent>
    </Dialog>
  );
}