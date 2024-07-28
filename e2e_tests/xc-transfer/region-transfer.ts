import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { getEncodedRegionId } from 'coretime-utils';
import assert from 'node:assert';
import {
  openHrmpChannel,
  setupRelayAsset,
  sleep,
  submitExtrinsic,
  transferRelayAssetToPara,
} from '../common';
import { UNIT } from '../consts';
import { configureBroker, purchaseRegion, startSales } from '../coretime.common';
import { ismpAddParachain, makeIsmpResponse, queryRequest } from '../ismp.common';
import { REGIONX_API_TYPES, REGIONX_CUSTOM_RPC } from '../types';

const REGIONX_SOVEREIGN_ACCOUNT = '5Eg2fntJ27qsari4FGrGhrMqKFDRnkNSR6UshkZYBGXmSuC8';

async function run(_nodeName: any, networkInfo: any, _jsArgs: any) {
  const { wsUri: regionXUri } = networkInfo.nodesByName['regionx-collator01'];
  const { wsUri: coretimeUri } = networkInfo.nodesByName['coretime-collator01'];
  const { wsUri: rococoUri } = networkInfo.nodesByName['rococo-validator01'];

  const regionXApi = await ApiPromise.create({
    provider: new WsProvider(regionXUri),
    types: { ...REGIONX_API_TYPES },
    rpc: REGIONX_CUSTOM_RPC,
  });
  const rococoApi = await ApiPromise.create({ provider: new WsProvider(rococoUri) });
  const coretimeApi = await ApiPromise.create({ provider: new WsProvider(coretimeUri) });

  // account to submit tx
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

  const txSetCoretimeXcmVersion = coretimeApi.tx.polkadotXcm.forceDefaultXcmVersion([3]);
  const txSetRelayXcmVersion = rococoApi.tx.xcmPallet.forceDefaultXcmVersion([3]);
  await submitExtrinsic(alice, coretimeApi.tx.sudo.sudo(txSetCoretimeXcmVersion), {});
  await submitExtrinsic(alice, rococoApi.tx.sudo.sudo(txSetRelayXcmVersion), {});

  await setupRelayAsset(regionXApi, alice);

  await openHrmpChannel(alice, rococoApi, 1005, 2000);
  await openHrmpChannel(alice, rococoApi, 2000, 1005);
  await ismpAddParachain(alice, regionXApi);

  // Needed for fee payment
  // Alice has relay tokens on Coretime chain by default, so no need to send there.
  await transferRelayAssetToPara(10n ** 12n, 2000, rococoApi, alice);

  await configureBroker(coretimeApi, alice);
  await startSales(coretimeApi, alice);

  const txSetBalance = coretimeApi.tx.balances.forceSetBalance(alice.address, 1000n * UNIT);
  await submitExtrinsic(alice, coretimeApi.tx.sudo.sudo(txSetBalance), {});

  const regionId = await purchaseRegion(coretimeApi, alice);
  if (!regionId) throw new Error('RegionId not found');

  const receiverKeypair = new Keyring();
  receiverKeypair.addFromAddress(alice.address);

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
  await submitExtrinsic(alice, reserveTransferToRegionX, {});

  await sleep(5000);

  const requestRecord = regionXApi.tx.regions.requestRegionRecord(regionId);
  await submitExtrinsic(alice, requestRecord, {});

  let regions = await regionXApi.query.regions.regions.entries();
  assert.equal(regions.length, 1);
  assert.deepStrictEqual(regions[0][0].toHuman(), [regionId]);

  let region = regions[0][1].toHuman() as any;
  assert(region.owner == alice.address);
  assert(typeof region.record.Pending === 'string');

  // Check the data on the Coretime chain:
  regions = await coretimeApi.query.broker.regions.entries();
  assert.equal(regions.length, 1);
  assert.deepStrictEqual(regions[0][0].toHuman(), [regionId]);
  assert.equal((regions[0][1].toHuman() as any).owner, REGIONX_SOVEREIGN_ACCOUNT);

  // Respond to the ISMP get request:
  const request = await queryRequest(regionXApi, region.record.Pending);
  await makeIsmpResponse(regionXApi, coretimeApi, request, alice.address);

  // The record should be set after ISMP response:
  regions = await regionXApi.query.regions.regions.entries();
  region = regions[0][1].toHuman() as any;
  assert(region.owner == alice.address);
  assert.equal(region.record.Available.end, '66');
  assert.equal(region.record.Available.paid, null);

  // Transfer the region back to the Coretime chain:
  const reserveTransferToCoretime = regionXApi.tx.polkadotXcm.limitedReserveTransferAssets(
    { V3: { parents: 1, interior: { X1: { Parachain: 1005 } } } }, // dest
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
    }, // ^^ beneficiary
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
              parents: 1,
              // chain: Rococo-Coretime, pallet: pallet_broker
              interior: { X2: [{ Parachain: 1005 }, { PalletInstance: 50 }] },
            },
          },
          fun: {
            NonFungible: {
              Index: getEncodedRegionId(regionId, regionXApi).toString(),
            },
          },
        },
      ],
    }, // ^^ asset
    feeAssetItem,
    weightLimit
  );
  await submitExtrinsic(alice, reserveTransferToCoretime, {});
  await sleep(5000);

  regions = await regionXApi.query.regions.regions.entries();
  assert.equal(regions.length, 0);

  regions = await coretimeApi.query.broker.regions.entries();
  assert.equal(regions.length, 1);
  assert.deepStrictEqual(regions[0][0].toHuman(), [regionId]);
  assert.equal((regions[0][1].toHuman() as any).owner, alice.address);
}

export { run };
