use hdi::prelude::*;

/// A Public Entry representing the whole canvas for a specific time bucket
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NftRecord {
   pub nft_id: String, // 2 x 4-bit pixels per u8
   pub contract_address: String
}

impl NftRecord {

    pub fn create(nft_id: String, contract_address: String) -> Self {
        debug!("Creating first snapshot");
        Self {
            nft_id,
            contract_address
        }
    }
}
 