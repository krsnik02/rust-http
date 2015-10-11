#[macro_use] extern crate log;

use std::net::{TcpListener, TcpStream};
use std::io::{Read};
use std::sync::Arc;
use std::thread;

pub use std::net::{ToSocketAddrs, SocketAddr};

pub type Result<T> = std::result::Result<T, HttpError>;

#[derive(Debug)]
pub enum HttpError {
    IoError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
}

impl From<std::io::Error> for HttpError {
    fn from(err : std::io::Error) -> HttpError {
        HttpError::IoError(err)
    }
}

impl From<std::str::Utf8Error> for HttpError {
    fn from(err : std::str::Utf8Error) -> HttpError {
        HttpError::Utf8Error(err)
    }
}

/// A HTTP server.
///
/// # Examples
///
/// ```no_run
/// # use self::http2;
/// use http2::{HttpServer, HttpConnection};
///
/// fn handle_connection(con: &mut HttpConnection) {
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
        where F: Fn(&mut HttpConnection) -> (),
              F: 'static + Send + Sync {
        let mut handles = vec![];
        let cb = Arc::new(cb);

        // TODO: there should be a clean way to break out of this loop
        for stream in self.tcp_listener.incoming() {
            match stream {
                Err(err) => error!("Error: {}", err),
                Ok(stream) => {
                    let mut con = HttpConnection::new(stream);
                    let cb = cb.clone();
                    handles.push(thread::spawn(move || cb(&mut con)));
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

/// A HTTP request
///
/// TODO: implement this fully
#[derive(Debug)]
pub struct HttpRequest;

/// A HTTP response
///
/// TODO: implement this fully
#[derive(Debug)]
pub struct HttpResponse;

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
        match self.tcp_stream.local_addr() {
            Ok(addr) => Ok(addr),
            Err(err) => Err(HttpError::IoError(err)),
        }
    }

    /// The address of the remote end of the connection.
    pub fn remote_addr(&self) -> Result<SocketAddr> {
        match self.tcp_stream.peer_addr() {
            Ok(addr) => Ok(addr),
            Err(err) => Err(HttpError::IoError(err)),
        }
    }

    /// Block until the next request arrives.
    ///
    /// TODO: parse the request
    pub fn next_request(&mut self) -> Result<HttpRequest> {

        // Block until there is an incoming message and then read it in
        // chunks of BUFSIZE bytes.
        const BUFSIZE : usize = 1024;
        let mut read_buf = [0u8; BUFSIZE];
        let mut accum_buf : Vec<u8> = Vec::with_capacity(BUFSIZE);
        while let Ok(count) = self.tcp_stream.read(&mut read_buf) {
            accum_buf.extend(&read_buf[..count]);
            if count < BUFSIZE { break; }
        }

        let text = try!(std::str::from_utf8(&accum_buf));
        info!("Read message: {:?}", text);

        // TODO: actually parse the request
        Ok(HttpRequest)
    }

    /// Send a response
    pub fn send_response(&self, response: HttpResponse) {
        unimplemented!();
    }
}
