import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { getEncodedRegionId, RegionId } from 'coretime-utils';
import assert from 'node:assert';
import {
  openHrmpChannel,
  RELAY_ASSET_ID,
  setupRelayAsset,
  sleep,
  submitExtrinsic,
  transferRelayAssetToPara,
} from '../common';
import { UNIT } from '../consts';
import { configureBroker, purchaseRegion, startSales } from '../coretime.common';
import { ismpAddParachain, makeIsmpResponse, queryRequest } from '../ismp.common';
import { REGIONX_API_TYPES, REGIONX_CUSTOM_RPC } from '../types';

async function run(nodeName: string, networkInfo: any, _jsArgs: any) {
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
  const bob = keyring.addFromUri('//Bob');

  const txSetCoretimeXcmVersion = coretimeApi.tx.polkadotXcm.forceDefaultXcmVersion([3]);
  const txSetRelayXcmVersion = rococoApi.tx.xcmPallet.forceDefaultXcmVersion([3]);
  await submitExtrinsic(alice, coretimeApi.tx.sudo.sudo(txSetCoretimeXcmVersion), {});
  await submitExtrinsic(alice, rococoApi.tx.sudo.sudo(txSetRelayXcmVersion), {});

  await setupRelayAsset(regionXApi, alice, 500n * UNIT);

  await openHrmpChannel(alice, rococoApi, 1005, 2000);
  await openHrmpChannel(alice, rococoApi, 2000, 1005);
  await ismpAddParachain(alice, regionXApi);

  // Needed for fee payment
  // Alice has relay tokens on Coretime chain by default, so no need to send there.
  await transferRelayAssetToPara(UNIT, 2000, rococoApi, alice);

  // 1. A region is listed on sale
  await configureBroker(coretimeApi, alice);
  await startSales(coretimeApi, alice);

  const txSetBalance = coretimeApi.tx.balances.forceSetBalance(alice.address, 1000n * UNIT);
  await submitExtrinsic(alice, coretimeApi.tx.sudo.sudo(txSetBalance), {});

  const regionId = await purchaseRegion(coretimeApi, alice);
  if (!regionId) throw new Error('RegionId not found');

  await transferRegionToRegionX(coretimeApi, regionXApi, alice, regionId);

  // 2. An order is created
  const paraId = 2000;
  const orderRequirements = {
    begin: 10,
    end: 20,
    coreOccupancy: 57600, // full core
  };

  const createOrderCall = regionXApi.tx.orders.createOrder(paraId, orderRequirements);
  await submitExtrinsic(alice, createOrderCall, {});
  // TODO: check state

  // 3. Fund the order
  // Give Bob tokens:
  const giveBalanceCall = regionXApi.tx.tokens.setBalance(
    bob.address,
    RELAY_ASSET_ID,
    200n * UNIT,
    0
  );
  await submitExtrinsic(alice, regionXApi.tx.sudo.sudo(giveBalanceCall), {});
  // TODO: check state

  // Bob contributes:
  const contributeCall = regionXApi.tx.orders.contribute(0, 10n * UNIT);
  await submitExtrinsic(bob, contributeCall, {});

  // 4. A trader sees a profitable opportunity and fulfills the order.
  const fulfillCall = regionXApi.tx.processor.fulfillOrder(0, regionId);
  await submitExtrinsic(alice, fulfillCall, {});
  // TODO: check state

  // 5. Ensure the region gets assigned to the specified parachain.
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

  let regions = await regionXApi.query.regions.regions.entries();
  let region = regions[0][1].toHuman() as any;
  assert(region.owner == sender.address);
  assert(typeof region.record.Pending === 'string');

  // Respond to the ISMP get request:
  const request = await queryRequest(regionXApi, region.record.Pending);
  await makeIsmpResponse(regionXApi, coretimeApi, request, sender.address);

  // The record should be set after ISMP response:
  regions = await regionXApi.query.regions.regions.entries();
  region = regions[0][1].toHuman() as any;
  assert.equal(region.record.Available.end, '66');
  assert.equal(region.record.Available.paid, null);
}

export { run };
