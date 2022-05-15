use async_std::io;
use std::error::Error;
use libp2p::gossipsub::{
    GossipsubEvent, 
    IdentTopic as Topic, 
    MessageAuthenticity,
};
use libp2p::{
    gossipsub, 
    identity, 
    swarm::SwarmEvent, 
    Multiaddr, 
    PeerId
};

use futures::{
    prelude::*, 
    select
};
use crate::config::Config;
use log::info;
pub async fn start_server(config: Config) -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    info!("Local peer id: {:?}", local_peer_id);
    let p2p_port = config.p2p_port();
    let bootnode = config.bootnode();
    let local_ip = format!("/ip4/{}/tcp/{}", "0.0.0.0", p2p_port);
    let transport = libp2p::development_transport(local_key.clone()).await?;

    let topic = Topic::new("local-test-network");

    let mut swarm = {
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .build()
            .expect("Failed building the GOSSIPSUB config");
        
        let mut gossipsub: gossipsub::Gossipsub =
            gossipsub::Gossipsub::new(
                MessageAuthenticity::Signed(local_key), gossipsub_config)
                    .expect("Failed creating an instance of the GOSSIPSUB");

        gossipsub.subscribe(&topic).unwrap();

        // Connects to a Peer
        if let Some(explicit) = std::env::args().nth(2) {
            let explicit = explicit.clone();
            match explicit.parse() {
                Ok(id) => gossipsub.add_explicit_peer(&id),
                Err(err) => info!("Invalid peer id: {:?}", err),
            }
        }

        libp2p::Swarm::new(transport, gossipsub, local_peer_id)
    };

    // Listens across all OS assinged interfaces
    swarm.listen_on(local_ip.parse().unwrap()).unwrap();

    // Connects to another peer (if one was specified)
    let address: Multiaddr = bootnode.parse().unwrap();
    match swarm.dial(address.clone()) {
        Ok(_) => info!("CONNECTED TO {:?}", address),
        Err(e) => info!("Dial {:?} failed: {:?}", address, e),
    };

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    // Kick it off
    loop {
        select! {
            line = stdin.select_next_some() => {
                if let Err(e) = swarm
                    .behaviour_mut()
                    .publish(
                        topic.clone(), 
                        line.unwrap()
                        .as_bytes())
                {
                    info!("Publish error: {:?}", e);
                }
            },
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(GossipsubEvent::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                }) => info!(
                    "MESSAGE RECEIVED FROM {:?} [ID: {}]: {}",
                    peer_id,
                    id,
                    String::from_utf8_lossy(&message.data)
                ),
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("Listening on {:?}", address);
                }
                _ => {}
            }
        }
    }
}