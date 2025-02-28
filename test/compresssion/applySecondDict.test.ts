import { applySecondDict } from '../../src/compression/applySecondDict';

describe('applySecondDict', () => {
  it('replaces longest patterns first', () => {
    expect(applySecondDict('.....')).toBe('!'); // 5 dots
    expect(applySecondDict('....')).toBe('#');  // 4 dots
  });

  it('handles overlapping patterns', () => {
    expect(applySecondDict('. ....')).toBe('&$'); // Corrected: removed space between . . and ...
  });

  it('handles empty string', () => {
    expect(applySecondDict('')).toBe('');
  });
  it('returns the same string if no patterns match', () => {
    expect(applySecondDict('ABC123')).toBe('ABC123'); // No replacement
  });

  it('handles mixed matching and non-matching characters', () => {
    expect(applySecondDict('ABC.....XYZ')).toBe('ABC!XYZ'); // Partial match
  });

  it('handles consecutive patterns', () => {
    expect(applySecondDict('..... ....')).toBe('! #'); // Ensures adjacent replacements
  });

  it('handles a single unmatched character', () => {
    expect(applySecondDict('Z')).toBe('Z'); // Should remain unchanged
  });

  it('handles patterns appearing multiple times', () => {
    expect(applySecondDict('..... .....')).toBe('! !'); // Two occurrences of '.....'
  });

  it('handles patterns at the start and end of a string', () => {
    expect(applySecondDict('.....X')).toBe('!X'); // Leading pattern
    expect(applySecondDict('X.....')).toBe('X!'); // Trailing pattern
  });

  it('handles input with multiple pattern variations', () => {
    expect(applySecondDict('.... .. ...')).toBe('# % $'); // Different pattern matches in one string
  });

  it('preserves case sensitivity', () => {
    expect(applySecondDict('AbC')).toBe('AbC'); // Should remain unchanged if not in dictionary
  });
});