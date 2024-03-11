use std::net::SocketAddr;

pub struct Info {
    pub replicaof: Option<SocketAddr>,
}
