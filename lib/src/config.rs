
#[derive(Debug, Clone)]
pub struct Config {
    p2p_port: u16,
    rpc_port: u16,
    bootnode: String,
}
impl Config {
    pub fn new(p2p_port: u16, rpc_port: u16, bootnode: String) -> Self {
        Config {
            p2p_port,
            rpc_port,
            bootnode,
        }
    }
    pub fn p2p_port(&self) -> u16 {
        self.p2p_port
    }
    pub fn rpc_port(&self) -> u16 {
        self.rpc_port
    }
    pub fn bootnode(&self) -> String {
        self.bootnode.to_string()
    }
}

// test config
