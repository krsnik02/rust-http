
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::Result;

pub struct Backend;

impl super::Backend for Backend {
    type Server = TcpListener;
    type Stream = TcpStream;

    fn bind(addr: SocketAddr) -> Result<TcpListener> {
        unimplemented!()
    }
}
