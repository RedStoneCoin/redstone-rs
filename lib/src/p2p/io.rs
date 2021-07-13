use std::{
    io::{Read, Write},
    net::SocketAddr,
};

use super::{
    message::{P2pMessage, LEN_BYTES},
    peers::{get_lock, is_incoming, unlock_peer},
};

pub fn send(
    data: &P2pMessage,
    to: &SocketAddr,
    read: bool,
) -> Result<Option<P2pMessage>, Box<dyn std::error::Error>> {
    // get the peer stream
    let incoming = is_incoming(to);
    let mut stream = get_lock(to, incoming)?;
    let encoded = data.to_string();
    stream.write(encoded.as_bytes())?;
    unlock_peer(stream, incoming)?;
    if read {
        // TODO: read from the stream and get response
        todo!()
    } else {
        return Ok(None);
    }
}

pub fn read(
    addr: &SocketAddr,
    timeout: u64,
) -> Result<Option<P2pMessage>, Box<dyn std::error::Error>> {
    let incoming = is_incoming(addr);
    let mut stream = get_lock(addr, incoming)?;
    loop {
        let buffer = [0u8; LEN_BYTES];
        // try and read LEN_BYTES bytes from stream

        if let Ok(_) = stream.read_exact(&mut buffer) {
            // decode this into a string (like 000000091)
            let len_string = String::from_utf8(buffer.to_vec())?;
            let len_string_trimmed = len_string.trim_start_matches("0").to_string();
            let len: usize = len_string_trimmed.parse()?;
            let buffer = [0u8; len];
            // read exacly len bytes

        }
        break; //TODO
    }
    todo!()
}
