import { Enum, Struct, u32, u64, u8, Vector, Option } from "scale-ts";

export type StateMachine = { Polkadot: number } | { Kusama: number };

export interface Get {
  source: string;
  dest: string;
  nonce: bigint;
  from: string;
  keys: Array<string>;
  height: bigint;
  context: string;
  timeout_timestamp: bigint;
}

export type IsmpRequest = { post: any } | { get: Get };

export const REGIONX_API_TYPES = {
  CoreIndex: 'u32',
  CoreMask: 'Vec<u8>',
  Timeslice: 'u32',
  RegionId: {
    begin: 'Timeslice',
    core: 'CoreIndex',
    mask: 'CoreMask',
  },
  RegionRecord: {
    end: 'Timeslice',
    owner: 'AccountId',
    paid: 'Option<Balance>',
  },
  HashAlgorithm: {
    _enum: ['Keccak', 'Blake2'],
  },
  StateMachineProof: {
    hasher: 'HashAlgorithm',
    storage_proof: 'Vec<Vec<u8>>',
  },
  SubstrateStateProof: {
    _enum: {
      OverlayProof: 'StateMachineProof',
      StateProof: 'StateMachineProof',
    },
  },
  LeafIndexQuery: {
    commitment: 'H256',
  },
  ConsensusStateId: '[u8; 4]',
  Relay: {
    relay: 'ConsensusStateId',
    para_id: 'u32'
  },
  StateMachine: {
    _enum: {
      Evm: 'u32',
      Polkadot: 'u32',
      Kusama: 'u32',
      Substrate: 'ConsensusStateId',
      Tendermint: 'ConsensusStateId',
      Relay: 'Relay'
    }
  },
  Post: {},
  Get: {
    source: 'StateMachine',
    dest: 'StateMachine',
    nonce: 'u64',
    from: 'Vec<u8>',
    keys: 'Vec<Vec<u8>>',
    height: 'u64',
    context: 'Vec<u8>',
    timeout_timestamp: 'u64',
  },
  Request: {
    _enum: {
      Post: 'Post',
      Get: 'Get',
    },
  },
};

export const REGIONX_CUSTOM_RPC = {
  ismp: {
    queryRequests: {
      description: '',
      params: [
        {
          name: 'query',
          type: 'Vec<LeafIndexQuery>',
        },
      ],
      type: 'Vec<Request>',
    },
  },
};
