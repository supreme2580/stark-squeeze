import { binaryToDots } from './binaryToDots';

describe('binaryToDots', () => {
  it('should convert a binary string to a dot representation string', () => {
    const binaryStr = '1101011101';
    const expected = '.. . ... .';
    expect(binaryToDots(binaryStr)).toBe(expected);
  });

  it('should pad the binary string to be a multiple of 5 bits', () => {
    const binaryStr = '1101';
    const expected = '.. .';
    expect(binaryToDots(binaryStr)).toBe(expected);
  });

  it('should handle an empty binary string', () => {
    const binaryStr = '';
    const expected = '';
    expect(binaryToDots(binaryStr)).toBe(expected);
  });

  it('should throw an error for invalid binary strings', () => {
    const binaryStr = '11012';
    expect(() => binaryToDots(binaryStr)).toThrow('Input must be a binary string.');
  });

  it('should correctly map each 5-bit chunk using firstDict', () => {
    const binaryStr = '0000100010';
    const expected = '. .';
    expect(binaryToDots(binaryStr)).toBe(expected);
  });

  it('should maintain the order of chunks while joining', () => {
    const binaryStr = '1100011001';
    const expected = '.. .. .';
    expect(binaryToDots(binaryStr)).toBe(expected);
  });
});