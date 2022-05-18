
use std::{error::Error, net::SocketAddr};
use quinn::Endpoint;
use quinn::RecvStream;
use quinn::SendStream;
// import file common.rs
// to use make_client_endpoint and make_server_endpoint
use crate::rs_p2p::common::{make_client_endpoint, make_server_endpoint};

/// Runs a QUIC server bound to given address and returns server certificate.
fn run_server(addr: SocketAddr) -> Result<Vec<u8>, Box<dyn Error>> {
    let (mut incoming, server_cert) = make_server_endpoint(addr)?;
    // accept a single connection
    tokio::spawn(async move {
        let quinn::NewConnection { connection, .. } = incoming.next().await.unwrap().await.unwrap();
        println!(
            "[server] incoming connection: addr={}",
            connection.remote_address()
        );
    });

    Ok(server_cert)
}

/// Attempt QUIC connection with the given server address.
async fn run_client(endpoint: &Endpoint, server_addr: SocketAddr) {
    let connect = endpoint.connect(server_addr, "localhost").unwrap();
    let quinn::NewConnection { connection, .. } = connect.await.unwrap();
    println!("[client] connected: addr={}", connection.remote_address());
}
// simple test
async fn test() {
}