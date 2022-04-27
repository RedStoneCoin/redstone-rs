// Redstone P2P module
// Redstone will have more then 25 p2p functions
// Every p2p function will be a message type
// We will check src/p2p/message_types.rs for message types
use libp2p::{identity, PeerId};
use std::error::Error;

#[async_std::main]
async fn start_server() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);
    Ok(())
}