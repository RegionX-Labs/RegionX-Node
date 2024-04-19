const BOB = "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc";
const ASSET_ID = 42;

async function run(nodeName, networkInfo, _jsArgs) {
  const { wsUri, userDefinedTypes } = networkInfo.nodesByName[nodeName];
  const api = await zombie.connect(wsUri, {
    ...userDefinedTypes,
    signedExtensions: {
    ChargeAssetTxPayment: {
      extrinsic: {
        tip: 'Compact<Balance>',
        assetId: 'Option<u32>'
      },
      payload: {}
    }
  }});

  await zombie.util.cryptoWaitReady();

  // account to submit tx
  const keyring = new zombie.Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");

  const createCall = api.tx.assets.forceCreate(ASSET_ID, alice.address, true, 50);
  const mintCall = api.tx.assets.mint(ASSET_ID, alice.address, 10n**6n);

  const sudo = api.tx.sudo.sudo(createCall);
  await submitExtrinsic(alice, sudo, {});
  await submitExtrinsic(alice, mintCall, {});

  const transferCall = api.tx.balances.transferKeepAlive(BOB, 10n**6n);
  const feePaymentAsset = api.registry.createType('Option<u32>', ASSET_ID);
  await submitExtrinsic(alice, transferCall, {assetId: feePaymentAsset});

  return 0;
}

async function submitExtrinsic(signer, call, options) {
  return new Promise(async (resolve, reject) => {
    const unsub = await call.signAndSend(signer, options, (result) => {
      console.log(`Current status is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(
          `Transaction included at blockHash ${result.status.asInBlock}`
        );
      } else if (result.status.isFinalized) {
        console.log(
          `Transaction finalized at blockHash ${result.status.asFinalized}`
        );
        unsub();
        return resolve();
      } else if (result.isError) {
        console.log(`Transaction error`);
        unsub();
        return resolve();
      }
    });
  });
}

module.exports = { run };
