import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { submitExtrinsic, submitUnsigned } from './common';
import { Get, IsmpRequest } from './types';
import { hexToU8a } from '@polkadot/util';
import { SubmittableExtrinsic } from '@polkadot/api/types';
import { IGetRequest, SubstrateChain } from './hyperbridge-sdk';

async function ismpAddParachain(signer: KeyringPair, regionXApi: ApiPromise) {
  const addParaCall = regionXApi.tx.ismpParachain.addParachain([{ id: 1005, slotDuration: 6000 }]);
  const sudoCall = regionXApi.tx.sudo.sudo(addParaCall);
  return submitExtrinsic(signer, sudoCall, {});
}

async function queryRequest(regionxApi: ApiPromise, commitment: string): Promise<IGetRequest> {
  const leafIndex = regionxApi.createType('LeafIndexQuery', { commitment });
  const requests = await (regionxApi as any).rpc.ismp.queryRequests([leafIndex]);
  // We only requested a single request so we only get one in the response.
  console.log(requests.toJSON());
  return requests.toJSON()[0].get as IGetRequest;
}

async function makeIsmpResponse(
  regionxWs: string,
  coretimeApi: ApiPromise,
  request: IGetRequest,
  responderAddress: string
): Promise<void> {
  const hashAt = (
    await coretimeApi.query.system.blockHash(Number(request.height))
  ).toString();
  const proofData = await coretimeApi.rpc.state.getReadProof([request.keys[0]], hashAt);

  const regionx = new SubstrateChain({
    ws: regionxWs,
    hasher: 'Blake2',
  });

  const tx = regionx.encode({
    kind: 'GetResponse',
    proof: {
      consensusStateId: 'PAS0',
      height: request.height,
      proof: proofData.toHex(),
      stateMachine: 'KUSAMA-1005'
    },
    responses: [{get: request, values: [{key: request.keys[0], value: '0x0'}] }],
    signer: responderAddress as any
  });

  const call = regionx.api?.tx.ismp.handleUnsigned(hexToU8a(tx).slice(2)) as SubmittableExtrinsic<'promise'> | undefined;
  if(!call) return;

  await submitUnsigned(call);
}

const isGetRequest = (request: IsmpRequest): request is { get: Get } => {
  return (request as { get: Get }).get !== undefined;
};

export { makeIsmpResponse, queryRequest, ismpAddParachain };
