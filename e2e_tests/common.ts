import { ApiPromise, Keyring } from "@polkadot/api";
import { SubmittableExtrinsic, SignerOptions } from "@polkadot/api/types";
import { KeyringPair } from "@polkadot/keyring/types";

const RELAY_ASSET_ID = 1;

async function submitExtrinsic(
	signer: KeyringPair,
	call: SubmittableExtrinsic<"promise">,
	options: Partial<SignerOptions>
): Promise<void> {
	return new Promise(async (resolve, reject) => {
		const unsub = await call.signAndSend(signer, options, (result) => {
			console.log(`Current status is ${result.status}`);
			if (result.status.isInBlock) {
				console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
			} else if (result.status.isFinalized) {
				console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
				unsub();
				return resolve();
			} else if (result.isError) {
				console.log(`Transaction error`);
				unsub();
				return reject();
			}
		});
	});
}

async function setupRelayAsset(
	api: ApiPromise,
	signer: KeyringPair,
	initialBalance: bigint = 10n ** 12n
) {
	const assetMetadata = {
		decimals: 12,
		name: "ROC",
		symbol: "ROC",
		existentialDeposit: 10n ** 3n,
		location: null,
		additional: null,
	};

	const assetSetupCalls = [
		api.tx.assetRegistry.registerAsset(assetMetadata, RELAY_ASSET_ID),
		api.tx.assetRate.create(RELAY_ASSET_ID, 1_000_000_000_000_000_000n), // 1 on 1
	];

	if (initialBalance > BigInt(0)) {
		assetSetupCalls.push(
			api.tx.tokens.setBalance(signer.address, RELAY_ASSET_ID, initialBalance, 0)
		);
	}

	const batchCall = api.tx.utility.batch(assetSetupCalls);
	const sudoCall = api.tx.sudo.sudo(batchCall);

	await submitExtrinsic(signer, sudoCall, {});
}

async function sleep(milliseconds: number) {
	return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

async function transferRelayAssetToRegionX(amount: string, relayApi: ApiPromise, signer: KeyringPair) {
  const receiverKeypair = new Keyring();
  receiverKeypair.addFromAddress(signer.address);

  const feeAssetItem = 0;
  const weightLimit = "Unlimited";
  const reserveTransfer = relayApi.tx.xcmPallet.limitedReserveTransferAssets(
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
            Fungible: amount,
          },
        },
      ],
    }, //asset
    feeAssetItem,
    weightLimit,
  );
  await submitExtrinsic(signer, reserveTransfer, {});
}

async function sleep(milliseconds: number) {
  return new Promise(resolve => setTimeout(resolve, milliseconds));
}

export { submitExtrinsic, setupRelayAsset, transferRelayAssetToRegionX, sleep, RELAY_ASSET_ID }
