const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api");
const {
  submitExtrinsic,
  setupRelayAsset,
  RELAY_ASSET_ID,
} = require("./common");

const PREIMAGE_HASH =
  "0xb8375f7ca0c64a384f2dd643a0d520977f3aae06e64afb8c960891eee5147bd1";

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
  const anna = keyring.addFromUri("//Anna");

  await setupRelayAsset(api, alice);

  const giveBalanceCall = api.tx.utility.batch([
    api.tx.tokens.setBalance(anna.address, RELAY_ASSET_ID, 10n ** 18n, 0),
    api.tx.balances.forceSetBalance(anna.address, 10n ** 12n),
  ]);
  await submitExtrinsic(alice, api.tx.sudo.sudo(giveBalanceCall), {});

  const remarkCallBytes = api.tx.system.remark("hey").toU8a();
  await submitExtrinsic(
    alice,
    api.tx.preimage.notePreimage(remarkCallBytes),
    {},
  );

  const submitProposal = api.tx.referenda.submit(
    { system: "Root" },
    { Lookup: { hash: PREIMAGE_HASH, len: remarkCallBytes.length } },
    { After: 5 },
  );
  await submitExtrinsic(anna, submitProposal, {});

  const placeDeposit = api.tx.referenda.placeDecisionDeposit(1);
  await submitExtrinsic(anna, placeDeposit, {});

  const voteCall = api.tx.convictionVoting.vote(1, {
    // Voting with relay chain tokens. We know this is true; otherwise, this call 
    // would fail, given that Anna doesn't have 10^16 RegionX tokens.
    Standard: { vote: { aye: true, conviction: "None" }, balance: 10n ** 16n },
  });
  await submitExtrinsic(anna, voteCall, {});
}

module.exports = { run };
