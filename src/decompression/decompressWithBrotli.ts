import { brotliDecompressSync } from "node:zlib";


export const decompressWithBrotli = (input: Buffer): string => {
    try {
        const decompressedBuffer = brotliDecompressSync(input);
        return decompressedBuffer.toString('utf-8');
    } catch(error: any) {
        throw new Error(`Brotli decompression failed: ${error.message}`);
    }
}