//! # Fragment Serialization
//!
//! This module provides functions for serializing and deserializing fragments using JSON format.
//!
//! ## Serialization
//!
//! The `fragment_to_string` function takes a `Fragment` reference and converts it into a JSON-formatted string.
//!
//! ## Deserialization
//!
//! The `string_to_fragment` function takes a JSON-formatted string and attempts to deserialize it into a `Fragment`.
//!
//! # Examples
//!
//! ```rust
//! use super::message::Fragment;
//! use serde_json;
//!
//! fn main() {
//!     // Example usage of fragment serialization and deserialization
//!     let sample_fragment = Fragment {
//!         // Initialize fields of the Fragment...
//!     };
//!
//!     // Serialize fragment to JSON
//!     let json_string_result = fragment_to_string(&sample_fragment);
//!
//!     match json_string_result {
//!         Ok(json_string) => {
//!             println!("Serialized Fragment as JSON: {}", json_string);
//!
//!             // Deserialize JSON back to Fragment
//!             let deserialized_fragment_result = string_to_fragment(&json_string);
//!
//!             match deserialized_fragment_result {
//!                 Ok(deserialized_fragment) => {
//!                     println!("Deserialized JSON to Fragment: {:?}", deserialized_fragment);
//!                 }
//!                 Err(err) => eprintln!("Error deserializing JSON: {}", err),
//!             }
//!         }
//!         Err(err) => eprintln!("Error serializing Fragment to JSON: {}", err),
//!     }
//! }
//! ```

use super::message::Fragment;

/// Convert a `Fragment` reference to a JSON-formatted string.
/// * `message` - The `Fragment` reference to convert.
/// * Return: a `Result` containing the JSON-formatted string if successful, or a `serde_json::Error` if an error occurred.
pub fn fragment_to_string(message: &Fragment) -> Result<String, serde_json::Error> {
    serde_json::to_string(message)
}

/// Convert a JSON-formatted string to a `Fragment`.
/// * `message` - The JSON-formatted string to convert.
/// * Return: a `Result` containing the `Fragment` if successful, or a `serde_json::Error` if an error occurred.
pub fn string_to_fragment(message: &str) -> Result<Fragment, serde_json::Error> {
    serde_json::from_str(message)
}
