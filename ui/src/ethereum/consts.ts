// ETH URL:
interface EthereumNetwork {
    id: number
    etherscanLink: string
  }
  
  export const kEthereumMainnetId = 1
  export const kEthereumGoerliId = 5

  export const ETHEREUM_NETWORK = { id: kEthereumGoerliId, etherscanLink: 'https://goerli.etherscan.io/tx/' }