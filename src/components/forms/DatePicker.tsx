import React, { useState, useEffect } from 'react';
import { format } from 'date-fns';
import { Calendar as CalendarIcon, ChevronLeft, ChevronRight } from 'lucide-react';
import { Calendar } from '../ui/calendar';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../ui/select';
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
  const [inputValue, setInputValue] = useState('');
  const [calendarDate, setCalendarDate] = useState<Date>(new Date());

  // Initialize input value from prop
  useEffect(() => {
    if (value) {
      const displayDate = format(new Date(value), 'dd-MM-yyyy');
      setInputValue(displayDate);
      setCalendarDate(new Date(value));
    } else {
      setInputValue('');
    }
  }, [value]);

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
    let inputVal = e.target.value;

    // Allow clearing the input
    if (inputVal === '') {
      setInputValue('');
      onChange('');
      return;
    }

    // Remove any non-digit characters except dashes
    inputVal = inputVal.replace(/[^\d-]/g, '');

    // Auto-format as DD-MM-YYYY
    if (inputVal.length <= 2) {
      setInputValue(inputVal);
    } else if (inputVal.length <= 4) {
      // Add dash after DD
      if (inputVal.length === 3 && !inputVal.includes('-')) {
        inputVal = inputVal.slice(0, 2) + '-' + inputVal.slice(2);
      }
      setInputValue(inputVal);
    } else if (inputVal.length <= 7) {
      // Add second dash after MM
      if (inputVal.length === 6 && inputVal.indexOf('-') === inputVal.lastIndexOf('-')) {
        inputVal = inputVal.slice(0, 5) + '-' + inputVal.slice(5);
      }
      setInputValue(inputVal);
    } else {
      // Limit to 10 characters (DD-MM-YYYY)
      inputVal = inputVal.slice(0, 10);
      setInputValue(inputVal);
    }

    // Try to parse and validate the date
    if (inputVal.length === 10) {
      const dateRegex = /^(\d{2})-(\d{2})-(\d{4})$/;
      const match = inputVal.match(dateRegex);

      if (match) {
        const [, day, month, year] = match;
        const isoDate = `${year}-${month}-${day}`;
        const date = new Date(isoDate);

        // Check if it's a valid date
        if (!isNaN(date.getTime()) &&
            date.getDate() === parseInt(day) &&
            date.getMonth() === parseInt(month) - 1 &&
            date.getFullYear() === parseInt(year)) {
          onChange(isoDate);
          setCalendarDate(date);
        }
      }
    }
  };

  const handleInputKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    // Allow backspace, delete, arrow keys, tab
    if (['Backspace', 'Delete', 'ArrowLeft', 'ArrowRight', 'Tab'].includes(e.key)) {
      return;
    }

    // Allow only digits
    if (!/\d/.test(e.key)) {
      e.preventDefault();
    }
  };

  const selectedDate = value ? new Date(value) : undefined;

  const currentYear = calendarDate.getFullYear();
  const currentMonth = calendarDate.getMonth();

  const years = Array.from({ length: 100 }, (_, i) => currentYear - 50 + i);
  const months = [
    'January', 'February', 'March', 'April', 'May', 'June',
    'July', 'August', 'September', 'October', 'November', 'December'
  ];

  const handleYearChange = (year: string) => {
    const newDate = new Date(calendarDate);
    newDate.setFullYear(parseInt(year));
    setCalendarDate(newDate);
  };

  const handleMonthChange = (monthIndex: string) => {
    const newDate = new Date(calendarDate);
    newDate.setMonth(parseInt(monthIndex));
    setCalendarDate(newDate);
  };

  const navigateYear = (direction: 'prev' | 'next') => {
    const newDate = new Date(calendarDate);
    newDate.setFullYear(currentYear + (direction === 'next' ? 1 : -1));
    setCalendarDate(newDate);
  };

  const navigateMonth = (direction: 'prev' | 'next') => {
    const newDate = new Date(calendarDate);
    if (direction === 'next') {
      newDate.setMonth(currentMonth + 1);
    } else {
      newDate.setMonth(currentMonth - 1);
    }
    setCalendarDate(newDate);
  };

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
          value={inputValue}
          onChange={handleInputChange}
          onKeyDown={handleInputKeyDown}
          placeholder={placeholder}
          disabled={disabled}
          className="pr-10"
          maxLength={10}
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
            <div className="bg-white rounded-lg shadow-lg border p-3 min-w-[280px]">
              {/* Year and Month Navigation */}
              <div className="flex items-center justify-between mb-4 space-x-2">
                <div className="flex items-center space-x-1">
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() => navigateYear('prev')}
                    className="p-1 h-8 w-8"
                  >
                    <ChevronLeft className="h-4 w-4" />
                  </Button>

                  <Select value={currentYear.toString()} onValueChange={handleYearChange}>
                    <SelectTrigger className="w-20 h-8 text-xs">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent className="max-h-40">
                      {years.map((year) => (
                        <SelectItem key={year} value={year.toString()}>
                          {year}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>

                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() => navigateYear('next')}
                    className="p-1 h-8 w-8"
                  >
                    <ChevronRight className="h-4 w-4" />
                  </Button>
                </div>

                <div className="flex items-center space-x-1">
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() => navigateMonth('prev')}
                    className="p-1 h-8 w-8"
                  >
                    <ChevronLeft className="h-4 w-4" />
                  </Button>

                  <Select value={currentMonth.toString()} onValueChange={handleMonthChange}>
                    <SelectTrigger className="w-24 h-8 text-xs">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {months.map((month, index) => (
                        <SelectItem key={index} value={index.toString()}>
                          {month.slice(0, 3)}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>

                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() => navigateMonth('next')}
                    className="p-1 h-8 w-8"
                  >
                    <ChevronRight className="h-4 w-4" />
                  </Button>
                </div>
              </div>

              {/* Calendar */}
              <Calendar
                mode="single"
                selected={selectedDate}
                onSelect={handleDateSelect}
                disabled={disabled}
                month={calendarDate}
                onMonthChange={setCalendarDate}
                initialFocus
                className="p-0"
              />

              <div className="flex justify-between mt-3 pt-2 border-t">
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={() => {
                    const today = new Date();
                    handleDateSelect(today);
                  }}
                >
                  Today
                </Button>
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

      <div className="text-xs text-slate-500 mt-1">
        Enter date as DD-MM-YYYY or use calendar
      </div>
    </div>
  );
}