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

export interface DestructuredPlacement {
  x: number;
  y: number;
  colorIndex: number;
}

export interface Snapshot {
  imageData: Uint8Array; // 2 x 4-bit pixels per u8
  placementCount: number; // Number of placements in this bucket
  bucketIndex: number; // Number of 'bucket_size_sec' since START.
}

export type DoublePixel = number;

export interface GetAuthorRankInput {
  author: AgentPubKey
  bucketIndex: number
}

/** DEBUGGING API */
export interface PlaceAtInput {
  placement: DestructuredPlacement
  bucketIndex: number
}

export const Root = "ROOT";
