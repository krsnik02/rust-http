//! A mio compatible implementation of the HTTP protocol
//!
//! # Examples
//! ```no_run
//! # extern crate http;
//! # extern crate mio;
//!
//! use http::*;
//! use mio::*;
//!
//! const SERVER: Token = Token(0);
//!
//! struct MyHandler(HttpServer);
//! impl Handler for MyHandler {
//!     type Timeout = ();
//!     type Message = ();
//!
//!     fn ready(&mut self, event_loop: &mut EventLoop<MyHandler>,
//!              token: Token, _: EventSet) {
//!         match token {
//!             SERVER => {
//!                 let MyHandler(ref mut server) = *self;
//!                 match server.accept() {
//!                     Ok(Some(con)) => {
//!                         // A connection was established
//!                         println!("Connection esablished");
//!                         event_loop.shutdown();
//!                     },
//!                     Ok(None) => {
//!                         // There wasn't actually a connection
//!                     },
//!                     Err(_) => {
//!                         // Some error occurred
//!                         event_loop.shutdown();
//!                     },
//!                 }
//!             },
//!             _ => panic!("Unrecognized token!"),
//!         }
//!     }
//! }
//!
//! # fn main() {
//! // Create a mio event loop
//! let mut event_loop = EventLoop::new().unwrap();
//!
//! // Create a server and bind it to an address
//! let server = HttpServer::bind("localhost:80").unwrap();
//!
//! // Register the server with the event_loop
//! server.register_self(&mut event_loop, SERVER);
//!
//! // Run the event loop
//! event_loop.run(&mut MyHandler(server));
//! # }
//! ```

extern crate mio;

use std::io::{Result, Error, ErrorKind};
use std::net::{SocketAddr, ToSocketAddrs};
use mio::{EventLoop, Handler, Token};
use mio::tcp::{TcpListener, TcpStream};

/// A HTTP/1.1 connection
pub struct HttpConnection {
    tcp_stream: TcpStream,
}

impl HttpConnection {
    /// Returns the address of the local endpoint
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.tcp_stream.local_addr()
    }

    /// Returns the address of the remote endpoint
    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.tcp_stream.peer_addr()
    }
}


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

/// A HTTP/1.1 server
pub struct HttpServer {
    tcp_listener: TcpListener,
}

impl HttpServer {
    /// Creates a new HttpServer and binds it to an address
    pub fn bind<A: ToSocketAddrs>( addr: A ) -> Result<HttpServer> {
        let addr = try!(to_addr(addr));
        let listener = try!(TcpListener::bind(&addr));
        Ok(HttpServer {
            tcp_listener: listener,
        })
    }

    /// Returns the bound address
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.tcp_listener.local_addr()
    }

    /// Accepts a new incoming connection
    pub fn accept(&self) -> Result<Option<HttpConnection>> {
        match self.tcp_listener.accept() {
            Ok(Some(stream)) => {
                Ok(Some(HttpConnection {
                    tcp_stream: stream
                }))
            },
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    /// Registers itself on a mio event loop
    pub fn register_self<H : Handler>(
        &self, event_loop: &mut EventLoop<H>, token: Token) -> Result<()> {
        event_loop.register(&self.tcp_listener, token)
    }
}
