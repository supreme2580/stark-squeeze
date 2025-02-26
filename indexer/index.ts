import { hash } from "https://esm.run/starknet@5.14";

export const config = {
    streamUrl: "https://starknet-sepolia.preview.apibara.org",
    startingBlock: 10_000,
    network: "starknet",
    finality: "DATA_STATUS_ACCEPTED",
    filter: {
      events: [
        {
          fromAddress: "0x04e07a94012b261feaf0925c61f3937bb5dc77e6b13ba11d5f2701cfd4b08942",
          keys: [hash.getSelectorFromName("DataAdded")]
        }
      ]
    },
    sinkType: "webhook",
    sinkOptions: {
      targetUrl: "http://localhost:3000/",
      raw: true
    },
};
  
// This transform does nothing.
export default function transform(block) {
    return block;
}