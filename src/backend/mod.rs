use std::net::SocketAddr;
use std::io::Result;

mod tcp;
mod ssl;

pub type TCPBackend = tcp::Backend;
pub type SSLBackend = ssl::Backend;

pub trait Backend {
    type Server;
    type Stream;

    fn bind(addr: SocketAddr) -> Result<Self::Server>;
}
