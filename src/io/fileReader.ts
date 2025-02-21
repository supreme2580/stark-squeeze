import { promises as fs } from "fs";

/**
 * Reads a file and converts its contents to a binary string.
 * 
 * This function reads the specified file as a buffer, converts each byte of the buffer
 * to its binary representation, and pads the resulting binary string to ensure its length
 * is a multiple of 5 bits.
 * 
 * @param {string} filePath - The path to the file to be read.
 * @returns {Promise<string>} - A promise that resolves to the padded binary string representation of the file contents.
 * 
 * @throws {Error} - Throws an error if the file cannot be read.
 * 
 * @example
 * const filePath = "path/to/your/file.txt";
 * readFileAsBinary(filePath)
 *   .then(binaryString => console.log("Binary string:", binaryString))
 *   .catch(error => console.error(error));
 */

async function readFileAsBinary(filePath: string): Promise<string> {
  try {
    // Read the file as a buffer
    const buffer: Buffer = await fs.readFile(filePath);

    // Convert the buffer to a binary string
    let binaryString = "";
    for (const byte of buffer) {
      binaryString += byte.toString(2).padStart(8, "0");
    }

    // Pad the binary string to a multiple of 5 bits
    const paddingLength: number = (5 - (binaryString.length % 5)) % 5;
    const paddedBinaryString: string = binaryString.padEnd(binaryString.length + paddingLength, "0");

    return paddedBinaryString;
  } catch (error) {
    // Handle file reading errors
    if (error instanceof Error) {
      throw new Error(`Error reading file: ${error.message}`);
    } else {
      throw new Error("An unknown error occurred while reading the file");
    }
  }
}

// Example usage
async function main() {
  try {
    const filePath = "path/to/your/file.txt"; // Use forward slashes or escape backslashes
    const binaryString = await readFileAsBinary(filePath);
    console.log("Binary string:", binaryString);
  } catch (error) {
    console.error(error);
  }
}

main();