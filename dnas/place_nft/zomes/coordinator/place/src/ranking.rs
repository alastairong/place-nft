use hdk::prelude::*;
// use hdk::prelude::holo_hash::AgentPubKeyB64;
use place_integrity::*;
use zome_utils::{zome_panic_hook};
use crate::utils::*;
use crate::holo_hash::hash_type::Agent;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthorRankInput {
  pub author: AgentPubKey,
  pub bucket_index: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPlacementAuthorInput {
  pub placement: u32,
  pub bucket_index: u32,
}

/// Return Render next snapshot rank of author
/// Return 0 if author has not published a pixel
#[hdk_extern]
pub fn get_author_rank(input: GetAuthorRankInput) -> ExternResult<u16> {
  std::panic::set_hook(Box::new(zome_panic_hook));
  // debug!("*** get_author_rank() CALLED");
  let mut placement_links = get_links(
    bucket_index_to_path(input.bucket_index).path_entry_hash()?,
    LinkTypes::PlacementLink,
    None,
  )?;
  // debug!("*** get_author_rank() placement_links: {:?}", placement_links);

  if placement_links.len() == 0 {
    // debug!("*** get_author_rank() no placement_links found");
    return Ok(0);
  }

  placement_links.sort_by(|a, b| b.timestamp.cmp(&a.timestamp)); //newest first
  // debug!("*** get_author_rank() SORTED placement_links: {:?}", placement_links);
  /* For each placement check if its author matchs input ; attribute rank according to the number
   * of placements we had to go through
   */
 
  let mut i = 1;
  debug!("*** get_author_rank() iterating to check author rank");
  for link in placement_links.iter() {
    let author_bytes = link.author.clone().into_inner();
    // debug!("*** get_author_rank() author_bytes: {:?}", author_bytes);
    let author: HoloHash<Agent> = HoloHash::from_raw_39(author_bytes).unwrap();
    // debug!("*** get_author_rank() author: {:?}", author);
    if author == input.author.clone().into() {
      return Ok(i);
    }
    i += 1;
  }
  Ok(i)
}
