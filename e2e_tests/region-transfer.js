const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api");
const { CONFIG, INITIAL_PRICE, UNIT, CORE_COUNT } = require("./consts");
const { submitExtrinsic } = require("./common");
const { CoreMask, Region, Id } = require("coretime-utils");

async function run(nodeName, networkInfo, _jsArgs) {
  const { wsUri: regionXUri } = networkInfo.nodesByName[nodeName];
  const { wsUri: coretimeUri } = networkInfo.nodesByName["coretime-collator01"];

  const regionXApi = await ApiPromise.create({ provider: new WsProvider(regionXUri) });
  const coretimeApi = await ApiPromise.create({ provider: new WsProvider(coretimeUri), types: { Id } });

  // account to submit tx
  const keyring = new zombie.Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");

  const setCoretimeXcmVersion = coretimeApi.tx.polkadotXcm.forceDefaultXcmVersion([3]);
  await submitExtrinsic(alice, coretimeApi.tx.sudo.sudo(setCoretimeXcmVersion), {});

  await configureBroker(coretimeApi, alice);
  await startSales(coretimeApi, alice);

  const setBalanceCall = coretimeApi.tx.tokens.forceSetBalance(alice.address, 1000 * UNIT);
  await submitExtrinsic(alice, coretimeApi.tx.sudo.sudo(setBalanceCall), {});

  const regionId = await purchaseRegion(coretimeApi, alice);
  const encodedId = new Region(regionId, {end: 0, owner: '', paid: null}).getEncodedRegionId(coretimeApi);

  const receiverKeypair = new Keyring();
  receiverKeypair.addFromAddress(alice.address);

  const feeAssetItem = 0;
  const weightLimit = "Unlimited";
  const reserveTransfer = coretimeApi.tx.polkadotXcm.limitedReserveTransferAssets(
    { V3: { parents: 1, interior: { X1: { Parachain: 2000 } } } }, //dest
    {
      V3: {
        parents: 0,
        interior: {
          X1: {
            AccountId32: {
              chain: "Any",
              id: receiverKeypair.pairs[0].publicKey,
            },
          },
        },
      },
    }, //beneficiary
    {
      V3: [
        {
          id: {
            Concrete: {
              parents: 0,
              interior: { X1: { PalletInstance: 50 } },
            },
          },
          fun: {
            NonFungible: {
              Index: encodedId
            }
          },
        },
      ],
    }, //asset
    feeAssetItem,
    weightLimit,
  );
  await submitExtrinsic(alice, reserveTransfer, {});
}

async function configureBroker(coretimeApi, signer) {
  const configCall = coretimeApi.tx.broker.configure(CONFIG);
  const sudo = coretimeApi.tx.sudo.sudo(configCall)
  return submitExtrinsic(signer, sudo, {});
}

async function startSales(coretimeApi, signer) {
  const startSaleCall = coretimeApi.tx.broker.startSales(INITIAL_PRICE, CORE_COUNT);
  const sudo = coretimeApi.tx.sudo.sudo(startSaleCall)
  return submitExtrinsic(signer, sudo, {});
}

async function purchaseRegion(coretimeApi, buyer) {
  const callTx = async (resolve) => {
    const purchase = coretimeApi.tx.broker.purchase(INITIAL_PRICE * 2);
    const unsub = await purchase.signAndSend(buyer, async (result) => {
      if (result.status.isInBlock) {
        const regionId = await getRegionId(coretimeApi);
        unsub();
        resolve(regionId);
      }
    });
  };

  return new Promise(callTx);
}

async function getRegionId(coretimeApi) {
  const events = await coretimeApi.query.system.events();

  for (const record of events) {
    const { event } = record;
    if (event.section === "broker" && event.method === "Purchased") {
      return event.data[1];
    }
  }

  return { begin: 0, core: 0, mask: CoreMask.voidMask() };
}

module.exports = { run };
