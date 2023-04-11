use hdk::prelude::*;
use nft_integrity::Badge;
use place_integrity::Snapshot;

/// Called the first time a zome call is made to the cell containing this zome
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
  Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn get_nft_image(_: ()) -> ExternResult<Vec<u8>> {
    
    // Search if they already have committed their badge. It should be on their chain
    // If it does, return the image
    if let Ok(records) = &query(ChainQueryFilter::new().entry_type(AppEntryName(Badge))) {
        let badge_record = records[0]; // Chain should not have more than one badge anyway so just discard the rest in this POC
        let badge: Badge = badge_record.entry.to_app_option(Badge)?.unwrap(); // does this deserialize correctly into the Badge? 
        Ok(badge.image_data)
    }; // Does this work?

    // Otherwise, first retrieve the final snapshot
    let final_bucket: u32 = 24 * 12; // 12 buckets per hour and 24 hours in this game 
    let final_snapshot: Snapshot = call(None, "place", "get_snapshot_at", None, final_bucket)?.unwrap();
    
    // And count number of placements this user had and generate badge
    if let Ok(records) = &query(ChainQueryFilter::new().entry_type(AppEntryName(Placement))) {
        if records.len() == 0 {
            Err("User has not made any placements and therefore does not have a valid badge")
        } else {
            let author = agent_info().agent_latest_pubkey;
            let badge = Badge::new(final_snapshot, records.len(), author.to_string());
            let _action_hash = publish_image(badge);
            Ok(badge.image_data)
        }
    }
}

pub struct GenerateHrlInput {
    badge: ActionHash,
    eth_address: String
}

#[hdk_extern]
fn generate_hrl(input: GenerateHrlInput) -> ExternResult<ActionHash> {
    
    if let Some(Record) = get(input.badge, Default::default())? {
        // Get entry_hash
        // Check that user has not created a link already?
        // Calculate anchor from hash(entry_hash, eth_address)
        // Create links in each direction
    } else {
        Err("Badge doesn't exist")
    }

    Ok()
}

fn publish_badge(badge: Badge) -> ExternResult<ActionHash> {
    let action_hash = create_entry(EntryTypes::Badge(badge))?;
    let entry_hash = hash_entry(badge)?;
    Ok(action_hash)
}