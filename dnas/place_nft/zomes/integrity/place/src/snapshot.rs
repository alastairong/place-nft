use hdi::prelude::*;
use crate::double_pixel::DoublePixel;
use crate::Placement;
use crate::globals::*;
use crate::LinkTypes;

/// For typescript bindings compatability
type Uint8Array = Vec<DoublePixel>;

/// A Public Entry representing the whole canvas for a specific time bucket
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
   pub image_data: Uint8Array, // 2 x 4-bit pixels per u8
   pub placement_count: u32, // Cumulative number of placements in this snapshot
   pub bucket_index: u32, // Number of 'bucket_size_sec' since START.
}

impl Snapshot {

    pub fn create_first() -> Self {
       // Create starting image data
       let length: u16 = CANVAS_SIZE * CANVAS_SIZE / 2;
       let image_data = vec![DoublePixel::new(0,0); length as usize];

       debug!("Creating first snapshot");
       Self {
            image_data,
            placement_count: 0,
            bucket_index: 0,
       }
    }
 
  
    /// Increment Snapshot to next time bucket with the given new placements
    pub fn increment(&mut self, placements: Vec<Placement>) {
       //assert!(self.image_data.len() == (CANVAS_SIZE * CANVAS_SIZE / 2) as usize);
       self.bucket_index = self.bucket_index + 1;
       self.placement_count = self.placement_count + placements.len() as u32;
       apply_pixels_to_canvas(&mut self.image_data, placements);
    }
 
 }
 
 
 /// Apply placements to 'image_data'
 pub fn apply_pixels_to_canvas(image_data: &mut Vec<DoublePixel>, placements: Vec<Placement>) {
    debug!("apply_pixels_to_canvas(): {} placements", placements.len());
    for placement in placements {
       //debug!("placing: {:?} | {}", placement, placement.index());
       let index: usize = (placement.index(CANVAS_SIZE) / 2) as usize;
       image_data[index].set_half(
          placement.color_index(),
          placement.index(CANVAS_SIZE) % 2 == 1,
       );
    }
 }
 
pub struct OldToNewSnapshotLink;
impl OldToNewSnapshotLink {

    pub fn link_type() -> LinkTypes {
        LinkTypes::OldToNewSnapshotLink
    }
}

pub struct SnapshotLink;
impl SnapshotLink {
    pub fn link_type() -> LinkTypes {
        LinkTypes::SnapshotLink
    }
}
