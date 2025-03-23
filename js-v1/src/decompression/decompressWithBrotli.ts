import { brotliDecompressSync } from "node:zlib";

/**
 * Decompresses data using the Brotli compression algorithm.
 *
 * This function takes a Buffer containing Brotli-compressed data,
 * decompresses it using Node's built-in brotliDecompressSync function,
 * and returns the result as a UTF-8 encoded string.
 *
 * @param {Buffer} input - The Brotli-compressed data as a Buffer.
 * @returns {string} The decompressed data as a UTF-8 string.
 * @throws {Error} If decompression fails, with details about the failure.
 *
 * @example
 * ```typescript
 * import { decompressWithBrotli } from './compression/decompressWithBrotli';
 * import { readFileSync } from 'fs';
 *
 * const compressedData = readFileSync('compressed.br');
 * const decompressedText = decompressWithBrotli(compressedData);
 * console.log(decompressedText); // Output: "Original uncompressed text"
 * ```
 */

export const decompressWithBrotli = (input: Buffer): string => {
    try {
        const decompressedBuffer = brotliDecompressSync(input);
        return decompressedBuffer.toString('utf-8');
    } catch(error: any) {
        throw new Error(`Brotli decompression failed: ${error.message}`);
    }
}
