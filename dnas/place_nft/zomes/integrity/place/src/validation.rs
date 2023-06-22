use hdi::prelude::holo_hash::hash_type::Agent;
use hdi::prelude::{*, ActionType, Action};
use crate::links::LinkTypes;
use crate::UnitEntryTypes;
use crate::Badge;
use ethers_core::types::*;
use anyhow::{Result};
use std::str::FromStr;

/* Validation Rules
1. Badge creator must demonstrate control of eth address (signature check)
2. Only 1 action per badge entry for this user // Only necessary if we enforce uniqueness of tokenuri in smart contract
3. HRL must be created by same agent as linked badge
4. Only 1 HRL per badge for this user  
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
  debug!("Validating create entry");
  // Check if the entry being created is a badge
    match entry.try_into() {
      Ok(badge) => {
        debug!("Entry is a badge");  
        let author = signed_hashed.hashed.content.author();
        let badge: Badge = badge;
        let created_badge_entry_hash = signed_hashed.hashed.content.entry_hash();
        
        // Validation rule 1
        debug!("Checking signature");
        let is_valid_signature = verify_eth_signature(&author, &badge.eth_address, &badge.eth_signed_contents)
        .map_err(|e| {
            error!("Error verifying signature: {}", e);
            wasm_error!(WasmErrorInner::Guest("Error verifying signature".into()))
        })?;
        
        if !is_valid_signature {
          return Ok(ValidateCallbackResult::Invalid("Badge creator must demonstrate control of eth address".to_string()))
        } 
        debug!("Signature match. Moving on to next validation rule");
        // Fetch agent history for further checks
        debug!("Fetching agent activity");
        let previous_action_hash = signed_hashed.hashed.prev_action().clone();
        let filter = ChainFilter::new(previous_action_hash); // start chain filter from previous action to skip current badge action
        let author_actions = must_get_agent_activity(author.clone(), filter)?;
        debug!("Fetched agent activity");
        // Define entry types for checks
        let placement_app_entry_type: EntryType = UnitEntryTypes::Placement.try_into()?; 
        let badge_app_entry_type: EntryType = UnitEntryTypes::Badge.try_into()?;

        // Create struct to track conditions
        let mut has_placement = false;
        let mut no_same_badge = true; // this exact badge hasn't been committed before

        debug!("Iterate through agent history");
        // Go through agent activity history for further validation
        for registered_action in author_actions.into_iter() {
          match registered_action.action.hashed.action_type() {
            // If the action creates an entry...
            ActionType::Create => {
              debug!("Found create action. Checking...");
              let entry_type = registered_action.action.hashed.content.entry_data().unwrap().1.clone();
              match entry_type {
                // Validation Rule 2
                et if et == badge_app_entry_type => {
                  debug!("Found another badge. Checking if it's the same badge...");
                  let entry_hash = registered_action.action.hashed.content.entry_hash().unwrap();
                  
                  if entry_hash == created_badge_entry_hash {
                    no_same_badge = false;
                    debug!("Badge already created. Validation failed");
                    continue
                  }
                },
                // Validation Rule 6
                et if et == placement_app_entry_type => {
                  debug!("Found placement");
                  has_placement = true;
                    continue
                  }
                _ => continue,
              };
            }
            _ => continue,
          };
        }

        if has_placement && no_same_badge {
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
  debug!("Validating create link");
  if hrl_link_type == create_link.hashed.link_type {
    debug!("This is an HRL");
    let link_author = create_link.hashed.content.author.clone();
    
    let target_address = create_link.hashed.content.target_address.clone();
    let target_action_hash = target_address.into_action_hash().unwrap(); 
    let target_action = must_get_action(target_action_hash.clone())?;
    let target_author = target_action.hashed.content.author();
    
    // Validation rule 3
    debug!("Checking that the author is also the author of the target badge");
    if target_author != &link_author {
      return Ok(ValidateCallbackResult::Invalid("Only badge author can generate HRL link".to_string()))
    }

    // Validation rule 5
    debug!("Checking that the HRL aligns with the badge content");
    let base_address = create_link.hashed.content.base_address.clone();
    let link_base = base_address.into_entry_hash().unwrap();
    let badge_record = must_get_valid_record(target_action_hash.clone())
    .map_err(|e| {
        error!("Error fetching linked badge for validation rule 5: {}", e);
        wasm_error!(WasmErrorInner::Guest("Error fetching linked badge for validation rule 5".into()))
      })?;

    let link_target_badge = badge_record.entry().to_app_option::<Badge>().unwrap().unwrap();
    let badge_eth_address = link_target_badge.eth_address;

    let base_path_string = format!("{}{}", target_action_hash, badge_eth_address);

    let base_path = Path::from(base_path_string.as_str()).path_entry_hash()?;

    if link_base != base_path {
      return Ok(ValidateCallbackResult::Invalid("HRL base address must be derived from target action and badge eth address".to_string()))
    }

    // // Validation rule 4
    debug!("Checking that the badge doesn't already have an HRL");
    let previous_action_hash = create_link.hashed.prev_action.clone();
    let filter = ChainFilter::new(previous_action_hash); // start chain filter from previous action to skip current badge action
    let author_actions = must_get_agent_activity(link_author.clone(), filter)?;
    debug!("Iterating through agent activity");
    // Go through agent activity history to look for another HRL link
    for registered_action in author_actions.into_iter() {

      match registered_action.action.hashed.content { 
        // If the action creates a link
        Action::CreateLink(create_link) => {
          if hrl_link_type == create_link.link_type {
            let link_target = create_link.target_address.into_action_hash().unwrap(); 
            if link_target == target_action_hash {
              return Ok(ValidateCallbackResult::Invalid("Only 1 HRL per badge for this user".to_string()))
            }
          }          
        },
        _ => continue,
      };
    }
    Ok(ValidateCallbackResult::Valid)
  } else {
    Ok(ValidateCallbackResult::Valid)
  }
}

fn verify_eth_signature(agent_pubkey: &HoloHash<Agent>, eth_address: &str, signature: &str) -> Result<bool> {
  debug!("Decoding address {:?}", eth_address);

  let new_eth_address;
  if eth_address.starts_with("0x") {
      new_eth_address = &eth_address[2..];
  } else {
      new_eth_address = eth_address;
  }
  let address_bytes = match hex::decode(new_eth_address) {
    Ok(bytes) => bytes,
    Err(e) => {
        debug!("Error decoding Ethereum address: {}", e);
        return Err(anyhow::Error::msg("Error decoding Ethereum address"));
    }
  };
  debug!("Address bytes: {:?}", address_bytes);
  let mut address_array = [0u8; 20];
  address_array.copy_from_slice(address_bytes.as_slice());
  let address = H160::from(address_array);
  debug!("Address decoded");
  let signature = ethers_core::types::Signature::from_str(signature).unwrap();
  debug!("Signature decoded");
  let message: RecoveryMessage = agent_pubkey.get_raw_39().try_into().ok().unwrap();
  debug!("Message is {:?}", message);
  debug!("Verifying signature");
  let verified = signature.verify(message, address);
  debug!("Verification result: {:?}", verified);
  if verified.is_ok() {
    Ok(true)
  } else {
    Ok(false)
  }
}


// Copy this implementation from the HDK
#[derive(
  Clone, PartialEq, Debug, Default, serde::Deserialize, serde::Serialize, SerializedBytes,
)]
#[repr(transparent)]
struct Component(#[serde(with = "serde_bytes")] Vec<u8>);

/// Wrap bytes.
impl From<Vec<u8>> for Component {
  fn from(v: Vec<u8>) -> Self {
      Self(v)
  }
}

impl From<&str> for Component {
  fn from(s: &str) -> Self {
      let bytes: Vec<u8> = s
          .chars()
          .flat_map(|c| (c as u32).to_le_bytes().to_vec())
          .collect();
      Self::from(bytes)
  }
}

#[derive(
  Clone, Debug, PartialEq, Default, serde::Deserialize, serde::Serialize, SerializedBytes,
)]
#[repr(transparent)]
struct Path(Vec<Component>);

impl From<&str> for Path {
  fn from(s: &str) -> Self {
      Self(vec![Component::from(s)])
  }
}

impl Path {
  /// What is the hash for the current [ `Path` ]?
  pub fn path_entry_hash(&self) -> ExternResult<holo_hash::EntryHash> {
      hash_entry(Entry::App(AppEntryBytes(
          SerializedBytes::try_from(self).map_err(|e| wasm_error!(e))?,
      )))
  }
}