import { firstDict } from "../constants/dictionaries";

export default function binaryToDots(binaryStr: string): string {
    // Pad binaryStr to be a multiple of 5
    while (binaryStr.length % 5 !== 0) {
      binaryStr = '0' + binaryStr;
    }
  
    // Split into 5-bit chunks
    const chunks = binaryStr.match(/.{1,5}/g) || [];
    const firstStep = chunks.map(chunk => firstDict[chunk] || '').join('');
  
    // Apply Second Dictionary Encoding
    const secondStep = firstStep.replace(/\.\.\.\.\./g, '!')
      .replace(/\.\.\.\./g, '#')
      .replace(/\.\.\./g, '$')
      .replace(/\.\./g, '%')
      .replace(/\. \./g, '&')
      .replace(/\./g, '*');
  
    return secondStep;
  }