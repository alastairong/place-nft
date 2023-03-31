use hdk::prelude::*;

/// Called the first time a zome call is made to the cell containing this zome
#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
  Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn get_NFT_image(_:()) -> ExternResult<ActionHash> {
   debug!("*** place_pixel() CALLED: {:?}", input);
   std::panic::set_hook(Box::new(zome_panic_hook));
   // Make sure not already placed
   let now = now();
   if already_placed(now)? {
      warn!("Pixel already placed for current bucket");
      return error("Pixel placed for current bucket")
   }
   // Prepare placement
   let placement = Placement::from_destructured(input);
   let path = get_path(now);
   // Commit
   let action_hash = create_entry(EntryTypes::Placement(placement.clone()))?;
   // Link to current bucket path
   let entry_hash = hash_entry(placement)?;
   debug!("*** place_pixel() path: {}", path_to_str(&path.clone().typed(LinkTypes::PlacementLink)?));
   let _ = create_link(
      path.path_entry_hash()?,
      entry_hash,
      LinkTypes::PlacementLink,
      LinkTag::from(()),
   )?;
   // Done
   Ok(action_hash)
}


async getNFTimage(): Promise<boolean> {
    return this.client.callZome({
        cap_secret: null,
        role_name: 'place_nft',
        zome_name: 'nft',
        fn_name: 'get_NFT_image',
        payload: null,
    });
    console.log("Calling getNFTimage");
    return true
}

async linkEthereumAddress(signature: string): Promise<boolean> {
    return this.client.callZome({
        cap_secret: null,
        role_name: 'place_nft',
        zome_name: 'nft',
        fn_name: 'link_ethereum_address',
        payload: signature,
    });