
<template>
    <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
    
    <div v-else style="display: flex; flex-direction: column">
      <div class="modal-container">
        <div class="modal">
          <h2>Game Ended - Mint an NFT?</h2>
          <div v-if="step === 1">
            <p>If you have participated in this collaborative art creation,</p>
            <p>you are eligible to mint a custom NFT based on the outcome.</p>
            <p>This NFT will be minted in Ethereum and requires Metamask integration</p>
            <button @click="this.step=2">Start</button>
          </div>
          <div v-else-if="step === 2">
            <p>First, sign with the ethereum key you would like to use to mint the NFT</p>
            <button @click="signWithEthereum">Sign</button>
          </div>
          <div v-else-if="step === 3">
            <p>The image below will be your NFT. Commit to chain?</p>
            <img> <!--Show image before it gets committed to chain-->
            <button @click="signWithEthereum">Commit</button>
          </div>
          <div v-else-if="step === 4">
            <p>Generate NFT on Ethereum. Note that this will cost gas</p>
            <img> <!--Show image before it gets committed to chain-->
            <button @click="signWithEthereum">Mint NFT</button>
          </div>
          <div v-else>
            <p>Done!</p>
            <p>Transaction Hash: </p>
            <!--Buttons to investigate a given NFT?-->
          </div>
        </div>
      </div>
    </div>

  </template>
  
  <script lang="ts">
  import { defineComponent, inject, ComputedRef } from 'vue';
  import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
  import { Interface } from './place_nft/place/interface';
  import '@material/mwc-circular-progress';
  
  export default defineComponent({
    data(): { loading: boolean; step: number, error: any } {
      return {
        loading: true,
        step: 1,
        error: undefined
      }
    },
    async mounted() {
      // TODO
    },
    methods: {
      signWithEthereum() {
        this.step++
      },
      generateImage() {
        this.step++
      },
      mintNFT() {
        // Perform any necessary actions to finish the process
        this.step++
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
  