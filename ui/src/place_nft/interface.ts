import { AppAgentClient, Record, AgentPubKeyB64, EntryHash, ActionHash, Action } from '@holochain/client';
import { Snapshot, Placement, GetAuthorRankInput, DestructuredPlacement, GenerateBadgeInput, GenerateHrlInput } from './types';

export class Interface {
    private client

    constructor(client: AppAgentClient) {
        this.client = client;
    }

    async getSnapshotAt(bucketIndex: number): Promise<Snapshot | null> {
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

    async publishStartingSnapshot(): Promise<Snapshot | null> {
        console.log("Calling publishStartingSnapshot");
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'publish_starting_snapshot',
            payload: null,
        });
    }

    async publishSnapshotAt(bucketIndex: number): Promise<Snapshot | null> {
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

    async getNFTimage(): Promise<any> { // TODO: Create a type for image
        return this.client.callZome({
            cap_secret: null,
            role_name: 'place_nft',
            zome_name: 'place',
            fn_name: 'get_badge',
            payload: null,
        });
    }

    // Generates a badge from the final snapshot of the canvas, the user's history, and the user's signature
    async generateNFTimage(ethAddress: string, ethSignedContents: string): Promise<any> { // TODO: Create a type for image
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

    // Generates a link from a badge's ethereum address to the badge action
    async generateHrl(badgeAction: ActionHash): Promise<ActionHash> {
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
}