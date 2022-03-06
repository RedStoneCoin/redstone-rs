use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
    static ref MSG_TYPES: HashMap<u16, &'static str> = get_message_types();
}

fn get_message_types() -> HashMap<u16, &'static str> {
    let mut message_types = HashMap::new();
    message_types.insert(0, "Raw / Invalid");
    message_types.insert(1, "Rehandshake");
    message_types.insert(2, "Send peerlist (ask)");
    message_types.insert(3, "Send peerlist (response)");
    message_types.insert(4, "Sync Request");
    message_types.insert(5, "Sync Close");
    message_types.insert(6, "Sync Acknowledged");
    message_types.insert(7, "Handshake Init");
    message_types.insert(8, "Handshake Response");
    message_types.insert(13, "Get Block Count (ask)");
    message_types.insert(14, "Get Block Count (response)");
    message_types.insert(15, "Get Global Block Count (ask)");
    message_types.insert(16, "Get Global Block Count (response)");
    message_types.insert(17, "Get Chain count (ask)");
    message_types.insert(18, "Get Chain count (response)");
    message_types.insert(19, "Ping");
    message_types.insert(20, "Pong");
    message_types.insert(21, "Get Peer List (ask)");
    message_types.insert(22, "Get Peer List (response)");
    message_types.insert(23, "Announce peer");
    message_types.insert(24, "Get Block (ask)");
    message_types.insert(25, "Get Block (response)");
    message_types.insert(26, "Shutdown");
    message_types
}

pub fn get_message_type_name(message_type: u16) -> &'static str {
    match MSG_TYPES.get(&message_type) {
        Some(name) => name,
        None => "Unknown",
    }
}

// Language: rust
// Test: lib/src/p2p/message_types.rs
