"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.imageToBase64 = imageToBase64;
const path_1 = require("path");
const sharp_1 = __importDefault(require("sharp"));
function imageToBase64(imagePath) {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const absolutePath = (0, path_1.join)(__dirname, imagePath);
            // Process the image with Sharp
            const processedImageBuffer = yield (0, sharp_1.default)(absolutePath)
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
    });
}
// Example usage
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const base64Image = yield imageToBase64('../assets/logo.png');
            console.log('Base64 string:', base64Image);
        }
        catch (error) {
            console.error(error);
        }
    });
}
main();
