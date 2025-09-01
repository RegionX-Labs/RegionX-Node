import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { submitExtrinsic, submitUnsigned } from './common';
import { Get, IsmpRequest } from './types';

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

  // The issue is that requests are empty. That is why it is passing as well...
  // At least we know handleUnsigned is handled successfully with zero requests.
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
        }],
      },
      proof: {
        height: {
          id: {
            stateId: {
              Kusama: 1005,
            },
            consensusStateId: 'PAS0',
          },
          height: request.get.height.toString(),
        },
        proof: substrateStateProof.toHex(),
      },
      signer: responderAddress,
    },
  }];
  
  console.log(response);

  await submitUnsigned(regionXApi.tx.ismp.handleUnsigned(response));
}

const isGetRequest = (request: IsmpRequest): request is { get: Get } => {
  return (request as { get: Get }).get !== undefined;
};

export { makeIsmpResponse, queryRequest, ismpAddParachain };
