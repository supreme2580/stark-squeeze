import { GetCIDResponse, PinataSDK, PinResponse } from "pinata-web3";
import { saveDecompressedFile } from "./reconstruction/saveDecompressedFile";

interface UploadOptions {
  file: File | Blob;
  fileName: string;
  fileType: string;
}

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
export async function upload(
  jwt: string,
  gateway: string,
  options: UploadOptions
): Promise<string> {
  try {
    const pinata = new PinataSDK({
      pinataJwt: jwt,
      pinataGateway: gateway,
    });

    const file = options.file instanceof File 
      ? options.file 
      : new File([options.file], options.fileName, { type: options.fileType });

    const upload = await pinata.upload.file(file);
    const url = `https://${gateway}/ipfs/${upload.IpfsHash}`;
    return url;
  } catch (error) {
    console.error(error);
    throw error;
  }
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
export async function getFile(jwt: string, gateway: string, hash: string): Promise<GetCIDResponse> {
    try {
        const pinata = new PinataSDK({
            pinataJwt: jwt,
            pinataGateway: gateway,
        });
        const file = await pinata.gateways.get(hash);
        console.log(file);
        return file;
    } catch (error) {
        console.error(error);
        throw error;
    }
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
export async function uploadFiles(
  jwt: string,
  gateway: string,
  filesArray: File[]
): Promise<PinResponse> {
  try {
    const pinata = new PinataSDK({
      pinataJwt: jwt,
      pinataGateway: gateway,
    });

    const upload = await pinata.upload.fileArray(filesArray);
    return upload;
  } catch (error) {
    console.error(error);
    throw error;
  }
}

