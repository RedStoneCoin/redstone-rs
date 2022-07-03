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
// struct
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
    static ref P2P_ID: String = "1".to_string();
    static ref P2P_UP_VER: String = "".to_string();
    static ref PEERLIST: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref MESSAGEIDS: HashSet<String> = HashSet::new();
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
}

fn remove_peer(id: String) {
    let mut peerlist = PEERLIST.lock().unwrap();
    peerlist.remove(&id);
    // get count of peers
    let count = peerlist.len();
    info!("Peerlist count: {}", count);
}




fn if_message_id_exist(message_id: &String) -> bool {
    MESSAGEIDS.contains(message_id)
}


fn message_handle(message_id: String, message_payload: String, message_type: u64, message_ip: String) -> String
{
    match message_type {
        0 => {
            add_peer(message_payload, message_ip);
            P2P_ID.to_string()
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
        } => {
            info!("message_id: {}, message_type: {}, message_payload: {}", message_id, message_type, message_payload);
            HttpResponse::Ok().body(format!("{}", message_handle(message_id, message_payload, message_type, ip)))
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

pub async fn connect(ip: String, id: String) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let body =  format!("\"{}.{}.{}\"", id, 0, "connect");
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Content-Encoding", "br, gzip".parse().unwrap());
    let mut request = client.post(&format!("http://{}/message_p2p", ip))
        .body(body)
        .headers(headers);
    let response = request.send().await?;
    //println!("{:?}", response);
    if response.status() == StatusCode::OK {
        let body = response.text().await?;
        info!("connected to peer {}:{}", body, ip);
        add_peer(body, ip);
    } 

    Ok(())

}

// fn send message args, peer_id message_type, message_payload, message_id

pub async fn send_message(peer_id: String, message_type: u64, message_payload: String, message_id: String) -> Result<(), reqwest::Error> {
    // peer_ip is gotten from peerlist
    let peer_ip = "todo";//PEERLIST.get(&peer_id).unwrap().ip.clone();

    let client = Client::new();
    let body =  format!("\"{}.{}.{}\"", message_id, message_type, message_payload);
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Content-Encoding", "br, gzip".parse().unwrap());
    let mut request = client.post(&format!("http://{}/message_p2p", peer_ip))
        .body(body)
        .headers(headers);
    let response = request.send().await?;
    println!("{:?}", response);
    Ok(())
}

pub async fn send_message_to_all(message_type: u64, message_payload: String, message_id: String) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let body =  format!("\"{}.{}.{}\"", message_id, message_type, message_payload);
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Content-Encoding", "br, gzip".parse().unwrap());
    let mut request = client.post(&format!("http://{}/message_p2p", "localhost:8080"))
        .body(body)
        .headers(headers);
    let response = request.send().await?;
    println!("{:?}", response);
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
        info!("Connected to bootnode {}", bootnode);
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