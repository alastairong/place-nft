use hdk::prelude::*;
use place_integrity::*;
use crate::placements::*;
use zome_utils::{zome_panic_hook, get_typed_from_links};
use crate::utils::*;

// A snapshot represents the state at the BEGINNING of the bucket

#[hdk_extern]
pub fn publish_starting_snapshot(_: ()) -> ExternResult<Option<Snapshot>> {
    debug!("*** publish_starting_snapshot() CALLED");
    std::panic::set_hook(Box::new(zome_panic_hook));
    let first = Snapshot::create_first();
    publish_snapshot(&first)?;
    // debug!("*** publish_starting_snapshot() first snapshot created: {}", first.bucket_index);
    Ok(Some(first))
}

// Called to publish a snapshot for a bucket
// We don't know if pixels have been placed in current bucket before snapshot made
// so we will create a snapshot based on all placements from previous bucket plus all placements so far from current bucket
// This means that some pixels may be double counted, but that's ok as it won't affect the final snapshot
// (i.e. it's just replaying the same moves)
#[hdk_extern]
pub fn publish_snapshot_at(bucket: u32) -> ExternResult<Option<Snapshot>> {
    debug!("*** publish_snapshot_at({}) CALLED", bucket);
    std::panic::set_hook(Box::new(zome_panic_hook));

    let maybe_current = get_snapshot_at(bucket)?;
    // Bail if current snapshot doesn't exist
    if maybe_current.is_some() {
        warn!("publish_snapshot_at({}) Aborting: snapshot already exists.", bucket);
        return Ok(maybe_current);
    }

    let maybe_previous = get_snapshot_at(bucket - 1)?;
    // Bail if current snapshot doesn't exist
    if maybe_previous.is_none() {
        warn!("publish_snapshot_at({}) Aborting: previous snapshot does not exist.", bucket);
        return Ok(None);
    }
    let mut base_snapshot = maybe_previous.unwrap();

    let mut last_bucket_placements = get_placements_at(bucket - 1)?;
    let mut current_bucket_placements = get_placements_at(bucket)?;

    last_bucket_placements.append(&mut current_bucket_placements);
    base_snapshot.increment(last_bucket_placements);

    publish_snapshot(&base_snapshot)?;
    return Ok(Some(base_snapshot));
}

// Zome Function
// Return Snapshot at given bucket, if any
#[hdk_extern]
pub fn get_snapshot_at(bucket_index: u32) -> ExternResult<Option<Snapshot>> {
    std::panic::set_hook(Box::new(zome_panic_hook));
    // debug!("*** get_snapshot_at() CALLED - bucket: {}", bucket_index);
    let bucket_path = bucket_index_to_path(bucket_index);
    // debug!("get_snapshot_at() at path: {}", path_to_str(&bucket_path.clone().typed(LinkTypes::SnapshotLink)?));
    let pairs: Vec<(Snapshot, Link)> = get_typed_from_links(
        bucket_path.path_entry_hash()?,
        LinkTypes::SnapshotLink,
        None,
   )?;
   if pairs.is_empty() {
        // warn!("Snapshot not found for bucket: {}", bucket_index);
        return Ok(None);
        //return error(&format!("Snapshot not found for bucket: {}", time_bucket_index));
   }
   Ok(Some(pairs[0].0.clone()))
}

pub fn publish_snapshot(snapshot: &Snapshot) -> ExternResult<ActionHash> {
    // Commit new rendering
    let action_hash = create_entry(EntryTypes::Snapshot(snapshot.to_owned()))?;
    let entry_hash = hash_entry(snapshot)?;
    // Link to current bucket
    let path = bucket_index_to_path(snapshot.bucket_index);
    let author = agent_info()?.agent_latest_pubkey;
    debug!("Publishing snapshot at index {}, path: {}", snapshot.bucket_index , path_to_str(&path.clone().typed(LinkTypes::SnapshotLink)?));
    let _ = create_link(
        path.path_entry_hash()?, 
        entry_hash,
        LinkTypes::SnapshotLink, 
        LinkTag::from(author.into_inner()))?;
    // Done
    Ok(action_hash)
}