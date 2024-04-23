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

  const assetMetadata = {
    decimals: 10,
    name: "DOT",
    symbol: "DOT",
    existentialDeposit: 10n**3n,
    location: null,
    additional: null
  };

  const assetSetupCalls = [
    api.tx.assetRegistry.registerAsset(assetMetadata, ASSET_ID),
    api.tx.assetRate.create(ASSET_ID, 2)
  ];
  const batchCall = api.tx.utility.batch(assetSetupCalls);
  const sudo = api.tx.sudo.sudo(batchCall);
  await submitExtrinsic(alice, sudo, {});

  const setBalanceCall = api.tx.tokens.setBalance(alice.address, ASSET_ID, 10n**6n, 0);
  await submitExtrinsic(alice, setBalanceCall, {});

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
