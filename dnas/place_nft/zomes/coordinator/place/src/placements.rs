use hdk::prelude::*;
use place_integrity::*;
use zome_utils::{zome_panic_hook, error, get_typed_from_links};
use crate::utils::*;
 

/// Add placement to current bucket
#[hdk_extern]
pub fn place_pixel(input: DestructuredPlacement) -> ExternResult<ActionHash> {
   debug!("*** place_pixel() CALLED: {:?}", input);
   std::panic::set_hook(Box::new(zome_panic_hook));
   // Make sure not already placed
   let now = now();
   if already_placed(now)? {
      warn!("Pixel already placed for current bucket");
      return error("Pixel placed for current bucket")
   }
   // Prepare placement
   let placement = Placement::from_destructured(input);
   let path = get_path(now);
   // Commit
   let action_hash = create_entry(EntryTypes::Placement(placement.clone()))?;
   // Link to current bucket path
   let entry_hash = hash_entry(placement)?;
   debug!("*** place_pixel() path: {}", path_to_str(&path.clone().typed(LinkTypes::PlacementLink)?));
   let _ = create_link(
      path.path_entry_hash()?,
      entry_hash,
      LinkTypes::PlacementLink,
      LinkTag::from(()),
   )?;
   // Done
   Ok(action_hash)
}

/// Return all placements at given bucket
#[hdk_extern]
pub fn get_placements_at(bucket_index: u32) -> ExternResult<Vec<Placement>> {
   std::panic::set_hook(Box::new(zome_panic_hook));
   debug!("*** get_placements_at() CALLED - {}", bucket_index);
   let time = bucket_index * BUCKET_SIZE_SEC + START_TIME;
   let bucket_path = get_path(time);
   debug!("*** get_placements_at() bucket_path: {}", path_to_str(&bucket_path.clone().typed(LinkTypes::PlacementLink)?));
   // Get placements at given bucket path
   let mut pairs: Vec<(Placement, Link)> = get_typed_from_links(
      bucket_path.path_entry_hash()?,
      LinkTypes::PlacementLink,
      None,
   )?;
   // Sort by Link timestamp
   pairs.sort_by(|a, b| b.1.timestamp.cmp(&a.1.timestamp));
   debug!("****** sorted pairs:");
   for pair in pairs.iter() {
      debug!(" - {:?}", pair.1.timestamp)
   }
   // Return only Placement
   let placements: Vec<Placement> = pairs.iter()
      .map(|pair| pair.0.clone())
      .collect();
   // Done
   debug!("*** get_placements_at() END - {}", placements.len());
   Ok(placements)
}

// Check in user if user has already placed a pixel in current bucket
#[hdk_extern]
pub fn already_placed(now: u32) -> ExternResult<bool> {
    let path = get_path(now);
    let mut link_exists = false;
    let query_args = ChainQueryFilter::default()
        .include_entries(false)
        .action_type(ActionType::CreateLink);

    let links = query(query_args)?;
    for link_record in links {
        match link_record.action() {
            Action::CreateLink(c) => {
                // get the link type
                let link_type = LinkTypes::from_type(
                    c.zome_index,
                    c.link_type,
                )?
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("Link type should be exist".to_string())
                    ),
                )?;
                // check if link is to current bucket path and is a placement link
                if c.base_address == path.path_entry_hash()?.into()
                    && link_type == LinkTypes::PlacementLink
                {
                    link_exists = true;
                    break;
                } else {
                    continue;
                }
            }
            _ => unreachable!(),
        };
    }
    
    Ok(link_exists)
}

// Get count of all my placements for NFT trophy stat
#[hdk_extern]
pub fn get_my_placements_count(_: ()) -> ExternResult<u16> {
    let query_args = ChainQueryFilter::default()
        .include_entries(false)
        .action_type(ActionType::CreateLink);

    let links = query(query_args)?;
    let mut placement_count = 0;
    for link_record in links {
        match link_record.action() {
            Action::CreateLink(c) => {
                // get the link type
                let link_type = LinkTypes::from_type(
                    c.zome_index,
                    c.link_type,
                )?
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("Link type should be exist".to_string())
                    ),
                )?;
                // check if link is to current bucket path and is a placement link
                if link_type == LinkTypes::PlacementLink
                {
                    placement_count = placement_count + 1;
                } else {
                    continue;
                }
            }
            _ => unreachable!(),
        };
    }

    Ok(placement_count)
}

/** DEBUGGING API */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaceAtInput {
   placement: DestructuredPlacement,
   bucket_index: u32,
}

/// Zome Function
/// Add placement to given bucket
#[hdk_extern]
pub fn place_pixel_at(input: PlaceAtInput) -> ExternResult<ActionHash> {
    debug!("*** place_pixel_at() CALLED: {:?}", input);
    std::panic::set_hook(Box::new(zome_panic_hook));
    let placement = Placement::from_destructured(input.placement);
    let time = input.bucket_index * BUCKET_SIZE_SEC + START_TIME; 

    let path = get_path(time);
    // Commit
    let action_hash = create_entry(EntryTypes::Placement(placement.clone()))?;
    // Link to current bucket path
    let entry_hash = hash_entry(placement)?;
    debug!("*** place_pixel() path: {}", path_to_str(&path.clone().typed(LinkTypes::PlacementLink)?));
    let author = agent_info()?.agent_latest_pubkey;
    let _ = create_link(
       path.path_entry_hash()?,
       entry_hash,
       LinkTypes::PlacementLink,
       LinkTag::from(author.into_inner()),
    )?;
    // Done
    Ok(action_hash)
}