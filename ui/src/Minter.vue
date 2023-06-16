
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
          <div v-if="isWalletConnected && !badgeAction">
            <p>No badge found. If you have participated in the game you can create an badge</p>
            <p>and NFT based on your contribution</p>
            <button @click="createBadge">Create Badge</button>
          </div>
          <div v-if="isWalletConnected && !!badgeAction"> 
            <img v-bind:src="badgeImage" />
            <div v-if="!nftRecord">
              <p>A badge has been found but no corresponding NFT</p>
              <p>Do you want to mint your NFT?</p>
              <button @click="mintNft">Mint NFT</button>
            </div>
            <div v-if="!!nftRecord">
              <p> Your NFT has already been minted!</p>
            </div>
            <div v-if="!isWalletConnected">
              <h2>Please connect your wallet to proceed</h2>
              <button @click="connect">Connect Wallet</button>
            </div>
          </div>

          <div v-if="isWalletConnected">
            <h2>NFT Viewer</h2>
            <p>You have the following NFTs</p>
            <ul>
              <li v-for="nft in usersNfts" :key="nft.nftId">
                {{ nft.nftId }}: <a v-if="nft.hrl" @click="viewNft(nft.hrl, $event)">{{ nft.hrl }}</a><span v-else>No HRL</span>
              </li>
            </ul>
            <img v-if="nftImage" :src="nftImage" />
          </div>
        </div>
      </div>
    </div>

  </template>
  
  <script lang="ts">
  import { defineComponent, inject, ComputedRef } from 'vue';
  import { Interface } from './place_nft/interface';
  import { Nft, NftData } from './ethereum/types';
  import '@material/mwc-circular-progress';
  import { CONTRACT_ADDRESS } from './ethereum/consts'
  import { AppAgentClient, Record, AgentPubKeyB64, EntryHash, ActionHash, Action, encodeHashToBase64 } from '@holochain/client';
  import { NftRecord, NftTokenUri } from './place_nft/types';
  import { ethers } from "ethers";
  import contractArtifact from '../../contract/artifacts/contracts/place_nft.sol/placeNFT.json';
  import WalletConnectProvider from "@walletconnect/web3-provider";
  import { markRaw } from 'vue';
  import { Alchemy, Network } from "alchemy-sdk";
  
  // Main page logic
  export default defineComponent({
    data(): { loading: boolean; error: any, badgeAction: ActionHash | null, badgeImageRaw: any, badgeImage: any, walletProvider: any, signer: any, hrl: string, nftRecord: NftRecord | null, isWalletConnected: Boolean, walletAddress: any, usersNfts: NftTokenUri[] | null, nftImage: any } {
      return {
        loading: true,
        error: undefined,
        badgeAction: null,
        badgeImageRaw: null,
        badgeImage: null,
        walletProvider: null,
        signer: null,
        hrl: "",
        nftRecord: null,
        isWalletConnected: false,
        walletAddress: null,
        usersNfts: null,
        nftImage: null,
      }
    },
    async mounted() {
      console.log("getting badge Image")
      this.getBadgeImage()
      console.log("got badge Image: " + this.badgeImageRaw)
      this.loading = false
      console.log("Initializing wallet provider")

      const provider = new WalletConnectProvider({
        infuraId: "a5238372835346588d9c347de0a2226e",
      });
      this.walletProvider = markRaw(provider);
      // console.log("Close any existing connection")
      // this.walletProvider.disconnect();
      // console.log("test logging?")
      // console.log(this.walletProvider)
      // console.log("test logging with ", this.walletProvider.connected)
      // console.log("test logging with2 ", this.walletProvider.accounts[0])
      this.isWalletConnected = this.walletProvider.connected;
      console.log("Initialized wallet provider, listening for connection")
      this.walletProvider.on('connect', async () => {
        this.isWalletConnected = true
      })
      
    },
    methods: {
      async getBadgeImage() {
        console.log("getting badge image")
        try {
          this.badgeAction = await this.happ.getBadgeAction()
        } catch (e) {
          console.log(e)
        }
        
        if (!!this.badgeAction) {
          this.badgeImageRaw = await this.happ.getBadge(this.badgeAction)
        }
      },

      async getNftRecord(hrl: String) {
        this.nftRecord = await this.happ.getNft(hrl)
      },

      async mintNft() {
        
        // instantiate smart contract
        const nftContractInstance = new ethers.Contract(CONTRACT_ADDRESS, contractArtifact.abi, this.signer);
        console.log("nftContractInstance")
        console.log(nftContractInstance)
        // make call to smart contract method
        try {
          if (!!this.badgeAction) {
            const tx = await nftContractInstance.mintNFT(encodeHashToBase64(this.badgeAction)); // make contract call
            console.log("tx")
            console.log(tx)
            const receipt = await tx.wait(); // wait for tx to be mined
            console.log("receipt")
            console.log(receipt)
            // look for broadcasted event with nftId
            const event = receipt.events.find((event: any) => event.event === 'Minted');
            const newItemId = event.args.newItemId;
            const nftId = newItemId.toNumber();
            console.log("nftId: ", nftId)
            this.nftRecord = {
              nftId,
              contractAddress: CONTRACT_ADDRESS
            }
            console.log("saving nft")
            console.log(this.hrl)
            await this.happ.saveNft(nftId, CONTRACT_ADDRESS, this.hrl, this.badgeAction)
          }
             
        } catch (e){
          console.log(e)
        }
      },

      async checkForNfts() {
        console.log("checking for nfts")
        const nftContractInstance = new ethers.Contract(CONTRACT_ADDRESS, contractArtifact.abi, this.signer);
      },

      async connect() {
        // your connect logic here
        console.log("connecting");
        await this.walletProvider.enable();
        console.log(this.walletProvider)
        console.log("test logging with ", this.walletProvider.connected)
        console.log("test logging with2 ", this.walletProvider.accounts[0])
        this.isWalletConnected = this.walletProvider.connected;
      },

      async createBadge() {
        try { 
          console.log("creating badge")
          this.badgeAction = await this.happ.generateBadgeImage(this.walletAddress, "Signed Placeholder")
        } catch (e) {
          console.log(e)
        }
        
        try {
          console.log("fetching and storing badge")
          if(!!this.badgeAction) {
            this.badgeImageRaw = await this.happ.getBadge(this.badgeAction)
          }
          
        } catch (e) {
          console.log(e)
        }
      },

      async fetchUserNfts() {
        try {
          // Super hacky to put this stuff here, BUT it's because this code shouldn't 
          // really be necessary if there was a proper gateway to retrieve NFTs
          const config = {
            apiKey: "aFQ94Sn1r_OOp0K6Bmge-3hfVsRB0-yD",
            network: Network.ETH_GOERLI,
          };
          const alchemy = new Alchemy(config);
          const rawData = await alchemy.nft.getNftsForOwner(this.walletAddress);
          this.usersNfts = rawData.ownedNfts
          .filter(nft => nft.contract.address.toLowerCase() === "0xCa70AE825357b2f062B51b324a8be238132Cb314".toLowerCase())
          .map(nft => ({
            nftId: nft.tokenId,
            hrl: nft.tokenUri ? nft.tokenUri.raw : undefined
          }));
        } catch (e) {
          console.log(e)
        }
      },

      async viewNft(hrl: string, event: Event) {
        console.log('viewNft called with', hrl, event);
        event.preventDefault();
        try {
          this.nftImage = await this.happ.viewNftImage(hrl)
        } catch (e) {
          console.log(e)
        }
      },
    },
    watch: {
      badgeImageRaw(newBadgeImageRaw) {
        console.log("badgeImageRaw changed")
        console.log(newBadgeImageRaw)
        let imageblob = new Blob([new Uint8Array(newBadgeImageRaw)], { type: 'image/png' });
        
        let reader = new FileReader();
        reader.onload = () => {
          this.badgeImage = reader.result
        };
        reader.readAsDataURL(imageblob);
      },

      badgeAction(newBadgeAction) {
        if (!!this.walletAddress && newBadgeAction) {
          this.hrl = encodeHashToBase64(newBadgeAction) + this.walletAddress;
        }
      },

      walletAddress(newWalletAddress) {
        if (!!this.badgeAction) {
          this.hrl = encodeHashToBase64(this.badgeAction) + newWalletAddress;
        }
      },

      async isWalletConnected(newValue) {
        if (newValue) {
          console.log("Wallet connected, creating ethers provider")
          const provider = new ethers.providers.Web3Provider(this.walletProvider);
          console.log("Getting ethers signer object")
          const signer = provider.getSigner();
          this.signer = markRaw(signer);
          console.log("Got ethers signer object: " + this.signer)
          try {
            console.log("Getting wallet address")
            this.walletAddress = await this.signer.getAddress()
          } catch (e) {
            console.log(e)
          }
          this.fetchUserNfts()
          console.log("test logging with2 ", this.walletProvider.accounts[0])
          console.log("Got wallet address: " + this.walletAddress)
          if (!!this.badgeAction) {
            console.log("Blah")
            this.hrl = encodeHashToBase64(this.badgeAction) + this.walletAddress
            try {
              this.nftRecord = await this.happ.getNft(this.hrl)
            } catch (e) {
              console.log(e)
            }
          }
        }
      }
    },

    setup() {
      const happ = (inject('placeInterface') as ComputedRef<Interface>).value;
      return {
        happ,
      };
    },
  })

</script>


 


