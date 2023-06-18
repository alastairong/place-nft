use hdi::prelude::*;

/// List of all Link types handled by this Zome
#[hdk_link_types]
pub enum LinkTypes {
    PlacementLink,
    SnapshotLink,
    OldToNewSnapshotLink,
    NewToOldSnapshotLink,
    HRLtoBadgeLink,
    // BadgetoHRLLink // Unnecessary since we can validate that the badge only has 1 HRL by checking the source chain, since we already validate that the link and badge author are the same
    HRLtoNftIdLink
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


// pub struct BadgetoHRLLink;
// impl BadgetoHRLLink {
//     const TAG: &'static [u8; 3] = b"hrl";
//     /// Create the tag
//     pub fn link_tag() -> LinkTag {
//         LinkTag::new(*Self::TAG)
//     }
//     pub fn link_type() -> LinkTypes {
//         LinkTypes::BadgetoHRLLink
//     }
// }

pub struct HRLtoNftIdLink;
impl HRLtoNftIdLink {
    const TAG: &'static [u8; 5] = b"nftid";
    /// Create the tag
    pub fn link_tag() -> LinkTag {
        LinkTag::new(*Self::TAG)
    }
    pub fn link_type() -> LinkTypes {
        LinkTypes::HRLtoNftIdLink
    }
}


