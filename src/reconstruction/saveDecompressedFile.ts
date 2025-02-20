import fs from 'fs';
import * as path from 'path';

/**
 * @param {string} filePath - The path where the file should be saved.
 * @param {Buffer} data - The buffer containing the decompressed data.
 * @returns {Promise<void>} - A promise that resolves when the file is saved.
*/
export async function saveDecompressedFile(filePath: string, data: Buffer): Promise<void> {
    try {
        const outputDir = path.dirname(filePath);
        await fs.promises.mkdir(outputDir, { recursive: true });
        await fs.promises.writeFile(filePath, data);
    } catch (error) {
        console.error(`Error saving file: ${error}`);
        throw error;
    }
}

// saveDecompressedFile('./output.txt', Buffer.from(`Hello, this is a test!`))
