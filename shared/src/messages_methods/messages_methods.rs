//! # Message Handling
//!
//! This module provides functions for sending and receiving messages over a TCP stream. Messages
//! consist of a `Fragment` and associated data.
//!
//! ## Sending Messages
//!
//! The `send_message` function takes a mutable reference to a `TcpStream`, a `Fragment`, and data
//! as a vector of bytes. It serializes the `Fragment` to JSON, calculates the total message size,
//! and sends both the JSON message size and content along with the data to the client.
//!
//! ## Receiving Messages
//!
//! The `read_message` function reads the total message size, JSON message size, and content from
//! the `TcpStream`. It then deserializes the JSON message to a `Fragment` and returns it along
//! with the associated data as a vector of bytes.
//!
//! # Examples
//!
//! ```rust
//! use std::{io, net::TcpStream};
//! use crate::messages::{fragment_method_json::{fragment_to_string, string_to_fragment}, message::Fragment};
//!
//! fn main() -> io::Result<()> {
//!     // Example usage of sending and receiving messages
//!     let mut stream = TcpStream::connect("127.0.0.1:8080")?;
//!
//!     // Prepare a sample Fragment and data
//!     let sample_fragment = Fragment {
//!         // Initialize fields of the Fragment...
//!     };
//!     let data = vec![1, 2, 3, 4, 5];
//!
//!     // Send message to the client
//!     send_message(&mut stream, sample_fragment.clone(), &data)?;
//!
//!     // Read message from the client
//!     let (received_fragment, received_data) = read_message(&mut stream)?;
//!
//!     println!("Received Fragment: {:?}", received_fragment);
//!     println!("Received Data: {:?}", received_data);
//!
//!     Ok(())
//! }
//! ```

use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use crate::messages::{
    fragment_method_json::{fragment_to_string, string_to_fragment},
    message::Fragment,
};

/// Send a message to the client.
/// * `stream` - The TCP stream to send the message over.
/// * `fragment` - The `Fragment` to send.
/// * `data` - The data to send.
/// * Return: an `io::Result` containing `()` if successful, or an `io::Error` if an error occurred.
pub fn send_message_to_client(
    stream: &mut TcpStream,
    fragment: Fragment,
    data: Vec<u8>,
) -> Result<(), io::Error> {
    send_message(stream, fragment, &data)
}

pub fn send_message(
    stream: &mut TcpStream,
    fragment: Fragment,
    data: &Vec<u8>,
) -> Result<(), io::Error> {
    let json_message = fragment_to_string(&fragment)?;

    let json_message_size = json_message.len() as u32;
    let data_message_size = data.len() as u32;
    let total_message_size: u32 = json_message_size + data_message_size;

    stream.write_all(&total_message_size.to_be_bytes())?;
    stream.write_all(&json_message_size.to_be_bytes())?;
    stream.write_all(&json_message.as_bytes())?;
    stream.write_all(data)?;

    Ok(())
}

/// Read a message from the client.
/// * `stream` - The TCP stream to read the message from.
/// * Return: a tuple containing the `Fragment` and associated data as a vector of bytes if successful, or an `io::Error` if an error occurred.
pub fn read_message(stream: &mut TcpStream) -> Result<(Fragment, Vec<u8>), io::Error> {
    let mut total_len_buf = [0; 4];
    match stream.read_exact(&mut total_len_buf) {
        Ok(_) => {}
        Err(err) => {
            println!("Error reading total message size");
            return Err(err);
        }
    };
    let total_message_size = u32::from_be_bytes(total_len_buf);

    let mut json_len_buf = [0; 4];
    stream.read_exact(&mut json_len_buf)?;
    let json_message_size = u32::from_be_bytes(json_len_buf);

    if total_message_size < json_message_size {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Json message size if bigger than total message size",
        ));
    }

    let data_message_size = total_message_size - json_message_size;

    let mut sbuf = vec![0_u8; json_message_size as usize];
    stream.read(&mut sbuf)?;
    let s = String::from_utf8_lossy(&sbuf);

    let fragment_request = string_to_fragment(&s.to_string());
    let fragment = match fragment_request {
        Ok(r) => r,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Message received by server cannot be deserialized",
            ));
        }
    };

    let mut data = vec![0_u8; data_message_size as usize];
    if let Err(e) = stream.read_exact(&mut data) {
        return Err(e.into());
    }

    Ok((fragment, data))
}
