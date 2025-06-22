import { ApiPromise, Keyring } from "@polkadot/api";
import { SignerOptions, SubmittableExtrinsic } from "@polkadot/api/types";
import { KeyringPair } from "@polkadot/keyring/types";

export const keyring = new Keyring({ type: "sr25519" });

export function log(message: string) {
  // Green log.
  console.log("\x1b[32m%s\x1b[0m", message);
}

export async function setBalance(api: ApiPromise, who: string, balance: string) {
  log(`Setting balance of ${who} to ${balance}`);

  const setBalanceCall = api.tx.balances.forceSetBalance(who, balance);
  return force(api, setBalanceCall);
}

export async function submitExtrinsic(
  signer: KeyringPair,
  call: SubmittableExtrinsic<"promise">,
  options: Partial<SignerOptions>
): Promise<void> {
  try {
    return new Promise((resolve, _reject) => {
      const unsub = call.signAndSend(signer, options, (result) => {
        console.log(`Current status is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
        } else if (result.status.isFinalized) {
          console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
          unsub.then();
          return resolve();
        } else if (result.isError) {
          console.log("Transaction error");
          unsub.then();
          return resolve();
        }
      });
    });
  } catch (e) {
    console.log(e);
  }
}

export async function submitUnsigned(call: SubmittableExtrinsic<'promise'>): Promise<void> {
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

export async function force(api: ApiPromise, call: SubmittableExtrinsic<"promise">): Promise<void> {
  const sudoCall = api.tx.sudo.sudo(call);

  const alice = keyring.addFromUri("//Alice");

  await submitExtrinsic(alice, sudoCall, {});
}

export async function sleep(milliseconds: number) {
  return new Promise((resolve) => setTimeout(resolve, milliseconds));
}
