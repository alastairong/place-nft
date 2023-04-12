use hdi::prelude::*;
use crate::links::LinkTypes;

/* Validation Rules
1. Only 1 badge per agent
2. Only 1 HRL per badge
3. Only 1 badge per HRL
4. Badge must be created by the same agent as the HRL
5. HRL must be based on the same ETH address as the badge
6. Badge creator must have placed a placement
 */



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

// Check if the badge author has placed a placement
fn validate_create_entry(
  signed_hashed: SignedHashed<EntryCreationAction>,
  entry: Entry,
) -> ExternResult<ValidateCallbackResult> {
  let entry_hash = hash_entry(entry)?;
  match must_get_entry(entry_hash) {
    Ok(_) => {return Ok(ValidateCallbackResult::Invalid("Entry already exists".to_string()))}, // Is this always going to fail because it runs after commit?
    Err(_) => {
      // Get all of the author's actions
      let author = signed_hashed.hashed.content.author();
      let action_hash = signed_hashed.as_hash().clone();
      let filter = ChainFilter::new(action_hash);
      let author_actions = must_get_agent_activity(author.clone(), filter)?;

      for registered_action in author_actions.into_iter() {
        match registered_action.action.hashed.action_type() {
          // If the action creates an entry, check if the entry is a placement
          ActionType::Create => {
            //   let entry = registered_action.action.hashed.content;
            //   if entry == EntryTypes::Placement {
            //     // If at least one entry is a placement validation will return true
            //   }
            continue
          },
          _ => continue,
        }
      }
      Ok(ValidateCallbackResult::Invalid("Badge creator must have placed a placement".to_string()))
    }
  }
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
    
    let target_action = must_get_action(target_action_hash)?;
    let target_author = target_action.hashed.content.author();
    if target_author == &link_author {
      Ok(ValidateCallbackResult::Valid)
    } else {
      Ok(ValidateCallbackResult::Invalid("Only badge author can generate HRL link".to_string()))
    }
  } else {
    Ok(ValidateCallbackResult::Invalid("Only hrl link type allowed".to_string())) // Once more link type validations are added we should change this to a match 
  }
  
  
}