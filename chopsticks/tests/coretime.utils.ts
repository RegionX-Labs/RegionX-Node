import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { RegionId } from "coretime-utils";

const UNIT = 10n ** 12n; // KSM has 12 decimals
const INITIAL_PRICE = 1n * UNIT;

export async function purchaseRegion(
  coretimeApi: ApiPromise,
  buyer: KeyringPair
): Promise<RegionId | null> {
  const callTx = async (resolve: (regionId: RegionId | null) => void) => {
    const purchase = coretimeApi.tx.broker.purchase(INITIAL_PRICE * 10n);
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

async function getRegionId(coretimeApi: ApiPromise): Promise<RegionId | null> {
  const events: any = await coretimeApi.query.system.events();

  for (const record of events) {
    const { event } = record;
    if (event.section === 'broker' && event.method === 'Purchased') {
      const data = event.data[1].toHuman();
      return data;
    }
  }

  return null;
}
