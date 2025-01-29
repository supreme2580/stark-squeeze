import { join } from 'path';
import sharp from 'sharp';

export async function imageToBase64(imagePath: string): Promise<string> {
  try {
    const absolutePath = join(__dirname, imagePath);
    
    // Process the image with Sharp
    const processedImageBuffer = await sharp(absolutePath)
      .jpeg({ 
        quality: 100, // Slightly higher quality
        progressive: true,
        mozjpeg: true // Use mozjpeg optimization for better compression
      })
      .toBuffer();
    
    // Convert to base64
    const base64String = processedImageBuffer.toString('base64');
    return base64String;
  } catch (error) {
    throw new Error(`Error converting image to base64: ${error}`);
  }
}

// Example usage
async function main() {
  try {
    const base64Image = await imageToBase64('../assets/logo.png');
    console.log('Base64 string:', base64Image);
  } catch (error) {
    console.error(error);
  }
}

main(); 