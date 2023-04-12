use hdk::prelude::*;
use nft_integrity::*;
use place_integrity::Snapshot;

/// Called the first time a zome call is made to the cell containing this zome
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
  Ok(InitCallbackResult::Pass)
}

// TODO: Accept an actionhash?
#[hdk_extern]
fn get_badge(_: ()) -> ExternResult<Vec<u8>> {
    
    // Search if they already have committed their badge. It should be on their chain
    // If it does, return the image
    if let Ok(records) = &query(ChainQueryFilter::new().entry_type(AppEntryName(Badge))) {
        let badge_record = records[0]; // Chain should not have more than one badge anyway so just discard the rest in this POC
        let badge_result = badge_record.entry.to_app_option()
            .map_err(|err| wasm_error!(WasmErrorInner::Guest(
                err.into()
            )));
        
        let badge = match badge_result {
            Ok(Some(badge)) => badge,
            Ok(None) => return Err(wasm_error!(WasmErrorInner::Guest(
                "Something went wrong".into()
            ))),
            Err(err) => return Err(err),
        };

        Ok(badge.image_data)
    } else {
        Err(wasm_error!(WasmErrorInner::Guest(
            "Badge not found".into()
        )))
    }
}

#[hdk_extern]
fn generate_badge(_: ()) -> ExternResult<ActionHash> {
    // Get final snapshot
    let final_bucket: u32 = 24 * 12; // 12 buckets per hour and 24 hours in this game 
    let final_snapshot: Snapshot = call(None, "place", "get_snapshot_at", None, final_bucket)?;
    
    // And count number of placements this user had and generate badge
    if let Ok(records) = &query(ChainQueryFilter::new().entry_type(EntryType::App(AppEntryName(Placement)))) {
        if records.len() == 0 {
            Err(wasm_error!(WasmErrorInner::Guest(
                "Only users who have placed a placement can generate a badge".into()
            )))
        } else {
            let author = agent_info()?.agent_latest_pubkey;
            let badge = Badge::new(final_snapshot, records.len() as u32, author.to_string());
            let action_hash = publish_badge(badge)?;
            Ok(action_hash)
        }
    } else {
        Err(wasm_error!(WasmErrorInner::Guest(
            "Only users who have placed a placement can generate a badge".into()
        )))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct GenerateHrlInput {
    badge: ActionHash,
}

#[hdk_extern]
fn generate_hrl(input: GenerateHrlInput) -> ExternResult<()> {
    
    if let Some(Record) = get(input.badge, Default::default())? { // Confirms that this badge exists
        // Fetch info for HRL
        let badge: Badge = Record.entry.to_app_option(Badge)?.unwrap();
        let eth_address = badge.eth_address;
       
        let hrl: String = input.badge.to_string();
        hrl.push_str(&eth_address); // In future this should be a hash so it can't be reverse engineered        
        let hrl_anchor = get_anchor_typed_path(&hrl)?;
        // Create link from HRL to badge
        create_link(
            hrl_anchor.path_entry_hash()?, // use hrl as anchor
            input.badge,         // use entry hash as target
            links::HRLtoBadgeLink::link_type(),
            links::HRLtoBadgeLink::link_tag(),
        )?;

        // Create link from badge to HRL
        create_link(
            input.badge, // use host agent pubkey as base
            hrl_anchor.path_entry_hash()?,         // use entry hash as target
            links::BadgetoHRLLink::link_type(),
            links::BadgetoHRLLink::link_tag(),
        )?;
        Ok(())
    } else {
        Err(wasm_error!(WasmErrorInner::Guest(
            "Badge not found".into()
        )))
    }
}

fn publish_badge(badge: Badge) -> ExternResult<ActionHash> {
    let action_hash = create_entry(EntryTypes::Badge(badge))?;
    let entry_hash = hash_entry(badge)?;
    Ok(action_hash)
}

fn get_anchor_typed_path(anchor: &str) -> ExternResult<TypedPath> {
    let typed_path = Path::from(anchor).typed(links::HRLtoBadgeLink::link_type())?;
    typed_path.ensure()?;
    Ok(typed_path)
}