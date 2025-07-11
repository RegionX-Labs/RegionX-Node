import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { getFreeBalance, sleep, submitExtrinsic, teleportAssetToPara } from '../common';

import assert from 'node:assert';

const TOLERANCE = 10n ** 10n;

async function run(nodeName: string, networkInfo: any, _jsArgs: any) {
  const { wsUri: regionXUri } = networkInfo.nodesByName[nodeName];
  const { wsUri: rococoUri } = networkInfo.nodesByName['rococo-validator01'];

  const rococoApi = await ApiPromise.create({ provider: new WsProvider(rococoUri) });
  const regionXApi = await ApiPromise.create({ provider: new WsProvider(regionXUri) });

  // account to submit tx
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

  const setXcmVersion = rococoApi.tx.xcmPallet.forceDefaultXcmVersion([3]);
  await submitExtrinsic(alice, rococoApi.tx.sudo.sudo(setXcmVersion), {});

  const receiverKeypair = new Keyring();
  receiverKeypair.addFromAddress(alice.address);

  const assertRegionXBalance = async (address: string, balance: bigint) => {
    const free = await getFreeBalance(regionXApi, address);

    console.log(`RegionX: ${BigInt(free).toString()} | Expected: ${balance}`);
    assert(balance - BigInt(free) < TOLERANCE);
  };

  const assertRococoBalance = async (address: string, balance: bigint) => {
    const free = await getFreeBalance(rococoApi, address);

    console.log(`Rococo: ${BigInt(free).toString()} | Expected: ${balance}`);
    assert(balance - BigInt(free) < TOLERANCE);
  };

  await assertRegionXBalance(alice.address, 0n);
  await assertRococoBalance(alice.address, 10n ** 18n);

  await teleportAssetToPara(rococoApi, alice, 2000, alice.address, 3n * 10n ** 12n);
  await sleep(5 * 1000);

  await assertRegionXBalance(alice.address, 3n * 10n ** 12n);
  await assertRococoBalance(alice.address, 10n ** 18n - 3n * 10n ** 12n);

  const regionXReserveTransfer = regionXApi.tx.polkadotXcm.limitedReserveTransferAssets(
    { V3: { parents: 1, interior: 'Here' } }, //dest
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
            Concrete: { parents: 1, interior: 'Here' },
          },
          fun: {
            Fungible: 10n ** 12n,
          },
        },
      ],
    }, //asset
    0,
    'Unlimited'
  );

  await submitExtrinsic(alice, regionXReserveTransfer, {});

  await sleep(5 * 1000);

  await assertRegionXBalance(alice.address, 2n * 10n ** 12n);
  await assertRococoBalance(alice.address, 10n ** 18n - 3n * 10n ** 12n);
}

export { run };
