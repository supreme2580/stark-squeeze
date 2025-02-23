import { GetCIDResponse, PinataSDK, PinResponse } from "pinata-web3";
import { Account, Contract, RpcProvider } from "starknet";
import crypto from 'crypto';

interface UploadOptions {
  file: File | Blob;
  fileName: string;
  fileType: string;
  cid_encryption_key?: string;
  should_encrypt?: boolean;
}

/**
 * Encrypts a string using the provided encryption key.
 * 
 * @param {string} text - The text to encrypt.
 * @param {string} key - The encryption key.
 * @returns {string} - The encrypted text.
 */
function encrypt(text: string, key: string): string {
  const iv = crypto.randomBytes(16);
  const cipher = crypto.createCipheriv('aes-256-cbc', Buffer.from(key), iv);
  let encrypted = cipher.update(text);
  encrypted = Buffer.concat([encrypted, cipher.final()]);
  return iv.toString('hex') + ':' + encrypted.toString('hex');
}

/**
 * Decrypts a string using the provided encryption key.
 * 
 * @param {string} text - The text to decrypt.
 * @param {string} key - The encryption key.
 * @returns {string} - The decrypted text.
 */
function decrypt(text: string, key: string): string {
  const textParts = text.split(':');
  const iv = Buffer.from(textParts.shift()!, 'hex');
  const encryptedText = Buffer.from(textParts.join(':'), 'hex');
  const decipher = crypto.createDecipheriv('aes-256-cbc', Buffer.from(key), iv);
  let decrypted = decipher.update(encryptedText);
  decrypted = Buffer.concat([decrypted, decipher.final()]);
  return decrypted.toString();
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
 * @param options.cid_encryption_key - Optional encryption key for the CID
 * @param options.should_encrypt - Optional flag to indicate if encryption should be applied
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
 *     fileType: "image/jpeg",
 *     cid_encryption_key: "your-encryption-key",
 *     should_encrypt: true
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
    let url = `https://${gateway}/ipfs/${upload.IpfsHash}`;

    if (options.should_encrypt && options.cid_encryption_key) {
      url = encrypt(url, options.cid_encryption_key);
    }

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
 * @param cid_encryption_key - Optional encryption key for the CID
 * 
 * @returns Promise that resolves to the GetCIDResponse object containing file details
 * 
 * @example
 * ```typescript
 * // Retrieve a file using its IPFS hash
 * const fileDetails = await getFile(
 *   "your-pinata-jwt",
 *   "gateway.pinata.cloud",
 *   "QmHash...",
 *   "your-encryption-key"
 * );
 * ```
 * 
 * @throws Will throw an error if the file retrieval fails
 */
export async function getFile(jwt: string, gateway: string, hash: string, cid_encryption_key?: string): Promise<GetCIDResponse> {
  try {
    const pinata = new PinataSDK({
      pinataJwt: jwt,
      pinataGateway: gateway,
    });

    if (cid_encryption_key) {
      hash = decrypt(hash, cid_encryption_key);
    }

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
 * @param cid_encryption_key - Optional encryption key for the CIDs
 * @param should_encrypt - Optional flag to indicate if encryption should be applied
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
 *   files,
 *   "your-encryption-key",
 *   true
 * );
 * // Returns: { IpfsHash: "Qm...", PinSize: 123, Timestamp: "2023-10-01T12:00:00Z" }
 * 
 * @throws Will throw an error if the upload fails
 */
export async function uploadFiles(
  jwt: string,
  gateway: string,
  filesArray: File[],
  cid_encryption_key?: string,
  should_encrypt?: boolean
): Promise<PinResponse> {
  try {
    const pinata = new PinataSDK({
      pinataJwt: jwt,
      pinataGateway: gateway,
    });

    const upload = await pinata.upload.fileArray(filesArray);

    if (should_encrypt && cid_encryption_key) {
      upload.IpfsHash = encrypt(upload.IpfsHash, cid_encryption_key);
    }

    return upload;
  } catch (error) {
    console.error(error);
    throw error;
  }
}

/**
 * Calls a contract function to add data, optionally encrypting the CID and name.
 * 
 * @param jwt - Pinata JWT authentication token
 * @param gateway - IPFS gateway domain (e.g., "gateway.pinata.cloud")
 * @param filesArray - Array of File objects to upload
 * @param cid_encryption_key - Optional encryption key for the CIDs
 * @param should_encrypt - Optional flag to indicate if encryption should be applied
 * @param name - Name of the data
 * @param file_type - Type of the file
 * @param file_format - Format of the file
 * 
 * @returns Promise that resolves to the transaction result
 * 
 * @example
 * ```typescript
 * const tx = await contract_call({
 *   jwt: "your-pinata-jwt",
 *   gateway: "gateway.pinata.cloud",
 *   filesArray: [new File(["hello world!"], "hello.txt", { type: "text/plain" })],
 *   cid_encryption_key: "your-encryption-key",
 *   should_encrypt: true,
 *   name: "example",
 *   file_type: "text/plain",
 *   file_format: "txt"
 * });
 * ```
 * 
 * @throws Will throw an error if the contract call fails
 */
export async function contract_call(
  { jwt, gateway, filesArray, cid_encryption_key, should_encrypt, name, file_type, file_format }:
    { jwt: string, gateway: string, filesArray: File[], cid_encryption_key?: string, should_encrypt?: boolean, name: string, file_type: string, file_format: string }
) {
  const provider = new RpcProvider({
    nodeUrl: process.env.STARKNET_SEPOLIA_NODE_URL || '',
  })
  const private_key = process.env.STARKNET_PRIVATE_KEY || '';
  const public_key = process.env.STARKNET_PUBLIC_KEY || '';
  const contract_address = process.env.STARKNET_CONTRACT_ADDRESS || '';
  const account = new Account(provider, public_key, private_key);
  const { abi } = await provider.getClassAt(contract_address);
  if (abi === undefined) {
    throw new Error('no abi found.');
  }
  const contract = new Contract(abi, contract_address, provider);
  contract.connect(account);
  const uploadResponse = await uploadFiles(jwt, gateway, filesArray, cid_encryption_key, should_encrypt);
  const cid = uploadResponse.IpfsHash;
  const encryptedName = should_encrypt ? encrypt(name, cid_encryption_key!) : name;
  const contract_call = contract.populate('add_data', [cid, should_encrypt || false, encryptedName, file_type, file_format]);
  const add_data = await contract.add_data(contract_call.calldata);
  const tx = await provider.waitForTransaction(add_data.transaction_hash);
  return tx;
}