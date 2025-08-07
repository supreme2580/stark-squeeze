import { bigint, pgTable, text, uuid, integer, timestamp, jsonb } from "drizzle-orm/pg-core";

// Cursor table for tracking indexer progress
export const cursorTable = pgTable("cursor_table", {
  id: uuid("id").primaryKey().defaultRandom(),
  endCursor: bigint("end_cursor", { mode: "number" }),
  uniqueKey: text("unique_key"),
});

// CompressionMappingStored events
export const compressionMappingTable = pgTable("compression_mappings", {
  id: uuid("id").primaryKey().defaultRandom(),
  uri: text("uri").notNull(),
  fileFormat: text("file_format").notNull(),
  compressedBy: integer("compressed_by").notNull(),
  originalSize: bigint("original_size", { mode: "number" }).notNull(),
  finalSize: bigint("final_size", { mode: "number" }).notNull(),
  chunkSize: bigint("chunk_size", { mode: "number" }).notNull(),
  chunkMappings: jsonb("chunk_mappings").notNull(),
  chunkValues: jsonb("chunk_values").notNull(),
  byteMappings: jsonb("byte_mappings").notNull(),
  byteValues: jsonb("byte_values").notNull(),
  reconstructionSteps: jsonb("reconstruction_steps").notNull(),
  metadata: jsonb("metadata").notNull(),
  blockNumber: bigint("block_number", { mode: "number" }).notNull(),
  transactionHash: text("transaction_hash").notNull(),
  createdAt: timestamp("created_at").defaultNow().notNull(),
});

// FileUploaded events
export const fileUploadedTable = pgTable("file_uploaded", {
  id: uuid("id").primaryKey().defaultRandom(),
  uri: text("uri").notNull(),
  owner: text("owner").notNull(),
  visibility: integer("visibility").notNull(), // 0=Public, 1=Private, 2=Shared
  blockNumber: bigint("block_number", { mode: "number" }).notNull(),
  transactionHash: text("transaction_hash").notNull(),
  createdAt: timestamp("created_at").defaultNow().notNull(),
});

// FileUpdated events
export const fileUpdatedTable = pgTable("file_updated", {
  id: uuid("id").primaryKey().defaultRandom(),
  uri: text("uri").notNull(),
  owner: text("owner").notNull(),
  blockNumber: bigint("block_number", { mode: "number" }).notNull(),
  transactionHash: text("transaction_hash").notNull(),
  createdAt: timestamp("created_at").defaultNow().notNull(),
});

// FileDeleted events
export const fileDeletedTable = pgTable("file_deleted", {
  id: uuid("id").primaryKey().defaultRandom(),
  uri: text("uri").notNull(),
  owner: text("owner").notNull(),
  blockNumber: bigint("block_number", { mode: "number" }).notNull(),
  transactionHash: text("transaction_hash").notNull(),
  createdAt: timestamp("created_at").defaultNow().notNull(),
});

// FileShared events
export const fileSharedTable = pgTable("file_shared", {
  id: uuid("id").primaryKey().defaultRandom(),
  uri: text("uri").notNull(),
  owner: text("owner").notNull(),
  sharedWith: text("shared_with").notNull(),
  blockNumber: bigint("block_number", { mode: "number" }).notNull(),
  transactionHash: text("transaction_hash").notNull(),
  createdAt: timestamp("created_at").defaultNow().notNull(),
});
