"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.default = binaryToDots;
const dictionaries_1 = require("../constants/dictionaries");
function binaryToDots(binaryStr) {
    // Pad binaryStr to be a multiple of 5
    while (binaryStr.length % 5 !== 0) {
        binaryStr = '0' + binaryStr;
    }
    // Split into 5-bit chunks
    const chunks = binaryStr.match(/.{1,5}/g) || [];
    const firstStep = chunks.map(chunk => dictionaries_1.firstDict[chunk] || '').join('');
    // Apply Second Dictionary Encoding
    const secondStep = firstStep.replace(/\.\.\.\.\./g, '!')
        .replace(/\.\.\.\./g, '#')
        .replace(/\.\.\./g, '$')
        .replace(/\.\./g, '%')
        .replace(/\. \./g, '&')
        .replace(/\./g, '*');
    return secondStep;
}
