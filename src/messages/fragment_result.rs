use super::complementary_types::u8data::U8Data;
use super::complementary_types::resolution::Resolution;
use super::complementary_types::range::Range;
use super::complementary_types::pixeldata::PixelData;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FragmentResult {
    id: U8Data,
    resolution: Resolution,
    range: Range,
    pixels: PixelData,
}

impl FragmentResult {
    pub fn new(id: U8Data, resolution: Resolution, range: Range, pixels: PixelData) -> FragmentResult {
        FragmentResult { id, resolution, range, pixels }
    }

    pub fn serialize(&self) -> String {
        let mut serialized = String::from("{\"FragmentResult\":");
        serialized.push_str(&serde_json::to_string(&self).expect("Could not serialize request"));
        serialized.push('}');
        serialized
    }
}