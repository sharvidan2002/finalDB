export const APP_NAME = 'Forest Office Staff Management';
export const APP_VERSION = '1.0.0';
export const ORGANIZATION = 'Divisional Forest Office - Vavuniya, Sri Lanka';

export const DATE_FORMATS = {
  DISPLAY: 'dd-MM-yyyy',
  ISO: 'yyyy-MM-dd',
  PICKER: 'yyyy-MM-dd',
} as const;

export const RETIREMENT_AGE = 60;

export const VALIDATION_RULES = {
  NIC_OLD_PATTERN: /^\d{9}[VXvx]$/,
  NIC_NEW_PATTERN: /^\d{12}$/,
  EMAIL_PATTERN: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
  PHONE_PATTERN: /^[\d\s\-\+\(\)]+$/,
  APPOINTMENT_NUMBER_PATTERN: /^[A-Z0-9\/\-]+$/,
} as const;

export const IMAGE_CONSTRAINTS = {
  MAX_SIZE: 5 * 1024 * 1024, // 5MB
  ACCEPTED_TYPES: ['image/jpeg', 'image/png', 'image/webp'] as const,
  CROP_ASPECT_RATIO: 3 / 4, // 3:4 aspect ratio for staff photos
  OUTPUT_WIDTH: 240,
  OUTPUT_HEIGHT: 320,
} as const;