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
    let ZomeInfo {id, ..} = zome_info()?; // There's only 1 app entry def in this zome
    let badge_app_entry_type = AppEntryDef::new(0.into(), id, EntryVisibility::Public);
    if let Ok(records) = &query(ChainQueryFilter::new().entry_type(EntryType::App(badge_app_entry_type.clone()))) {
        let badge_result = records[0].entry.to_app_option() // Chain should not have more than one badge anyway so just discard the rest in this POC
            .map_err(|err| wasm_error!(WasmErrorInner::Guest(
                err.into()
            )));
        
        let badge: Badge = match badge_result {
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

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct GenerateBadgeInput {
    eth_address: String,
    eth_signed_contents: String,
}

#[hdk_extern]
fn generate_badge(input: GenerateBadgeInput) -> ExternResult<ActionHash> {
    // Get final snapshot
    let final_bucket: u32 = 24 * 12; // 12 buckets per hour and 24 hours in this game

    let final_snapshot_result: Result<Snapshot, WasmError> = call(
        CallTargetCell::Local,
        ZomeName("place".into()),
        FunctionName("get_snapshot_at".into()),
        None,
        final_bucket,
    )
    .and_then(|snapshot_call_result| match snapshot_call_result {
        ZomeCallResponse::Ok(response) => response
            .decode()
            .map_err(|err| wasm_error!(WasmErrorInner::Guest(err.into()))),
        _ => {
            error!(
                "Place zome returned error when fetching final snapshot: {:?} \n",
                snapshot_call_result
            );
            Err(wasm_error!(WasmErrorInner::Guest("Error fetching final snapshot".into())))
        }
    })
    .map_err(|e| {
        error!("Failed to call Place zome to fetch snapshot: {:?}", e);
        wasm_error!(WasmErrorInner::Guest("Error calling Place zome".into()))
    });

    let final_snapshot = final_snapshot_result?;
    debug!("Final snapshot: {:?}", final_snapshot);
    
    // And count number of placements this user had and generate badge
    let placement_app_entry_type = AppEntryDef::new(0.into(), 0.into(), EntryVisibility::Public); 
    if let Ok(records) = &query(ChainQueryFilter::new().entry_type(EntryType::App(placement_app_entry_type))) {
        if records.len() == 0 {
            Err(wasm_error!(WasmErrorInner::Guest(
                "Only users who have placed a placement can generate a badge".into()
            )))
        } else {
            let author = agent_info()?.agent_latest_pubkey;
            let badge = Badge::new(final_snapshot, records.len() as u32, &author.to_string(), input.eth_address, input.eth_signed_contents);
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
    badge_action: ActionHash,
}

#[hdk_extern]
fn generate_hrl(input: GenerateHrlInput) -> ExternResult<()> {
    
    if let Some(record) = get(input.badge_action.clone(), Default::default())? { // Confirms that this badge exists
        
        // Extract info for HRL
        let badge_result = record.entry.to_app_option()
            .map_err(|err| wasm_error!(WasmErrorInner::Guest(
                err.into()
            )));
        
        let badge: Badge = match badge_result {
            Ok(Some(badge)) => badge,
            Ok(None) => return Err(wasm_error!(WasmErrorInner::Guest(
                "Something went wrong".into()
            ))),
            Err(err) => return Err(err),
        };

        let eth_address = badge.eth_address;
       
        let mut hrl: String = input.badge_action.to_string();
        hrl.push_str(&eth_address); // In future this should be a hash so it can't be reverse engineered        
        let hrl_anchor = get_anchor_typed_path(&hrl)?;
        // Create link from HRL to badge
        create_link(
            hrl_anchor.path_entry_hash()?, // use hrl as anchor
            input.badge_action.clone(),         // use entry hash as target
            links::HRLtoBadgeLink::link_type(),
            links::HRLtoBadgeLink::link_tag(),
        )?;

        // Create link from badge to HRL
        create_link(
            input.badge_action.clone(), // use host agent pubkey as base
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
    let action_hash = create_entry(EntryTypes::Badge(badge.clone()))?;
    let _entry_hash = hash_entry(badge)?;
    Ok(action_hash)
}

fn get_anchor_typed_path(anchor: &str) -> ExternResult<TypedPath> {
    let typed_path = Path::from(anchor).typed(links::HRLtoBadgeLink::link_type())?;
    typed_path.ensure()?;
    Ok(typed_path)
}