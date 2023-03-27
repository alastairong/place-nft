import type { JsonRpcPayload, JsonRpcResponse } from 'web3-core-helpers'

export interface RequestArguments {
  method: string
  params?: unknown[]
  [key: string]: unknown
}

export interface ExternalProvider {
  chainId?: string
  selectedAddress?: string
  isMetaMask?: boolean
  isBraveWallet?: boolean
  networkVersion?: string
  sendAsync: (
    payload: JsonRpcPayload,
    callback?: (error: Error | null, result?: JsonRpcResponse) => Promise<unknown>
  ) => void
  send?: (
    payload: JsonRpcPayload,
    callback: (error: Error | null, result?: JsonRpcResponse) => unknown
  ) => void
  request?: (request: RequestArguments) => Promise<unknown>
  on?: (eventName: string | symbol, listener: (...args: unknown[]) => void) => void
}

export interface CustomWindow extends Window {
  ethereum: ExternalProvider
}

export interface Error {
  code?: number
  message?: string
}

export interface MetaMaskTransactionReference {
    uuid: string,
    email: string,
    message: Message, 
    signature: number[]
  }
  
  export interface DomainData {
    name: string
    version: string
    chainId: number
    salt: string
  }
  
  export interface Message {
    // eslint-disable-next-line @typescript-eslint/naming-convention
    NFTAuthor: string
    // eslint-disable-next-line @typescript-eslint/naming-convention
    ethereumAddress: string
  }
  
  export interface SignatureData {
    types: {
      // eslint-disable-next-line @typescript-eslint/naming-convention
      EIP712Domain: Array<{ name: string, type: string }>
      // eslint-disable-next-line @typescript-eslint/naming-convention
      authorEthAddressLink: Array<{ name: string, type: string }>
    }
    domain: DomainData
    primaryType: string
    message: Message
  }
  
  export interface SignatureReturn {
    message: Message
    signature: string
  }

  export function isSignatureReturn(data: SignatureReturn | boolean): data is SignatureReturn {
    return data !== undefined
  }
  
