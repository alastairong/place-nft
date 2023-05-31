import { expect } from "chai";
import { ethers } from "hardhat";


describe("place_nft", async function () {
  const [owner, otherAccount] = await ethers.getSigners();
  console.log("HOORAY!")
  it("Should mint an NFT where the url is the badge action and sender address concatenated", async function () {
    const badgeAction = "hcAkkthisisafakeactionhash1234"
    console.log("HOORAY!")
    const NFT = await ethers.getContractFactory("placeNFT");
    const nft = await NFT.deploy();

    const nftId = await nft.connect(otherAccount).mintNFT(badgeAction);

    const nftURI = await nft.connect(otherAccount).tokenURI(nftId);

    expect(nftURI).to.equal(`${badgeAction}${owner.address}`)
  });
});