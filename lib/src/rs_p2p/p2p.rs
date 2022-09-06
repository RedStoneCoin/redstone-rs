// HTTP based P2P network
// Peerlist is a hashmap of ID to Peer
// To send messages, use the send_message function which is post request to the peer
// recive messages are just respone from requests
// Actix is used for the http
// -- This may be rewritten to use libp2p or tcp/ip

// Node types:
// Normal node:
// - Syncs the blocks and passes them to the other peers, receives and passes on transactions
// Validator node:
// -  Does the same as a normal node but goes and has rounds on the p2p for validating blocks
// NewChain Proposer node:
// - This node mesures network stability and peformance and proposes new chains when the network is unstable


// How to check if node is validator?
// Ask peer for his type:
// If validator asks to verify random message with private key
// verify the message and checks the address in the validator db 

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
    static ref P2P_PORT: Mutex<u64> = Mutex::new(0);
    static ref P2P_UP_VER: String = "".to_string();
    static ref PEERLIST: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref MESSAGEIDS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

fn get_peerlist() -> HashMap<String, String> {
    let mut peerlist = PEERLIST.lock().unwrap();
    peerlist.clone()
}

async fn check_online_nodes() -> Result<(), reqwest::Error> {
    let mut peerlist = get_peerlist();
    if peerlist.len() == 0 {
        info!("No peers to check");
        return Ok(());
    }

    for (id, ip) in peerlist.iter() {
        let client = Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert("Content-Encoding", "br, gzip".parse().unwrap());
        match client.get(&format!("http://{}/ping", ip)).headers(headers).send().await {
            Ok(_) => {
                info!("{} is online", ip);
            },
            Err(_) => {
                info!("{} is offline", ip);
                remove_peer(id.to_string());
            }
        }
    }
    Ok(())
}

fn check_peer(id: String,ip: String) -> bool {
    let mut peerlist = PEERLIST.lock().unwrap();
    // check if peer with the same ip and port is already in the list
    if peerlist.contains_key(&id) {
        if peerlist[&id] == ip {
            return true;
        }
    }
    false
}


