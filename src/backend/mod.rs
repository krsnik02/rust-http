use std::net::SocketAddr;
use std::io::Result;
use mio::Evented;

mod tcp;

pub type TCPBackend = tcp::Backend;

pub trait Backend {
    type Server : Evented;
    type Stream;

    fn bind(addr: SocketAddr) -> Result<Self::Server>;
}
