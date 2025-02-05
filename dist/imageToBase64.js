"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.imageToBase64 = imageToBase64;
exports.base64ToImage = base64ToImage;
const path_1 = require("path");
const sharp_1 = __importDefault(require("sharp"));
async function imageToBase64(imagePath) {
    try {
        const absolutePath = (0, path_1.join)(__dirname, imagePath);
        // Process the image with Sharp
        const processedImageBuffer = await (0, sharp_1.default)(absolutePath)
            .jpeg({
            quality: 100, // Slightly higher quality
            progressive: true,
            mozjpeg: true // Use mozjpeg optimization for better compression
        })
            .toBuffer();
        // Convert to base64
        const base64String = processedImageBuffer.toString('base64');
        return base64String;
    }
    catch (error) {
        throw new Error(`Error converting image to base64: ${error}`);
    }
}
async function base64ToImage(base64String, outputPath) {
    try {
        // Remove data URL prefix if present
        const base64Data = base64String.replace(/^data:image\/\w+;base64,/, '');
        // Convert base64 to buffer
        const imageBuffer = Buffer.from(base64Data, 'base64');
        // Save as PNG using Sharp
        await (0, sharp_1.default)(imageBuffer)
            .png()
            .toFile(outputPath);
    }
    catch (error) {
        throw new Error(`Error converting base64 to image: ${error}`);
    }
}
