const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api");
const { submitExtrinsic, setupRelayAsset } = require("./common");

async function run(nodeName, networkInfo, _jsArgs) {
  const { wsUri: regionXUri } = networkInfo.nodesByName[nodeName];
  const { wsUri: rococoUri } = networkInfo.nodesByName["rococo-validator01"];
  const { wsUri: coretimeUri } = networkInfo.nodesByName["coretime-collator01"];

  const rococoApi = await ApiPromise.create({
    provider: new WsProvider(rococoUri),
  });
  const regionXApi = await ApiPromise.create({provider: new WsProvider(regionXUri)});

  // account to submit tx
  const keyring = new zombie.Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");

  const setRelayXcmVersion = rococoApi.tx.xcmPallet.forceDefaultXcmVersion([3]);
  await submitExtrinsic(alice, rococoApi.tx.sudo.sudo(setRelayXcmVersion), {});
  const setCoretimeXcmVersion = coretimeUri.tx.xcmPallet.forceDefaultXcmVersion([3]);
  await submitExtrinsic(alice, coretimeUri.tx.sudo.sudo(setCoretimeXcmVersion), {});

  await setupRelayAsset(regionXApi, alice);

  const receiverKeypair = new Keyring();
  receiverKeypair.addFromAddress(alice.address);

  const feeAssetItem = 0;
  const weightLimit = "Unlimited";
  const reserveTransfer = rococoApi.tx.xcmPallet.limitedReserveTransferAssets(
    { V3: { parents: 0, interior: { X1: { Parachain: 2000 } } } }, //dest
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
            Concrete: { parents: 0, interior: "Here" },
          },
          fun: {
            Fungible: 10n ** 9n,
          },
        },
      ],
    }, //asset
    feeAssetItem,
    weightLimit,
  );
  await submitExtrinsic(alice, reserveTransfer, {});
}

module.exports = { run };
