use hdi::prelude::*;

/// List of all Link types handled by this Zome
#[hdk_link_types]
pub enum LinkTypes {
    PlacementLink,
    SnapshotLink,
    OldToNewSnapshotLink,
    NewToOldSnapshotLink,
    HRLtoBadgeLink,
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


