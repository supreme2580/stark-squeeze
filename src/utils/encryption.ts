import { randomBytes, createCipheriv, createDecipheriv } from 'crypto';

/**
 * Encrypts a string using the provided encryption key.
 * 
 * @param {string} text - The text to encrypt.
 * @param {string} key - The encryption key.
 * @returns {string} - The encrypted text.
 */
export function encrypt(text: string, key: string): string {
  const iv = randomBytes(16);
  const cipher = createCipheriv('aes-256-cbc', Buffer.from(key, 'hex'), iv);
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

export function decrypt(text: string, key: string): string {
  const textParts = text.split(':');
  const iv = Buffer.from(textParts.shift()!, 'hex');
  const encryptedText = Buffer.from(textParts.join(':'), 'hex');
  const decipher = createDecipheriv('aes-256-cbc', Buffer.from(key, 'hex'), iv);
  let decrypted = decipher.update(encryptedText);
  decrypted = Buffer.concat([decrypted, decipher.final()]);
  return decrypted.toString();
}