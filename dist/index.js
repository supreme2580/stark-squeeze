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
exports.uploadFiles = uploadFiles;
exports.contract_call = contract_call;
const pinata_web3_1 = require("pinata-web3");
const starknet_1 = require("starknet");
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
/**
 * Uploads an array of files to IPFS using Pinata as a folder and returns the IPFS hash and other details
 *
 * @param jwt - Pinata JWT authentication token
 * @param gateway - IPFS gateway domain (e.g., "gateway.pinata.cloud")
 * @param filesArray - Array of File objects to upload as a folder
 *
 * @returns Promise that resolves to the PinResponse object containing the IPFS hash and other details
 *
 * @example
 * typescript
 * // Upload multiple files as a folder
 * const files = [
 *   new File(["hello world!"], "hello.txt", { type: "text/plain" }),
 *   new File(["hello world again!"], "hello2.txt", { type: "text/plain" })
 * ];
 * const response = await uploadFiles(
 *   "your-pinata-jwt",
 *   "gateway.pinata.cloud",
 *   files
 * );
 * // Returns: { IpfsHash: "Qm...", PinSize: 123, Timestamp: "2023-10-01T12:00:00Z" }
 *
 *
 * @throws Will throw an error if the upload fails
 */
function uploadFiles(jwt, gateway, filesArray, encrypted) {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            const pinata = new pinata_web3_1.PinataSDK({
                pinataJwt: jwt,
                pinataGateway: gateway,
            });
            const upload = yield pinata.upload.fileArray(filesArray);
            //todo use encrypted to encrypt the upload (cid) or not
            return upload;
        }
        catch (error) {
            console.error(error);
            throw error;
        }
    });
}
function contract_call(_a) {
    return __awaiter(this, arguments, void 0, function* ({ jwt, gateway, filesArray, encrypted, name, file_type, file_format }) {
        const provider = new starknet_1.RpcProvider({
            nodeUrl: process.env.STARKNET_SEPOLIA_NODE_URL || '',
        });
        const private_key = process.env.STARKNET_PRIVATE_KEY || '';
        const public_key = process.env.STARKNET_PUBLIC_KEY || '';
        const contract_address = process.env.STARKNET_CONTRACT_ADDRESS || '';
        const account = new starknet_1.Account(provider, public_key, private_key);
        const { abi } = yield provider.getClassAt(contract_address);
        if (abi === undefined) {
            throw new Error('no abi found.');
        }
        const contract = new starknet_1.Contract(abi, contract_address, provider);
        contract.connect(account);
        const cid = uploadFiles(jwt, gateway, filesArray, encrypted);
        const contract_call = contract.populate('add_data', [cid, encrypted, name, file_type, file_format]);
        const add_data = yield contract.add_data(contract_call.calldata);
        const tx = yield provider.waitForTransaction(add_data.transaction_hash);
        return tx;
    });
}
