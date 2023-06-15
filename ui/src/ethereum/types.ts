export interface Nft {
  contract: {
      address: string;
  };
  id: {
      tokenId: string;
      tokenMetadata: {
          tokenType: string;
      };
  };
  title: string;
  description: string;
  tokenUri: {
      raw: string;
      gateway: string;
  };
  media: {
      raw: string;
      gateway: string;
  }[];
  metadata: {
      name: string;
      description: string;
      image: string;
      external_url: string;
      attributes: {
          value: string;
          trait_type: string;
      }[];
  };
  timeLastUpdated: string;
}

export interface NftData {
  ownedNfts: Nft[];
  totalCount: number;
  blockHash: string;
}

export interface Message {
// eslint-disable-next-line @typescript-eslint/naming-convention
NFTAuthor: string
// eslint-disable-next-line @typescript-eslint/naming-convention
ethereumAddress: string
}

export interface SignatureReturn {
message: Message
signature: string
}