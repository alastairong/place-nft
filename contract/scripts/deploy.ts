import { ethers } from "hardhat";

async function main() {

  const PlaceNFT = await ethers.getContractFactory("placeNFT")
  
    // Start deployment, returning a promise that resolves to a contract object
    const placeNFT = await PlaceNFT.deploy()
    await placeNFT.deployed()
    console.log("Contract deployed to address:", placeNFT.address)
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
