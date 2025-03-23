import { firstDict } from "../../src/constants/dictionaries";

/**
 * Converts a dot representation string back to a binary string.
 *
 * This function splits the dot representation into mapped patterns,
 * finds the corresponding binary chunk using the reverse of `firstDict`,
 * and joins them into a single binary string.
 *
 * @param {string} dotsStr - The dot representation string to convert.
 * @returns {string} The binary string.
 * @throws {Error} If the input contains unknown patterns.
 *
 * @example
 * ```typescript
 * import { dotsToBinary } from './compression/dotsToBinary';
 *
 * const dotsStr = ".. . . . .";
 * const binaryStr = dotsToBinary(dotsStr);
 * console.log(binaryStr); // Output: "1101011101"
 * ```
 */
export function dotsToBinary(dotsStr: string): string {
  // Create a reverse mapping of firstDict
  const reverseDict: Record<string, string> = Object.fromEntries(
    Object.entries(firstDict).map(([key, value]) => [value, key])
  );

  // Split input string into chunks
  const chunks = dotsStr.split(' ');

  // Map each chunk using reverseDict
  const binaryChunks = chunks.map(chunk => {
    if (!(chunk in reverseDict)) {
      throw new Error(`Unknown pattern: ${chunk}`);
    }
    return reverseDict[chunk];
  });

  // Join the binary chunks into a single string and remove padding zeros
  return binaryChunks.join('').replace(/0+$/, '');
}
