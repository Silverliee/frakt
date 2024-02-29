use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use crate::messages::{
    fragment_method_json::{fragment_to_string, string_to_fragment},
    message::Fragment,
};

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
