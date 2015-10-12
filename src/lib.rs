extern crate mio;

use std::io;
use std::result;
use std::net::{SocketAddr, ToSocketAddrs};
use mio::{EventLoop, Handler, Token};
use mio::tcp::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct HttpError;

pub type Result<T> = result::Result<T, HttpError>;


pub struct HttpConnection {
    tcp_stream: TcpStream,
}

impl HttpConnection {
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.tcp_stream.local_addr()
            .map_err(|_| HttpError)
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.tcp_stream.peer_addr()
            .map_err(|_| HttpError)
    }
}




pub struct HttpServer {
    tcp_listener: TcpListener,
}

impl HttpServer {
    pub fn bind<A: ToSocketAddrs>( addr: A ) -> Result<HttpServer> {
        unimplemented!()
    }

    pub fn accept(&self) -> Result<Option<HttpConnection>> {
        unimplemented!()
    }

    /// Registers itself on the given `EventLoop`.
    pub fn register_self<H : Handler>(
        &self, event_loop: &mut EventLoop<H>, token: Token)
        -> io::Result<()> {
        event_loop.register(&self.tcp_listener, token)
    }
}
