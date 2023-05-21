import Web3 from 'web3';
const web3 = new Web3(window.web3.currentProvider);
const contract = require("../artifacts/contracts/MyNFT.sol/MyNFT.json")

const contractAddress = "0x5a738a5c5fe46a1fd5ee7dd7e38f722e2aef7778"

const nftContract = new web3.eth.Contract(contract.abi, contractAddress)
