use hdk::prelude::*;
use place_integrity::*;
use zome_utils::{zome_panic_hook};

#[hdk_extern]
fn get_badge_action(_: ()) -> ExternResult<Option<ActionHash>> {
    std::panic::set_hook(Box::new(zome_panic_hook));
    // Search if they already have committed their badge. It should be on their chain
    // If it does, return the hash of the action
    // let ZomeInfo {id, ..} = zome_info()?; // There's only 1 app entry def in this zome
    // let badge_app_entry_type = AppEntryDef::new(0.into(), id, EntryVisibility::Public);
    if let Ok(records) = &query(ChainQueryFilter::new().entry_type(UnitEntryTypes::Badge.try_into()?)) {
        if records.is_empty() {
            Ok(None)
        } else {
            let badge_action = records[0].action_address().clone();
            Ok(Some(badge_action))
        }
    } else {
        Err(wasm_error!(WasmErrorInner::Guest(
            "Cannot deserialise into a badge".into()
        )))
    }
}

#[hdk_extern]
fn get_badge(action_hash: ActionHash) -> ExternResult<String> {
    std::panic::set_hook(Box::new(zome_panic_hook));
    let maybe_record = get(action_hash, GetOptions::default())?;
    
    let badge = match maybe_record {
        Some(record) => record.entry().to_app_option::<Badge>().unwrap().unwrap(),
        None => return Err(wasm_error!(WasmErrorInner::Guest(
            "No badge exists at that address".into()
        ))),
    };

    Ok(badge.image_data)
}

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct GenerateBadgeInput {
    eth_address: String,
    eth_signed_contents: String,
}

#[hdk_extern]
fn generate_badge(input: GenerateBadgeInput) -> ExternResult<ActionHash> {
    // Get final snapshot
    let final_bucket: u32 = HOURS_OF_GAMEPLAY * 60 * 60 / BUCKET_SIZE_SEC;
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
    
    // And count number of placements this user had and generate badge
    // let placement_app_entry_type = AppEntryDef::new(0.into(), 0.into(), EntryVisibility::Public); 
    if let Ok(records) = &query(ChainQueryFilter::new().entry_type(UnitEntryTypes::Placement.try_into()?)) {
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
#[serde(rename_all = "camelCase")]
pub struct GenerateHrlInput {
    badge_action: ActionHash,
}

#[hdk_extern]
fn generate_hrl(input: GenerateHrlInput) -> ExternResult<String> {
    std::panic::set_hook(Box::new(zome_panic_hook));
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
       
        let hrl_anchor = Path::from(&hrl).path_entry_hash()?;
        // Create link from HRL to badge
        create_link(
            hrl_anchor, // use hrl as anchor
            input.badge_action.clone(),        
            links::HRLtoBadgeLink::link_type(),
            links::HRLtoBadgeLink::link_tag(),
        )?;

        Ok(hrl)
    } else {
        Err(wasm_error!(WasmErrorInner::Guest(
            "Badge not found".into()
        )))
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
#[serde(rename_all = "camelCase")]
pub struct SaveNftInput {
    nft_id: u32,
    contract_address: String,
    hrl: String,
    badge_action: ActionHash,
}

#[hdk_extern]
fn save_nft(input: SaveNftInput) -> ExternResult<ActionHash> { // TODO! reconcile this with generate_hrl zomecall
    std::panic::set_hook(Box::new(zome_panic_hook));
    let nft_record = NftRecord::new(input.nft_id, input.contract_address);
    let action_hash = create_entry(EntryTypes::NftRecord(nft_record.clone()))?;
    
    // Create link from HRL to NFT, for verification purposes
    let hrl_anchor = Path::from(input.hrl).path_entry_hash()?;

    create_link(
        hrl_anchor.clone(),         
        action_hash.clone(),       
        links::HRLtoNftIdLink::link_type(),
        links::HRLtoNftIdLink::link_tag(),
    )?;

    Ok(action_hash)
}

#[hdk_extern]
fn get_nft(hrl: String) -> ExternResult<Option<NftRecord>> { // retrieve the registered NFT for a given HRL
    std::panic::set_hook(Box::new(zome_panic_hook));
    let hrl_anchor = Path::from(hrl).path_entry_hash()?;

    let links_result = get_links(
        hrl_anchor,         
        LinkTypes::HRLtoNftIdLink,
        Some(HRLtoNftIdLink::link_tag())
    )?;

    if links_result.is_empty() {
        Ok(None)
    } else {
        let target: ActionHash = ActionHash::from(links_result[0].target.clone()).into(); // There should only be one link
        let maybe_record = get(target, GetOptions::default())?;

        match maybe_record {
            Some(record) => {
                let maybe_nft_record = record.entry().to_app_option::<NftRecord>()
                    .map_err(|err| wasm_error!(WasmErrorInner::Guest(err.into())))?;
                    
                Ok(maybe_nft_record)
            },
            None => Ok(None)
        }           
    }
}

#[hdk_extern]
fn view_nft_image(hrl: String) -> ExternResult<Option<String>> { // retrieve the badge image for a given HRL

    std::panic::set_hook(Box::new(zome_panic_hook));
    let hrl_anchor = Path::from(hrl).path_entry_hash()?;

    let links_result = get_links(
        hrl_anchor,         
        LinkTypes::HRLtoBadgeLink,
        Some(HRLtoBadgeLink::link_tag())
    )?;

    if links_result.is_empty() {

        Ok(None)
    } else {

        let target: ActionHash = ActionHash::from(links_result[0].target.clone()).into(); // There should only be one link
        let maybe_record = get(target, GetOptions::default())?;
        let badge = match maybe_record {
            Some(record) => record.entry().to_app_option::<Badge>().unwrap().unwrap(),
            None => return Err(wasm_error!(WasmErrorInner::Guest(
                "No badge exists at that address".into()
            ))),
        };

        Ok(Some(badge.image_data))
    }
}

fn publish_badge(badge: Badge) -> ExternResult<ActionHash> {
    let action_hash = create_entry(EntryTypes::Badge(badge.clone()))?;
    Ok(action_hash)
}
