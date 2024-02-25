use std::io;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub fn serialize(&self) -> Result<String,io::Error> {
        let mut serialized = String::from("{\"FragmentRequest\":");
        if let Ok(string_serialized) = &serde_json::to_string(&self) {
            serialized.push_str(string_serialized);
            serialized.push('}');
            Ok(serialized)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Erreur lors de la sérialisation du message FragmentRequest"))
        }
        
    }
}
