import { promises as fs } from "fs";

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