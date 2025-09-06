import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { openHrmpChannel, submitExtrinsic } from '../common';
import { UNIT } from '../consts';
import { configureBroker, purchaseRegion, startSales } from '../coretime.common';
import { getRequestCommitment, ismpAddParachain } from '../ismp.common';
import { REGIONX_API_TYPES, REGIONX_CUSTOM_RPC } from '../types';
import { transferRegionToCoretimeChain, transferRegionToRegionX } from '../xc-regions.common';

const PARA_2000_CHILD = 'F7fq1jMZkfuCuoMTyiEVAP2DMpMt18WopgBqTJznLihLNbZ';

async function run(_nodeName: any, networkInfo: any, _jsArgs: any) {
  const { wsUri: regionXUri } = networkInfo.nodesByName['regionx-collator01'];
  const { wsUri: coretimeUri } = networkInfo.nodesByName['coretime-collator01'];
  const { wsUri: relayUri } = networkInfo.nodesByName['kusama-validator01'];

  const regionXApi = await ApiPromise.create({
    provider: new WsProvider(regionXUri),
    types: { ...REGIONX_API_TYPES },
    rpc: REGIONX_CUSTOM_RPC,
  });
  const relayApi = await ApiPromise.create({ provider: new WsProvider(relayUri) });
  const coretimeApi = await ApiPromise.create({ provider: new WsProvider(coretimeUri) });

  // account to submit tx
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

  // 0xb391a5b9a9fb2d4f6e26ede50664026ff64fd370e076048b77dfc9a2653f5082
  /*
  [
    {
      get: {
        source: 'KUSAMA-2000',
        dest: 'KUSAMA-1005',
        nonce: 0,
        from: '0x726567696f6e7370',
        keys: [Array],
        height: 27,
        context: '0x',
        timeout_timestamp: 0
      }
    }
  ]
  */
  console.log(getRequestCommitment(regionXApi, {
    source: {Kusama: 2000},
    dest: {Kusama: 1005},
    nonce: 0,
    from: '0x726567696f6e7370',
    keys: [
      '0x4dcb50595177a3177648411a42aca0f53dc63b0b76ffd6f80704a090da6f87197b8ad2503224d551cb1c49dc3958c83d200000000000ffffffffffffffffffff'
    ],
    height: 27,
    context: '0x',
    timeoutTimestamp: 0,
  }));
  /*

  await submitExtrinsic(
    alice,
    relayApi.tx.balances.transferKeepAlive(PARA_2000_CHILD, 10n * UNIT),
    {}
  );

  await openHrmpChannel(alice, relayApi, regionXApi);
  await ismpAddParachain(alice, regionXApi);

  await configureBroker(coretimeApi, alice);
  await startSales(coretimeApi, alice);

  const regionId = await purchaseRegion(coretimeApi, alice);
  if (!regionId) throw new Error('RegionId not found');

  // Transferring to the RegionX chain should work:
  // NOTE: the function contains checks, and if any of them fail, the test will fail.
  await transferRegionToRegionX(coretimeApi, regionXApi, alice, regionId);

  // Transferring back to the Coretime chain should work:
  // NOTE: the function contains checks, and if any of them fail, the test will fail.
  // await transferRegionToCoretimeChain(coretimeApi, regionXApi, alice, regionId);
  */
}

export { run };
