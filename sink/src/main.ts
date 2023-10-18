import "dotenv/config";
import { createConnectTransport } from "@bufbuild/connect-web";
import {
  createRequest,
  isEmptyMessage,
  streamBlocks,
  unpackMapOutput,
  createAuthInterceptor,
  createRegistry,
  fetchSubstream,
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

const fetchPackage = async () => {
  return await fetchSubstream(SPKG_URL);
};

const main = async () => {
  const pkg = await fetchPackage();
  const registry = createRegistry(pkg);

  const transport = createConnectTransport({
    baseUrl: "https://api.streamingfast.io",
    // @ts-expect-error type error
    interceptors: [createAuthInterceptor(STREAMINGFAST_KEY)],
    useBinaryFormat: true,
  });

  const request = createRequest({
    substreamPackage: pkg,
    outputModule: MODULE,
    productionMode: true,
    startBlockNum: 4333100,
    stopBlockNum: 4333139,
  });

  // Iterate over blocks
  for await (const response of streamBlocks(transport, request)) {
    const output = unpackMapOutput(response, registry);

    if (output !== undefined && !isEmptyMessage(output)) {
      const outputAsJson = output.toJson({ typeRegistry: registry });
      console.log(outputAsJson);
    }
  }
};

await main();
