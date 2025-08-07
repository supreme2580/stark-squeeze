import { hash } from "starknet";

/**
 * Event signatures for the contract events
 * Note: These are placeholder signatures and should be calculated from the actual event names
 * using the Starknet event signature calculation method
 */
export const EVENT_SIGNATURES = {
  CompressionMappingStored: hash.getSelectorFromName("contracts::Starksqueeze::CompressionMappingStoredEvent") as `0x${string}`,
  FileUploaded: hash.getSelectorFromName("contracts::Starksqueeze::FileUploadedEvent") as `0x${string}`,
  FileUpdated: hash.getSelectorFromName("contracts::Starksqueeze::FileUpdatedEvent") as `0x${string}`,
  FileDeleted: hash.getSelectorFromName("contracts::Starksqueeze::FileDeletedEvent") as `0x${string}`,
  FileShared: hash.getSelectorFromName("contracts::Starksqueeze::FileSharedEvent") as `0x${string}`,
} as const;

/**
 * Event signature to handler name mapping
 */
export const EVENT_SIGNATURE_TO_TYPE = {
  [EVENT_SIGNATURES.CompressionMappingStored]: "CompressionMappingStored",
  [EVENT_SIGNATURES.FileUploaded]: "FileUploaded",
  [EVENT_SIGNATURES.FileUpdated]: "FileUpdated",
  [EVENT_SIGNATURES.FileDeleted]: "FileDeleted",
  [EVENT_SIGNATURES.FileShared]: "FileShared",
} as const;

export type EventSignature = keyof typeof EVENT_SIGNATURE_TO_TYPE;
