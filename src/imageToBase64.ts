import { join } from 'path';
import sharp from 'sharp';

export async function imageToBase64(imagePath: string): Promise<string> {
  try {
    const absolutePath = join(__dirname, imagePath);
    
    // Process the image with Sharp
    const processedImageBuffer = await sharp(absolutePath)
      .resize(300, 300, { // Adjust dimensions as needed
        fit: 'inside',
        withoutEnlargement: true
      })
      .jpeg({ // Convert to JPEG with compression
        quality: 80, // Adjust quality (0-100)
        progressive: true
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