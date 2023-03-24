
<template>
    <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
    
    <div v-else style="display: flex; flex-direction: column">
      <div class="modal-container">
        <div class="modal">
          <h2>Game Ended - Mint your NFT?</h2>
          <img> <!--Show image before it gets committed to chain-->
          <div v-if="step === 1">
            <p>If you have participated in this collaborative art creation,</p>
            <p>you are eligible to mint a custom NFT based on the outcome.</p>
            <p>This NFT will be minted in Ethereum and requires Metamask integration</p>
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
  import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
  import { Interface } from './place_nft/place/interface';
  import '@material/mwc-circular-progress';
  import detectEthereumProvider from '@metamask/detect-provider'

  

  // Main page logic
  export default defineComponent({
    data(): { loading: boolean; step: number, error: any, txhash: string, NFTimage: any, isInstalled: boolean, provider: any } {
      return {
        loading: true,
        step: 1,
        error: undefined,
        txhash: "",
        NFTimage: undefined,
        isInstalled: false,
        provider: undefined,
      }
    },
    async mounted() {
      this.NFTimage = this.happ.getNFTimage() // this function should check if an image exists, and if not, generate one
      this.provider = await detectEthereumProvider();
    },
    methods: {
      signWithEthereum() {
        const ethereumAddress = Metamask.connect()// Connect Metamask if not connected
        const signature = sign(hHolochainKey)// Sign Holochain key with Metamask
        this.happ.linkEthereumAddress(ethereumAddress, signature)// Commit signature and Eth address to Holochain
        this.step++
      },
      generateNFT() {
        // Call Smart Contract
        this.step++
      },
      goToUrl() {
        window.location.href = "https://example.com";
      },
    },
    setup() {
      const happ = (inject('placeInterface') as ComputedRef<Interface>).value;
      return {
        happ,
      };
    },
  })


  async function signWithMetamask({
    hotAddress,
    holoFuelAddress
  }: {
    hotAddress: string
    holoFuelAddress: string
  }): Promise<SignatureReturn | boolean> {
    const domainData: DomainData = {
      // Defining the chain
      // 1 - Ethereum Main Net
      // 5 - Goerli testnet
      chainId: ETHEREUM_NETWORK?.id ?? kEhtereumGoerliId,
      // User-friendly name to the specific contract we are signing for,
      // visible to user on MetaMask popup
      name: 'HoloFuel Reserve Purchase Website',
      // Our internal version number
      version: '2',
      // Unique, 32-byte value hardcoded into both the contract and the dApp
      // meant as a last-resort means to distinguish our dApp from others
      salt: '0xf2d857f4a3edcb9b78b4d503bfe733db1e3f6cdc2b7971ee739626c97e86a558'
    }

    // Message shown to the user in MetaMask popup
    // Those fields must match the fields defined in hfHotLinkType
    // otherwise it won't be shown in the popup
    const message: Message = {
      HoloFuelAddress: holoFuelAddress,
      HOTAddress: hotAddress
    }

    const data: SignatureData = {
      // Types defined in the contract
      types: {
        EIP712Domain: domainType,
        HfHotLink: hfHotLinkType
      },
      domain: domainData,
      primaryType: 'HfHotLink',
      message
    }

    const signatureRequest = {
      method: 'eth_signTypedData_v4',
      params: [hotAddress, JSON.stringify(data)],
      from: hotAddress
    }

    isSigning.value = true

    const provider: ExternalProvider | null = await detectEthereumProvider({
      mustBeMetaMask: true
    })

    if (!provider) {
      showTipToast({
        message: t('reserve.step_two.no_metamask'),
        type: EAlertType.danger
      })

      isSigning.value = false

      return false
    }

    try {
      // Make signing request via MetaMask
      const result: string | unknown = await provider?.request(signatureRequest)

      if (isString(result)) {
        /* eslint-disable @typescript-eslint/no-magic-numbers */
        const signature: string = result.substring(2)
        // ECDSA signatures consist of two numbers (integers): r and s.
        // Ethereum also uses an additional v (recovery identifier) variable.
        // The signature can be notated as {r, s, v}.
        const r: string = `0x${signature.substring(0, 64)}`
        const s: string = `0x${signature.substring(64, 128)}`
        const v: number = parseInt(signature.substring(128, 130), 16)
        /* eslint-enable @typescript-eslint/no-magic-numbers */

        return { data, signature, r, s, v }
      }
    } catch (error) {
      if (isError(error)) {
        if (error.code === EMetaMaskError.userRejectedRequest) {
          // Hide tip if visible at the moment the error occurs
          tipMessage.value = ''
          isTipToastVisible.value = false
          isSigning.value = false
        }
      }
    }

    return false
  }

  async function signHoloFuelAddress(): Promise<boolean> {
    if (holoFuelWallet.value) {
      try {
        closeTipToast()

        // Show tip telling user to confirm connection on MetaMask
        // 10 seconds after user started the connection process
        // and still not finished it.
        if (!tipMessage.value) {
          showTipTimeout = setTimeout(() => {
            showTipToast({
              message: t('reserve.step_two.pending_signature_request')
            })
          }, kShowTipTimeout)
        }

        const result = await signWithMetamask({
          hotAddress: props.hotWallet,
          holoFuelAddress: holoFuelWallet.value.value
        })

        if (isSignatureReturn(result)) {
          const formattedSignature: number[] = toByteArray(result.signature)

          const { recoveredSigner } = await saveHoloFuelHotLink({
            ...result,
            signature: formattedSignature
          })

          // Clear timeout if user has confirmed connection.
          if (showTipTimeout) {
            clearTimeout(showTipTimeout)
            showTipTimeout = null
          }

          isSigning.value = false

          return recoveredSigner === props.hotWallet
        }

        return false
      } catch (error) {
        showTipToast({
          isAutoHiding: true,
          type: EAlertType.danger,
          message: t('$.errors.unexpected', { error })
        })

        return false
      }
    }

    return false
  }

  function toByteArray(hexString: string): number[] {
    const result = []

    /* eslint-disable @typescript-eslint/no-magic-numbers */
    for (let i = 0; i < hexString.length; i += 2) {
      result.push(parseInt(hexString.substr(i, 2), 16))
    }
    /* eslint-enable @typescript-eslint/no-magic-numbers */

    return result
  }

  // Types defined in the contract
  const domainType = [
    { name: 'name', type: 'string' },
    { name: 'version', type: 'string' },
    { name: 'chainId', type: 'uint256' },
    { name: 'salt', type: 'bytes32' }
  ]

  const hfHotLinkType = [
    { name: 'HoloFuelAddress', type: 'string' },
    { name: 'HOTAddress', type: 'address' }
  ]

  async function connectWallet(): Promise<void> {

  try {
    isConnecting.value = true

    const provider: ExternalProvider | null = await detectEthereumProvider({
      mustBeMetaMask: true
    })

    if (!provider) {
      errorMessage.value = t('reserve.step_one.no_metamask')
      isConnecting.value = false
      return
    } else {
      provider.on('accountsChanged', handleMetaMaskApproval)
    }

    try {
      // Show tip telling user to confirm connection on MetaMask
      // 10 seconds after user started the connection process
      // and still not finished it.
      if (!tipMessage.value) {
        showTipTimeout.value = setTimeout(() => {
          isTipToastAutoHiding.value = false
          tipMessage.value = t('reserve.step_one.pending_confirmation')
          isTipToastVisible.value = true
        }, kShowTipTimeout)
      }

      // eslint-disable-next-line @typescript-eslint/no-unsafe-call,@typescript-eslint/no-unsafe-assignment
      await provider.request({ method: 'eth_requestAccounts' })
    } catch (error) {
      if (isError(error)) {
        if (error.code === EMetaMaskError.userRejectedRequest) {
          // Hide tip if visible at the moment the error occurs
          tipMessage.value = ''
          isTipToastVisible.value = false
          isConnecting.value = false
        }

        // When user has already confirmation request pending in MetaMask
        if (error.code === EMetaMaskError.pendingConfirmation) {
          isTipToastAutoHiding.value = true
          tipMessage.value = t('reserve.step_one.pending_confirmation')
          isTipToastVisible.value = true
        }
      }
    }
  } catch (error) {}
}
  </script>
  