//Contract based on [https://docs.openzeppelin.com/contracts/3.x/erc721](https://docs.openzeppelin.com/contracts/3.x/erc721)
// SPDX-License-Identifier: MIT 
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";

contract placeNFT is ERC721URIStorage {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;

    constructor() ERC721("placeNFT", "NFT") {}
    event Minted(uint256 newItemId);

    function mintNFT(string memory badgeAction)
        public
        returns (uint256)
    {
        _tokenIds.increment();

        uint256 newItemId = _tokenIds.current();
        string memory senderAddress = Strings.toHexString(uint256(uint160(msg.sender)), 20);
        
        string memory tokenURI = string(abi.encodePacked(badgeAction, senderAddress));
        _mint(msg.sender, newItemId);
        _setTokenURI(newItemId, tokenURI);
        
        emit Minted(newItemId);

        return newItemId;
    }
}
