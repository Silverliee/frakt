use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FragmentRequest {
    worker_name: String,
    maximal_work_load: u32,
}

impl FragmentRequest {
    pub fn new(worker_name: String, maximal_work_load: u32) -> FragmentRequest {
        FragmentRequest {
            worker_name,
            maximal_work_load,
        }
    }

    pub fn serialize(&self) -> String {
        let mut serialized = String::from("{\"FragmentRequest\":");
        serialized.push_str(&serde_json::to_string(&self).expect("Could not serialize request"));
        serialized.push('}');
        serialized
    }
}