"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const applySecondDict_1 = require("../../src/compression/v1/applySecondDict");
describe('applySecondDict', () => {
    it('replaces longest patterns first', () => {
        expect((0, applySecondDict_1.applySecondDict)('.....')).toBe('!'); // 5 dots
        expect((0, applySecondDict_1.applySecondDict)('....')).toBe('#'); // 4 dots
    });
    it('handles overlapping patterns', () => {
        expect((0, applySecondDict_1.applySecondDict)('. ....')).toBe('&$'); // Corrected: removed space between . . and ...
    });
    it('handles empty string', () => {
        expect((0, applySecondDict_1.applySecondDict)('')).toBe('');
    });
});
