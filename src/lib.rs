extern crate mio;

use std::io::{Result, Error, ErrorKind};
use std::net::{SocketAddr, ToSocketAddrs};
use mio::{EventLoop, Handler, Token};
use mio::tcp::{TcpListener, TcpStream};

pub struct HttpConnection {
    tcp_stream: TcpStream,
}

impl HttpConnection {
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.tcp_stream.local_addr()
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.tcp_stream.peer_addr()
    }
}


/// Convert `A: ToSocketAddrs` to `SocketAddr`
fn to_addr<A: ToSocketAddrs>( addr: A ) -> Result<SocketAddr> {
    let mut iter = try!(addr.to_socket_addrs());
    match iter.next() {
        Some(addr) => Ok(addr),
        None => {
            Err(Error::new(ErrorKind::InvalidInput,
                           "Not a socket address"))
        },
    }
}


pub struct HttpServer {
    tcp_listener: TcpListener,
}

impl HttpServer {
    pub fn bind<A: ToSocketAddrs>( addr: A ) -> Result<HttpServer> {
        let addr = try!(to_addr(addr));
        let listener = try!(TcpListener::bind(&addr));
        Ok(HttpServer {
            tcp_listener: listener,
        })
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.tcp_listener.local_addr()
    }

    pub fn accept(&self) -> Result<Option<HttpConnection>> {
        unimplemented!()
    }

    /// Registers itself on the given `EventLoop`.
    pub fn register_self<H : Handler>(
        &self, event_loop: &mut EventLoop<H>, token: Token) -> Result<()> {
        event_loop.register(&self.tcp_listener, token)
    }
}
