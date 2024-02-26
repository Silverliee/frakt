use super::message::Fragment;

pub fn fragment_to_string(message: &Fragment) -> Result<String, serde_json::Error> {
    serde_json::to_string(message)
}

pub fn string_to_fragment(message: &str) -> Result<Fragment, serde_json::Error> {
    serde_json::from_str(message)
}
