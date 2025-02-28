import { brotliCompressSync } from "node:zlib";
import { decompressWithBrotli } from "../../src/decompression/decompressWithBrotli"

// Helper function to compress data for testing
const compressWithBrotli = (input: string): Buffer => {
    return brotliCompressSync(Buffer.from(input, "utf-8"));
};

describe("decompressWithBrotli", () => {
    test("should correctly decompress a valid Brotli-compressed string", () => {
        const originalText = "Hello, Brotli!";
        const compressedData = compressWithBrotli(originalText);
        
        const result = decompressWithBrotli(compressedData);
        
        expect(result).toBe(originalText);
    });

    test("should throw an error for invalid compressed data", () => {
        const invalidData = Buffer.from("invalid data");
        
        expect(() => decompressWithBrotli(invalidData)).toThrow("Brotli decompression failed");
    });

    test("should throw an error for an empty buffer", () => {
        const emptyBuffer = Buffer.alloc(0);
        
        expect(() => decompressWithBrotli(emptyBuffer)).toThrow("Brotli decompression failed");
    });
});
