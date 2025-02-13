import { GetCIDResponse, PinataSDK } from "pinata-web3";

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
 * Uploads multiple files to IPFS using Pinata and returns their CIDs
 * 
 * @param jwt - Pinata JWT authentication token
 * @param gateway - IPFS gateway domain (e.g., "gateway.pinata.cloud")
 * @param files - Array of objects containing file, fileName, and fileType
 * 
 * @returns Promise that resolves to an array of CIDs of the uploaded files
 * 
 * @example
 * ```typescript
 * // Upload multiple image files
 * const files = [
 *   { file: new File([blob1], "image1.jpg", { type: "image/jpeg" }), fileName: "image1.jpg", fileType: "image/jpeg" },
 *   { file: new File([blob2], "image2.png", { type: "image/png" }), fileName: "image2.png", fileType: "image/png" }
 * ];
 * const cids = await uploadFiles(
 *   "your-pinata-jwt",
 *   "gateway.pinata.cloud",
 *   files
 * );
 * // Returns: ["QmHash1...", "QmHash2..."]
 * ```
 * 
 * @throws Will throw an error if any of the uploads fail
 */
export async function uploadFiles(
  jwt: string,
  gateway: string,
  files: UploadOptions[]
): Promise<string[]> {
  try {
    const pinata = new PinataSDK({
      pinataJwt: jwt,
      pinataGateway: gateway,
    });

    const cids: string[] = [];

    for (const fileOptions of files) {
      const file = fileOptions.file instanceof File 
        ? fileOptions.file 
        : new File([fileOptions.file], fileOptions.fileName, { type: fileOptions.fileType });

      const upload = await pinata.upload.file(file);
      cids.push(upload.IpfsHash);
    }
    return cids;
  } catch (error) {
    console.error(error);
    throw error;
  }
}
