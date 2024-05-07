import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { CONFIG, INITIAL_PRICE, UNIT, CORE_COUNT } from "../consts";
import { submitExtrinsic, sleep } from "../common";
import { KeyringPair } from "@polkadot/keyring/types";
import { CoreMask, getEncodedRegionId, Id, RegionId } from "coretime-utils";
import assert from 'node:assert';

async function run(_nodeName: any, networkInfo: any, _jsArgs: any) {
  const { wsUri: regionXUri } = networkInfo.nodesByName["regionx-collator01"];
  const { wsUri: coretimeUri } = networkInfo.nodesByName["coretime-collator01"];
  const { wsUri: rococoUri } = networkInfo.nodesByName["rococo-validator01"];

  const regionXApi = await ApiPromise.create({ provider: new WsProvider(regionXUri) });
  const rococoApi = await ApiPromise.create({ provider: new WsProvider(rococoUri) });
  const coretimeApi = await ApiPromise.create({ provider: new WsProvider(coretimeUri), types: { Id } });

  // account to submit tx
  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");

  const setCoretimeXcmVersion = coretimeApi.tx.polkadotXcm.forceDefaultXcmVersion([3]);
  await submitExtrinsic(alice, coretimeApi.tx.sudo.sudo(setCoretimeXcmVersion), {});

  await openHrmpChannel(alice, rococoApi, 1005, 2000);
  await openHrmpChannel(alice, rococoApi, 2000, 1005);

  await configureBroker(coretimeApi, alice);
  await startSales(coretimeApi, alice);

  const setBalanceCall = coretimeApi.tx.balances.forceSetBalance(alice.address, 1000 * UNIT);
  await submitExtrinsic(alice, coretimeApi.tx.sudo.sudo(setBalanceCall), {});

  const regionId = await purchaseRegion(coretimeApi, alice);
  const encodedId = getEncodedRegionId(regionId, coretimeApi).toString();

  const receiverKeypair = new Keyring();
  receiverKeypair.addFromAddress(alice.address);

  const feeAssetItem = 0;
  const weightLimit = "Unlimited";
  const reserveTransferToRegionX = coretimeApi.tx.polkadotXcm.limitedReserveTransferAssets(
    { V3: { parents: 1, interior: { X1: { Parachain: 2000 } } } }, //dest
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
            Concrete: {
              parents: 0,
              interior: { X1: { PalletInstance: 50 } },
            },
          },
          fun: {
            NonFungible: {
              Index: encodedId
            }
          },
        },
      ],
    }, //asset
    feeAssetItem,
    weightLimit,
  );
  await submitExtrinsic(alice, reserveTransferToRegionX, {});

  await sleep(5000);

  const regions = (await regionXApi.query.regions.regions.entries());
  assert.equal(regions.length, 1);
  assert.deepStrictEqual(regions[0][0].toHuman(), [{ begin: '34', core: '0', mask: "0xffffffffffffffffffff" }]);
  assert.deepStrictEqual(regions[0][1].toHuman(), { owner: alice.address, record: 'Pending' });

  const reserveTransferToCoretime = coretimeApi.tx.polkadotXcm.limitedReserveTransferAssets(
    { V3: { parents: 1, interior: { X1: { Parachain: 1005 } } } }, //dest
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
            Concrete: {
              parents: 1,
              interior: { X2: [ {Parachain: 2000}, { PalletInstance: 50 }] },
            },
          },
          fun: {
            NonFungible: {
              Index: encodedId
            }
          },
        },
      ],
    }, //asset
    feeAssetItem,
    weightLimit,
  );
  await submitExtrinsic(alice, reserveTransferToCoretime, {});
}

async function openHrmpChannel(signer: KeyringPair, relayApi: ApiPromise, senderParaId: number, recipientParaId: number) {
  const newHrmpChannel = [
    senderParaId,
    recipientParaId,
    8, // Max capacity
    102400, // Max message size
  ];

  const openHrmp = relayApi.tx.parasSudoWrapper.sudoEstablishHrmpChannel(...newHrmpChannel);
  const sudoCall = relayApi.tx.sudo.sudo(openHrmp);

  const callTx = async (resolve: any) => {
    const unsub = await sudoCall.signAndSend(signer, (result) => {
      if (result.status.isInBlock) {
        unsub();
        resolve();
      }
    });
  };

  return new Promise(callTx);
}

async function configureBroker(coretimeApi: ApiPromise, signer: KeyringPair): Promise<void> {
  const configCall = coretimeApi.tx.broker.configure(CONFIG);
  const sudo = coretimeApi.tx.sudo.sudo(configCall)
  return submitExtrinsic(signer, sudo, {});
}

async function startSales(coretimeApi: ApiPromise, signer: KeyringPair): Promise<void> {
  const startSaleCall = coretimeApi.tx.broker.startSales(INITIAL_PRICE, CORE_COUNT);
  const sudo = coretimeApi.tx.sudo.sudo(startSaleCall)
  return submitExtrinsic(signer, sudo, {});
}

async function purchaseRegion(coretimeApi: ApiPromise, buyer: KeyringPair): Promise<RegionId> {
  const callTx = async (resolve: (regionId: RegionId) => void) => {
    const purchase = coretimeApi.tx.broker.purchase(INITIAL_PRICE * 2);
    const unsub = await purchase.signAndSend(buyer, async (result: any) => {
      if (result.status.isInBlock) {
        const regionId = await getRegionId(coretimeApi);
        unsub();
        resolve(regionId);
      }
    });
  };

  return new Promise(callTx);
}

async function getRegionId(coretimeApi: ApiPromise): Promise<RegionId> {
  const events: any = await coretimeApi.query.system.events();

  for (const record of events) {
    const { event } = record;
    if (event.section === "broker" && event.method === "Purchased") {
      const data =  event.data[1].toHuman();
      return { begin: data.begin, core: data.core, mask: new CoreMask(data.mask) }
    }
  }

  return { begin: 0, core: 0, mask: CoreMask.voidMask() };
}

export { run };
