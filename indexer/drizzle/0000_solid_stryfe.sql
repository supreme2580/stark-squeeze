CREATE TABLE IF NOT EXISTS "compression_mappings" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"uri" text NOT NULL,
	"file_format" text NOT NULL,
	"compressed_by" integer NOT NULL,
	"original_size" bigint NOT NULL,
	"final_size" bigint NOT NULL,
	"chunk_size" bigint NOT NULL,
	"chunk_mappings" jsonb NOT NULL,
	"chunk_values" jsonb NOT NULL,
	"byte_mappings" jsonb NOT NULL,
	"byte_values" jsonb NOT NULL,
	"reconstruction_steps" jsonb NOT NULL,
	"metadata" jsonb NOT NULL,
	"block_number" bigint NOT NULL,
	"transaction_hash" text NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "cursor_table" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"end_cursor" bigint,
	"unique_key" text
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "file_deleted" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"uri" text NOT NULL,
	"owner" text NOT NULL,
	"block_number" bigint NOT NULL,
	"transaction_hash" text NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "file_shared" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"uri" text NOT NULL,
	"owner" text NOT NULL,
	"shared_with" text NOT NULL,
	"block_number" bigint NOT NULL,
	"transaction_hash" text NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "file_updated" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"uri" text NOT NULL,
	"owner" text NOT NULL,
	"block_number" bigint NOT NULL,
	"transaction_hash" text NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "file_uploaded" (
	"id" uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
	"uri" text NOT NULL,
	"owner" text NOT NULL,
	"visibility" integer NOT NULL,
	"block_number" bigint NOT NULL,
	"transaction_hash" text NOT NULL,
	"created_at" timestamp DEFAULT now() NOT NULL
);
