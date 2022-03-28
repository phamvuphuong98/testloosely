const { ApiPromise, WsProvider } = require('@polkadot/api');

// Construct parameters for API instance
const wsProvider = new WsProvider('ws://localhost:9944');
const rpc = {
    getKittyCount: {
    get: {
      description: "Gets the sum of the two storage values in sum-storage pallet via a runtime api.",
      params: [],
      type: "u32",
    }
  }
}

async function main() {
  // Construct the actual api
  const api = await ApiPromise.create({
    provider: wsProvider,
    rpc,
  });

  // Query the custom RPC that uses the runtimeAPI
  let kittyCount = ( await api.rpc.getKittyCount.get() ).toNumber();
  console.log(`Kitty Count ${kittyCount}`);
}

main().catch(console.error).finally(() => process.exit());