import { ApiPromise, Keyring } from '@polkadot/api';
import { SignerOptions, SubmittableExtrinsic } from '@polkadot/api/types';
import { KeyringPair } from '@polkadot/keyring/types';
import { stringToU8a } from '@polkadot/util';
import { encodeAddress } from '@polkadot/util-crypto';

const RELAY_ASSET_ID = 1;

async function submitExtrinsic(
  signer: KeyringPair,
  call: SubmittableExtrinsic<'promise'>,
  options: Partial<SignerOptions>
): Promise<void> {
  return new Promise((resolve, reject) => {
    const unsub = call.signAndSend(signer, options, ({ status, isError }) => {
      console.log(`Current status is ${status}`);
      if (status.isInBlock) {
        console.log(`Transaction included at blockHash ${status.asInBlock}`);
      } else if (status.isFinalized) {
        console.log(`Transaction finalized at blockHash ${status.asFinalized}`);
        unsub.then();
        return resolve();
      } else if (isError) {
        console.log('Transaction error');
        unsub.then();
        return reject();
      }
    });
  });
}

async function submitUnsigned(call: SubmittableExtrinsic<'promise'>): Promise<void> {
  return new Promise((resolve, reject) => {
    const unsub = call.send(({ status, isError }) => {
      console.log(`Current status is ${status}`);
      if (status.isInBlock) {
        console.log(`Transaction included at blockHash ${status.asInBlock}`);
      } else if (status.isFinalized) {
        console.log(`Transaction finalized at blockHash ${status.asFinalized}`);
        unsub.then();
        return resolve();
      } else if (isError) {
        console.log('Transaction error');
        unsub.then();
        return reject();
      }
    });
  });
}

// Transfer the relay chain asset to the parachain specified by paraId.
// Receiver address is same as the sender's.
async function transferRelayAssetToPara(
  relayApi: ApiPromise,
  signer: KeyringPair,
  paraId: number,
  receiver: string,
  amount: bigint
) {
  const receiverKeypair = new Keyring();
  receiverKeypair.addFromAddress(receiver);

  const feeAssetItem = 0;
  const weightLimit = 'Unlimited';

  const transferKind = paraId < 2000 ? 'limitedTeleportAssets' : 'limitedReserveTransferAssets';

  const teleport = relayApi.tx.xcmPallet[transferKind](
    { V3: { parents: 0, interior: { X1: { Parachain: paraId } } } }, //dest
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
            Concrete: { parents: 0, interior: 'Here' },
          },
          fun: {
            Fungible: amount,
          },
        },
      ],
    }, //asset
    feeAssetItem,
    weightLimit
  );
  await submitExtrinsic(signer, teleport, {});
}

async function openHrmpChannel(signer: KeyringPair, relayApi: ApiPromise, regionxApi: ApiPromise) {
  const openHrmpCall = relayApi.tx.hrmp.establishChannelWithSystem(1005);
  const xcmCall = regionxApi.tx.polkadotXcm.send(
    { V3: { parents: 1, interior: 'Here' } },
    {
      V3: [
        {
          WithdrawAsset: [
            {
              id: {
                Concrete: {
                  parents: 0,
                  interior: 'Here',
                },
              },
              fun: {
                Fungible: 5 * Math.pow(10, 10),
              },
            },
          ],
        },
        {
          BuyExecution: {
            fees: {
              id: {
                Concrete: {
                  parents: 0,
                  interior: 'Here',
                },
              },
              fun: {
                Fungible: 5 * Math.pow(10, 10),
              },
            },
            weightLimit: 'Unlimited',
          },
        },
        {
          Transact: {
            originKind: 'Native',
            requireWeightAtMost: {
              refTime: 1000000000,
              proofSize: 65536,
            },
            call: {
              encoded: openHrmpCall.method.toHex(),
            },
          },
        },
      ],
    }
  );

  const sudoCall = regionxApi.tx.sudo.sudo(xcmCall);

  return submitExtrinsic(signer, sudoCall, {});
}

async function sleep(milliseconds: number) {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}

const getAddressFromModuleId = (moduleId: string): string => {
  if (moduleId.length !== 8) {
    console.log('Module Id must be 8 characters (i.e. `py/trsry`)');
    return '';
  }
  const address = stringToU8a(('modl' + moduleId).padEnd(32, '\0'));
  return encodeAddress(address);
};

const getFreeBalance = async (api: ApiPromise, address: string): Promise<bigint> => {
  const {
    data: { free },
  } = (await api.query.system.account(address)).toJSON() as any;
  return BigInt(free);
};

function log(message: string) {
  // Green log.
  console.log('\x1b[32m%s\x1b[0m', message);
}

export {
  RELAY_ASSET_ID,
  log,
  sleep,
  openHrmpChannel,
  submitExtrinsic,
  submitUnsigned,
  transferRelayAssetToPara,
  getAddressFromModuleId,
  getFreeBalance,
};
