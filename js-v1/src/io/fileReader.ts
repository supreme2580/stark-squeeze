import { promises as fs } from "fs";
import { ProgressBar } from "./progressBar";

/**
 * Reads a file and converts its contents to a binary string.
 * 
 * This function takes the path to a file, reads the file as a buffer, and then converts each byte of the buffer
 * into its binary representation. It ensures that the resulting binary string's length is a multiple of 5 bits
 * by padding it with zeros if necessary.
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

export async function readFileAsBinary(filePath: string): Promise<string> {
  try {
    // Read the file as a buffer
    const buffer: Buffer = await fs.readFile(filePath);
    
    // Create progress bar
    const progressBar = new ProgressBar(buffer.length);
    
    // Convert the buffer to a binary string with progress updates
    const binaryString = Array.from(buffer, (byte, index) => {
      progressBar.update(index + 1);
      return byte.toString(2).padStart(8, "0");
    }).join("");

    // Pad the binary string to a multiple of 5 bits
    const paddedBinaryString: string = binaryString.padEnd(binaryString.length + (5 - (binaryString.length % 5)) % 5, "0");
    
    // Finish progress bar
    progressBar.finish();
    
    return paddedBinaryString;
  } catch (error) {
    // Handle file reading errors
    const errorMessage = error instanceof Error ? error.message : "Unknown error";
    throw new Error(`Error reading file: ${errorMessage}`);
  }
}