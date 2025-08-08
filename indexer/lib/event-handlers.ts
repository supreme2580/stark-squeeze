import type { Event } from "@apibara/starknet";
import { decodeEvent } from "@apibara/starknet";
import { starksqueezeAbi } from "./contract-abi";
import type { ConsolaInstance } from "apibara/plugins";
import { PgQueryResultHKT, PgTransaction } from "drizzle-orm/pg-core";
import { ExtractTablesWithRelations } from "drizzle-orm";
import * as schema from "./schema";

export interface EventHandlerParams {
  event: Event;
  blockNumber: bigint;
  transactionHash: string;
  database: PgTransaction<PgQueryResultHKT, Record<string, never>, ExtractTablesWithRelations<Record<string, never>>>;
  logger: ConsolaInstance;
}

/**
 * Handles CompressionMappingStored events using decodeEvent
 */
export async function handleCompressionMappingStored({
  event,
  blockNumber,
  transactionHash,
  database,
  logger
}: EventHandlerParams): Promise<void> {
  const decodedEvent = decodeEvent({
    abi: starksqueezeAbi,
    eventName: "contracts::Starksqueeze::CompressionMappingStoredEvent",
    event: event,
  });

  const {
    uri,
    file_format,
    compressed_by,
    original_size,
    final_size,
    chunk_size,
    chunk_mappings,
    chunk_values,
    byte_mappings,
    byte_values,
    reconstruction_steps,
    metadata
  } = decodedEvent.args;

  await database.insert(schema.compressionMappingTable).values({
    uri: String(uri),
    fileFormat: String(file_format),
    compressedBy: Number(compressed_by),
    originalSize: Number(original_size),
    finalSize: Number(final_size),
    chunkSize: Number(chunk_size),
    chunkMappings: JSON.stringify(chunk_mappings),
    chunkValues: JSON.stringify(chunk_values),
    byteMappings: JSON.stringify(byte_mappings),
    byteValues: JSON.stringify(byte_values),
    reconstructionSteps: JSON.stringify(reconstruction_steps),
    metadata: JSON.stringify(metadata),
    blockNumber: Number(blockNumber),
    transactionHash,
  });

  logger.info(`Indexed CompressionMappingStored event for URI: ${String(uri)} with format: ${String(file_format)} (sizes: ${Number(original_size)} -> ${Number(final_size)}, chunk: ${Number(chunk_size)})`);
}

/**
 * Handles FileUploaded events using decodeEvent
 */
export async function handleFileUploaded({
  event,
  blockNumber,
  transactionHash,
  database,
  logger
}: EventHandlerParams): Promise<void> {
  const decodedEvent = decodeEvent({
    abi: starksqueezeAbi,
    eventName: "contracts::Starksqueeze::FileUploadedEvent",
    event: event,
  });

  const { uri, owner, visibility } = decodedEvent.args;

  await database.insert(schema.fileUploadedTable).values({
    uri: String(uri),
    owner: String(owner),
    visibility: Number(visibility),
    blockNumber: Number(blockNumber),
    transactionHash,
  });

  logger.info(`Indexed FileUploaded event for URI: ${String(uri)} by owner: ${String(owner)}`);
}

/**
 * Handles FileUpdated events using decodeEvent
 */
export async function handleFileUpdated({
  event,
  blockNumber,
  transactionHash,
  database,
  logger
}: EventHandlerParams): Promise<void> {
  const decodedEvent = decodeEvent({
    abi: starksqueezeAbi,
    eventName: "contracts::Starksqueeze::FileUpdatedEvent",
    event: event,
  });

  const { uri, owner } = decodedEvent.args;

  await database.insert(schema.fileUpdatedTable).values({
    uri: String(uri),
    owner: String(owner),
    blockNumber: Number(blockNumber),
    transactionHash,
  });

  logger.info(`Indexed FileUpdated event for URI: ${String(uri)} by owner: ${String(owner)}`);
}

/**
 * Handles FileDeleted events using decodeEvent
 */
export async function handleFileDeleted({
  event,
  blockNumber,
  transactionHash,
  database,
  logger
}: EventHandlerParams): Promise<void> {
  const decodedEvent = decodeEvent({
    abi: starksqueezeAbi,
    eventName: "contracts::Starksqueeze::FileDeletedEvent",
    event: event,
  });

  const { uri, owner } = decodedEvent.args;

  await database.insert(schema.fileDeletedTable).values({
    uri: String(uri),
    owner: String(owner),
    blockNumber: Number(blockNumber),
    transactionHash,
  });

  logger.info(`Indexed FileDeleted event for URI: ${String(uri)} by owner: ${String(owner)}`);
}

/**
 * Handles FileShared events using decodeEvent
 */
export async function handleFileShared({
  event,
  blockNumber,
  transactionHash,
  database,
  logger
}: EventHandlerParams): Promise<void> {
  const decodedEvent = decodeEvent({
    abi: starksqueezeAbi,
    eventName: "contracts::Starksqueeze::FileSharedEvent",
    event: event,
  });

  const { uri, owner, shared_with } = decodedEvent.args;

  await database.insert(schema.fileSharedTable).values({
    uri: String(uri),
    owner: String(owner),
    sharedWith: String(shared_with),
    blockNumber: Number(blockNumber),
    transactionHash,
  });

  logger.info(`Indexed FileShared event for URI: ${String(uri)} shared with: ${String(shared_with)}`);
}

/**
 * Event handler mapping for easy dispatch
 */
export const eventHandlers = {
  CompressionMappingStored: handleCompressionMappingStored,
  FileUploaded: handleFileUploaded,
  FileUpdated: handleFileUpdated,
  FileDeleted: handleFileDeleted,
  FileShared: handleFileShared,
} as const;

export type EventType = keyof typeof eventHandlers;
