import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { RELAY_ASSET_ID, setupRelayAsset, submitExtrinsic } from '../common';

async function run(nodeName: string, networkInfo: any, _jsArgs: any) {
  const { wsUri } = networkInfo.nodesByName[nodeName];
  const api = await ApiPromise.create({ provider: new WsProvider(wsUri) });

  // account to submit tx
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  const bob = keyring.addFromUri('//Bob');

  // relay asset is needed for purchasing the region on sale.
  await setupRelayAsset(api, alice, 500n * 10n**12n);

  const paraId = 2000;
  const orderRequirements = {
    begin: 10,
    end: 20,
    coreOccupancy: 57600 // full core
  };
  // 1. Create order
  const createOrderCall = api.tx.orders.createOrder(paraId, orderRequirements);
  await submitExtrinsic(alice, createOrderCall, {});

  // 2. A region is listed on sale

  // 3. Fund the order

  // Fund Bob's account:
  const giveBalanceCall = api.tx.tokens.setBalance(bob.address, RELAY_ASSET_ID, 200n * 10n ** 12n, 0);
  await submitExtrinsic(alice, api.tx.sudo.sudo(giveBalanceCall), {});

  // Bob contributes:
  const contributeCall = api.tx.orders.contribute(0, 10n * 10n ** 12n);
  await submitExtrinsic(bob, contributeCall, {});

  // 4. A trader sees a profitable opportunity and fulfills the order.

  // 5. Ensure the region gets assigned to the specified parachain.
}

export { run };
