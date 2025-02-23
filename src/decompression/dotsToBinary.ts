import { firstDict } from "../constants/dictionaries";

/**
 * Converts a binary string to a dot-based notation using a two-step encoding process.
 * 
 * @description
 * This function performs a binary-to-dots conversion through two main steps:
 * 1. First Step Encoding:
 *    - Pads the input binary string to ensure it's divisible by 5
 *    - Splits the binary string into 5-bit chunks
 *    - Maps each 5-bit chunk to its corresponding dot pattern using firstDict
 * 
 * 2. Second Step Encoding:
 *    - Replaces specific dot patterns with special characters:
 *      - '.....': Replaced with '!'
 *      - '....': Replaced with '#'
 *      - '...': Replaced with '$'
 *      - '..': Replaced with '%'
 *      - '. .': Replaced with '&'
 *      - '.': Replaced with '*'
 * 
 * @param binaryStr - The input binary string to be converted
 *                   Must contain only '0' and '1' characters
 * 
 * @returns A string containing the dot notation encoded with special characters
 * 
 * @example
 * ```typescript
 * binaryToDots('1010') // Returns the encoded dot pattern
 * binaryToDots('00000') // Returns single special character
 * ```
 * 
 * @throws Will not throw errors but may return unexpected results if:
 * - Input contains characters other than '0' and '1'
 * - firstDict mapping is incomplete or contains invalid entries
 * 
 * @dependencies
 * - Requires firstDict from "../constants/dictionaries"
 * - firstDict should map 5-bit binary strings to their dot patterns
 * 
 * @note
 * - The function automatically pads the input with leading zeros if needed
 * - The output length may be shorter than the input due to pattern compression
 */

export default function binaryToDots(binaryStr: string): string {
    // Pad binaryStr to be a multiple of 5
    while (binaryStr.length % 5 !== 0) {
      binaryStr = '0' + binaryStr;
    }
  
    // Split into 5-bit chunks
    const chunks = binaryStr.match(/.{1,5}/g) || [];
    const firstStep = chunks.map(chunk => firstDict[chunk] || '').join('');
  
    // Apply Second Dictionary Encoding
    const secondStep = firstStep.replace(/\.\.\.\.\./g, '!')
      .replace(/\.\.\.\./g, '#')
      .replace(/\.\.\./g, '$')
      .replace(/\.\./g, '%')
      .replace(/\. \./g, '&')
      .replace(/\./g, '*');
  
    return secondStep;
  }