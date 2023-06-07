use hdi::prelude::*;
// use image::{ImageBuffer, Rgba};
// use imageproc::{drawing::draw_text_mut};
use rusttype::{Font, Scale};
use photon_rs::native::{open_image, save_image};
use crate::snapshot::Snapshot;
use crate::double_pixel::DoublePixel;

// Defines coordinates and pixel colors for each placement in a Snapshot. 
// Used when applying Snapshot to template
pub struct PixelStruct {
   x: u32,
   y: u32,
//    pixel: Rgba<u8>
}

// Template constants
const TEMPLATE_OFFSET: (u32, u32) = (0, 0);
const PLACEMENT_COUNT_OFFSET: (u32, u32) = (80, 135);
const AUTHOR_OFFSET: (u32, u32) = (45, 148);
const FONT_DATA: &[u8; 168260] = include_bytes!("../Roboto-Regular.ttf");
const TEMPLATE_DATA: &[u8] = include_bytes!("../template.png");

/// A Public Entry representing the whole canvas for a specific time bucket
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
   pub image_data: Vec<u8>,
   pub eth_address: String,
   pub eth_signed_contents: String,
}

// impl Badge {
//     pub fn new(final_snapshot: Snapshot, placement_count: u32, author: &str, eth_address: String, eth_signed_contents: String) -> Self {
//       let mut img = image::load_from_memory(TEMPLATE_DATA).unwrap().to_rgba8();

//       // apply the snapshot image to the template output buffer
//       let pixel_array = convert_snapshot_to_pixel_array(final_snapshot.image_data);
      
//       for pixel in pixel_array.iter() {
//          img.put_pixel(pixel.x + TEMPLATE_OFFSET.0, pixel.y + TEMPLATE_OFFSET.1, pixel.pixel);
//       }
//       let font = Font::try_from_bytes(FONT_DATA as &[u8]).unwrap();
//       img = write_placement_count(placement_count, &font, img);
//       img = write_author_name(author, &font, img);

//       Self {
//          image_data: img.into_raw(),
//          eth_address,
//          eth_signed_contents, // we will check this in the validation
//       }
//     }
// }

// fn convert_snapshot_to_pixel_array(snapshot: Vec<DoublePixel>) -> Vec<PixelStruct> {
    
//    // first unpack the doublepixels into normal uint4s representing the 16 colors
//    let mut unpacked_array: Vec<u8> = Vec::new();
//    for doublepixel in snapshot.iter() {
//        unpacked_array.push(doublepixel.lower());
//        unpacked_array.push(doublepixel.upper());
//    }

//    // then convert the uint4s into PixelStructs 
//    let mut pixel_array: Vec<PixelStruct> = Vec::new();
//    for i in 0..unpacked_array.len() {
//        let color = COLOR_PALETTE[unpacked_array[i] as usize];
//        pixel_array.push(PixelStruct {
//            x: i as u32 % 128,
//            y: i as u32 / 128,
//            pixel: hex_to_rgba(color)
//        })
//    }
//    pixel_array
// }

// fn hex_to_rgba(hex: &str) -> Rgba<u8> {
//    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
//    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
//    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
//    let a = if hex.len() > 7 {
//        u8::from_str_radix(&hex[7..9], 16).unwrap()
//    } else {
//        255
//    };
//    Rgba([r, g, b, a])
// }

// fn write_placement_count(count: u32, font: &Font, mut image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
//    // Set the scale (font size)
//    let scale = Scale { x: 11.0, y: 11.0 };

//    // Set the position where the text will be drawn
//    let x = PLACEMENT_COUNT_OFFSET.0 as i32;
//    let y = PLACEMENT_COUNT_OFFSET.1 as i32;

//    // Set the text color
//    let color = Rgba([0u8, 0u8, 0u8, 255]);

//    // Draw the text onto the ImageBuffer
//    let count_text = count.to_string();
//    draw_text_mut(&mut image, color, x, y, scale, font, &count_text);
//    image
// }

// fn write_author_name(name: &str, font: &Font, mut image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
//    // Set the scale (font size)
//    let scale = Scale { x: 12.0, y: 12.0 };

//    // Set the position where the text will be drawn
//    let x = AUTHOR_OFFSET.0 as i32;
//    let y = AUTHOR_OFFSET.1 as i32;

//    // Set the text color
//    let color = Rgba([0u8, 0u8, 0u8, 255]);

//    // Draw the text onto the ImageBuffer
//    draw_text_mut(&mut image, color, x, y, scale, font, name);
//    image
// }

const COLOR_PALETTE: [&str; 16] = [
   "#FFFFFF",
   "#E4E4E4",
   "#888888",
   "#222222",
   "#FDA1D3",
   "#F82200",
   "#F09200",
   "#A86839",
   "#E6DA00",
   "#7BE400",
   "#0FC300",
   "#34D7E0",
   "#2B84CD",
   "#3200F4",
   "#DE64EA",
   "#8E0A85",
];