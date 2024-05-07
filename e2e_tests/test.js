const { ApiPromise, WsProvider } = require("@polkadot/api");
const {Id, getEncodedRegionId, CoreMask} = require("coretime-utils");

const run = async() => {
  const regionXApi = await ApiPromise.create({ provider: new WsProvider("ws://127.0.0.1:33247"), types: {Id} });

  console.log(getEncodedRegionId({begin: 34, core: 0, mask: new CoreMask("0xffffffffffffffffffff")}, regionXApi).toString());
}

run();