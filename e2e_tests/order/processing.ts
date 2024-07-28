import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import {
  openHrmpChannel,
  RELAY_ASSET_ID,
  setupRelayAsset,
  submitExtrinsic,
  transferRelayAssetToPara,
} from '../common';
import { UNIT } from '../consts';
import { ismpAddParachain } from '../ismp.common';
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
  await transferRelayAssetToPara(UNIT, 2000, rococoApi, alice);

  const paraId = 2000;
  const orderRequirements = {
    begin: 10,
    end: 20,
    coreOccupancy: 57600, // full core
  };
  // 1. Create order
  const createOrderCall = regionXApi.tx.orders.createOrder(paraId, orderRequirements);
  await submitExtrinsic(alice, createOrderCall, {});

  // 2. A region is listed on sale

  // 3. Fund the order

  // Fund Bob's account:
  const giveBalanceCall = regionXApi.tx.tokens.setBalance(
    bob.address,
    RELAY_ASSET_ID,
    200n * UNIT,
    0
  );
  await submitExtrinsic(alice, regionXApi.tx.sudo.sudo(giveBalanceCall), {});

  // Bob contributes:
  const contributeCall = regionXApi.tx.orders.contribute(0, 10n * UNIT);
  await submitExtrinsic(bob, contributeCall, {});

  // 4. A trader sees a profitable opportunity and fulfills the order.

  // 5. Ensure the region gets assigned to the specified parachain.
}

async function transferRegionToRegionX() {}

export { run };
