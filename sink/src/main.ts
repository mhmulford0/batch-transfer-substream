import "dotenv/config";
import { createGrpcTransport } from "@connectrpc/connect-node";
import {
  createAuthInterceptor,
  createRegistry,
  createRequest,
  fetchSubstream,
  isEmptyMessage,
  streamBlocks,
  unpackMapOutput,
} from "@substreams/core";

if (!process.env.SPKG_URL) {
  throw new Error("SPKG_URL required");
}

if (!process.env.STREAMINGFAST_KEY) {
  throw new Error("STREAMINGFAST_KEY required");
}
if (!process.env.MODULE) {
  throw new Error("MODULE name is required");
}

const { SPKG_URL, STREAMINGFAST_KEY, MODULE } = process.env;

const substream = await fetchSubstream(SPKG_URL);
const registry = createRegistry(substream);
const transport = createGrpcTransport({
  baseUrl: "https://sepolia.eth.streamingfast.io:443",
  httpVersion: "2",
  interceptors: [createAuthInterceptor(STREAMINGFAST_KEY)],
  jsonOptions: {
    typeRegistry: registry,
  },
});

const request = createRequest({
  substreamPackage: substream,
  outputModule: MODULE,
  productionMode: true,
  startBlockNum: 4333100,
  stopBlockNum: 4333139,
});

async function main() {
  for await (const response of streamBlocks(transport, request)) {
    const output = unpackMapOutput(response, registry);
    if (output !== undefined && !isEmptyMessage(output)) {
      console.dir(output.toJson({ typeRegistry: registry }));
    }
  }
}

await main();
