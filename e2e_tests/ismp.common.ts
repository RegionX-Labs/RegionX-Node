import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { sleep, submitExtrinsic, submitUnsigned } from './common';
import { Get, IsmpRequest } from './types';
import { encodePacked, keccak256, toHex } from 'viem';
import { keccakAsHex } from '@polkadot/util-crypto';

async function ismpAddParachain(signer: KeyringPair, regionXApi: ApiPromise) {
  const addParaCall = regionXApi.tx.ismpParachain.addParachain([{ id: 1005, slotDuration: 6000 }]);
  const sudoCall = regionXApi.tx.sudo.sudo(addParaCall);
  return submitExtrinsic(signer, sudoCall, {});
}

async function queryRequest(regionxApi: ApiPromise, commitment: string): Promise<IsmpRequest> {
  const leafIndex = regionxApi.createType('LeafIndexQuery', { commitment });
  const requests = await (regionxApi as any).rpc.ismp.queryRequests([leafIndex]);
  // We only requested a single request so we only get one in the response.
  console.log(requests.toJSON());
  return requests.toJSON()[0] as IsmpRequest;
}

async function makeIsmpResponse(
  regionXApi: ApiPromise,
  coretimeApi: ApiPromise,
  request: IsmpRequest,
  responderAddress: string
): Promise<void> {
  console.log(request);
  if (!isGetRequest(request)) {
    console.log('not get request');
    new Error('Expected a Get request');
    return;
  }

  const hashAt = (
    await coretimeApi.query.system.blockHash(Number(request.get.height))
  ).toString();
  const proofData = await coretimeApi.rpc.state.getReadProof([request.get.keys[0]], hashAt);

  const stateMachineProof = regionXApi.createType('StateMachineProof', {
    hasher: 'Blake2',
    storage_proof: proofData.proof,
  });

  const substrateStateProof = regionXApi.createType('SubstrateStateProof', {
    StateProof: stateMachineProof,
  });

  const response = [{
    Response: {
      datagram: {
        Request: [{
          Get: {
            source: { Kusama: 2000 },
            dest: { Kusama: 1005 },
            nonce: request.get.nonce,
            from: request.get.from,
            keys: request.get.keys,
            height: request.get.height,
            context: request.get.context,
            timeoutTimestamp: request.get.timeout_timestamp,
          }
        }]
      },
      proof: {
        height: {
          id: {
            stateId: { Kusama: 1005 },
            consensusStateId: 'PAS0',
          },
          height: request.get.height,
        },
        proof: substrateStateProof.toHex(),
      },
      signer: responderAddress,
    },
  }];

  // console.log(getRequestCommitment({
  //   source: 'KUSAMA-2000',
  //   dest: 'KUSAMA-1005',
  //   nonce: request.get.nonce,
  //   from: request.get.from,
  //   keys: request.get.keys,
  //   height: request.get.height,
  //   context: request.get.context,
  //   timeoutTimestamp: request.get.timeout_timestamp,
  // }));

  console.log(JSON.stringify(response));

  await submitUnsigned(regionXApi.tx.ismp.handleUnsigned(response));
  await sleep(360 * 1000);
}

export function getRequestCommitment(regionXApi: ApiPromise, get: any): string {
	// const keysEncoding = "0x".concat(get.keys.map((key: string) => key.slice(2)).join(""))
	// return keccak256(
	// 	encodePacked(
	// 		["bytes", "bytes", "uint64", "uint64", "uint64", "bytes", "bytes", "bytes"],
	// 		[
	// 			toHex(get.source),
	// 			toHex(get.dest),
	// 			get.nonce,
	// 			get.height,
	// 			get.timeoutTimestamp,
	// 			get.from,
	// 			keysEncoding as any,
	// 			get.context,
	// 		],
	// 	),
	// )
  const reqEnum = regionXApi.createType('Request', {
    Get: {
      source: get.source,                     // e.g. { Kusama: 1005 }
      dest: get.dest,                         // e.g. { Kusama: 2000 }
      nonce: get.nonce,             // u64
      from: get.from,                  // Bytes
      keys: get.keys,              // Vec<Vec<u8>>
      height: get.height,           // u64
      timeout_timestamp: get.timeoutTimestamp, // u64
    }
  });

  const bytes = reqEnum.toU8a();

  // ISMP uses keccak256 for request/response commitments
  const commitment = keccakAsHex(bytes);
  return commitment;
}

const isGetRequest = (request: IsmpRequest): request is { get: Get } => {
  return (request as { get: Get }).get !== undefined;
};

export { makeIsmpResponse, queryRequest, ismpAddParachain };
