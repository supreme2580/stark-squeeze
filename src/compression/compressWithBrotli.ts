import { brotliCompressSync } from "node:zlib";

export default function compressWithBrotli(input: string): Buffer {
    const buffer = Buffer.from(input, 'utf-8');
    return brotliCompressSync(buffer);
}