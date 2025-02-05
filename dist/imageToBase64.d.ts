declare function imageToBase64(imagePath: string): Promise<string>;
declare function base64ToImage(base64String: string, outputPath: string): Promise<void>;
export { imageToBase64, base64ToImage };
