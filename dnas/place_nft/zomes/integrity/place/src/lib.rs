use hdi::prelude::*;
pub mod placement;
pub mod snapshot;
pub mod double_pixel;
pub mod globals;
pub mod links;
pub mod badge;
pub mod minter;
pub mod nft_record;
mod validation;

pub use crate::placement::*;
pub use crate::snapshot::*;
pub use crate::double_pixel::*;
pub use crate::globals::*;
pub use crate::badge::*;
pub use crate::minter::*;
pub use crate::links::*;
pub use crate::validation::*;
pub use crate::nft_record::*;

#[derive(Serialize, Deserialize)]
#[hdk_entry_defs]
#[serde(tag = "type")]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
   #[entry_def(required_validations = 2, visibility = "public", cache_at_agent_activity = true)]
   Placement(Placement),
   #[entry_def(required_validations = 2, visibility = "public", cache_at_agent_activity = true)]
   Snapshot(Snapshot),
   #[entry_def(required_validations = 2, visibility = "public", cache_at_agent_activity = true)]
   Badge(Badge),
   #[entry_def(required_validations = 2, visibility = "public", cache_at_agent_activity = true)]
   NftRecord(NftRecord),
}


/// Validation you perform during the genesis process. Nobody else on the network performs it, only you.
/// There *is no* access to network calls in this callback
#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
