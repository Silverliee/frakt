//TODO : add fractal
use super::complementary_types::u8data::U8Data;
use super::complementary_types::resolution::Resolution;
use super::complementary_types::range::Range;
use super::fractal::fractal::Fractal;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FragmentTask {
    id: U8Data,
    fractal: Fractal,
    max_iteration: u16,
    resolution: Resolution,
    range: Range,
}

impl FragmentTask {
    // pub fn new(id: U8Data, max_iteration: u16, resolution: Resolution, range: Range) -> FragmentTask {
    //     FragmentTask { id, max_iteration, resolution, range }
    // }

    //TODO: voir si y a pas plus simple
    pub fn deserialize(json: &str) -> FragmentTask {
        let mut res = json.replacen("{\"FragmentTask\":", "", 1);
        res.pop(); //honteux

        serde_json::from_str(&res).expect("Could not deserialize FragmentTask")
    }

    //TODO: voir si y a pas plus simple
    pub fn serialize(&self) -> String {
        let mut serialized = String::from("{\"FragmentTask\":");
        serialized.push_str(&serde_json::to_string(&self).expect("Could not serialize request"));
        serialized.push('}');
        serialized
    }
}