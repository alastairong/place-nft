
<template>
    <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
    
    <div v-else style="display: flex; flex-direction: column">
      <div class="modal-container">
        <div class="modal">
          <h2>Game Ended - Mint your NFT?</h2>
          <img> <!--Show image before it gets committed to chain-->

          <p>If you have participated in this collaborative art creation,</p>
          <p>you are eligible to mint a custom NFT based on the outcome.</p>
           <p>This NFT will be minted in Ethereum and requires Metamask integration</p>
          <div v-if="step === 1">  
            <div v-if="!isInstalled">
              <p>Metamask is not installed</p>
              <p>Please install Metamask</p>
            </div>
            <div v-else-if="!isConnected">
              <p>Metamask is not connected</p>
              <button @click="connectWallet">Connect Metamask</button>
            </div>
            <div v-else>
              <p>Metamask is connected</p>
              <button @click="this.step=2">Start</button>
          </div>
          <div v-else-if="step === 2">
            <p>Sign with the ethereum key you would like to use to mint the NFT</p>
            <p>This connects your ethereum and Holochain keys so that anyone can</p>
            <p>confirm that you had authority to mint the NFT</p>
            <button @click="signWithEthereum">Sign</button>
          </div>
          <div v-else-if="step === 3">
            <p>Generate NFT on Ethereum. Note that this will cost gas</p>
            
            <button @click="generateNFT">Mint NFT</button>
          </div>
          <div v-else>
            <p>Done!</p>
            <p>Transaction Hash: {{ txhash }}</p>
            <button @click="goToUrl">View NFT</button>
          </div>
        </div>
      </div>
    </div>

  </template>
  
  <script lang="ts">
  import { defineComponent, inject, ComputedRef } from 'vue';
  import { Interface } from './place_nft/interface';
  import '@material/mwc-circular-progress';
  import detectEthereumProvider from '@metamask/detect-provider'
  import MetaMaskOnboarding from '@metamask/onboarding'
  import type { CustomWindow, ExternalProvider, DomainData, SignatureData, Message, SignatureReturn } from './ethereum/types'
  import {isSignatureReturn, isString} from './ethereum/utils'
  import { ETHEREUM_NETWORK } from './ethereum/consts'
  
  declare let window: CustomWindow

  // Main page logic
  export default defineComponent({
    data(): { loading: boolean; step: number, error: any, txhash: string, NFTimage: any, isInstalled: boolean, isConnected: boolean, ethereumAddress: string, NFTAuthor: string } {
      return {
        loading: true,
        step: 1,
        error: undefined,
        txhash: "",
        NFTimage: undefined,
        isInstalled: false,
        isConnected: false,
        ethereumAddress: "",
        NFTAuthor: "",
      }
    },
    async mounted() {
      this.NFTimage = await this.happ.getNFTimage() // this function should check if an image exists, and if not, generate one

      this.isInstalled = MetaMaskOnboarding.isMetaMaskInstalled()
      if (!this.isInstalled) {
        return
      }

      const { ethereum } = window
      if (ethereum.selectedAddress) {
        this.isConnected = true
      }
    
    },
    methods: {
      async signWithEthereum() {
        const signature = await this.signNFTauthor()// Sign Holochain key with Metamask
        if (signature) {
          const isSigned = await this.happ.linkEthereumAddress(signature)// Commit signature and Eth address to Holochain
          if (isSigned) {
            this.step++
          }
        }
        console.log("Signing with Ethereum keys failed")
      },

      async generateNFT() {
        // Call Smart Contract
        this.step++
      },

      goToUrl() {
        window.location.href = "https://example.com";
      },

      async connectWallet(): Promise<void> {
        try {
          this.isConnected = true

          const provider: ExternalProvider | null = await detectEthereumProvider({
            mustBeMetaMask: true
          })

          if (!provider) {
            this.isConnected = false
            return
          } else {
            provider.on('accountsChanged', this.handleMetaMaskApproval)
          }

          try {
            // eslint-disable-next-line @typescript-eslint/no-unsafe-call,@typescript-eslint/no-unsafe-assignment
            await provider.request({ method: 'eth_requestAccounts' }) // this is done to connect
          } catch (error) {
            console.log(error)
          }
          
        } catch (error) {}
      },

      handleMetaMaskApproval(accounts: string[]): void {
        if (!accounts[0]) {
          this.isConnected = false
        } 
        this.ethereumAddress = accounts[0]
      },

      async signNFTauthor(): Promise<string | false> {
        if (this.ethereumAddress) {
          try {
            const result = await this.signWithMetamask(
              this.NFTAuthor,
              this.ethereumAddress,
            )

            if (isSignatureReturn(result)) {
              // Do we need the signature as a byte array?
              // const formattedSignature: number[] = toByteArray(result.signature)
              return result.signature
            }
            return false
          } catch (error) {
            return false
          }
        }
        return false
      },

      async signWithMetamask(NFTAuthor: string, ethereumAddress: string): Promise<SignatureReturn | false> {
        const domainData: DomainData = {
          // Defining the chain
          // 1 - Ethereum Main Net
          // 5 - Goerli testnet
          chainId: ETHEREUM_NETWORK.id,
          // User-friendly name to the specific contract we are signing for,
          // visible to user on MetaMask popup
          name: 'Place NFT Proof-of-Concept',
          // Our internal version number
          version: '1',
          // Unique, 32-byte value hardcoded into both the contract and the dApp
          // meant as a last-resort means to distinguish our dApp from others
          salt: '0xf2d857f4a3edcb9b78b4d503bfe733db1e3f6cdc2b7971ee739626c97e86a558'
        }
      
        // Message shown to the user in MetaMask popup
        // Those fields must match the fields defined in hfHotLinkType
        // otherwise it won't be shown in the popup
        const message: Message = {
          NFTAuthor: NFTAuthor,
          ethereumAddress: ethereumAddress
        }
      
        const data: SignatureData = {
          // Types defined in the contract
          types: {
            EIP712Domain: domainType,
            authorEthAddressLink: AuthorEthAddressLinkType
          },
          domain: domainData,
          primaryType: 'AuthorEthAddressLink',
          message
        }
      
        const signatureRequest = {
          method: 'eth_signTypedData_v4',
          params: [ethereumAddress, JSON.stringify(data)],
          from: ethereumAddress
        }
        
        try {
          // Make signing request via MetaMask
          const result: string | unknown = await provider?.request(signatureRequest)
      
          if (isString(result)) {
            /* eslint-disable @typescript-eslint/no-magic-numbers */
            const signature: string = result.substring(2)
            /* eslint-enable @typescript-eslint/no-magic-numbers */
      
            return { message, signature }
          }
        } catch (error) {
          console.log(error)
        }
      
        return false
      }
    },

    setup() {
      const happ = (inject('placeInterface') as ComputedRef<Interface>).value;
      return {
        happ,
      };
    },
  })

// Types defined in the contract
const domainType = [
  { name: 'name', type: 'string' },
  { name: 'version', type: 'string' },
  { name: 'chainId', type: 'uint256' },
  { name: 'salt', type: 'bytes32' }
]

const AuthorEthAddressLinkType = [
  { name: 'NFTAuthor', type: 'string' },
  { name: 'ETHAddress', type: 'address' }
]

</script>


 


