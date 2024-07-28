import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { setupRelayAsset, submitExtrinsic } from '../common';

async function run(nodeName: string, networkInfo: any, _jsArgs: any) {
  const { wsUri } = networkInfo.nodesByName[nodeName];
  const api = await ApiPromise.create({ provider: new WsProvider(wsUri) });

  // account to submit tx
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

  // relay asset is needed for purchasing the region on sale.
  await setupRelayAsset(api, alice);

  // 1. Create order

  // 2. Fund the order

  // 3. A region is listed on sale

  // 4. A trader sees a profitable opportunity and fulfills the order.

  // 5. Ensure the region gets assigned to the specified parachain.
}

export { run };
