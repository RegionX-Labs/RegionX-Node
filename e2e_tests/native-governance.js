const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api");
const { submitExtrinsic, setupRelayAsset, RELAY_ASSET_ID } = require("./common");

const PREIMAGE_HASH = "0xb8375f7ca0c64a384f2dd643a0d520977f3aae06e64afb8c960891eee5147bd1";

async function run(nodeName, networkInfo, _jsArgs) {
  const { wsUri } = networkInfo.nodesByName[nodeName];
  const api = await ApiPromise.create({
    provider: new WsProvider(wsUri),
    signedExtensions: {
      ChargeAssetTxPayment: {
        extrinsic: {
          tip: "Compact<Balance>",
          assetId: "Option<AssetId>",
        },
        payload: {},
      },
    },
  });

  // account to submit tx
  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");

  await setupRelayAsset(api, alice);

  const spendCallBytes = api.tx.treasury.spendLocal(10n**6n, alice.address).toU8a();
  await submitExtrinsic(alice, api.tx.preimage.notePreimage(spendCallBytes), {});

  const submitProposal = api.tx.nativeReferenda.submit(
    { Origins: "SmallTipper" },
    { Lookup: { hash: PREIMAGE_HASH, len: spendCallBytes.length } },
    { After: 5 },
  );
  await submitExtrinsic(alice, submitProposal, {});

  const placeDeposit = api.tx.nativeReferenda.placeDecisionDeposit(0);
  await submitExtrinsic(alice, placeDeposit, {});

  const voteCall = api.tx.nativeConvictionVoting.vote(0, {
    Standard: { vote: { aye: true, conviction: "None" }, balance: 10n ** 16n },
  });
  await submitExtrinsic(alice, voteCall, {});
}

module.exports = { run };
