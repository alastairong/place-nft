// Unclear if this is needed yet. Questions to address:
// 1. Do we need to validate that the minter owns the ethereum address (it's clear that the minter is the author of the badge)?

// use hdi::prelude::*;

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Message {
//    nft_author: AgentPubKey, // Maybe this should be a Holo Hash?
//    ethereum_address: String // Ethereum address
// }

// /// Public entry representing the ethereum address of the minter
// #[hdk_entry_helper]
// #[derive(Clone, PartialEq)]
// pub struct Minter {
//    pub address: String, // This is an ethereum address so maybe we should store it as a 20 byte array
//    pub author: AgentPubKey, // This is the agent public key
//    pub signature: String, // This is the agent public key signed by the ethereum key
// }

// impl Minter {
//    pub fn new(author: AgentPubKey, address: String, signature: String) -> Self {
      
//       // Check that signature is valid 
//       let message = Message {
//         nft_author: author, // actual function?
//         ethereum_address: address
//       };  

//       // Verify signature TBD
      
//       Self {
//          address,
//          author, // think through whether this field is needed or if we can validate the author without it. What is the problem we're trying to solve?
//          signature,
//       }
//    }
// }