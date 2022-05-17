use bytes::Bytes;
use qp2p::{Config, Endpoint};
use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};
#[derive(Default, Ord, PartialEq, PartialOrd, Eq, Clone, Copy)]
struct XId(pub [u8; 32]);

// example marco polo p2p
pub async fn start() -> Result<(),Box<dyn std::error::Error>> {
    const MSG_MARCO: &str = "marco";
    const MSG_POLO: &str = "polo";

    // collect cli args
    let args: Vec<String> = env::args().collect();

    // create an endpoint for us to listen on and send from.
    let (node, mut incoming_conns, _contact) = Endpoint::new_peer(
        SocketAddr::from((Ipv4Addr::LOCALHOST, 3031)),
        &[],
        Config {
            idle_timeout: Duration::from_secs(60 * 60).into(), // 1 hour idle timeout.
            ..Default::default()
        },
    )
    .await?;

    // if we received args then we parse them as SocketAddr and send a "marco" msg to each peer.
    if true {
            let peer: SocketAddr = "127.0.0.1:3030"
                .parse()
                .expect("Invalid SocketAddr.  Use the form 127.0.0.1:1234");
            let msg = Bytes::from(MSG_MARCO);
            println!("Sending to {:?} --> {:?}\n", peer, msg);
            let (conn, mut incoming) = node.connect_to(&peer).await?;
            conn.send(msg.clone()).await?;
            // `Endpoint` no longer having `connection_pool` to hold established connection.
            // Which means the connection get closed immediately when it reaches end of life span.
            // And causes the receiver side a sending error when reply via the in-coming connection.
            // Hence here have to listen for the reply to avoid such error
            let reply = incoming.next().await?.unwrap();
            println!("Received from {:?} --> {:?}", peer, reply);
        println!("Done sending");
    }

    println!("\n---");
    println!("Listening on: {:?}", node.public_addr());
    println!("---\n");

    // loop over incoming connections
    while let Some((connection, mut incoming_messages)) = incoming_conns.next().await {
        let src = connection.remote_address();
        // loop over incoming messages
        while let Some(bytes) = incoming_messages.next().await? {
            println!("Received from {:?} --> {:?}", src, bytes);
            if bytes == *MSG_MARCO {
                let reply = Bytes::from(MSG_POLO);
                connection.send(reply.clone()).await?;
                println!("Replied to {:?} --> {:?}", src, reply);
            }
            println!();
        }
    }
    Ok(())
}
