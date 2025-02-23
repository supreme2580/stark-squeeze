import { firstDict } from "../constants/dictionaries";

export function binaryToDots(binaryStr: string): string {
  // Validate input
  if (!/^[01]*$/.test(binaryStr)) {
    throw new Error('Input must be a binary string.');
  }

  // Pad binaryStr to be a multiple of 5
  const paddingLength = (5 - (binaryStr.length % 5)) % 5;
  const paddedBinaryStr = binaryStr.padEnd(binaryStr.length + paddingLength, '0');

  // Split into 5-bit chunks
  const chunks = paddedBinaryStr.match(/.{1,5}/g) || [];

  // Map each chunk using firstDict
  const mappedChunks = chunks.map(chunk => firstDict[chunk] || '');

  // Join the mapped patterns into a single encoded string
  return mappedChunks.join(' ');
}