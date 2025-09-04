import React, { useState } from 'react';
import { format } from 'date-fns';
import { Calendar as CalendarIcon } from 'lucide-react';
import { Calendar } from '../ui/calendar';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { cn } from '../../lib/utils';

interface DatePickerProps {
  value?: string;
  onChange: (date: string) => void;
  placeholder?: string;
  disabled?: boolean;
  className?: string;
  label?: string;
  required?: boolean;
}

export function DatePicker({
  value,
  onChange,
  placeholder = "Select date",
  disabled = false,
  className,
  label,
  required = false,
}: DatePickerProps) {
  const [isOpen, setIsOpen] = useState(false);
  const [inputValue, setInputValue] = useState(value || '');

  const handleDateSelect = (date: Date | undefined) => {
    if (date) {
      const formattedDate = format(date, 'yyyy-MM-dd');
      const displayDate = format(date, 'dd-MM-yyyy');

      setInputValue(displayDate);
      onChange(formattedDate);
      setIsOpen(false);
    }
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const inputVal = e.target.value;
    setInputValue(inputVal);

    // Try to parse DD-MM-YYYY format
    const dateRegex = /^(\d{2})-(\d{2})-(\d{4})$/;
    const match = inputVal.match(dateRegex);

    if (match) {
      const [, day, month, year] = match;
      const isoDate = `${year}-${month}-${day}`;
      const date = new Date(isoDate);

      if (!isNaN(date.getTime())) {
        onChange(isoDate);
      }
    }
  };

  // Format display value
  const displayValue = value ?
    format(new Date(value), 'dd-MM-yyyy') :
    inputValue;

  const selectedDate = value ? new Date(value) : undefined;

  return (
    <div className={cn("relative", className)}>
      {label && (
        <label className="block text-sm font-medium text-slate-700 mb-2">
          {label} {required && <span className="text-red-500">*</span>}
        </label>
      )}

      <div className="relative">
        <Input
          type="text"
          value={displayValue}
          onChange={handleInputChange}
          placeholder={placeholder}
          disabled={disabled}
          className="pr-10"
        />

        <Button
          type="button"
          variant="ghost"
          size="icon"
          className="absolute right-0 top-0 h-full px-3"
          onClick={() => setIsOpen(!isOpen)}
          disabled={disabled}
        >
          <CalendarIcon className="h-4 w-4" />
        </Button>

        {isOpen && (
          <div className="absolute top-full left-0 z-50 mt-1">
            <div className="bg-white rounded-lg shadow-lg border p-3">
              <Calendar
                mode="single"
                selected={selectedDate}
                onSelect={handleDateSelect}
                disabled={disabled}
                initialFocus
              />
              <div className="flex justify-end mt-2">
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={() => setIsOpen(false)}
                >
                  Close
                </Button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}