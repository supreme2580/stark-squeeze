import { readFileSync } from 'fs';
import { join } from 'path';

export function imageToBase64(imagePath: string): string {
  try {
    // Read the image file
    const absolutePath = join(__dirname, imagePath);
    const image = readFileSync(absolutePath);
    
    // Convert to base64
    const base64String = image.toString('base64');
    console.log(base64String);
    return base64String;
  } catch (error) {
    throw new Error(`Error converting image to base64: ${error}`);
  }
}

// Example usage
try {
  const base64Image = imageToBase64('../assets/logo.png');
  console.log('Base64 string:', base64Image);
} catch (error) {
  console.error(error);
} 