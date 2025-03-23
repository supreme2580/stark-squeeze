import binaryToBuffer from '../../src/reconstruction/binaryToBuffer';

describe('binaryToBuffer', () => {
  describe('valid inputs', () => {
    test('converts single byte binary string to buffer', () => {
      const result = binaryToBuffer('10101010');
      expect(Buffer.isBuffer(result)).toBe(true);
      expect(result.length).toBe(1);
      expect(result[0]).toBe(170);
    });

    test('converts multi-byte binary string to buffer', () => {
      const result = binaryToBuffer('1010101011111111');
      expect(Buffer.isBuffer(result)).toBe(true);
      expect(result.length).toBe(2);
      expect(result[0]).toBe(170); 
      expect(result[1]).toBe(255);
    });

    test('handles binary string with length not multiple of 8 by padding', () => {
      const result = binaryToBuffer('1010101');
      expect(Buffer.isBuffer(result)).toBe(true);
      expect(result.length).toBe(1);
      expect(result[0]).toBe(170);
    });

    test('handles minimum valid input (single bit)', () => {
      const result = binaryToBuffer('1');
      expect(Buffer.isBuffer(result)).toBe(true);
      expect(result.length).toBe(1);
      expect(result[0]).toBe(128);
    });

    test('handles all zeros', () => {
      const result = binaryToBuffer('00000000');
      expect(Buffer.isBuffer(result)).toBe(true);
      expect(result.length).toBe(1);
      expect(result[0]).toBe(0);
    });

    test('handles all ones', () => {
      const result = binaryToBuffer('11111111');
      expect(Buffer.isBuffer(result)).toBe(true);
      expect(result.length).toBe(1);
      expect(result[0]).toBe(255);
    });

    test('handles longer sequences requiring multiple bytes', () => {
      const result = binaryToBuffer('101010101111111100000000');
      expect(Buffer.isBuffer(result)).toBe(true);
      expect(result.length).toBe(3);
      expect(result[0]).toBe(170);
      expect(result[1]).toBe(255);
      expect(result[2]).toBe(0);
    });
  });

  describe('invalid inputs', () => {
    test('throws error for empty string', () => {
      expect(() => binaryToBuffer('')).toThrow('Invalid binary string');
    });

    test('throws error for non-binary characters', () => {
      expect(() => binaryToBuffer('1010201')).toThrow('Invalid binary string');
      expect(() => binaryToBuffer('abcdef')).toThrow('Invalid binary string');
      expect(() => binaryToBuffer('1234567')).toThrow('Invalid binary string');
    });

    test('throws error for special characters', () => {
      expect(() => binaryToBuffer('1010!@#')).toThrow('Invalid binary string');
    });

    test('throws error for spaces in input', () => {
      expect(() => binaryToBuffer('1010 1010')).toThrow('Invalid binary string');
    });

    test('throws error for undefined input', () => {
      expect(() => binaryToBuffer(undefined as unknown as string)).toThrow();
    });

    test('throws error for null input', () => {
      expect(() => binaryToBuffer(null as unknown as string)).toThrow();
    });
  });

  describe('edge cases', () => {
    test('handles multiple padding scenarios', () => {
      // Test all possible padding lengths (1 to 7 bits needing padding)
      const testCases = [
        { input: '1', expected: 128 },
        { input: '11', expected: 192 },
        { input: '111', expected: 224 },
        { input: '1111', expected: 240 },
        { input: '11111', expected: 248 },
        { input: '111111', expected: 252 },
        { input: '1111111', expected: 254 },
      ];

      testCases.forEach(({ input, expected }) => {
        const result = binaryToBuffer(input);
        expect(result.length).toBe(1);
        expect(result[0]).toBe(expected);
      });
    });

    test('handles large binary strings', () => {
      // Create a 1000-bit string (125 bytes)
      const longBinaryString = '1'.repeat(1000);
      const result = binaryToBuffer(longBinaryString);
      expect(Buffer.isBuffer(result)).toBe(true);
      expect(result.length).toBe(125);
      // All bytes should be 255 (11111111)
      expect(result.every(byte => byte === 255)).toBe(true);
    });
  });
});