async fn add_peer(id: String, ip: String) {
    let mut peerlist = PEERLIST.lock().unwrap();
    peerlist.insert(id.clone(), ip.clone());
    let count = peerlist.len();
    info!("Peerlist count: {}", count);
    for (id, ip) in peerlist.iter() {
        info!("Peer: {} {}", id.clone(), ip.clone());
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



fn set_p2p_port(port: u64) {
    let mut p2p_port = P2P_PORT.lock().unwrap();
    *p2p_port = port;
}

fn get_p2p_port() -> u64 {
    let p2p_port = P2P_PORT.lock().unwrap();
    *p2p_port
}

// peer list to string
fn peerlist_to_string() -> String {
    let mut peerlist = get_peerlist();
    let mut s = String::new();
    for (id, ip) in peerlist.iter() {
        s.push_str(&format!("{}={};", id, ip));
    }
    s
}




async fn message_handle(message_id: String, message_payload: String, message_type: u64, message_ip: String,port: u64) -> String {
    check_online_nodes().await.unwrap();
    match message_type {
        0 => {
            let peer_ip = format!("{}:{}",message_ip,port);
            for (id, ip) in get_peerlist().iter() {
                if ip == &peer_ip.clone() {
                    info!("{} is already in the list", peer_ip);
                    // remove peer from the list
                    remove_peer(id.clone());
                    return "0x0".to_string();
                }
            }
            add_peer(message_payload, peer_ip.clone()).await;
            // return P2P_ID and peerlist
            let mut peerlist = get_peerlist();
            return format!("{}-{}", P2P_ID.clone(), peerlist_to_string());

        }
        1 => {
            remove_peer(message_payload);
            P2P_ID.to_string()
        }
        // get peer list
        2 => {
            if get_peerlist().len() == 0 {
                return "0x0".to_string();
            }
            return peerlist_to_string();
        }
        // new peer connected here is it
        3 => {
            if check_peer(message_payload.clone(), message_ip.clone()) {
                return "0x0".to_string();
            }
            // connect ip, our port
            connect(message_payload.clone().to_string(),P2P_ID.to_string(), get_p2p_port()).await;

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
            HttpResponse::Ok().body(format!("{}", message_handle(message_id, message_payload, message_type, ip, message_port).await))
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
// connect no peerlist, it connects without the first peer
pub async fn connect_npl(ip: String, id: String,port: u64) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let body =  format!("\"{}.{}.{}.{}\"", 0, 0, id.clone(), port.clone());
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Content-Encoding", "br, gzip".parse().unwrap());
    let mut request = client.post(&format!("http://{}/message_p2p", ip.clone()))
        .body(body)
        .headers(headers.clone());
    let response = request.send().await?;
    if response.status() == StatusCode::OK {
        let body = response.text().await?;
        if  body == "0x0" {
            info!("We got rejected for connecting to this peer");
        } 
        // not seperate peer id and peerlist
        let mut split = body.split("-");
        let peer_id = split.next().unwrap().to_string();
        let peer_list = split.next().unwrap().to_string();
        info!("connected peer_id: {}, peer_list: {:?}", peer_id, peer_list);
        add_peer(peer_id, ip.clone()).await;        
    } 
    Ok(())
}

pub async fn connect(ip: String, id: String,port: u64) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let body =  format!("\"{}.{}.{}.{}\"", 0, 0, id.clone(), port.clone());
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Content-Encoding", "br, gzip".parse().unwrap());
    let mut request = client.post(&format!("http://{}/message_p2p", ip.clone()))
        .body(body)
        .headers(headers.clone());
    let response = request.send().await?;
    if response.status() == StatusCode::OK {
        let body = response.text().await?;
        if  body == "0x0" {
            info!("We got rejected for connecting to this peer");
        } 
        // not seperate peer id and peerlist
        let mut split = body.split("-");
        let peer_id = split.next().unwrap().to_string();
        let peer_list = split.next().unwrap().to_string();
        info!("connected peer_id: {}, peer_list: {:?}", peer_id, peer_list);
        add_peer(peer_id, ip.clone()).await;        
        let mut split = peer_list.split(";");
        for peer in split {
            if peer == "" {
                continue;
            }
            let mut split = peer.split("=");
            let peer_id = split.next().unwrap().to_string();
            let peer_ip = split.next().unwrap().to_string();
            if peer_id != P2P_ID.clone() {
                info!("peer_id: {}, peer_ip: {}", peer_id, peer_ip);
                connect_npl(peer_ip.clone(), P2P_ID.to_string(), get_p2p_port()).await;
            }
        }

    } 
    Ok(())
}

pub async fn send_message(peer_id: String,message_type: u64, message_id: u64,message_payload: String) -> String {
    let client = Client::new();
    let peer = get_peer(peer_id);
    // we send message to peer
    let body =  format!("\"{}.{}.{}.{}\"", message_id, message_type, message_payload, get_p2p_port());
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Content-Encoding", "br, gzip".parse().unwrap());
    let mut request = client.post(&format!("http://{}/message_p2p", peer))
        .body(body)
        .headers(headers);
    let response = request.send().await.unwrap();
    if response.status() == StatusCode::OK {
        let body = response.text().await.unwrap();
        info!("sent message to peer {}:{}", body, peer);
        return body;
    }
    return "0".to_string();
}
#[get("/test_p2p")]
async fn test_p2p() -> impl Responder {
    // sends message to all peers
    let mut peerlist = get_peerlist();
    for (id, ip) in peerlist.iter() {
        return HttpResponse::Ok().body(send_message(id.to_string(), 0, 0,"yrd".to_string()).await);
        break;
    }
    return HttpResponse::Ok().body("test p2p");
}

#[get("/ping")]
async fn ping() -> impl Responder {
    // just returns 0
    return HttpResponse::Ok().body("0");
}


pub async fn start_http(port: u64, bootnode: String) -> std::io::Result<()> {
    let ip = format!("{}:{}", "127.0.0.1", port);
    info!("Starting p2p server on {}", ip);
    HttpServer::new(|| {
        App::new()
            .service(post_message)
            .service(test_p2p)
            .service(ping)
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
    set_p2p_port(port);

    start_http(port, bootnode.clone()).await?;
    Ok(())
}