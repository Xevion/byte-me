/**
 * Formats a number of bytes into a human-readable string using binary units (KiB, MiB, GiB, TiB).
 *
 * - For values less than 1024, returns the value in bytes (e.g., "512 B").
 * - For larger values, converts to the appropriate unit and formats:
 *   - If the integer part is 1000 or more, shows no decimal (e.g., "1024 KiB").
 *   - If the decimal part is at least 0.1, shows one decimal place (e.g., 1228.8 KiB formats as "1.2 MiB").
 *   - Otherwise, shows no decimal (e.g., 1075.2 MiB == 1.05 GiB, formats as "1 GiB").
 *
 * @param v - The number of bytes to format.
 * @returns The formatted string with the appropriate unit.
 */
export function formatBytes(v: number): string {
  if (v < 1024) return `${v} B`;

  const units = ["KiB", "MiB", "GiB", "TiB"];
  let unitIndex = -1;
  let value = v;

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex++;
  }

  const intValue = Math.floor(value);
  const decimal = value - intValue;

  if (intValue >= 1000) {
    // More than 3 digits, no decimal
    return `${intValue} ${units[unitIndex]}`;
  } else if (decimal >= 0.1) {
    // Show 1 decimal if decimal >= 0.1
    return `${value.toFixed(1)} ${units[unitIndex]}`;
  } else {
    // No decimal
    return `${intValue} ${units[unitIndex]}`;
  }
}
