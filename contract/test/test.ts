import { expect } from "chai";
import { ethers } from "hardhat";


describe("place_nft", function () {
  it("Should mint an NFT where the url is the badge action and sender address concatenated", async function () {
    const [owner, otherAccount] = await ethers.getSigners();
    console.log("owner address: ", owner.address);
    console.log("other account address: ", otherAccount.address);
    const badgeAction = "hcakkthisisafakeactionhash1234"
    const NFT = await ethers.getContractFactory("placeNFT");
    const nft = await NFT.deploy();
    console.log("Contract deployed");
    
    const tx = await nft.connect(otherAccount).mintNFT(badgeAction);

    const receipt = await tx.wait();
    const event = receipt.events.find((event: any) => event.event === 'Minted');
    const newItemId = event.args.newItemId;
    const nftId = newItemId.toNumber();

    console.log("NFT minted: ", nftId);
    const nftURI = await nft.connect(otherAccount).tokenURI(nftId);

    expect(nftURI).to.equal(`${badgeAction}${otherAccount.address}`.toLowerCase())
  });
});