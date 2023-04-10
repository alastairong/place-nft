use hdk::prelude::*;

/// Called the first time a zome call is made to the cell containing this zome
#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
  Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn get_NFT_image(_:()) -> ExternResult<ByteArray> {
    // Check if the image entry exists (how does the zome call look this up? Is there an anchor?)
    // If it does, return the image
    // If it doesn't, generate the image and commit
    // return commit
    let image = ByteArray::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]); // TBD
    Ok(image)
}

pub struct GenerateHrlInput {
    badge: ActionHash,
    eth_address: String
}

#[hdk_extern]
pub fn generate_hrl(input: GenerateHrlInput) -> ExternResult<ActionHash> {
    
    if let Some(Record) = get(input.badge, Default::default())? {
        // Get action_hash
        // Check that user has not created a link already?
        // Create links in each direction
    } else {
        Err("Badge doesn't exist")
    }

    Ok()
}

fn publish_image(image: Badge) -> ExternResult<ActionHash> {
    let action_hash = create_entry(EntryTypes::Badge(image))?;
    let entry_hash = hash_entry(image)?;
    Ok(action_hash)
}