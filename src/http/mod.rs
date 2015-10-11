use std::io::{Result, BufReader, BufWriter, Stdin, Stdout};
use std::net::ToSocketAddrs;
use super::backend::Backend;

pub struct Request;
pub struct Response;

pub type CbRequest<T> = Fn(Request, &mut Server<T>) -> Response;
pub type CbStdin<T>   = Fn(&mut BufReader<Stdin>, &mut Server<T>) -> ();
pub type CbStdout<T>  = Fn(&mut BufWriter<Stdout>, &mut Server<T>) -> ();

pub struct Server<T: Backend> {
    backend: T,
}

impl <T: Backend> Server<T> {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        unimplemented!()
    }

    pub fn on_request(&self, callback: &CbRequest<T>) {
        unimplemented!()
    }

    pub fn on_stdin(&self, callback: &CbStdin<T>) {
        unimplemented!()
    }

    pub fn on_stdout(&self, callback: &CbStdout<T>) {
        unimplemented!()
    }

    pub fn run_event_loop(&self) {
        unimplemented!()
    }

    pub fn shutdown(&self) {
        unimplemented!()
    }
}

/*
use std::io::{self, Result};
use std::net::ToSocketAddrs;

use mio;
use mio::tcp::TcpListener;

pub type RequestHandler = Box<Fn(HttpRequest) -> HttpResponse>;

/// A HTTP server.
///
/// # Examples
///
/// ```no_run
/// # use self::http;
/// use http::*;
///
/// // Callback that handles requests
/// fn on_request(request: HttpRequest) -> HttpResponse {
///   // ...
///   # HttpResponse
/// }
///
/// // Create and bind server
/// let mut server = HttpServer::bind("127.0.0.1:80").unwrap();
///
/// // Accept connections and process them
/// server.listen(Some(Box::new(on_request)));
/// ```
///
pub struct HttpServer {
    tcp_server: TcpListener,
    on_request: Option<RequestHandler>,
}

const TOK_SERVER: mio::Token = mio::Token(0);

impl HttpServer {
    /// Create a new `HttpServer` and bind it to a network address.
    ///
    /// # Arguments
    ///  - `addr` - The address to bind.
    ///
    /// # Failures
    ///  - `addr` is not a valid socket address.
    ///  - A socket could not be bound to `addr`.
    ///
    ///    The most common causes of this error are:
    ///    - the port is already in use, and
    ///    - the port requires elevated permissions to bind.
    ///
    pub fn bind<T: ToSocketAddrs>(addr: T) -> Result<HttpServer> {
        let invalid_address = io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid socket address");

        // Get `SocketAddr` from argument `addr`
        let addr = try!(match addr.to_socket_addrs() {
            Ok(mut addrs) => {
                match addrs.next() {
                    Some(addr) => Ok(addr),
                    None => Err(invalid_address),
                }
            },
            Err(_) => Err(invalid_address),
        });

        // Bind the server
        let server = try!(TcpListener::bind(&addr));
        info!("Server listening on {}", addr);
        Ok(HttpServer {
            tcp_server: server,
            on_request: None,
        })
    }

    /// Listen for incoming connections.
    ///
    /// # Arguments
    ///  - `on_request` - A optional callback which is called for every
    ///                   incoming request.
    pub fn listen(&mut self, on_request: Option<RequestHandler>) {
        self.on_request = on_request;
        let mut event_loop = mio::EventLoop::new().unwrap();
        event_loop.register(&self.tcp_server, TOK_SERVER);
        event_loop.run(self);
    }
}

impl mio::Handler for HttpServer {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut mio::EventLoop<HttpServer>,
             token: mio::Token, events: mio::EventSet) {
        if token == TOK_SERVER {
            assert!(events.is_readable());
            match self.tcp_server.accept() {
                Ok(Some(_)) => {
                    info!("Accepted a connection");
                    event_loop.shutdown();
                },
                Ok(None) => {},
                Err(e) => {
                    error!("Error accepting an incoming connection: {}", e);
                    event_loop.shutdown();
                }
            }
        }
        else {
            unreachable!();
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
*/
