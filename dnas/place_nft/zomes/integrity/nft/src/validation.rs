use hdi::prelude::*;

///
#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
  match op {
    Op::RegisterCreateLink(RegisterCreateLink {
        create_link,
    }) => validate_link_create(create_link),
    _ => Ok(ValidateCallbackResult::Valid),
  }
}

pub fn validate_link_create(
    _create_link: SignedHashed<CreateLink>,
) -> ExternResult<ValidateCallbackResult> {
    println!("TODO: Check if this is a link from nft to badge, and if so, check that the link creator is the badge author");
    Ok(ValidateCallbackResult::Valid)
}