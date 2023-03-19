use hdi::prelude::*;

/// A Public Entry representing the whole canvas for a specific time bucket
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
   pub image_data: Uint8Array, // 2 x 4-bit pixels per u8
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
}