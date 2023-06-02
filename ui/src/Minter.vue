
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
            <button @click="connect">Connect Wallet</button>
          </div>
          <div v-if="isWalletConnected && !badgeImage">
            <p>No badge found. If you have participated in the game you can create an badge</p>
            <p>and NFT based on your contribution</p>
            <button>Create badge</button>
          </div>
          <div v-if="isWalletConnected && !!badgeImage"> 
            <img v-bind:src="badgeImage" />
            <div v-if="!nftRecord">
              <p>A badge has been found but no corresponding NFT</p>
              <p>Do you want to mint your NFT?</p>
              <button>Mint NFT</button>
            </div>
            <div v-if="!!nftRecord">
              <p> Your NFT has already been minted. You can find it here:</p>
              <p>PLACEHOLDER: Link to NFT based on NFTID? BY USING THE BADGE ACTION TO CALCULATE THE HRL AND FIND THE NFT</p>
            </div>
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
  import { NftRecord } from './place_nft/types';
  import { EthereumProvider } from '@walletconnect/ethereum-provider'

  // Main page logic
  export default defineComponent({
    data(): { loading: boolean; error: any, badgeAction: ActionHash | null, badgeImageRaw: any, badgeImage: any, walletProvider: any, hrl: String, nftRecord: NftRecord | null, isConnected: Boolean } {
      return {
        loading: true,
        error: undefined,
        badgeAction: null,
        badgeImageRaw: null,
        badgeImage: null,
        walletProvider: null,
        hrl: "",
        nftRecord: null,
        isConnected: true
      }
    },
    async mounted() {
      this.getBadgeImage()
      this.loading = false

      this.walletProvider = await EthereumProvider.init({
        projectId: "0bdbc2e75cc18b77f5097aa944842208",
        showQrModal: true,
        qrModalOptions: { themeMode: 'dark' },
        chains: ["eip155:5"], // Goerli https://eips.ethereum.org/EIPS/eip-155#list-of-chain-ids
        methods: ['eth_sendTransaction', 'personal_sign'],
        events: ['chainChanged', 'accountsChanged'],
        metadata: {
          name: 'Place-NFT',
          description: 'My Dapp description',
          url: 'https://my-dapp.com',
          icons: ['https://my-dapp.com/logo.png']
        }
      })

      this.walletProvider.on('connect', this.isConnected = true)
      
    },
    methods: {
      async getBadgeImage() {
        this.badgeAction = await this.happ.getBadgeAction()
      
        if (!!this.badgeAction) {
          this.badgeImageRaw = await this.happ.getBadge(this.badgeAction)
          this.hrl = this.badgeAction + "";
          this.getNftRecord(this.hrl);
        }
      },

      async getNftRecord(hrl: String) {
        this.nftRecord = await this.happ.getNft(hrl)
      },

      async saveNftRecord(nftId: String, contractAddress: String) {
        this.nftRecord = {
          nftId,
          contractAddress
        }
        
        await this.happ.saveNft(nftId, contractAddress, this.hrl)
      },

      connect() {
        // your connect logic here
        this.walletProvider?.connect();
      }
    },
    watch: {
      badgeImageRaw(newBadgeImageRaw) {
        let imageblob = new Blob([new Uint8Array(newBadgeImageRaw)], { type: 'image/png' });
        this.badgeImage = URL.createObjectURL(imageblob);
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


 


