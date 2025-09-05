import React, { useState, useRef, useCallback, useEffect } from 'react';
import { Upload, X, User, Crop, Move, ZoomIn, ZoomOut, Check } from 'lucide-react';
import { Button } from '../ui/button';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from '../ui/dialog';
import { cn } from '../../lib/utils';
import { IMAGE_CONSTRAINTS } from '../../lib/constants';

interface ImageUploadProps {
  value?: string;
  onChange: (base64Image: string | undefined) => void;
  className?: string;
  label?: string;
  disabled?: boolean;
}

interface CropArea {
  x: number;
  y: number;
  width: number;
  height: number;
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
  const [cropArea, setCropArea] = useState<CropArea>({ x: 0, y: 0, width: 200, height: 200 });
  const [isDragging, setIsDragging] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });
  const [imageScale, setImageScale] = useState(1);
  const [imagePosition, setImagePosition] = useState({ x: 0, y: 0 });
  const [naturalSize, setNaturalSize] = useState({ width: 0, height: 0 });

  const fileInputRef = useRef<HTMLInputElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const imageRef = useRef<HTMLImageElement>(null);
  const cropperRef = useRef<HTMLDivElement>(null);

  const handleFileSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    const fileType = file.type as typeof IMAGE_CONSTRAINTS.ACCEPTED_TYPES[number];
    const isAcceptedType =
      IMAGE_CONSTRAINTS.ACCEPTED_TYPES.includes(fileType) || file.type.startsWith('image/');

    if (!isAcceptedType) {
      alert('Please select a valid image file (JPEG, PNG, or WebP)');
      return;
    }

    if (file.size > IMAGE_CONSTRAINTS.MAX_SIZE) {
      alert('File size must be less than 5MB');
      return;
    }

    const reader = new FileReader();
    reader.onload = (e) => {
      const result = e.target?.result as string;
      setOriginalImage(result);
      setIsDialogOpen(true);

      // Reset crop settings
      setImageScale(1);
      setImagePosition({ x: 0, y: 0 });
      setCropArea({ x: 50, y: 50, width: 200, height: 200 });
    };
    reader.readAsDataURL(file);
  };

  const handleImageLoad = useCallback(() => {
    if (imageRef.current) {
      const { naturalWidth, naturalHeight } = imageRef.current;
      setNaturalSize({ width: naturalWidth, height: naturalHeight });

      // Set initial crop area to center of image
      const containerWidth = 400;
      const containerHeight = 300;
      const scale = Math.min(containerWidth / naturalWidth, containerHeight / naturalHeight);
      setImageScale(scale);

      const displayWidth = naturalWidth * scale;
      const displayHeight = naturalHeight * scale;
      const startX = (containerWidth - displayWidth) / 2;
      const startY = (containerHeight - displayHeight) / 2;
      setImagePosition({ x: startX, y: startY });

      // Center crop area
      const cropSize = Math.min(displayWidth, displayHeight) * 0.6;
      setCropArea({
        x: startX + (displayWidth - cropSize) / 2,
        y: startY + (displayHeight - cropSize) / 2,
        width: cropSize,
        height: cropSize
      });
    }
  }, []);

  const handleMouseDown = useCallback((e: React.MouseEvent, action: 'drag' | 'resize') => {
    e.preventDefault();
    const rect = cropperRef.current?.getBoundingClientRect();
    if (!rect) return;

    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    setDragStart({ x, y });

    if (action === 'drag') {
      setIsDragging(true);
    } else {
      setIsResizing(true);
    }
  }, []);

  const handleMouseMove = useCallback((e: MouseEvent) => {
    if (!cropperRef.current) return;

    const rect = cropperRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    const deltaX = x - dragStart.x;
    const deltaY = y - dragStart.y;

    if (isDragging) {
      setCropArea(prev => {
        const newX = Math.max(0, Math.min(400 - prev.width, prev.x + deltaX));
        const newY = Math.max(0, Math.min(300 - prev.height, prev.y + deltaY));
        return { ...prev, x: newX, y: newY };
      });
      setDragStart({ x, y });
    } else if (isResizing) {
      setCropArea(prev => {
        const newWidth = Math.max(50, Math.min(400 - prev.x, prev.width + deltaX));
        const newHeight = Math.max(50, Math.min(300 - prev.y, prev.height + deltaY));
        return { ...prev, width: newWidth, height: newHeight };
      });
      setDragStart({ x, y });
    }
  }, [isDragging, isResizing, dragStart]);

  const handleMouseUp = useCallback(() => {
    setIsDragging(false);
    setIsResizing(false);
  }, []);

  useEffect(() => {
    if (isDragging || isResizing) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      return () => {
        document.removeEventListener('mousemove', handleMouseMove);
        document.removeEventListener('mouseup', handleMouseUp);
      };
    }
  }, [isDragging, isResizing, handleMouseMove, handleMouseUp]);

  const handleZoom = (delta: number) => {
    setImageScale(prev => {
      const newScale = Math.max(0.1, Math.min(3, prev + delta));

      // Adjust position to keep image centered
      if (imageRef.current) {
        const { naturalWidth, naturalHeight } = imageRef.current;
        const newWidth = naturalWidth * newScale;
        const newHeight = naturalHeight * newScale;

        setImagePosition({
          x: (400 - newWidth) / 2,
          y: (300 - newHeight) / 2
        });
      }

      return newScale;
    });
  };

  const handleCropAndSave = () => {
    const canvas = canvasRef.current;
    const image = imageRef.current;
    if (!canvas || !image || !originalImage) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Calculate the actual crop coordinates relative to the original image
    const scaleX = naturalSize.width / (naturalSize.width * imageScale);
    const scaleY = naturalSize.height / (naturalSize.height * imageScale);

    const actualCropX = (cropArea.x - imagePosition.x) * scaleX;
    const actualCropY = (cropArea.y - imagePosition.y) * scaleY;
    const actualCropWidth = cropArea.width * scaleX;
    const actualCropHeight = cropArea.height * scaleY;

    // Set canvas size for output
    canvas.width = IMAGE_CONSTRAINTS.OUTPUT_WIDTH;
    canvas.height = IMAGE_CONSTRAINTS.OUTPUT_HEIGHT;

    // Draw the cropped portion
    ctx.drawImage(
      image,
      Math.max(0, actualCropX),
      Math.max(0, actualCropY),
      Math.min(actualCropWidth, naturalSize.width - actualCropX),
      Math.min(actualCropHeight, naturalSize.height - actualCropY),
      0, 0,
      canvas.width,
      canvas.height
    );

    // Convert to base64
    const croppedBase64 = canvas.toDataURL('image/jpeg', 0.8);
    const base64Data = croppedBase64.split(',')[1];

    onChange(base64Data);
    setIsDialogOpen(false);
    setOriginalImage('');

    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
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
            <p>You can freely crop, move, and resize the image</p>
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

      {/* Interactive Crop Dialog */}
      <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
        <DialogContent className="max-w-2xl">
          <DialogHeader>
            <DialogTitle className="flex items-center space-x-2">
              <Crop className="h-5 w-5" />
              <span>Crop Image</span>
            </DialogTitle>
          </DialogHeader>

          <div className="space-y-4">
            {/* Zoom Controls */}
            <div className="flex items-center justify-center space-x-4">
              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={() => handleZoom(-0.1)}
                className="flex items-center space-x-1"
              >
                <ZoomOut className="h-4 w-4" />
                <span>Zoom Out</span>
              </Button>

              <span className="text-sm text-slate-600">
                Zoom: {Math.round(imageScale * 100)}%
              </span>

              <Button
                type="button"
                variant="outline"
                size="sm"
                onClick={() => handleZoom(0.1)}
                className="flex items-center space-x-1"
              >
                <ZoomIn className="h-4 w-4" />
                <span>Zoom In</span>
              </Button>
            </div>

            {/* Cropping Area */}
            <div className="relative">
              <div
                ref={cropperRef}
                className="relative w-[400px] h-[300px] mx-auto border-2 border-slate-300 rounded-lg overflow-hidden bg-slate-100"
              >
                {originalImage && (
                  <>
                    {/* Background Image */}
                    <img
                      ref={imageRef}
                      src={originalImage}
                      alt="Original"
                      className="absolute"
                      style={{
                        left: imagePosition.x,
                        top: imagePosition.y,
                        width: naturalSize.width * imageScale,
                        height: naturalSize.height * imageScale,
                        pointerEvents: 'none',
                        userSelect: 'none'
                      }}
                      onLoad={handleImageLoad}
                    />

                    {/* Overlay */}
                    <div className="absolute inset-0 bg-black bg-opacity-50" />

                    {/* Crop Area */}
                    <div
                      className="absolute border-2 border-white bg-transparent cursor-move"
                      style={{
                        left: cropArea.x,
                        top: cropArea.y,
                        width: cropArea.width,
                        height: cropArea.height,
                        boxShadow: '0 0 0 9999px rgba(0, 0, 0, 0.5)'
                      }}
                      onMouseDown={(e) => handleMouseDown(e, 'drag')}
                    >
                      {/* Crop Preview */}
                      <img
                        src={originalImage}
                        alt="Crop preview"
                        className="absolute pointer-events-none"
                        style={{
                          left: -(cropArea.x - imagePosition.x),
                          top: -(cropArea.y - imagePosition.y),
                          width: naturalSize.width * imageScale,
                          height: naturalSize.height * imageScale,
                        }}
                      />

                      {/* Resize Handle */}
                      <div
                        className="absolute bottom-0 right-0 w-4 h-4 bg-white border border-slate-400 cursor-se-resize"
                        onMouseDown={(e) => {
                          e.stopPropagation();
                          handleMouseDown(e, 'resize');
                        }}
                      />

                      {/* Move Icon */}
                      <div className="absolute top-2 left-2 text-white">
                        <Move className="h-4 w-4" />
                      </div>
                    </div>
                  </>
                )}
              </div>
            </div>

            <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
              <div className="flex items-start space-x-2">
                <Crop className="h-4 w-4 text-blue-600 mt-0.5" />
                <div className="text-sm text-blue-800">
                  <p className="font-medium mb-1">How to crop:</p>
                  <ul className="space-y-1 text-xs">
                    <li>• Drag the white box to move the crop area</li>
                    <li>• Drag the bottom-right corner to resize</li>
                    <li>• Use zoom controls to adjust image size</li>
                    <li>• Final output: {IMAGE_CONSTRAINTS.OUTPUT_WIDTH}×{IMAGE_CONSTRAINTS.OUTPUT_HEIGHT} pixels</li>
                  </ul>
                </div>
              </div>
            </div>
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
              className="flex items-center space-x-2"
            >
              <Check className="h-4 w-4" />
              <span>Crop & Save</span>
            </Button>
          </DialogFooter>

          {/* Hidden canvas for processing */}
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