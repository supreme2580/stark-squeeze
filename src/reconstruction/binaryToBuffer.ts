export default function binaryToBuffer(binaryStr: string): Buffer {
    // Error handling for invalid input
    if (!/^[01]+$/.test(binaryStr)) {
      throw new Error("Invalid binary string: must contain only 0s and 1s.");
    }
  
    // Pad with zeros if necessary
    const paddingLength = 8 - (binaryStr.length % 8);
    if (paddingLength !== 8) {
      binaryStr += "0".repeat(paddingLength);
    }
  
    // Split into 8-bit chunks
    const chunks = [];
    for (let i = 0; i < binaryStr.length; i += 8) {
      chunks.push(binaryStr.substring(i, i + 8));
    }
  
    // Convert each chunk to a byte
    const bytes = chunks.map(chunk => {
      const byte = parseInt(chunk, 2);
      if (isNaN(byte)) {
         throw new Error(`Invalid binary chunk: ${chunk}`); // More specific error
      }
      return byte;
    });
    // Construct a buffer from the byte array
    return Buffer.from(bytes);
  }
