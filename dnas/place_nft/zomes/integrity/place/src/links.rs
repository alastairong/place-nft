use hdi::prelude::*;

/// List of all Link types handled by this Zome
#[hdk_link_types]
pub enum LinkTypes {
    HRLtoBadgeLink,
    BadgetoHRLLink,
    // BadgeToMinterAddressLink, // Do we need reverse to be able to look up all badges minted by a given address?
}

pub struct HRLtoBadgeLink;
impl HRLtoBadgeLink {
    const TAG: &'static [u8; 5] = b"asset";
    /// Create the tag
    pub fn link_tag() -> LinkTag {
        LinkTag::new(*Self::TAG)
    }
    pub fn link_type() -> LinkTypes {
        LinkTypes::HRLtoBadgeLink
    }
}


pub struct BadgetoHRLLink;
impl BadgetoHRLLink {
    const TAG: &'static [u8; 3] = b"hrl";
    /// Create the tag
    pub fn link_tag() -> LinkTag {
        LinkTag::new(*Self::TAG)
    }
    pub fn link_type() -> LinkTypes {
        LinkTypes::BadgetoHRLLink
    }
}


// pub struct BadgeToMinterAddressLink;
// impl BadgeToMinterAddressLink {
//     const TAG: &'static [u8; 14] = b"allowed_minter";
//     /// Create the tag
//     pub fn link_tag() -> LinkTag {
//         LinkTag::new(*Self::TAG)
//     }
//     pub fn link_type() -> LinkTypes {
//         LinkTypes::BadgeToMinterAddressLink
//     }
// }