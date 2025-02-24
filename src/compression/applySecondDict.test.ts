import { applySecondDict } from '../compression/applySecondDict';

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
});