import { brotliCompressSync } from "node:zlib";

/**
 * This function compresses a given string using Brotli compression
 * Compresses a given string using Brotli compression.
 * Converts the input string to a UTF-8 encoded Buffer.
 * Applies Brotli compression using Node.js' `brotliCompressSync`.
 * Returns the compressed data as a Buffer.
 *
 * @param {string} input - The input string to be compressed.
 * @returns {Buffer} - The Brotli-compressed output as a Buffer.
 
 * @example
 
 * ```typescript
 *import compressWithBrotli from "./compressWithBrotli";

  // Example input string
  const inputString = "Hello, World!";

  // Compress the string
  const compressedBuffer = compressWithBrotli(inputString);
  console.log("Compressed Buffer:", compressedBuffer);
  console.log("Compressed Buffer (Base64):", compressedBuffer.toString("base64")); //Output

 * ```
*/

export default function compressWithBrotli(input: string): Buffer {
    const buffer = Buffer.from(input, 'utf-8');
    return brotliCompressSync(buffer);
}