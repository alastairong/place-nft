use hdi::prelude::*;
pub mod placement;
pub mod snapshot;
pub mod double_pixel;
pub mod globals;

pub use crate::placement::*;
pub use crate::snapshot::*;
pub use crate::double_pixel::*;
pub use crate::globals::*;

#[hdk_entry_defs]
#[unit_enum(PlaceEntryTypes)]
pub enum PlaceEntry {
   #[entry_def(required_validations = 2, visibility = "public")]
   Placement(Placement),
   #[entry_def(required_validations = 2, visibility = "public")]
   Snapshot(Snapshot),
}


/// List of all Link types handled by this Zome
#[hdk_link_types]
pub enum LinkTypes {
    PlacementLink,
    SnapshotLink,
    OldToNewSnapshotLink,
    NewToOldSnapshotLink,
    // Links to path?
    // Links to some anchor point? First link?
    // Think about links vs. paths?
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
