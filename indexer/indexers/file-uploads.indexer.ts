import { defineIndexer } from "apibara/indexer";
import { useLogger } from "apibara/plugins";
import { drizzleStorage, useDrizzleStorage } from "@apibara/plugin-drizzle";
import { drizzle } from "@apibara/plugin-drizzle";
import { StarknetStream } from "@apibara/starknet";
import type { ApibaraRuntimeConfig } from "apibara/types";
import * as schema from "../lib/schema";
import { EVENT_SIGNATURES, EVENT_SIGNATURE_TO_TYPE } from "../lib/constants";
import { eventHandlers } from "../lib/event-handlers";

export default function (runtimeConfig: ApibaraRuntimeConfig) {
  const { startingBlock, streamUrl } = runtimeConfig["fileUploads"];
  const contractAddress = process.env["CONTRACT_ADDRESS"] as `0x${string}`;
  const db = drizzle({
    schema,
  });

  return defineIndexer(StarknetStream)({
    streamUrl,
    finality: "accepted",
    startingBlock: BigInt(startingBlock),
    filter: {
      events: [
        {
          address: contractAddress,
          keys: [EVENT_SIGNATURES.CompressionMappingStored],
        },
        {
          address: contractAddress,
          keys: [EVENT_SIGNATURES.FileUploaded],
        },
        {
          address: contractAddress,
          keys: [EVENT_SIGNATURES.FileUpdated],
        },
        {
          address: contractAddress,
          keys: [EVENT_SIGNATURES.FileDeleted],
        },
        {
          address: contractAddress,
          keys: [EVENT_SIGNATURES.FileShared],
        },
      ],
    },
    plugins: [
      drizzleStorage({ db, migrate: { migrationsFolder: "./drizzle" } }),
    ],
    async transform({ block }) {
      const logger = useLogger();
      const { db: database } = useDrizzleStorage();

      if (!block.events || block.events.length === 0) {
        return;
      }

      logger.info(`Processing block ${block.header.blockNumber} with ${block.events.length} events`);

      for (const event of block.events) {
        const eventKey = event.keys[0] as keyof typeof EVENT_SIGNATURE_TO_TYPE;
        const blockNumber = Number(block.header.blockNumber);
        const transactionHash = event.transactionHash;

        try {
          const eventType = EVENT_SIGNATURE_TO_TYPE[eventKey];
          if (eventType && eventHandlers[eventType]) {
            await eventHandlers[eventType]({
              event,
              blockNumber: BigInt(blockNumber),
              transactionHash,
              database,
              logger,
            });
          } else {
            logger.warn(`Unknown event signature: ${eventKey}`);
          }
        } catch (error) {
          logger.error(`Error processing event ${eventKey}:`, error);
        }
      }
    },
  });
}


