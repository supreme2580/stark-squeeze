import { firstDict } from "../constants/dictionaries";

/**
 * Converts a binary string to a dot representation string.
 *
 * This function pads the binary string to be a multiple of 5 bits,
 * splits it into 5-bit chunks, maps each chunk using the provided
 * firstDict object, and joins the mapped patterns into a single
 * encoded string.
 *
 * @param {string} binaryStr - The binary string to convert.
 * @returns {string} The dot representation of the binary string.
 * @throws {Error} If the input is not a valid binary string.
 *
 * @example
 * ```typescript
 * import { binaryToDots } from './compression/binaryToDots';
 *
 * const binaryStr = '1101011101';
 * const encodedStr = binaryToDots(binaryStr);
 * console.log(encodedStr); // Output: ".. . . . ."
 * ```
 */
export function binaryToDots(binaryStr: string): string {
  // Validate input
  if (!/^[01]*$/.test(binaryStr)) {
    throw new Error('Input must be a binary string.');
  }

  // Pad binaryStr to be a multiple of 5
  const paddingLength = (5 - (binaryStr.length % 5)) % 5;
  const paddedBinaryStr = binaryStr.padEnd(binaryStr.length + paddingLength, '0');

  // Split into 5-bit chunks
  const chunks = paddedBinaryStr.match(/.{1,5}/g) || [];

  // Map each chunk using firstDict
  const mappedChunks = chunks.map(chunk => firstDict[chunk] || '');

  // Join the mapped patterns into a single encoded string
  return mappedChunks.join(' ');
}