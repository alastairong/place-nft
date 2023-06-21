import { AppAgentClient, Record, AgentPubKeyB64, EntryHash, ActionHash, Action, encodeHashToBase64 } from '@holochain/client';
import { Snapshot, Placement, NftRecord, GetAuthorRankInput, DestructuredPlacement, GenerateBadgeInput, GenerateHrlInput, SaveNftInput, StealBadgeInput } from './types';

export class Interface {
    private client

    constructor(client: AppAgentClient) {
        this.client = client;
    }

    async myPubKey(): Promise<AgentPubKeyB64> {
        const pubKeyBytes = this.client.myPubKey;
        return encodeHashToBase64(pubKeyBytes);
    }

    async getSnapshotAt(bucketIndex: number): Promise<Snapshot | undefined> {
        console.log("getSnapshotAt called at bucketIndex: " + bucketIndex);
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'get_snapshot_at',
            payload: bucketIndex,
        });
    }

    async getPlacementsAt(bucketIndex: number): Promise<Placement[]> {
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'get_placements_at',
            payload: bucketIndex,
        });
    }

    async publishStartingSnapshot(): Promise<Snapshot | undefined> {
        console.log("Calling publishStartingSnapshot");
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'publish_starting_snapshot',
            payload: null,
        });
    }

    async publishSnapshotAt(bucketIndex: number): Promise<Snapshot | undefined> {
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'publish_snapshot_at',
            payload: bucketIndex,
        });
    }


    async placePixel(placement: DestructuredPlacement): Promise<ActionHash> {
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'place_pixel',
            payload: placement,
        });
    }

    async getAuthorRank(bucketIndex: number): Promise<number> {
        const payload: GetAuthorRankInput = {
            author: this.client.myPubKey,
            bucketIndex: bucketIndex,
          };

        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'get_author_rank',
            payload: payload,
        });
    }

    async alreadyPlaced(currentTime: number): Promise<boolean> {
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'already_placed',
            payload: currentTime,
        });
    }

    async getBadge(actionHash: ActionHash): Promise<string | undefined> { 
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'get_badge',
            payload: actionHash,
        });
    }

    async getBadgeAction(): Promise<ActionHash | undefined> { 
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'get_badge_action',
            payload: null,
        });
    }

    // Generates a badge from the final snapshot of the canvas, the user's history, and the user's signature
    async generateBadgeImage(ethAddress: string, ethSignedContents: string): Promise<ActionHash> { 
        const payload: GenerateBadgeInput = {
            ethAddress: ethAddress,
            ethSignedContents: ethSignedContents    
        };

        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'generate_badge',
            payload: payload,
        });
    }

    // Generates a pair of links between the HRL of (badge_action&&eth_address) to the badge action
    async generateHrl(badgeAction: ActionHash): Promise<string> {
        const payload: GenerateHrlInput = {
            badgeAction: badgeAction
        };
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'generate_hrl',
            payload: payload,
        });
    }

    async saveNft(nftId: string, contractAddress: string, hrl: string, badgeAction: ActionHash): Promise<ActionHash> {
        const payload: SaveNftInput = {
            nftId,
            contractAddress,
            hrl,
            badgeAction
        };
        console.log("calling saveNft with payload: " + JSON.stringify(payload))
        console.log(encodeHashToBase64(payload.badgeAction))
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'save_nft',
            payload: payload,
        });
    }

    async getNft(hrl: String): Promise<NftRecord | undefined> {
    
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'get_nft',
            payload: hrl,
        });
    }

    async viewNftImage(hrl: String): Promise<string | undefined> { 
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'view_nft_image',
            payload: hrl,
        });
    }

// ============================== DEMO FUNCTIONS ==============================

    // Generates a pair of links between the HRL of (badge_action&&eth_address) to the badge action
    async stealBadge(badgeAction: ActionHash, tokenUri: string): Promise<string> {
        const payload: StealBadgeInput = {
            badgeAction: badgeAction,
            tokenUri: tokenUri
        };
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'steal_badge',
            payload: payload,
        });
    }
}