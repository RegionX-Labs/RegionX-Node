import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { sleep, submitExtrinsic, submitUnsigned } from "./utils";
import { KeyringPair } from "@polkadot/keyring/types";
import { getEncodedRegionId, RegionId } from "coretime-utils";
import { makeIsmpResponse, queryRequest } from "./ismp.utils";
import { purchaseRegion } from "./coretime.utils";

const RELAY_ENDPOINT = "ws://127.0.0.1:8002";
const CORETIME_ENDPOINT = "ws://127.0.0.1:8000";
const REGIONX_ENDPOINT = "ws://127.0.0.1:8001";

export const keyring = new Keyring({ type: "sr25519" });

async function run() {
  await cryptoWaitReady();

  const coretimeWsProvider = new WsProvider(CORETIME_ENDPOINT);
  const coretimeApi = await ApiPromise.create({ provider: coretimeWsProvider });

  const regionxWsProvider = new WsProvider(REGIONX_ENDPOINT);
  const regionxApi = await ApiPromise.create({ provider: regionxWsProvider });

  const alice = keyring.addFromUri("//Alice");

  const regionId = await purchaseRegion(coretimeApi, alice);
  if (!regionId) throw new Error('RegionId not found');

  await transferRegionToRegionX(coretimeApi, regionxApi, alice, regionId);
}

async function transferRegionToRegionX(
  coretimeApi: ApiPromise,
  regionXApi: ApiPromise,
  sender: KeyringPair,
  regionId: RegionId
) {
  const receiverKeypair = new Keyring();
  receiverKeypair.addFromAddress(sender.address);

  const feeAssetItem = 0;
  const weightLimit = 'Unlimited';
  const reserveTransferToRegionX = coretimeApi.tx.polkadotXcm.limitedReserveTransferAssets(
    { V3: { parents: 1, interior: { X1: { Parachain: 2000 } } } }, //dest
    {
      V3: {
        parents: 0,
        interior: {
          X1: {
            AccountId32: {
              chain: 'Any',
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
              parents: 1,
              interior: 'Here',
            },
          },
          fun: {
            Fungible: 10n ** 10n,
          },
        }, // ^^ fee payment asset
        {
          id: {
            Concrete: {
              parents: 0,
              interior: { X1: { PalletInstance: 50 } },
            },
          },
          fun: {
            NonFungible: {
              Index: getEncodedRegionId(regionId, coretimeApi).toString(),
            },
          },
        },
      ],
    }, //asset
    feeAssetItem,
    weightLimit
  );
  await submitExtrinsic(sender, reserveTransferToRegionX, {});

  await sleep(5000);

  const requestRecord = regionXApi.tx.regions.requestRegionRecord(regionId);
  await submitUnsigned(requestRecord);

  let regions = await regionXApi.query.regions.regions.entries();
  // assert.equal(regions.length, 1);
  // assert.deepStrictEqual(regions[0][0].toHuman(), [regionId]);

  let region = regions[0][1].toHuman() as any;
  // assert(region.owner == sender.address);
  // assert(typeof region.record.Pending === 'string');

  // Check the data on the Coretime chain:
  regions = await coretimeApi.query.broker.regions.entries();
  // assert.equal(regions.length, 1);
  // assert.deepStrictEqual(regions[0][0].toHuman(), [regionId]);
  // assert.equal((regions[0][1].toHuman() as any).owner, REGIONX_SOVEREIGN_ACCOUNT);

  // Respond to the ISMP get request:
  const request = await queryRequest(regionXApi, region.record.Pending);
  await makeIsmpResponse(regionXApi, coretimeApi, request, sender.address);

  // The record should be set after ISMP response:
  regions = await regionXApi.query.regions.regions.entries();
  region = regions[0][1].toHuman() as any;
  // assert(region.owner == sender.address);
}

run().then(() => process.exit(0));
