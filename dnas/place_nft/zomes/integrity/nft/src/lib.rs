use hdi::prelude::*;

pub mod links;
pub mod badge;
pub mod minter;

#[derive(Serialize, Deserialize)]
#[hdk_entry_defs]
#[serde(tag = "type")]
#[unit_enum(EntryTypesTypes)]
pub enum EntryTypes {
   #[entry_def(required_validations = 2, visibility = "public")]
   Badge(Badge),
   #[entry_def(required_validations = 2, visibility = "public")]
   Minter(Minter),
}



/// Validation you perform during the genesis process. Nobody else on the network performs it, only you.
/// There *is no* access to network calls in this callback
#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

///
#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
  match op {
    _ => Ok(ValidateCallbackResult::Valid),
  }
}