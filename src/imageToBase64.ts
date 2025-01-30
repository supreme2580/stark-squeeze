import { join } from 'path';
import sharp from 'sharp';

async function imageToBase64(imagePath: string): Promise<string> {
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

async function base64ToImage(base64String: string, outputPath: string): Promise<void> {
  try {
    // Remove data URL prefix if present
    const base64Data = base64String.replace(/^data:image\/\w+;base64,/, '');
    
    // Convert base64 to buffer
    const imageBuffer = Buffer.from(base64Data, 'base64');
    
    // Save as PNG using Sharp
    await sharp(imageBuffer)
      .png()
      .toFile(outputPath);
  } catch (error) {
    throw new Error(`Error converting base64 to image: ${error}`);
  }
}

// Remove the example usage section since this will be a module
export { imageToBase64, base64ToImage };