/**
 * Converts a binary string into a Buffer.
 *
 * - Splits the binary string into 8-bit chunks.
 * - Converts each chunk to a byte.
 * - Constructs a buffer from the byte array.
 * - Ensures the binary string length is a multiple of 8 (pads with zeros if necessary).
 * - Includes error handling for invalid binary input.
 *
 * @param {string} binaryStr - The binary string to convert (only contains '0' and '1').
 * @returns {Buffer} - A buffer containing the converted bytes.
 * @throws {Error} - Throws an error if the input contains non-binary characters.
 */
export default function binaryToBuffer(binaryStr: string): Buffer {
  // Error handling for invalid input
  if (!/^[01]+$/.test(binaryStr)) {
      throw new Error("Invalid binary string: must contain only 0s and 1s.");
  }

  // Pad with zeros if necessary to ensure length is a multiple of 8
  const paddingLength = 8 - (binaryStr.length % 8);
  if (paddingLength !== 8) {
      binaryStr += "0".repeat(paddingLength);
  }

  // Split into 8-bit chunks
  const chunks = [];
  for (let i = 0; i < binaryStr.length; i += 8) {
      chunks.push(binaryStr.substring(i, i + 8));
  }

  // Convert each chunk to a byte
  const bytes = chunks.map(chunk => {
      const byte = parseInt(chunk, 2);
      if (isNaN(byte)) {
          throw new Error(`Invalid binary chunk: ${chunk}`); // More specific error
      }
      return byte;
  });

  // Construct a buffer from the byte array
  return Buffer.from(bytes);
}
