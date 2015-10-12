
use std::net::SocketAddr;
use mio::tcp::{TcpListener, TcpStream};
use std::io::Result;

pub struct Backend;

impl super::Backend for Backend {
    type Server = TcpListener;
    type Stream = TcpStream;

    fn bind(addr: SocketAddr) -> Result<TcpListener> {
        TcpListener::bind(&addr)
    }
}
