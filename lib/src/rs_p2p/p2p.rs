// Redstone-p2p is a multithreaded Rust library for redstone p2p network.
use tokio::net::TcpStream;
use std::error::Error;
use async_std::prelude::Future;
use tokio::io::AsyncReadExt;
use std::collections::HashMap;
use tokio::net::TcpListener;
// import to socket address
use std::net::SocketAddr;
// log
use log::{info, debug, error};
pub struct P2p {
    id: String,
    peerlist: HashMap<String, TcpStream>,
    our_ip: String,
    our_port: u16,
}
pub struct Message {
    pub id: String,
    pub msg: String,
}
impl Message {
    pub fn new(id: String, msg: String) -> Message {
        Message {
            id: id,
            msg: msg,
        }
    }
    pub fn deserialize(msg: String) -> Message {
        let mut msg_split = msg.split(".");
        let id = msg_split.next().unwrap().to_string();
        let msg = msg_split.next().unwrap().to_string();
        Message::new(id, msg)
    }
    pub fn serialize(&self) -> String {
        format!("{}.{}", self.id, self.msg)
    }
}

impl P2p {
    pub fn new(id: String,our_ip: String, our_port: u16) -> P2p {
        P2p {
            id: id,
            peerlist: HashMap::new(),
            our_ip: our_ip,
            our_port: our_port,
        }
    }

    pub fn add_peer(&mut self, id: String, stream: TcpStream) {
        self.peerlist.insert(id, stream);
    }

    pub fn remove_peer(&mut self, id: String) {
        self.peerlist.remove(&id);
    }
    pub fn send_message(&mut self, id: String, msg: String) {
        let mut stream = self.peerlist.get(&id).unwrap();
        let msg = Message::new(self.id.clone(), msg);
        let msg = msg.serialize();
        stream.try_write(msg.as_bytes()).unwrap();
    }
    pub async fn connect_to_peer(&mut self,ip: String, port: u16) {
        let mut stream = TcpStream::connect((ip, port)).await.unwrap();
        let msg = Message::new(self.id.clone(), "ask_id".to_string());
        let msg = msg.serialize();
        stream.try_write(msg.as_bytes()).unwrap();
    }
    // mutlithreaded tcp listener for the p2p network
    pub async fn start_listening(&mut self) {
        info!("Listening on {}", self.our_ip);
        let addr: SocketAddr = self.our_ip.parse().unwrap();
        let mut listener = TcpListener::bind(addr).await.unwrap();
        loop {
            let (mut stream, addr) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                let mut msg = String::new();
                loop {
                    let n = stream.read(&mut buf).await.unwrap();
                    if n == 0 {
                        break;
                    }
                    msg.push_str(&String::from_utf8_lossy(&buf[..n]));
                    if msg.ends_with(".") {
                        break;
                    }
                }
                let msg = Message::deserialize(msg);
                println!("{}", msg.msg);
                if msg.msg == "ask_id" {
                    let msg = Message::new(self.id.clone(), "give_id".to_string());
                    let msg = msg.serialize();
                    stream.try_write(msg.as_bytes()).unwrap();
                }
                if msg.msg == "give_id" {
                    // add peer to peerlist
                    let id = msg.id;
                    self.add_peer(id, stream);
                }
            });
        }
    }
}


// returns p2p
pub async fn start(port: u16, bootnode: String) {
    let mut start_ip = format!("{}:{}", "127.0.0.2", port + 1);
    let mut p2p = P2p::new("1".to_string(), start_ip.clone(), port);
    info!("ip: {}", start_ip);
    p2p.start_listening().await;
    // connect to bootnode
    p2p.connect_to_peer(bootnode, port).await;
}