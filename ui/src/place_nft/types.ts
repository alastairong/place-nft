import { 
  Record, 
  ActionHash, 
  SignedActionHashed,
  DnaHash,
  EntryHash, 
  AgentPubKey,
  Create,
  Update,
  Delete,
  CreateLink,
  DeleteLink
} from '@holochain/client';

export type PlaceSignal = {
  type: 'EntryCreated';
  action: SignedActionHashed<Create>;
  app_entry: EntryTypes;
} 

export type EntryTypes =
 | ({ type: 'Placement'; } & Placement)
 | ({  type: 'Snapshot'; } & Snapshot);


export type Placement = {
  pixel: number;
}

export type DestructuredPlacement = {
  x: number;
  y: number;
  colorIndex: number;
}

export type Snapshot = {
  imageData: Uint8Array; // 2 x 4-bit pixels per u8
  placementCount: number; // Number of placements in this bucket
  bucketIndex: number; // Number of 'bucket_size_sec' since START.
}

export type DoublePixel = number;

export type NftRecord = {
  nftId: String,
  contractAddress: String
}

export interface GetAuthorRankInput {
  author: AgentPubKey
  bucketIndex: number
}

export interface GenerateBadgeInput {
  ethAddress: string
  ethSignedContents: string
}

export interface GenerateHrlInput {
  badgeAction: ActionHash
}

export interface SaveNftInput {
  nftId: String,
  contractAddress: String,
  hrl: String
};


/** DEBUGGING API */
export interface PlaceAtInput {
  placement: DestructuredPlacement
  bucketIndex: number
}

export const Root = "ROOT";
