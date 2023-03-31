use hdi::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
   nft_author: String, // Maybe this should be a Holo Hash?
   ethereum_address: String // Ethereum address
}


/// Public entry representing the ethereum address of the minter
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Minter {
   pub address: String, // This is an ethereum address so maybe we should store it as a 20 byte array
   pub signature: String, // This is the agent public key signed by the ethereum key
}

impl Minter {
   pub fn new(address: String, signature: String) -> Self {
      
      // Check that signature is valid 
      let message = Message {
        nft_author: self.pubkey(), // actual function?
        ethereum_address: address
      };  

      // Verify signature TBD
      
      Self {
         address,
         signature,
      }
   }
}