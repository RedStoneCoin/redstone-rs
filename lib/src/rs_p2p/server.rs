
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use tentacle::{
    builder::{MetaBuilder, ServiceBuilder},
    bytes::Bytes,
    context::{ProtocolContext, ProtocolContextMutRef, ServiceContext},
    secio::{peer_id::PeerId, SecioKeyPair},
    service::{ProtocolHandle, ServiceEvent, TargetProtocol, TargetSession},
    traits::{ServiceHandle, ServiceProtocol},
    SessionId,
};
use log::*;
use serde::{Deserialize, Serialize};
struct AppServiceHandle;

impl ServiceHandle for AppServiceHandle {
    fn handle_event(&mut self, _control: &mut ServiceContext, event: ServiceEvent) {
        if let ServiceEvent::ListenStarted { address: _ } = event {
            // println!("Listen started");Wil
        }
        log::info!("handle_event: {:?}", event);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Peers {
    reachable_peers: Vec<String>,
    disconnected_peers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    recipient: String,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Payload {
    Peers(Peers),
    Message(Message),
}

struct State {
    reachable_peers: HashMap<PeerId, Vec<SessionId>>,
    connected_peers: HashSet<PeerId>,
    pending_message: Option<Message>,
}

impl State {
    /// Disconnects from the session and return the no-longer reachable peers.
    fn disconnect(&mut self, id: SessionId) -> Vec<PeerId> {
        let mut removed = Vec::new();
        self.reachable_peers.retain(|k, v| {
            v.retain(|e| *e != id);
            if v.is_empty() {
                // no longer reachable
                removed.push(k.clone());
                false
            } else {
                true
            }
        });

        log::debug!("STATE: {:?}", self.reachable_peers);

        removed
    }

    fn handle_peers(&mut self, context: ProtocolContextMutRef, peers: Peers) {
        let mut added = Vec::new();
        let mut removed = Vec::new();

        let session = context.session;
        let self_peer_id = context.key_pair().expect("secio").peer_id();

        for peer_id in std::iter::once(session.remote_pubkey.as_ref().expect("secio").peer_id())
            .chain(
                peers
                    .reachable_peers
                    .into_iter()
                    .filter_map(|peer_id| PeerId::from_str(&peer_id).ok()),
            )
        {
            // ignore self and direct connections
            if peer_id != self_peer_id {
                let connections = self.reachable_peers.entry(peer_id.clone()).or_default();
                if connections.is_empty() {
                    added.push(peer_id.to_base58());
                }
                if connections.iter().position(|x| *x == session.id).is_none()
                    // Filter the already directly connected peers
                    && (connections.is_empty() || self.connected_peers.get(&peer_id).is_none())
                {
                    connections.push(session.id);
                }
            }
        }

        for peer_id in peers
            .disconnected_peers
            .into_iter()
            .filter_map(|peer_id| PeerId::from_str(&peer_id).ok())
        {
            if let Some(v) = self.reachable_peers.get_mut(&peer_id) {
                v.retain(|e| *e != session.id);
                if v.is_empty() {
                    removed.push(peer_id.to_base58());
                }
            }
        }
        self.reachable_peers.retain(|_k, v| !v.is_empty());

        log::debug!("STATE: {:?}", self.reachable_peers);

        if !(added.is_empty() && removed.is_empty()) {
            let payload = Payload::Peers(Peers {
                reachable_peers: added,
                disconnected_peers: removed,
            });
            let bytes = Bytes::from(serde_json::to_vec(&payload).expect("serialize to JSON"));
            context
                .filter_broadcast(TargetSession::All, context.proto_id, bytes)
                .expect("broadcast message");
        }
    }

    fn handle_message(&mut self, context: ProtocolContextMutRef, message: Message) {
        let self_peer_id = context.key_pair().expect("secio").peer_id().to_base58();

        if self_peer_id == message.recipient {
            log::info!("Receive message to self: {}", message.message);
            return;
        }

        if let Ok(peer_id) = PeerId::from_str(&message.recipient) {
            if let Some(sessions) = self.reachable_peers.get(&peer_id) {
                let payload = Payload::Message(message);
                let bytes = Bytes::from(serde_json::to_vec(&payload).expect("serialize to JSON"));
                context
                    .filter_broadcast(
                        TargetSession::Multi(sessions.clone()),
                        context.proto_id,
                        bytes,
                    )
                    .expect("broadcast message");
            }
        }
    }
}

impl ServiceProtocol for State {
    fn init(&mut self, _context: &mut ProtocolContext) {}

    fn connected(&mut self, context: ProtocolContextMutRef, _version: &str) {
        let session = context.session;
        log::info!("p2p-message connected to {}", session.address);
        let remote_peer_id = session.remote_pubkey.as_ref().expect("secio").peer_id();
        self.connected_peers.insert(remote_peer_id);
        // Send `peers`.
        let ids: Vec<_> = self
            .reachable_peers
            .keys()
            .map(|id| id.to_base58())
            .collect();
        let payload = Payload::Peers(Peers {
            reachable_peers: ids,
            disconnected_peers: Vec::new(),
        });
        let bytes = Bytes::from(serde_json::to_vec(&payload).expect("serialize to JSON"));

        context.send_message(bytes).expect("send message");

        // Send `message`
        if let Some(message) = self.pending_message.take() {
            let payload = Payload::Message(message);
            let bytes = Bytes::from(serde_json::to_vec(&payload).expect("serialize to JSON"));

            context.send_message(bytes).expect("send message");
        }
    }

    fn disconnected(&mut self, context: ProtocolContextMutRef) {
        let session = context.session;
        log::info!("p2p-message disconnected from {}", session.address);

        let remote_peer_id = session.remote_pubkey.as_ref().expect("secio").peer_id();
        self.connected_peers.remove(&remote_peer_id);

        let peers = self.disconnect(session.id);
        if !peers.is_empty() {
            // Send `peers`.
            let ids: Vec<_> = peers.into_iter().map(|id| id.to_base58()).collect();
            let payload = Payload::Peers(Peers {
                reachable_peers: Vec::new(),
                disconnected_peers: ids,
            });
            let bytes = Bytes::from(serde_json::to_vec(&payload).expect("serialize to JSON"));

            context
                .filter_broadcast(TargetSession::All, context.proto_id, bytes)
                .expect("broadcast message");
        }
    }

    fn received(&mut self, context: ProtocolContextMutRef, data: Bytes) {
        let session = context.session;
        let payload_result: serde_json::Result<Payload> = serde_json::from_slice(&data);
        if let Ok(payload) = payload_result {
            log::info!(
                "p2p-message received from {}: {:?}",
                session.address,
                payload
            );

            match payload {
                Payload::Peers(peers) => self.handle_peers(context, peers),
                Payload::Message(message) => self.handle_message(context, message),
            }
        }
    }
}

struct AppArgs {
    port: u16,
    bootnode: Option<String>,
    target_peer_id: Option<String>,
    message: Option<String>,
}

impl Default for AppArgs {
    fn default() -> Self {
        Self {
            port: 1234,
            bootnode: None,
            target_peer_id: None,
            message: None,
        }
    }
}

/// Parses the command line args.
///
/// ## Usage
///
/// * `p2p-message`: start a node listening on the default port 1234.
/// * `p2p-message port`: start a node listening on the specified port.
/// * `p2p-message port bootnode`: start a node listening on the specified port and connect to
/// another node as the bootnode.
/// * `p2p-message port bootnode target_peer_id message`: start a node, connect to the bootnode, then send a message to `target_peer_id`.
fn parse_args() -> AppArgs {
    let mut parsed_args = AppArgs::default();
    parsed_args.port = 1234;
    parsed_args.bootnode = Some("/ip4/127.0.0.1/tcp/1234".to_string());
    parsed_args.target_peer_id = Some("QmQG5eQDnsHPh7x1RjwF63ZVvooNtgbjh2GUEmv6Z86zqM".to_string());
    parsed_args.message = Some(1.to_string());
    parsed_args
}

pub fn launch() {
    let args = parse_args();

    let mut rt = tokio::runtime::Runtime::new().expect("create tokio runtime");

    rt.block_on(async {
        let key_pair = SecioKeyPair::secp256k1_generated();
        log::info!(
            "listen on /ip4/127.0.0.1/tcp/{}/p2p/{}",
            args.port,
            key_pair.peer_id().to_base58()
        );
        // info! out peer id and key pair
        log::info!("peer_id: {}", key_pair.peer_id().to_base58());

        let pending_message = args.message.as_ref().and_then(|message| {
            args.target_peer_id.as_ref().map(|recipient| Message {
                recipient: recipient.clone(),
                message: message.clone(),
            })
        });
        let protocol_meta = MetaBuilder::new()
            .id(0.into())
            .service_handle(move || {
                let state = Box::new(State {
                    reachable_peers: HashMap::new(),
                    connected_peers: HashSet::new(),
                    pending_message: pending_message,
                });
                ProtocolHandle::Callback(state)
            })
            .build();

        let mut app_service = ServiceBuilder::default()
            .insert_protocol(protocol_meta)
            .key_pair(key_pair)
            // By default, tentacle auto closes the connection when it is idle for more than 10
            // seconds. Set this timeout to 1 day for this sample application.
            .timeout(std::time::Duration::new(86640, 0))
            .build(AppServiceHandle);

        app_service
            .listen(format!("/ip4/127.0.0.1/tcp/{}", args.port).parse().unwrap())
            .await
            .expect("listen");

        if let Some(bootnode) = args.bootnode {
            log::info!("dial {}", bootnode);
            app_service
                .dial(
                    bootnode.parse().expect("bootnode multiaddr"),
                    TargetProtocol::All,
                )
                .await
                .expect("connect bootnode");
        }

        {
            use futures::stream::StreamExt;
            while app_service.next().await.is_some() {
                // loop
            }
        }
    });
}
