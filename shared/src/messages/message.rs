use serde::{Deserialize, Serialize};

use crate::{
    complementary_types::{
        pixeldata::PixelData, range::Range, resolution::Resolution, u8data::U8Data,
    },
    fractal_implementation::fractal::FractalDescriptor,
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Fragment {
    FragmentResult(FragmentResult),
    FragmentRequest(FragmentRequest),
    FragmentTask(FragmentTask),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FragmentTask {
    pub id: U8Data,
    pub fractal: FractalDescriptor,
    pub max_iteration: u16,
    pub resolution: Resolution,
    pub range: Range,
}

impl FragmentTask {
    pub fn new(
        id: U8Data,
        fractal: FractalDescriptor,
        max_iteration: u16,
        resolution: Resolution,
        range: Range,
    ) -> FragmentTask {
        FragmentTask {
            id,
            fractal,
            max_iteration,
            resolution,
            range,
        }
    }

    // //TODO: voir si y a pas plus simple
    // pub fn deserialize(json: &str) -> Result<FragmentTask, std::io::Error> {
    //     let mut res = json.replacen("{\"FragmentTask\":", "", 1);
    //     res.pop();

    //     if let Ok(fragment_task) = serde_json::from_str(&res) {
    //         Ok(fragment_task)
    //     } else {
    //         Err(io::Error::new(
    //             io::ErrorKind::Other,
    //             "Erreur lors de la desérialisation du message FragmentTask",
    //         ))
    //     }
    // }

    // pub fn serialize(&self) -> Result<String, std::io::Error> {
    //     let mut serialized = String::from("{\"FragmentTask\":");
    //     if let Ok(string_serialized) = &serde_json::to_string(&self) {
    //         serialized.push_str(string_serialized);
    //         serialized.push('}');
    //         Ok(serialized)
    //     } else {
    //         Err(io::Error::new(
    //             io::ErrorKind::Other,
    //             "Erreur lors de la sérialisation du message FragmentTask",
    //         ))
    //     }
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

    // pub fn serialize(&self) -> Result<String, io::Error> {
    //     let mut serialized = String::from("{\"FragmentResult\":");
    //     if let Ok(string_serialized) = &serde_json::to_string(&self) {
    //         serialized.push_str(string_serialized);
    //         serialized.push('}');
    //         Ok(serialized)
    //     } else {
    //         Err(io::Error::new(
    //             io::ErrorKind::Other,
    //             "Erreur lors de la sérialisation du message FragmentResult",
    //         ))
    //     }
    // }

    // pub fn deserialize(json: &str) -> Result<FragmentResult, std::io::Error> {
    //     let mut res = json.replacen("{\"FragmentResult\":", "", 1);
    //     res.pop();

    //     if let Ok(fragment_result) = serde_json::from_str(&res) {
    //         Ok(fragment_result)
    //     } else {
    //         Err(io::Error::new(
    //             io::ErrorKind::Other,
    //             "Erreur lors de la desérialisation du message FragmentTask",
    //         ))
    //     }
    // }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FragmentRequest {
    pub worker_name: String,
    pub maximal_work_load: u32,
}

impl FragmentRequest {
    pub fn new(worker_name: String, maximal_work_load: u32) -> FragmentRequest {
        FragmentRequest {
            worker_name,
            maximal_work_load,
        }
    }

    // pub fn serialize(&self) -> Result<String, io::Error> {
    //     let mut serialized = String::from("{\"FragmentRequest\":");
    //     if let Ok(string_serialized) = &serde_json::to_string(&self) {
    //         serialized.push_str(string_serialized);
    //         serialized.push('}');
    //         Ok(serialized)
    //     } else {
    //         Err(io::Error::new(
    //             io::ErrorKind::Other,
    //             "Erreur lors de la sérialisation du message FragmentRequest",
    //         ))
    //     }
    // }

    // pub fn deserialize(json: &str) -> Result<FragmentRequest, std::io::Error> {
    //     let mut res = json.replacen("{\"FragmentRequest\":", "", 1);
    //     res.pop();

    //     if let Ok(fragment_request) = serde_json::from_str(&res) {
    //         Ok(fragment_request)
    //     } else {
    //         Err(io::Error::new(
    //             io::ErrorKind::Other,
    //             "Erreur lors de la desérialisation du message FragmentTask",
    //         ))
    //     }
    // }
}
