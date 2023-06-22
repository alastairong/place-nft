
<template>
    <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
    
    <div v-else style="display: flex; flex-direction: column">
      <div class="modal-container">
        <div class="modal">
          <button class="flip-button" @click="flipModal">Test Me</button>
          <div v-if="!isFlipped">
            <!-- front side of modal -->
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
          <div v-else>
            <!-- back side of modal -->
            <h3>Badge Shenanigans:</h3>
            <h4>Mint Fake NFT:</h4>
            <form @submit="mintFakeNft">
              <label for="badge-action">Badge Action (Id):</label>
              <input type="text" id="badge-action" v-model="testBadgeAction" />
              <button type="submit">Submit</button>
            </form>
            <h4>Create HRL:</h4>
            <form @submit="submitBadgeStealTest">
              <label for="badge-action">Badge Action (Id):</label>
              <input type="text" id="badge-action" v-model="testBadgeAction" />
              <label for="token-uri">NFT Token Uri:</label>
              <input type="text" id="token-uri" v-model="testTokenUri" />
              <button type="submit">Submit</button>
            </form>

            <div>
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
    </div>

  </template>
  
  <script lang="ts">
  import { defineComponent, inject, ComputedRef } from 'vue';
  import { Interface } from './place_nft/interface';
  import '@material/mwc-circular-progress';
  import { CONTRACT_ADDRESS } from './ethereum/consts'
  import { AppAgentClient, Record, AgentPubKeyB64, EntryHash, ActionHash, Action, encodeHashToBase64, decodeHashFromBase64 } from '@holochain/client';
  import { NftRecord, NftTokenUri } from './place_nft/types';
  import { Message } from './ethereum/types';
  import { ethers } from "ethers";
  import contractArtifact from '../../contract/artifacts/contracts/place_nft.sol/placeNFT.json';
  import WalletConnectProvider from "@walletconnect/web3-provider";
  import { markRaw } from 'vue';
  import { Alchemy, Network } from "alchemy-sdk";
  import util from 'util';
  
  // Main page logic
  export default defineComponent({
    data(): { loading: boolean; isFlipped: boolean; error: any, badgeAction: ActionHash | undefined, badgeImage: string | undefined, walletProvider: any, signer: any, hrl: string, nftRecord: NftRecord | undefined, isWalletConnected: Boolean, walletAddress: any, usersNfts: NftTokenUri[] | undefined, nftImage: any, testBadgeAction: string, testTokenUri: string } {
      return {
        loading: true,
        isFlipped: false,
        error: undefined,
        badgeAction: undefined,
        badgeImage: undefined,
        walletProvider: undefined,
        signer: undefined,
        hrl: "",
        nftRecord: undefined,
        isWalletConnected: false,
        walletAddress: undefined,
        usersNfts: undefined,
        nftImage: undefined,
        testBadgeAction: "",
        testTokenUri: "",
      }
    },
    async mounted() {
      this.getBadgeImage()
      if (!!this.badgeImage) console.log("got badge Image: " + this.badgeImage.substring(0, 30))
      this.loading = false
      const provider = new WalletConnectProvider({
        infuraId: "a5238372835346588d9c347de0a2226e",
      });
      this.walletProvider = markRaw(provider);
      this.isWalletConnected = this.walletProvider.connected;
      console.log("Initialized wallet provider, listening for connection")
      this.walletProvider.on('connect', async () => {
        this.isWalletConnected = true
      })
      
    },
    methods: {
      flipModal() {
        this.isFlipped = !this.isFlipped;
      },

      async getBadgeImage() {
        console.log("getting badge image")
        try {
          this.badgeAction = await this.happ.getBadgeAction() 
        } catch (e) {
          console.log(e)
        }
        
        if (!!this.badgeAction) {
          this.badgeImage = await this.happ.getBadge(this.badgeAction) 
        }
      },

      async getNftRecord(hrl: String) {
        this.nftRecord = await this.happ.getNft(hrl) 
      },

      async mintNft() {
        
        // instantiate smart contract
        const nftContractInstance = new ethers.Contract(CONTRACT_ADDRESS, contractArtifact.abi, this.signer);
        // make call to smart contract method
        try {
          if (!!this.badgeAction) {
            const tx = await nftContractInstance.mintNFT(encodeHashToBase64(this.badgeAction)); // make contract call
            const receipt = await tx.wait(); // wait for tx to be mined

            // look for broadcasted event with nftId
            const event = receipt.events.find((event: any) => event.event === 'Minted');
            const newItemId = event.args.newItemId;
            const nftId = newItemId.toNumber();
            this.nftRecord = {
              nftId,
              contractAddress: CONTRACT_ADDRESS
            }
            const hrl = await this.happ.generateHrl(this.badgeAction) // set up the actual HRLs and return the real one
            await this.happ.saveNft(nftId, CONTRACT_ADDRESS, hrl, this.badgeAction)
            this.fetchUserNfts()
          }
             
        } catch (e){
          console.log("Error with minting NFT")
          console.log(util.inspect(e, { depth: null }));
        }
      },

      async checkForNfts() {
        console.log("checking for nfts")
        const nftContractInstance = new ethers.Contract(CONTRACT_ADDRESS, contractArtifact.abi, this.signer);
      },

      async connect() {
        // your connect logic here
        await this.walletProvider.disconnect();
        await this.walletProvider.enable();
        console.log(this.walletProvider)
        this.isWalletConnected = this.walletProvider.connected;
      },

      async createBadge() {
        try { 
          console.log("creating badge")
           // TODO, change the message that gets signed in the UI (see validation logic in the zome)
          let pubkey = await this.happ.myPubKey()
          console.log("my pub key: " + pubkey)
          const signature = await this.signer.signMessage(pubkey) // sign the payload
          console.log(signature)
          this.badgeAction = await this.happ.generateBadgeImage(this.walletAddress, signature) // we don't send the rest of the info as validation logic should be able to calculate it from the badge entry
        } catch (e) {
          console.log(util.inspect(e, { depth: null }));
        }
        
        try {
          if(!!this.badgeAction) {
            this.badgeImage = await this.happ.getBadge(this.badgeAction)
          }
          
        } catch (e) {
          console.log(util.inspect(e, { depth: null }));
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
          console.log(util.inspect(e, { depth: null }));
        }
      },

      async viewNft(hrl: string, event: Event) {
        
        event.preventDefault();
        try {
          this.nftImage = await this.happ.viewNftImage(hrl)
        } catch (e) {
          console.log(util.inspect(e, { depth: null }));
        }
      },

      async mintFakeNft(event: Event) {
        event.preventDefault();
        // call another function and pass the badgeAction value to it
        try {
          // instantiate smart contract
          const nftContractInstance = new ethers.Contract(CONTRACT_ADDRESS, contractArtifact.abi, this.signer);
          // make call to smart contract method
                  
          const tx = await nftContractInstance.mintNFT(this.testBadgeAction); // make contract call
          const receipt = await tx.wait(); // wait for tx to be mined

          // look for broadcasted event with nftId
          const ethEvent = receipt.events.find((event: any) => event.event === 'Minted');
          const newItemId = ethEvent.args.newItemId;
          const nftId = newItemId.toNumber();

          await this.fetchUserNfts()
        } catch (e) {
          console.log(util.inspect(e, { depth: null }));
        }
        
      },

      async submitBadgeStealTest(event: Event) {
        console.log("submitting badge steal test")
        event.preventDefault();
        try {
          let actionByteArray = decodeHashFromBase64(this.testBadgeAction)  // need to convert string to uint8array
          console.log(actionByteArray)

          let tokenUri = this.testTokenUri
          await this.happ.stealBadge(actionByteArray, tokenUri)
        } catch (e) {
          console.log(util.inspect(e, { depth: null }));
        }

        this.fetchUserNfts()
      },

      

    },
    watch: {
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
          const signer = provider.getSigner();
          this.signer = markRaw(signer);

          try {
            const rawWalletAddress: string = await this.signer.getAddress();
            this.walletAddress = rawWalletAddress.toLowerCase();
          } catch (e) {
            console.log(e)
          }
          this.fetchUserNfts()
          console.log("Got wallet address: " + this.walletAddress)
          if (!!this.badgeAction) {
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


<style>
  img {
    border: 3px solid #333;
    padding: 10px;
    background-color: #eee;
  }

  .flip-button {
    position: absolute;
    top: 10px;
    right: 10px;
  }

  .modal {
    /* ... other styles */
    transition: transform 1s;
  }

  .modal.is-flipped {
    transform: rotateY(180deg);
  }

  form {
    display: flex;
    flex-direction: column;
  }

  label {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }

  input[type="text"] {
    width: 40%;
  }

</style>
