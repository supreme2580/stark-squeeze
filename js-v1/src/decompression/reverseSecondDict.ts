/**
 * Reverses the encoding applied by a second dictionary.
 *
 * This function takes an encoded string where specific substrings
 * (defined in `secondDict`) have been replaced with unique symbols.
 * It then reverses the transformation by mapping encoded substrings
 * back to their original symbols.
 *
 * @param {string} encodedStr - The encoded string containing transformed symbols.
 * @returns {string} The decoded string with original symbols restored.
 *
 * @example
 * ```
 * const encoded = "..... .... ...";
 * const decoded = reverseSecondDict(encoded);
 * console.log(decoded); // Output: "!*#"
 * ```
 */
export function reverseSecondDict(encodedStr: string): string {
    const secondDict: Record<string, string> = {
      "!": ".....",
      "*": "....",
      "#": "..."
    };
  
    const inverseSecondDict: Record<string, string> = Object.fromEntries(
      Object.entries(secondDict).map(([key, value]) => [value, key])
    );
  
    const sortedKeys = Object.keys(inverseSecondDict).sort((a, b) => b.length - a.length);
  
    let decodedStr = encodedStr;
  
    for (const key of sortedKeys) {
      decodedStr = decodedStr.split(key).join(inverseSecondDict[key]);
    }
  
    return decodedStr;
  }
  
