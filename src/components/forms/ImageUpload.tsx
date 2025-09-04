import React, { useState, useRef } from 'react';
import { Upload, X, User, Crop } from 'lucide-react';
import { Button } from '../ui/button';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from '../ui/dialog';
import { cn } from '../../lib/utils';
// import { fileToBase64 } from '../../lib/utils';
import { IMAGE_CONSTRAINTS } from '../../lib/constants';

interface ImageUploadProps {
  value?: string;
  onChange: (base64Image: string | undefined) => void;
  className?: string;
  label?: string;
  disabled?: boolean;
}

export function ImageUpload({
  value,
  onChange,
  className,
  label,
  disabled = false,
}: ImageUploadProps) {
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [originalImage, setOriginalImage] = useState<string>('');
  const fileInputRef = useRef<HTMLInputElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const handleFileSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    // Cast file.type to the array's element type to satisfy TypeScript
    const fileType = file.type as typeof IMAGE_CONSTRAINTS.ACCEPTED_TYPES[number];

    // Validate file type (also allow generic image/* as a fallback)
    const isAcceptedType =
      IMAGE_CONSTRAINTS.ACCEPTED_TYPES.includes(fileType) || file.type.startsWith('image/');

    if (!isAcceptedType) {
      alert('Please select a valid image file (JPEG, PNG, or WebP)');
      return;
    }

    // Validate file size
    if (file.size > IMAGE_CONSTRAINTS.MAX_SIZE) {
      alert('File size must be less than 5MB');
      return;
    }

    const reader = new FileReader();
    reader.onload = (e) => {
      const result = e.target?.result as string;
      setOriginalImage(result);
      setIsDialogOpen(true);
    };
    reader.readAsDataURL(file);
  };

  const handleCropAndSave = () => {
    const canvas = canvasRef.current;
    if (!canvas || !originalImage) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const img = new Image();
    img.onload = () => {
      // Set canvas size to desired output size
      canvas.width = IMAGE_CONSTRAINTS.OUTPUT_WIDTH;
      canvas.height = IMAGE_CONSTRAINTS.OUTPUT_HEIGHT;

      // Calculate dimensions to maintain aspect ratio and center crop
      const srcAspectRatio = img.width / img.height;
      const targetAspectRatio = IMAGE_CONSTRAINTS.CROP_ASPECT_RATIO;

      let srcX = 0, srcY = 0, srcWidth = img.width, srcHeight = img.height;

      if (srcAspectRatio > targetAspectRatio) {
        // Source is wider, crop width
        srcWidth = img.height * targetAspectRatio;
        srcX = (img.width - srcWidth) / 2;
      } else {
        // Source is taller, crop height
        srcHeight = img.width / targetAspectRatio;
        srcY = (img.height - srcHeight) / 2;
      }

      // Draw cropped image
      ctx.drawImage(
        img,
        srcX, srcY, srcWidth, srcHeight,
        0, 0, canvas.width, canvas.height
      );

      // Convert to base64 (JPEG at 80% quality)
      const croppedBase64 = canvas.toDataURL('image/jpeg', 0.8);
      const base64Data = croppedBase64.split(',')[1];

      onChange(base64Data);
      setIsDialogOpen(false);
      setOriginalImage('');

      // Clear file input value (if any)
      if (fileInputRef.current) {
        fileInputRef.current.value = '';
      }
    };
    img.src = originalImage;
  };

  const handleRemove = () => {
    onChange(undefined);
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  const triggerFileInput = () => {
    if (!disabled) {
      fileInputRef.current?.click();
    }
  };

  return (
    <div className={cn("space-y-2", className)}>
      {label && (
        <label className="block text-sm font-medium text-slate-700">
          {label}
        </label>
      )}

      <div className="flex items-start space-x-4">
        {/* Image Preview */}
        <div className="flex-shrink-0">
          <div className={cn(
            "w-24 h-32 border-2 border-dashed border-slate-300 rounded-lg flex items-center justify-center overflow-hidden",
            value ? "border-solid border-slate-400" : "border-dashed",
            disabled && "opacity-50"
          )}>
            {value ? (
              <img
                src={`data:image/jpeg;base64,${value}`}
                alt="Staff photo"
                className="w-full h-full object-cover"
              />
            ) : (
              <User className="h-8 w-8 text-slate-400" />
            )}
          </div>
        </div>

        {/* Controls */}
        <div className="flex-1 space-y-2">
          <div className="flex space-x-2">
            <Button
              type="button"
              variant="outline"
              size="sm"
              onClick={triggerFileInput}
              disabled={disabled}
              className="flex items-center space-x-2"
            >
              <Upload className="h-4 w-4" />
              <span>{value ? 'Change Photo' : 'Upload Photo'}</span>
            </Button>

            {value && (
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={handleRemove}
                disabled={disabled}
                className="flex items-center space-x-2 text-red-600 hover:text-red-700"
              >
                <X className="h-4 w-4" />
                <span>Remove</span>
              </Button>
            )}
          </div>

          <div className="text-xs text-slate-500">
            <p>Recommended: 3:4 ratio (e.g., 240×320px)</p>
            <p>Max size: 5MB • Formats: JPEG, PNG, WebP</p>
          </div>
        </div>
      </div>

      {/* Hidden file input */}
      <input
        ref={fileInputRef}
        type="file"
        accept={IMAGE_CONSTRAINTS.ACCEPTED_TYPES.join(',')}
        onChange={handleFileSelect}
        className="hidden"
        disabled={disabled}
      />

      {/* Crop Dialog */}
      <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
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

            <p className="text-sm text-slate-600">
              The image will be automatically cropped to 3:4 aspect ratio and resized to {IMAGE_CONSTRAINTS.OUTPUT_WIDTH}×{IMAGE_CONSTRAINTS.OUTPUT_HEIGHT} pixels.
            </p>
          </div>

          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => {
                setIsDialogOpen(false);
                setOriginalImage('');
              }}
            >
              Cancel
            </Button>
            <Button
              type="button"
              onClick={handleCropAndSave}
            >
              Crop & Save
            </Button>
          </DialogFooter>

          {/* Hidden canvas for image processing */}
          <canvas
            ref={canvasRef}
            className="hidden"
            width={IMAGE_CONSTRAINTS.OUTPUT_WIDTH}
            height={IMAGE_CONSTRAINTS.OUTPUT_HEIGHT}
          />
        </DialogContent>
      </Dialog>
    </div>
  );
}
