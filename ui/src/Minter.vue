
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
            <button @click="createBadge">Create Badge</button>
          </div>
          <div v-if="isWalletConnected && !!badgeImage"> 
            <img v-bind:src="badgeImage" />
            <div v-if="!nftRecord">
              <p>A badge has been found but no corresponding NFT</p>
              <p>Do you want to mint your NFT?</p>
              <button @click="mintNft">Mint NFT</button>
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
  import { EthereumProvider } from '@walletconnect/ethereum-provider';
  import { getAddress } from 'viem'

  // Main page logic
  export default defineComponent({
    data(): { loading: boolean; error: any, badgeAction: ActionHash | null, badgeImageRaw: any, badgeImage: any, walletProvider: any, hrl: string, nftRecord: NftRecord | null, isWalletConnected: Boolean, walletAddress: string } {
      return {
        loading: true,
        error: undefined,
        badgeAction: null,
        badgeImageRaw: null,
        badgeImage: null,
        walletProvider: null,
        hrl: "",
        nftRecord: null,
        isWalletConnected: false,
        walletAddress: ""
      }
    },
    async mounted() {
      this.getBadgeImage()
      this.loading = false

      this.walletProvider = await EthereumProvider.init({
        projectId: "0bdbc2e75cc18b77f5097aa944842208",
        showQrModal: true,
        qrModalOptions: { themeMode: 'dark' },
        chains: [5], // Goerli https://eips.ethereum.org/EIPS/eip-155#list-of-chain-ids
        methods: ['eth_sendTransaction', 'personal_sign'],
        events: ['connect', 'accountsChanged'],
        metadata: {
          name: 'Place-NFT',
          description: 'My Dapp description',
          url: 'https://my-dapp.com',
          icons: ['https://my-dapp.com/logo.png']
        }
      })

      this.walletProvider.on('connect', async () => {
        this.isWalletConnected = true
        const accounts = await this.walletProvider.enable()
        this.walletAddress = getAddress(accounts[0]!)

        if (!!this.badgeAction) {
          this.hrl = this.badgeAction + this.walletAddress
          this.nftRecord = await this.happ.getNft(this.hrl)
        }
      })
      
    },
    methods: {
      async getBadgeImage() {
        this.badgeAction = await this.happ.getBadgeAction()
      
        if (!!this.badgeAction) {
          this.badgeImageRaw = await this.happ.getBadge(this.badgeAction)
        }
      },

      async getNftRecord(hrl: String) {
        this.nftRecord = await this.happ.getNft(hrl)
      },

      async mintNft(contractAddress: String) {
        
        const nftId = "TBD" // make contract call
        
        this.nftRecord = {
          nftId,
          contractAddress
        }
        
        await this.happ.saveNft(nftId, contractAddress, this.hrl)
      },

      connect() {
        // your connect logic here
        this.walletProvider?.connect();
      },

      async createBadge() {
        this.badgeAction = await this.happ.generateBadgeImage(this.walletAddress, "Signed Placeholder")
        this.badgeImageRaw = await this.happ.getBadge(this.badgeAction)
      },

    },
    watch: {
      badgeImageRaw(newBadgeImageRaw) {
        let imageblob = new Blob([new Uint8Array(newBadgeImageRaw)], { type: 'image/png' });
        this.badgeImage = URL.createObjectURL(imageblob);
      },

      badgeAction(newBadgeAction) {
        if (!!this.walletAddress) {
          this.hrl = newBadgeAction + this.walletAddress;
        }
      },

      walletAddress(newWalletAddress) {
        if (!!this.badgeAction) {
          this.hrl = this.badgeAction + newWalletAddress;
        }
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


 


