use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    net::{SocketAddr, TcpStream},
    sync::Mutex,
    thread,
    time::Duration,
};

lazy_static! {
    static ref INCOMING: Mutex<HashMap<SocketAddr, (TcpStream, bool)>> = Mutex::new(HashMap::new());
    static ref OUTGOING: Mutex<HashMap<SocketAddr, (TcpStream, bool)>> = Mutex::new(HashMap::new());
}

pub fn is_incoming(addr: &SocketAddr) -> bool {
    if (*INCOMING.lock().unwrap()).contains_key(addr) {
        return true;
    }
    false
}

pub fn get_lock(
    addr: &SocketAddr,
    incoming: bool,
) -> Result<TcpStream, Box<dyn std::error::Error>> {
    match incoming {
        true => {
            let mut lock = INCOMING.lock()?;
            if let Some((stream, locked)) = lock.get_mut(&addr) {
                while *locked {
                    thread::sleep(Duration::from_millis(10));
                }
                *locked = true;
                return Ok(stream.try_clone()?);
            } else {
                return Err("peer not in hashmap".into());
            }
        }
        false => {
            let mut lock = OUTGOING.lock()?;

            if let Some((stream, locked)) = lock.get_mut(&addr) {
                while *locked {
                    thread::sleep(Duration::from_millis(10));
                }
                *locked = true;
                return Ok(stream.try_clone()?);
            } else {
                return Err("peer not in hashmap".into());
            }
        }
    }
}

/// # Unlock Peer
/// Takes in the stream returned by lock peer and unlocks the peer
pub fn unlock_peer(peer: TcpStream, incoming: bool) -> Result<(), Box<dyn std::error::Error>> {
    let addr = peer.peer_addr()?;
    match incoming {
        true => {
            let mut lock = INCOMING.lock()?;
            if lock.contains_key(&addr) {
                lock.insert(addr, (peer, false));
                return Ok(());
            } else {
                return Err("peer not in hashmap".into());
            }
        }
        false => {
            let mut lock = OUTGOING.lock()?;

            if lock.contains_key(&addr) {
                lock.insert(addr, (peer, false));
                return Ok(());
            } else {
                return Err("peer not in hashmap".into());
            }
        }
    }
}
