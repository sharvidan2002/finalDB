import { VALIDATION_RULES } from './constants';

interface NICInfo {
  isValid: boolean;
  isOldFormat: boolean;
  newFormat?: string;
  oldFormat?: string;
  birthYear?: number;
  dayOfYear?: number;
  gender?: 'Male' | 'Female';
  serial?: string;
  checkDigit?: string;
}

/**
 * Validates if a NIC number is in old or new format
 */
export function validateNIC(nic: string): { isValid: boolean; isOldFormat: boolean } {
  const cleanNIC = nic.trim().toUpperCase();

  if (VALIDATION_RULES.NIC_OLD_PATTERN.test(cleanNIC)) {
    return { isValid: true, isOldFormat: true };
  }

  if (VALIDATION_RULES.NIC_NEW_PATTERN.test(cleanNIC)) {
    return { isValid: true, isOldFormat: false };
  }

  return { isValid: false, isOldFormat: false };
}

/**
 * Converts old format NIC to new format
 * Old format: YY DDD SSS C + letter (V/X)
 * New format: YYYY DDD SSSS C (12 digits, no letter)
 */
export function convertOldToNew(oldNIC: string): string | null {
  const cleanNIC = oldNIC.trim().toUpperCase();

  if (!VALIDATION_RULES.NIC_OLD_PATTERN.test(cleanNIC)) {
    return null;
  }

  // Extract parts: YY DDD SSS C V/X
  const yy = cleanNIC.substring(0, 2);
  const ddd = cleanNIC.substring(2, 5);
  const sss = cleanNIC.substring(5, 8);
  const c = cleanNIC.substring(8, 9);

  // Convert YY to YYYY
  const year = parseInt(yy);
  const fullYear = year >= 0 && year <= 30 ? 2000 + year : 1900 + year;

  // Pad serial to 4 digits (0SSS becomes SSSS with leading zero)
  const ssss = '0' + sss;

  // Combine: YYYY DDD SSSS C
  const newNIC = `${fullYear}${ddd}${ssss}${c}`;

  return newNIC;
}

/**
 * Extracts information from NIC number
 */
export function extractNICInfo(nic: string): NICInfo {
  const cleanNIC = nic.trim().toUpperCase();
  const validation = validateNIC(cleanNIC);

  if (!validation.isValid) {
    return { isValid: false, isOldFormat: false };
  }

  try {
    if (validation.isOldFormat) {
      // Old format: YY DDD SSS C V/X
      const yy = cleanNIC.substring(0, 2);
      const ddd = cleanNIC.substring(2, 5);
      const sss = cleanNIC.substring(5, 8);
      const c = cleanNIC.substring(8, 9);

      const year = parseInt(yy);
      const birthYear = year >= 0 && year <= 30 ? 2000 + year : 1900 + year;
      const dayOfYear = parseInt(ddd);

      // Determine gender (day > 500 means female)
      const actualDayOfYear = dayOfYear > 500 ? dayOfYear - 500 : dayOfYear;
      const gender: 'Male' | 'Female' = dayOfYear > 500 ? 'Female' : 'Male';

      const newFormat = convertOldToNew(cleanNIC);

      return {
        isValid: true,
        isOldFormat: true,
        newFormat: newFormat || undefined,
        oldFormat: cleanNIC,
        birthYear,
        dayOfYear: actualDayOfYear,
        gender,
        serial: sss,
        checkDigit: c,
      };
    } else {
      // New format: YYYY DDD SSSS C
      const yyyy = cleanNIC.substring(0, 4);
      const ddd = cleanNIC.substring(4, 7);
      const ssss = cleanNIC.substring(7, 11);
      const c = cleanNIC.substring(11, 12);

      const birthYear = parseInt(yyyy);
      const dayOfYear = parseInt(ddd);

      // Determine gender (day > 500 means female)
      const actualDayOfYear = dayOfYear > 500 ? dayOfYear - 500 : dayOfYear;
      const gender: 'Male' | 'Female' = dayOfYear > 500 ? 'Female' : 'Male';

      // Convert to old format for reference
      const shortYear = birthYear.toString().substring(2, 4);
      const oldSerial = ssss.substring(1, 4); // Remove leading zero
      const oldFormat = `${shortYear}${ddd}${oldSerial}${c}V`;

      return {
        isValid: true,
        isOldFormat: false,
        newFormat: cleanNIC,
        oldFormat,
        birthYear,
        dayOfYear: actualDayOfYear,
        gender,
        serial: ssss,
        checkDigit: c,
      };
    }
  } catch (error) {
    return { isValid: false, isOldFormat: false };
  }
}

/**
 * Auto-converts NIC to new format if it's in old format
 */
export function normalizeNIC(nic: string): { newFormat: string; oldFormat?: string } {
  const info = extractNICInfo(nic);

  if (!info.isValid) {
    throw new Error('Invalid NIC number');
  }

  if (info.isOldFormat && info.newFormat) {
    return {
      newFormat: info.newFormat,
      oldFormat: info.oldFormat,
    };
  } else {
    return {
      newFormat: info.newFormat!,
      oldFormat: info.oldFormat,
    };
  }
}

/**
 * Calculates age from date of birth
 */
export function calculateAge(dateOfBirth: string): number {
  const today = new Date();
  const birth = new Date(dateOfBirth);

  let age = today.getFullYear() - birth.getFullYear();
  const monthDiff = today.getMonth() - birth.getMonth();

  if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birth.getDate())) {
    age--;
  }

  return age;
}

/**
 * Calculates retirement date (age 60)
 */
export function calculateRetirementDate(dateOfBirth: string): string {
  const birth = new Date(dateOfBirth);
  const retirement = new Date(birth.getFullYear() + 60, birth.getMonth(), birth.getDate());

  return retirement.toISOString().split('T')[0]; // Return YYYY-MM-DD format
}