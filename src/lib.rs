#[macro_use] extern crate log;

use std::net::{TcpListener, TcpStream};
use std::io::Result;
use std::sync::Arc;
use std::thread;

pub use std::net::{ToSocketAddrs, SocketAddr};

/// A HTTP server.
///
/// # Examples
///
/// ```no_run
/// # use self::http2;
/// use http2::{HttpServer, HttpConnection};
///
/// fn handle_connection(con: HttpConnection) {
///   // ...
/// }
///
/// let server = HttpServer::bind("127.0.0.1:80").unwrap();
///
/// // accept connections and process them
/// server.listen(handle_connection);
///
/// // close the http server
/// drop(server);
/// ```
///
#[derive(Debug)]
pub struct HttpServer {
    /// The underlying TCP socket listener
    tcp_listener: TcpListener,
}

impl HttpServer {
    /// Create a new `HttpServer` and bind it to a network address.
    ///
    /// # Arguments
    ///  - `addr` - The address to bind.
    ///
    /// # Failures
    ///  - The given address could not be bound.
    ///
    ///    Reasons for this error include
    ///    - the port already being in use, and
    ///    - the port requiring elevated permissions.
    ///
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<HttpServer> {
        let listener = try!(TcpListener::bind(addr));

        // Log address we are listening to
        if let Ok(addr) = listener.local_addr() {
            info!("Listening on address {}", addr);
        }
        else {
            info!("Listening on unknown address");
        }

        Ok(HttpServer { tcp_listener: listener })
    }

    /// Listen for incoming connections.
    ///
    /// A new thread is spawned whenever a connection is established.
    ///
    /// # Arguments
    ///  - `cb` - A callback which is run in a new thread for each connection.
    ///
    /// # Panics
    ///  - A spawned thread could not be joined. (TODO: fix this)
    ///
    pub fn listen<F>(&self, cb: F)
        where F: Fn(HttpConnection) -> (),
              F: 'static + Send + Sync {
        let mut handles = vec![];
        let cb = Arc::new(cb);

        // TODO: there should be a clean way to break out of this loop
        for stream in self.tcp_listener.incoming() {
            match stream {
                Err(err) => error!("Error: {}", err),
                Ok(stream) => {
                    let con = HttpConnection::new(stream);
                    let cb = cb.clone();
                    handles.push(thread::spawn(move || cb(con)));
                },
            }
        }

        // Join all spawned threads
        for handle in handles {
            // TODO: proper error handling
            handle.join().unwrap();
        }
    }
}


/// A HTTP connection
#[derive(Debug)]
pub struct HttpConnection {
    /// The underlying TCP stream.
    tcp_stream: TcpStream,
}

impl HttpConnection {
    /// Create a new `HttpConnection`
    ///
    /// # Arguments
    ///  - `stream` - The underlying TCP stream.
    fn new(stream: TcpStream) -> HttpConnection {
        HttpConnection { tcp_stream: stream }
    }

    /// The address of the local end of the connection.
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.tcp_stream.local_addr()
    }

    /// The address of the remote end of the connection.
    pub fn remote_addr(&self) -> Result<SocketAddr> {
        self.tcp_stream.peer_addr()
    }
}
