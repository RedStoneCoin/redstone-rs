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
use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, StatusCode};
use std::sync::Mutex;
use rand::Rng;
// struct
struct Message {
    id: String,
    msg_type: u64,
    payload: String,
    port: u64,
}
impl Message {
    fn new(id: String, msg_type: u64, payload: String, port: u64) -> Message {
        Message {
            id,
            msg_type,
            payload,
            port
        }
    }
    // to string connected by dot
    fn to_string(&self) -> String {
        format!("{}.{}.{}.{}", self.id, self.msg_type, self.payload, self.port)
    }
    // from string connected by dot
    fn from_string(s: String) -> Message {
        let mut split = s.split(".");
        let id = split.next().unwrap().to_string();
        let msg_type = split.next().unwrap().parse::<u64>().unwrap();
        let payload = split.next().unwrap().to_string();
        let port = split.next().unwrap().parse::<u64>().unwrap();
        Message {
            id,
            msg_type,
            payload,
            port
        }
    }
}



lazy_static! {
    static ref P2P_ID: String = {
        let mut rng = rand::thread_rng();
        let id = rng.gen_ascii_chars().take(10).collect::<String>();
        id
    };
    // p2p port will be set later
    static ref P2P_PORT: u64 = 0;
    static ref P2P_UP_VER: String = "".to_string();
    static ref PEERLIST: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref MESSAGEIDS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

fn get_peerlist() -> HashMap<String, String> {
    let mut peerlist = PEERLIST.lock().unwrap();
    peerlist.clone()
}

fn add_peer(id: String, ip: String) {
    let mut peerlist = PEERLIST.lock().unwrap();
    peerlist.insert(id, ip);
    // get count of peers
    let count = peerlist.len();
    info!("Peerlist count: {}", count);
    for (id, ip) in peerlist.iter() {
        info!("Peer: {} {}", id, ip);
    }
}

fn remove_peer(id: String) {
    let mut peerlist = PEERLIST.lock().unwrap();
    peerlist.remove(&id);
    // get count of peers
    let count = peerlist.len();
    info!("Peerlist count: {}", count);
}

fn get_peer(id: String) -> String {
    let mut peerlist = PEERLIST.lock().unwrap();
    peerlist[&id].clone()
}

fn peer_to_string(id: String, ip: String) -> String {
    format!("{}.{}", id, ip)
}

fn peer_from_string(s: String) -> (String, String) {
    let mut split = s.split(".");
    let id = split.next().unwrap().to_string();
    let ip = split.next().unwrap().to_string();
    (id, ip)
}


fn message_handle(message_id: String, message_payload: String, message_type: u64, message_ip: String,port: u64) -> String {
    match message_type {
        0 => {
            add_peer(message_payload, format!("{}.{}",message_ip,port));
            P2P_ID.to_string()
        }
        1 => {
            remove_peer(message_payload);
            P2P_ID.to_string()
        }
        // get peer list
        2 => {
            let mut peerlist = PEERLIST.lock().unwrap();
            let mut peerlist_string = String::new();
            for (id, ip) in peerlist.iter() {
                peerlist_string.push_str(&format!("{}.{}.", id, ip));
            }
            peerlist_string
        }
        // new peer connected here is it
        3 => {
            let mut id_peer = peer_from_string(message_payload.clone()).clone().0;
            let mut ip_peer = peer_from_string(message_payload.clone()).clone().1;
            let mut peerlist = PEERLIST.lock().unwrap();
            if peerlist.contains_key(&id_peer) {
            } else {
                peerlist.insert(id_peer, ip_peer);
            }
            return "0".to_string();
        }
        _ => {
            error!("Invalid message type {}", message_type);
            "".to_string()
        }
    }
}




#[post("/message_p2p")]
async fn post_message(
    mut data: web::Json<String>,
    req_body: String,
    req: HttpRequest
) -> impl Responder {
    let ip = req.connection_info().remote_addr().unwrap().to_string();
    // if format is not correct, return error
    match Message::from_string(data.clone()) {
        // if message is correct, handle it
        Message {
            id: message_id,
            msg_type: message_type,
            payload: message_payload,
            port: message_port,

        } => {
            info!("message_id: {}, message_type: {}, message_payload: {}, port: {}", message_id, message_type, message_payload, message_port);
            HttpResponse::Ok().body(format!("{}", message_handle(message_id, message_payload, message_type, ip, message_port)))
        }
        _ => {
            return HttpResponse::BadRequest().body("Bad Request");
        }
    }
}
async fn index(
    req: HttpRequest
) -> impl Responder {
    // get ip from request
    let ip = req.connection_info().remote_addr().unwrap().to_string();
    info!("new http");
    HttpResponse::Ok().body(format!("redstone p2p: {:?}", ip))
}

// get peerlist

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

pub async fn connect(ip: String, id: String,port: u64) -> Result<(), reqwest::Error> {
    let client = Client::new();
    // message.id message type payload
    // messsage payload is our_id
    let body =  format!("\"{}.{}.{}.{}\"", 0, 0, id, port);
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Content-Encoding", "br, gzip".parse().unwrap());
    let mut request = client.post(&format!("http://{}/message_p2p", ip))
        .body(body)
        .headers(headers);
    let response = request.send().await?;
    if response.status() == StatusCode::OK {
        let body = response.text().await?;
        info!("connected to peer {}:{}", body, ip);
        add_peer(body, ip);
    } 
    Ok(())
}

pub async fn send_message(peer_id: String,message_type: u64, message_id: u64) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let peer = get_peer(peer_id);
    // we send message to peer
    let body =  format!("\"{}.{}.{}.{}\"", message_type, message_id, P2P_ID.to_string(), P2P_PORT.to_string());
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Content-Encoding", "br, gzip".parse().unwrap());
    let mut request = client.post(&format!("http://{}/message_p2p", peer))
        .body(body)
        .headers(headers);
    let response = request.send().await?;
    if response.status() == StatusCode::OK {
        let body = response.text().await?;
        info!("message sent to peer {}:{}", body, peer);
    }
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
        connect(bootnode.clone(), P2P_ID.to_string(), port).await.unwrap();
        // wait for it and then ask for the peerlist
        //GetPeerListP2P().await.unwrap();
    } else {
        info!("Bootnode is the same as this node");
    }
    Ok(())
}

pub async fn start(port: u64, bootnode: String) -> std::io::Result<()> {
    // start http server and connect to bootnode asynchronously
    // create 32bit key for id
    P2P_PORT = port;

    start_http(port, bootnode.clone()).await?;
    Ok(())
}