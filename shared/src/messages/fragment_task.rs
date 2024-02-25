

use std::io;

use serde::{Deserialize, Serialize};

use crate::{complementary_types::{range::Range, resolution::Resolution, u8data::U8Data}, fractal_implementation::fractal::FractalDescriptor};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FragmentTask {
    pub id: U8Data,
    pub fractal: FractalDescriptor,
    pub max_iteration: u16,
    pub resolution: Resolution,
    pub range: Range,
}

impl FragmentTask {
    pub fn new(id: U8Data, fractal: FractalDescriptor, max_iteration: u16, resolution: Resolution, range: Range) -> FragmentTask {
         FragmentTask { id, fractal,max_iteration, resolution, range }
     }

    //TODO: voir si y a pas plus simple
    pub fn deserialize(json: &str) -> Result<FragmentTask, std::io::Error>{
        let mut res = json.replacen("{\"FragmentTask\":", "", 1);
        res.pop();

        if let Ok(fragment_task) = serde_json::from_str(&res) {
            Ok(fragment_task)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Erreur lors de la desérialisation du message FragmentTask"))
        }
    }

    pub fn serialize(&self) -> Result<String, std::io::Error> {
        let mut serialized = String::from("{\"FragmentTask\":");
        if let Ok(string_serialized) = &serde_json::to_string(&self) {
            serialized.push_str(string_serialized);
            serialized.push('}');
            Ok(serialized)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Erreur lors de la sérialisation du message FragmentTask"))
        }
    }
}
