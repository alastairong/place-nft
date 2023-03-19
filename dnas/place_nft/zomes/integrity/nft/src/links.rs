use hdi::prelude::*;

/// List of all Link types handled by this Zome
#[hdk_link_types]
pub enum LinkTypes {
    NFTtoBadgeLink,
    BadgetoNFTLink,
    BadgeToMinterAddressLink, // Do we need reverse to be able to look up all badges minted by a given address?
}

pub struct NFTtoBadgeLink;
impl NFTtoBadgeLink {
    const TAG: &'static [u8; 5] = b"asset";
    /// Create the tag
    pub fn link_tag() -> LinkTag {
        LinkTag::new(*Self::TAG)
    }
    pub fn link_type() -> LinkTypes {
        LinkTypes::NFTtoBadgeLink
    }
}


pub struct BadgetoNFTLink;
impl BadgetoNFTLink {
    const TAG: &'static [u8; 6] = b"nft_id";
    /// Create the tag
    pub fn link_tag() -> LinkTag {
        LinkTag::new(*Self::TAG)
    }
    pub fn link_type() -> LinkTypes {
        LinkTypes::BadgetoNFTLink
    }
}


pub struct BadgeToMinterAddressLink;
impl BadgeToMinterAddressLink {
    const TAG: &'static [u8; 14] = b"allowed_minter";
    /// Create the tag
    pub fn link_tag() -> LinkTag {
        LinkTag::new(*Self::TAG)
    }
    pub fn link_type() -> LinkTypes {
        LinkTypes::BadgeToMinterAddressLink
    }
}