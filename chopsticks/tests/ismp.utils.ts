import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { submitExtrinsic, submitUnsigned } from './utils';
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
  return requests.toJSON()[0] as IsmpRequest;
}

async function makeIsmpResponse(
  regionXApi: ApiPromise,
  coretimeApi: ApiPromise,
  request: IsmpRequest,
  responderAddress: string
): Promise<void> {
  if (isGetRequest(request)) {
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
    const response = regionXApi.tx.ismp.handleUnsigned([
      {
        Response: {
          datagram: {
            Request: [request],
          },
          proof: {
            height: {
              id: {
                stateId: 1005,
                consensusStateId: 'PAS0',
              },
              height: request.get.height.toString(),
            },
            proof: substrateStateProof.toHex(),
          },
          signer: responderAddress,
        },
      },
    ]);
    await submitUnsigned(response);
  } else {
    new Error('Expected a Get request');
  }
}

const isGetRequest = (request: IsmpRequest): request is { get: Get } => {
  return (request as { get: Get }).get !== undefined;
};

export { makeIsmpResponse, queryRequest, ismpAddParachain };
