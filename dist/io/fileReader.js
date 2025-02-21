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
Object.defineProperty(exports, "__esModule", { value: true });
const fs_1 = require("fs");
function readFileAsBinary(filePath) {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            // Read the file as a buffer
            const buffer = yield fs_1.promises.readFile(filePath);
            // Convert the buffer to a binary string
            let binaryString = "";
            for (const byte of buffer) {
                binaryString += byte.toString(2).padStart(8, "0");
            }
            // Pad the binary string to a multiple of 5 bits
            const paddingLength = (5 - (binaryString.length % 5)) % 5;
            const paddedBinaryString = binaryString.padEnd(binaryString.length + paddingLength, "0");
            return paddedBinaryString;
        }
        catch (error) {
            // Handle file reading errors
            if (error instanceof Error) {
                throw new Error(`Error reading file: ${error.message}`);
            }
            else {
                throw new Error("An unknown error occurred while reading the file");
            }
        }
    });
}
// Example usage
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const filePath = "C:/Users/USER/Documents/onlydust.txt"; // Use forward slashes or escape backslashes
            const binaryString = yield readFileAsBinary(filePath);
            console.log("Binary string:", binaryString);
        }
        catch (error) {
            console.error(error);
        }
    });
}
main();
