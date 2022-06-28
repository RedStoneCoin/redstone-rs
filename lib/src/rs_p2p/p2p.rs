// HTTP based P2P network
// Peerlist is a hashmap of ID to Peer
// To send messages, use the send_message function which is post request to the peer
// recive messages are just respone from requests
// Actix is used for the http

// HTTP
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::HttpRequest;
use std::collections::HashMap;
use log::{error, info};
use lazy_static::lazy_static;
use std::collections::HashSet;
use reqwest::{Client, StatusCode};
// struct
#[derive(Clone)]
struct Peer {
    id: String,
    ip: String,
}
struct Message {
    id: String,
    msg_type: u64,
    payload: String,
}
impl Message {
    fn new(id: String, msg_type: u64, payload: String) -> Message {
        Message {
            id,
            msg_type,
            payload,
        }
    }
    // to string connected by dot
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.id, self.msg_type, self.payload)
    }
    // from string connected by dot
    fn from_string(s: String) -> Message {
        let mut split = s.split(".");
        let id = split.next().unwrap().to_string();
        let msg_type = split.next().unwrap().parse::<u64>().unwrap();
        let payload = split.next().unwrap().to_string();
        Message {
            id,
            msg_type,
            payload,
        }
    }
}



lazy_static! {
    static ref P2P_ID: String = "".to_string();
    static ref P2P_UP_VER: String = "".to_string();
    static ref PEERLIST: HashMap<String, Peer> = HashMap::new();
    static ref MESSAGEIDS: HashSet<String> = HashSet::new();
}



fn peerlist_to_string(peerlist: &HashMap<String, Peer>) -> String {
    let mut s = String::new();
    for (id, peer) in peerlist.iter() {
        s.push_str(&format!("{}.{}.{}", id, peer.ip, peer.id));
    }
    s
}

fn peerlist_from_string(s: String) -> HashMap<String, Peer> {
    let mut peerlist = HashMap::new();
    let mut split = s.split(".");
    while let Some(id) = split.next() {
        let ip = split.next().unwrap();
        let id = split.next().unwrap();
        peerlist.insert(id.to_string(), Peer {
            id: id.to_string(),
            ip: ip.to_string(),
        });
    }
    peerlist
}

fn if_message_id_exist(message_id: &String) -> bool {
    MESSAGEIDS.contains(message_id)
}

fn message_handle(message_id: String, message_payload: String, message_type: u64, message_ip: String) -> String
{
    // payload types
    // 1 = connect, payload is peer_id
    return "".to_string();
}




#[post("/message_p2p")]
async fn post_message(
    mut data: web::Json<String>,
    req_body: String,
    req: HttpRequest
) -> impl Responder {
    let ip = req.connection_info().remote_addr().unwrap().to_string();
    let message = Message::from_string(data.clone());
    let message_id = message.id.clone();
    let message_type = message.msg_type;
    let message_payload = message.payload.clone();
    info!("message_id: {}, message_type: {}, message_payload: {}", message_id, message_type, message_payload);
    HttpResponse::Ok().body(format!("{}", message_handle(message_id, message_payload, message_type, ip)))
}
async fn index(
    req: HttpRequest
) -> impl Responder {
    // get ip from request
    let ip = req.connection_info().remote_addr().unwrap().to_string();
    info!("new http");
    HttpResponse::Ok().body(format!("redstone p2p: {:?}", ip))
}

/* examle
    let server = HttpServer::new(|| {
        App::new()
            .service(post_message)
            .route("/p2p", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;
*/

// function connect that uses reqwest to connect to a peer
pub async fn connect(ip: String, id: String) -> Result<(), reqwest::Error> {
    let client = Client::new();
    // header content type is application/json
    // body is a string
    Ok(())
}


pub async fn start_http(port: u64, bootnode: String) -> std::io::Result<()> {
    let ip = format!("{}:{}", "127.0.0.1", port);
    info!("Starting p2p server on {}", ip);
    HttpServer::new(|| {
        App::new()
            .service(post_message)
            .route("/p2p", web::get().to(index))
    })
    .bind(ip.clone() )?
    .run()
    .await.unwrap();
    // dr
    Ok(())
}

pub async fn start_other(port: u64, bootnode: String) -> std::io::Result<()> {
    let ip = format!("{}:{}", "127.0.0.1", port);
    // connect to bootnode
    if bootnode != ip.clone() {
        connect(bootnode.clone(), P2P_ID.to_string()).await.unwrap();
    } else {
        info!("Bootnode is the same as this node");
    }
    Ok(())
}

pub async fn start(port: u64, bootnode: String) -> std::io::Result<()> {
    // start http server and connect to bootnode asynchronously
    start_http(port, bootnode.clone()).await?;
    Ok(())
}