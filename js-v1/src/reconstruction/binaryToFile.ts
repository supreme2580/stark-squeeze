import fs from 'fs';

// Define some colors for the terminal output
const COLORS = {
  GREEN: '\x1b[32m', // Green for the progress bar
  CYAN: '\x1b[36m',  // Cyan for percentages
  RESET: '\x1b[0m'   // Reset to default terminal color
};

/**
 * Takes a binary string (like '10101010...') and saves it to a file,
 * showing a progress bar in the terminal as it writes.
 *
 * @param binaryString - The string of 0s and 1s to save
 * @param filePath - Where to save the file
 * @param chunkSize - How many bytes to write at a time (default is 1024)
 */
export function binary_to_file(binaryString: string, filePath: string, chunkSize: number = 1024): void {
  // Make sure the input only has 0s and 1s
  if (!/^[01]+$/.test(binaryString)) {
    throw new Error("Oops! The binary string should only contain 0s and 1s.");
  }

  // Pad the string with zeros if it’s not a multiple of 8 (since 1 byte = 8 bits)
  const bitsToAdd = (8 - (binaryString.length % 8)) % 8;
  if (bitsToAdd > 0) {
    binaryString += "0".repeat(bitsToAdd);
  }

  const totalBytes = binaryString.length / 8;
  let bytesWritten = 0;

  // Open a stream to write the file
  const fileStream = fs.createWriteStream(filePath, { flags: 'w' });
  const progressBarWidth = 50; 
  const updateEveryMs = 100;   
  let lastUpdateTime = Date.now();

  // Break the binary string into manageable chunks
  for (let start = 0; start < binaryString.length; start += chunkSize * 8) {
    // Get the next chunk of bits (up to chunkSize bytes worth)
    const end = Math.min(start + chunkSize * 8, binaryString.length);
    const binaryChunk = binaryString.substring(start, end);

    // Convert this chunk from binary to bytes
    const chunkBytes: number[] = [];
    for (let bitIndex = 0; bitIndex < binaryChunk.length; bitIndex += 8) {
      const byteString = binaryChunk.substring(bitIndex, bitIndex + 8);
      const byteValue = parseInt(byteString, 2); // Convert '10101010' to a number
      chunkBytes.push(byteValue);
    }

    // Turn the bytes into a buffer and write it to the file
    const bufferChunk = Buffer.from(chunkBytes);
    fileStream.write(bufferChunk);
    bytesWritten += bufferChunk.length;

    // Show progress, but not too often (to keep it smooth)
    const currentTime = Date.now();
    if (currentTime - lastUpdateTime >= updateEveryMs) {
      lastUpdateTime = currentTime;
      showProgress(bytesWritten, totalBytes, progressBarWidth);
    }
  }

  // Show the final progress (100%) and clean up
  showProgress(bytesWritten, totalBytes, progressBarWidth);
  console.log(); // Add a new line after the progress bar
  fileStream.end(); // Close the file
}

/**
 * Shows a cool progress bar in the terminal with percentage and size info.
 *
 * @param bytesWritten - How many bytes we’ve written so far
 * @param totalBytes - The total number of bytes to write
 * @param barWidth - How many characters wide the progress bar should be
 */
function showProgress(bytesWritten: number, totalBytes: number, barWidth: number): void {
  const percentageDone = (bytesWritten / totalBytes) * 100;
  const filledBarLength = Math.floor((bytesWritten / totalBytes) * barWidth);
  const emptyBarLength = barWidth - filledBarLength;

  const progressBar = `${COLORS.GREEN}${'█'.repeat(filledBarLength)}${COLORS.RESET}${'-'.repeat(emptyBarLength)}`;
  const writtenKB = (bytesWritten / 1024).toFixed(2);
  const totalKB = (totalBytes / 1024).toFixed(2);
  const sizeDisplay = `${writtenKB} KB / ${totalKB} KB`;

  const percentageDisplay = `${COLORS.CYAN}${percentageDone.toFixed(1)}%${COLORS.RESET}`;

  process.stdout.write(`\rProgress: [${progressBar}] ${percentageDisplay} | ${sizeDisplay}`);
}

// Example usage:
// binary_to_file('1010101011110000', 'output.bin');