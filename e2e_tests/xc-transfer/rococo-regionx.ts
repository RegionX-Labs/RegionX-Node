import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { RELAY_ASSET_ID, setupRelayAsset, sleep, submitExtrinsic } from "../common";

import assert from "node:assert";

async function run(nodeName: string, networkInfo: any, _jsArgs: any) {
	const { wsUri: regionXUri } = networkInfo.nodesByName[nodeName];
	const { wsUri: rococoUri } = networkInfo.nodesByName["rococo-validator01"];

	const rococoApi = await ApiPromise.create({ provider: new WsProvider(rococoUri) });
	const regionXApi = await ApiPromise.create({ provider: new WsProvider(regionXUri) });

	// account to submit tx
	const keyring = new Keyring({ type: "sr25519" });
	const alice = keyring.addFromUri("//Alice");

	const setXcmVersion = rococoApi.tx.xcmPallet.forceDefaultXcmVersion([3]);
	await submitExtrinsic(alice, rococoApi.tx.sudo.sudo(setXcmVersion), {});

	const BALANCE = 10n ** 9n;

	await setupRelayAsset(regionXApi, alice, BALANCE);

	const receiverKeypair = new Keyring();
	receiverKeypair.addFromAddress(alice.address);

	const { free: balanceBefore } = (
		await regionXApi.query.tokens.accounts(alice.address, RELAY_ASSET_ID)
	).toJSON() as any;

	console.log(`balanceBefore = ${balanceBefore}`);

	assert.equal(BigInt(balanceBefore), BALANCE);

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
						Fungible: BALANCE,
					},
				},
			],
		}, //asset
		feeAssetItem,
		weightLimit
	);
	await submitExtrinsic(alice, reserveTransfer, {});

	await sleep(15 * 1000);

	const { free: balanceAfter } = (
		await regionXApi.query.tokens.accounts(alice.address, RELAY_ASSET_ID)
	).toJSON() as any;

	console.log(`balanceAfter = ${balanceAfter}`);
}

export { run };
