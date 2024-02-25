use std::io;

use serde::{Deserialize, Serialize};

use crate::complementary_types::{pixeldata::PixelData, range::Range, resolution::Resolution, u8data::U8Data};

use super::fragment_task::FragmentTask;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FragmentResult {
    pub id: U8Data,
    pub resolution: Resolution,
    pub range: Range,
    pub pixels: PixelData,
}

impl FragmentResult {
    pub fn new(
        id: U8Data,
        resolution: Resolution,
        range: Range,
        pixels: PixelData,
    ) -> FragmentResult {
        FragmentResult {
            id,
            resolution,
            range,
            pixels,
        }
    }

    pub fn create(task: &FragmentTask) -> FragmentResult {
        let pixel_data = PixelData::new(
            task.id.offset + task.id.count,
            task.resolution.nx as u32 * task.resolution.ny as u32,
        );
        let id = task.id;
        let resolution = task.resolution;
        let range = task.range;
        return FragmentResult::new(id, resolution, range, pixel_data);
    }

    pub fn serialize(&self) -> Result<String, io::Error> {
        let mut serialized = String::from("{\"FragmentResult\":");
        if let Ok(string_serialized) = &serde_json::to_string(&self) {
            serialized.push_str(string_serialized);
            serialized.push('}');
            Ok(serialized)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Erreur lors de la sérialisation du message FragmentResult"))
        }
    }
}
