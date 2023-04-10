use hdi::prelude::*;
use crate::links::LinkTypes;
use place_integrity::EntryTypes;

///
#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
  match op {
    Op::StoreEntry(StoreEntry { action, entry, .. }) => validate_create_entry(action, entry),
    Op::RegisterCreateLink(RegisterCreateLink {
        create_link,
    }) => validate_create_link(create_link),
    _ => Ok(ValidateCallbackResult::Valid),
  }
}

fn validate_create_entry(
  signed_hashed: SignedHashed<EntryCreationAction>,
  entry: Entry,
) -> ExternResult<ValidateCallbackResult> {
  let author = signed_hashed.hashed.content.author();
  let filter: ChainFilter; // TODO - chainfilter

  let author_actions = must_get_agent_activity(author.clone(), filter)?;

  for action in author_actions.into_iter() {
    // check if the action is a CreateEntry
    // if a CreateEntry, check if the entry is a placement
    if entry_type == EntryTypes::Placement {
      return Ok(ValidateCallbackResult::Valid);
    }
  }
  Ok(ValidateCallbackResult::Invalid("Badge creator must have placed a placement".to_string()))
}

pub fn validate_create_link(
    create_link: SignedHashed<CreateLink>,
) -> ExternResult<ValidateCallbackResult> {
  let ScopedLinkType {
    zome_index: _,
    zome_type: hrl_link_type,
  } = LinkTypes::HRLtoBadgeLink.try_into()?;

  if hrl_link_type == create_link.hashed.link_type {
    let link_author = create_link.hashed.content.author;
    let target_action_hash = create_link.hashed.content.target_address.into_action_hash().unwrap();
    let target_author = must_get_action(target_action_hash)?.hashed.content.author();
    if target_author == &link_author {
      Ok(ValidateCallbackResult::Valid)
    } else {
      Ok(ValidateCallbackResult::Invalid("Only badge author can generate HRL link".to_string()))
    }
  } else {
    Ok(ValidateCallbackResult::Invalid("Only hrl link type allowed".to_string())) // Once more link type validations are added we should change this to a match 
  }
  
  
}