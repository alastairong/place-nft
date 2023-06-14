use hdi::prelude::*;

/// A Public Entry representing the whole canvas for a specific time bucket
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NftRecord {
   pub nft_id: u32, 
   pub contract_address: String
}

impl NftRecord {

    pub fn new(nft_id: u32, contract_address: String) -> Self {
        debug!("Creating first snapshot");
        Self {
            nft_id,
            contract_address
        }
    }
}
