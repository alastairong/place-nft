import { expect } from "chai";
import { ethers } from "hardhat";


describe("place_nft", function () {
  it("Should mint an NFT where the url is the badge action and sender address concatenated", async function () {
    console.log(1)
    const [owner, otherAccount] = await ethers.getSigners();
    console.log(2)
    const badgeAction = "hcAkkthisisafakeactionhash1234"
    console.log(3)
    const NFT = await ethers.getContractFactory("placeNFT");
    console.log(4)
    const nft = await NFT.deploy();
    console.log(5)

    const nftId = await nft.connect(otherAccount).mintNFT(badgeAction);
    console.log(6)
    const nftURI = await nft.connect(otherAccount).tokenURI(nftId);
    console.log(7)
    expect(nftURI).to.equal(`${badgeAction}${owner.address}`)
  });
});