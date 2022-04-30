pub struct Config {
    network_id: Vec<u8>,
    coin_name: String,
    decimal_places: u8,
    starting_chains: u64,
    p2p_port: u16,
    rpc_port: u16,
    seed_nodes: Vec<String>,
}
