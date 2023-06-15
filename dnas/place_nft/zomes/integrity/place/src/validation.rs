use hdi::prelude::*;
use crate::links::LinkTypes;
use crate::Badge;

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

  // Check if the entry being created is a badge
    match entry.try_into() {
      Ok(badge) => { 
        let _badge: Badge = badge;
        // If it is, get the author's history
        let author = signed_hashed.hashed.content.author();
        let action_hash = signed_hashed.as_hash().clone();
      let filter = ChainFilter::new(action_hash);
      let author_actions = must_get_agent_activity(author.clone(), filter)?;

      // search history for past entry creations
      for registered_action in author_actions.into_iter() {
        match registered_action.action.hashed.action_type() {
          // If the action creates an entry, check if the entry is a placement
          ActionType::Create => {
            let placement_app_entry_type = AppEntryDef::new(0.into(), 0.into(), EntryVisibility::Public);
            let entry_type = registered_action.action.hashed.content.entry_type().unwrap().clone();
            if entry_type == EntryType::App(placement_app_entry_type) {
              return Ok(ValidateCallbackResult::Valid)
            }
          },
          _ => continue,
        }
      }
      Ok(ValidateCallbackResult::Invalid("Badge creator must have placed a placement".to_string()))
    },
    Err(_) => Ok(ValidateCallbackResult::Valid),
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
    debug!("Validating HRL link");
    let link_author = create_link.hashed.content.author;
    let target_action_hash = create_link.hashed.content.target_address.into_action_hash().unwrap();
    debug!("Still validating");
    let target_action = must_get_action(target_action_hash)?;
    let target_author = target_action.hashed.content.author();
    if target_author == &link_author {
      debug!("Validated HRL link");
      Ok(ValidateCallbackResult::Valid)
    } else {
      Ok(ValidateCallbackResult::Invalid("Only badge author can generate HRL link".to_string()))
    }
  } else {
    debug!("Not an HRL link");
    Ok(ValidateCallbackResult::Valid)
  }
  
  
}