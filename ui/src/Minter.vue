
<template>
    <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
    
    <div v-else style="display: flex; flex-direction: column">
      <div class="modal-container">
        <div class="modal">
          <h1>Game has ended!</h1>
          <div v-if="!isWalletConnected">
            <h2>Please connect your wallet to proceed</h2>
          </div>
          <div v-if="isWalletConnected && !badgeImage">
            <p>No badge found. If you have participated in the game you can create an badge</p>
            <p>and NFT based on your contribution</p>
            <button>Create badge</button>
          </div>
          <div v-if="isWalletConnected && !!badgeImage && !linkToNFT">
            <p>PLACEHOLDER FOR BADGE IMAGE</p>
            <p>A badge has been found but no corresponding NFT</p>
            <p>Do you want to mint your NFT?</p>
            <button>Mint NFT</button>
          </div>
          <div v-if="!!linkToNFT">
            <p>PLACEHOLDER FOR BADGE IMAGE</p>
            <p> Your NFT has already been minted. You can find it here:</p>
            <p>PLACEHOLDER: Link to NFT based on NFTID? BY USING THE BADGE ACTION TO CALCULATE THE HRL AND FIND THE NFT</p>
          </div>
        </div>
      </div>
    </div>

  </template>
  
  <script lang="ts">
  import { defineComponent, inject, ComputedRef } from 'vue';
  import { Interface } from './place_nft/interface';
  import '@material/mwc-circular-progress';
  import { ETHEREUM_NETWORK } from './ethereum/consts'
  import { AppAgentClient, Record, AgentPubKeyB64, EntryHash, ActionHash, Action } from '@holochain/client';

  // Main page logic
  export default defineComponent({
    data(): { loading: boolean; error: any, badgeAction: ActionHash | undefined, badgeImage: any, isWalletConnected: boolean, linkToNFT: string | undefined } {
      return {
        loading: true,
        error: undefined,
        badgeAction: undefined,
        badgeImage: undefined,
        isWalletConnected: false,
        linkToNFT: undefined
      }
    },
    async mounted() {
      // TODO
    },
    methods: {
      async getBadgeImage() {
        this.badgeAction = await this.happ.getBadgeAction()
        this.badgeImage = await this.happ.getBadge(this.badgeAction)
      },

      async linkToNFT() {
        this.linkToNFT = await this.happ.getNFT()
      },
    },
    setup() {
      const happ = (inject('placeInterface') as ComputedRef<Interface>).value;
      return {
        happ,
      };
    },
  })

</script>


 


