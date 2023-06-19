use hdi::prelude::*;
use crate::links::LinkTypes;
use crate::UnitEntryTypes;
use crate::Badge;
use web3::signing::{keccak256, recover};
use web3::types::Recovery;
use anyhow::{anyhow, Result};

/* Validation Rules
1. Badge creator must demonstrate control of eth address (signature check)
2. Only 1 action per badge entry for this user (other users can't commit same badge entry since agentKey is part of entry)
3. HRL must be created by same agent as linked badge
4. Only 1 HRL per badge for this user (other users can't create HRL to same badge since we check that the author is the same)
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

fn validate_create_entry(
  signed_hashed: SignedHashed<EntryCreationAction>,
  entry: Entry,
) -> ExternResult<ValidateCallbackResult> {

  // Check if the entry being created is a badge
    match entry.try_into() {
      Ok(badge) => { 
        let author = signed_hashed.hashed.content.author();
        let badge: Badge = badge;
        let created_badge_entry_hash = signed_hashed.hashed.content.entry_hash();
        
        // Validation rule 1
        let is_valid_signature = verify_eth_signature(&author.to_string(), &badge.eth_address, &badge.eth_signed_contents)
        .map_err(|e| {
            error!("Error verifying signature: {}", e);
            wasm_error!(WasmErrorInner::Guest("Error verifying signature".into()))
        })?;
        
        if !is_valid_signature {
          return Ok(ValidateCallbackResult::Invalid("Badge creator must demonstrate control of eth address".to_string()))
        } 
      
        // Fetch agent history for further checks
        let previous_action_hash = signed_hashed.hashed.prev_action().clone();
        let filter = ChainFilter::new(previous_action_hash); // start chain filter from previous action to skip current badge action
        let author_actions = must_get_agent_activity(author.clone(), filter)?;

        // Define entry types for checks
        let placement_app_entry_type: EntryType = UnitEntryTypes::Placement.try_into()?; 
        let badge_app_entry_type: EntryType = UnitEntryTypes::Badge.try_into()?;

        // Create struct to track conditions
        let mut hasPlacement = false;
        let mut noSameBadge = true; // this exact badge hasn't been committed before


        // Go through agent activity history for further validation
        for registered_action in author_actions.into_iter() {
          match registered_action.action.hashed.action_type() {
            // If the action creates an entry...
            ActionType::Create => {
              let entry_type = registered_action.action.hashed.content.into_entry_data().unwrap().1;
              match entry_type {
                // Validation Rule 2
                badge_app_entry_type => {
                  let entry_hash = registered_action.action.hashed.content.entry_hash().unwrap();
                  
                  if entry_hash == created_badge_entry_hash {
                    noSameBadge = false;
                    continue
                  }
                },
                // Validation Rule 6
                placement_app_entry_type => {
                    hasPlacement = true;
                    continue
                  }
                _ => continue,
              };
            }
            _ => continue,
          };
        }

        if hasPlacement && noSameBadge {
          Ok(ValidateCallbackResult::Valid)
        } else {
          Ok(ValidateCallbackResult::Invalid("This creator cannot create this badge".to_string()))
        }

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
    let link_author = create_link.hashed.content.author.clone();
    let target_action_hash = create_link.hashed.content.target_address.into_action_hash().unwrap(); 
    let target_action = must_get_action(target_action_hash)?;
    let target_author = target_action.hashed.content.author();
    
    // Validation rule 3
    if target_author != &link_author {
      return Ok(ValidateCallbackResult::Invalid("Only badge author can generate HRL link".to_string()))
    }

    // Validation rule 5
    // let link_base = create_link.hashed.content.base_address.into_entry_hash().unwrap();
    // let badge_record = must_get_valid_record(target_action_hash)
    // .map_err(|e| {
    //     error!("Error fetching linked badge for validation rule 5: {}", e);
    //     wasm_error!(WasmErrorInner::Guest("Error fetching linked badge for validation rule 5".into()))
    //   })?;

    // let link_target_badge = badge_record.entry().to_app_option::<Badge>().unwrap().unwrap();
    // let badge_eth_address = link_target_badge.eth_address;

    // let base_path_string = format!("{}{}", target_action_hash, badge_eth_address);
    // let base_path = Path::from(base_path_string).path_entry_hash()?;

    // Validation rule 4
    let previous_action_hash = create_link.hashed.prev_action.clone();
    let filter = ChainFilter::new(previous_action_hash); // start chain filter from previous action to skip current badge action
    let author_actions = must_get_agent_activity(link_author.clone(), filter)?;

    // Go through agent activity history for further validation
    for registered_action in author_actions.into_iter() {
      
      match registered_action.action.hashed.action_type() {
        // If the action creates a link...
        ActionType::CreateLink => {
          match registered_action.action.hashed { // get the target for the link
            // Validation rule 2
            hrl_link_type => {
              let link_target: HoloHashed<Action> = registered_action.action.hashed.link_target().into();
              if link_target == target_action_hash {
                return Ok(ValidateCallbackResult::Invalid("Only 1 HRL per badge for this user".to_string()))
              }
            },
            _ => continue,
          };
        }
        _ => continue,
      };
    }
    Ok(ValidateCallbackResult::Valid)

    
  } else {
    Ok(ValidateCallbackResult::Valid)
  }
}

fn verify_eth_signature(agent_pubkey: &str, eth_address: &str, signature: &str) -> Result<bool> {
  
  let message = {agent_pubkey.to_string() + &eth_address};
  let eth_message = keccak256(
    format!(
        "{}{}{}",
        "\x19Ethereum Signed Message:\n",
        message.len(),
        message
    )
    .as_bytes(),
  );

  let signature = hex::decode(signature)?;

  let recovery = Recovery::from_raw_signature(eth_message, &signature)?;
  let recovery_id = recovery.recovery_id().ok_or(anyhow!("Invalid signature"))?;

  let pubkey = recover(&eth_message, &signature[..64], recovery_id)?;
  let pubkey = format!("{:02X?}", pubkey);

  Ok(pubkey.to_ascii_lowercase() == eth_address.to_ascii_lowercase())
}