import binaryToDots from '../../src/decompression/dotsToBinary';
import { firstDict } from '../../src/constants/dictionaries';

describe('binaryToDots', () => {
  // Test padding functionality
  test('pads binary string to multiple of 5', () => {
    expect(binaryToDots('1')).toBe('*');     // '1' -> '00001' -> '.' -> '*'
    expect(binaryToDots('11')).toBe('%');    // '11' -> '00011' -> '..' -> '%'
    expect(binaryToDots('111')).toBe('$');   // '111' -> '00111' -> '...' -> '$'
  });

  // Test basic conversions using firstDict
  test('converts basic 5-bit patterns correctly', () => {
    expect(binaryToDots('00001')).toBe('*');  // '.' -> '*'
    expect(binaryToDots('00011')).toBe('%');  // '..' -> '%'
    expect(binaryToDots('00111')).toBe('$');  // '...' -> '$'
    expect(binaryToDots('01111')).toBe('#');  // '....' -> '#'
    expect(binaryToDots('11111')).toBe('!');  // '.....' -> '!'
  });

  // Test space handling
  test('handles spaces in patterns correctly', () => {
    expect(binaryToDots('00101')).toBe('&');    // '. .' -> '&'
    expect(binaryToDots('10101')).toBe('& *');  // '. . .' -> '& *'
  });

  // Test multiple chunks
  test('handles multiple 5-bit chunks correctly', () => {
    expect(binaryToDots('0000100001')).toBe('%');    // '00001' + '00001' -> '.' + '.' -> '..' -> '%'
    expect(binaryToDots('1111111111')).toBe('!!');    // '11111' + '11111' -> '.....' + '.....' -> '!!'
  });

  // Test edge cases
  test('handles edge cases correctly', () => {
    expect(binaryToDots('')).toBe('');  // Empty string
    expect(binaryToDots('00000')).toBe('');  // All zeros
    expect(binaryToDots('000000000000000')).toBe('');  // Multiple chunks of zeros
  });

  // Test second dictionary replacements
  test('applies replacement patterns correctly', () => {
    const testCases = [
      ['11111', '!'],      // '.....' -> '!'
      ['01111', '#'],      // '....' -> '#'
      ['00111', '$'],      // '...' -> '$'
      ['00011', '%'],      // '..' -> '%'
      ['00101', '&'],      // '. .' -> '&'
      ['00001', '*']       // '.' -> '*'
    ];

    testCases.forEach(([input, expected]) => {
      expect(binaryToDots(input)).toBe(expected);
    });
  });
});
