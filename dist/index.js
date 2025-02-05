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
exports.upload = upload;
exports.getFile = getFile;
const pinata_web3_1 = require("pinata-web3");
/**
 * Uploads a file to IPFS using Pinata and returns the IPFS gateway URL
 *
 * @param jwt - Pinata JWT authentication token
 * @param gateway - IPFS gateway domain (e.g., "gateway.pinata.cloud")
 * @param options - Upload options object
 * @param options.file - File or Blob to upload
 * @param options.fileName - Name of the file
 * @param options.fileType - MIME type of the file (e.g., "image/jpeg")
 *
 * @returns Promise that resolves to the complete IPFS gateway URL of the uploaded file
 *
 * @example
 * ```typescript
 * // Upload an image file
 * const imageFile = new File([blob], "image.jpg", { type: "image/jpeg" });
 * const url = await upload(
 *   "your-pinata-jwt",
 *   "gateway.pinata.cloud",
 *   {
 *     file: imageFile,
 *     fileName: "image.jpg",
 *     fileType: "image/jpeg"
 *   }
 * );
 * // Returns: https://gateway.pinata.cloud/ipfs/Qm...
 * ```
 *
 * @throws Will throw an error if the upload fails
 */
function upload(jwt, gateway, options) {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const pinata = new pinata_web3_1.PinataSDK({
                pinataJwt: jwt,
                pinataGateway: gateway,
            });
            const file = options.file instanceof File
                ? options.file
                : new File([options.file], options.fileName, { type: options.fileType });
            const upload = yield pinata.upload.file(file);
            const url = `https://${gateway}/ipfs/${upload.IpfsHash}`;
            return url;
        }
        catch (error) {
            console.error(error);
            throw error;
        }
    });
}
/**
 * Retrieves a file from IPFS using Pinata gateway
 *
 * @param jwt - Pinata JWT authentication token
 * @param gateway - IPFS gateway domain (e.g., "gateway.pinata.cloud")
 * @param hash - IPFS hash (CID) of the file to retrieve
 *
 * @returns Promise that resolves to the GetCIDResponse object containing file details
 *
 * @example
 * ```typescript
 * // Retrieve a file using its IPFS hash
 * const fileDetails = await getFile(
 *   "your-pinata-jwt",
 *   "gateway.pinata.cloud",
 *   "QmHash..."
 * );
 * ```
 *
 * @throws Will throw an error if the file retrieval fails
 */
function getFile(jwt, gateway, hash) {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const pinata = new pinata_web3_1.PinataSDK({
                pinataJwt: jwt,
                pinataGateway: gateway,
            });
            const file = yield pinata.gateways.get(hash);
            console.log(file);
            return file;
        }
        catch (error) {
            console.error(error);
            throw error;
        }
    });
}
