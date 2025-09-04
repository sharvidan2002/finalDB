import React, { useState, useEffect } from 'react';
import { Input } from '../ui/input';
import { validateNIC, normalizeNIC } from '../../lib/nicConverter';
import { cn } from '../../lib/utils';

interface NICInputProps {
  value?: string;
  onChange: (nicData: { newFormat: string; oldFormat?: string }) => void;
  placeholder?: string;
  disabled?: boolean;
  className?: string;
  label?: string;
  required?: boolean;
  error?: string;
}

export function NICInput({
  value = '',
  onChange,
  placeholder = "Enter NIC number",
  disabled = false,
  className,
  label,
  required = false,
  error,
}: NICInputProps) {
  const [inputValue, setInputValue] = useState(value);
  const [isValid, setIsValid] = useState(true);
  const [formatInfo, setFormatInfo] = useState<string>('');

  useEffect(() => {
    setInputValue(value);
  }, [value]);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const inputVal = e.target.value.trim().toUpperCase();
    setInputValue(inputVal);

    if (!inputVal) {
      setIsValid(true);
      setFormatInfo('');
      return;
    }

    const validation = validateNIC(inputVal);
    setIsValid(validation.isValid);

    if (validation.isValid) {
      try {
        const nicData = normalizeNIC(inputVal);
        onChange(nicData);

        if (validation.isOldFormat) {
          setFormatInfo(`Converted to new format: ${nicData.newFormat}`);
        } else {
          setFormatInfo('New format NIC');
        }
      } catch (error) {
        setIsValid(false);
        setFormatInfo('Invalid NIC format');
      }
    } else {
      setFormatInfo('Invalid NIC format');
    }
  };

  return (
    <div className={cn("space-y-2", className)}>
      {label && (
        <label className="block text-sm font-medium text-slate-700">
          {label} {required && <span className="text-red-500">*</span>}
        </label>
      )}

      <Input
        type="text"
        value={inputValue}
        onChange={handleInputChange}
        placeholder={placeholder}
        disabled={disabled}
        className={cn(
          !isValid && inputValue && "border-red-500 focus:border-red-500",
          isValid && inputValue && "border-green-500"
        )}
      />

      {inputValue && (
        <div className="flex items-center space-x-2">
          <div className={cn(
            "h-2 w-2 rounded-full",
            isValid ? "bg-green-500" : "bg-red-500"
          )} />
          <p className={cn(
            "text-xs",
            isValid ? "text-green-600" : "text-red-600"
          )}>
            {formatInfo}
          </p>
        </div>
      )}

      {error && (
        <p className="text-xs text-red-600">{error}</p>
      )}

      <div className="text-xs text-slate-500">
        Accepts both old format (123456789V) and new format (200012345678)
      </div>
    </div>
  );
}