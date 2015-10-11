#[macro_use] extern crate log;
extern crate mio;

pub use std::net::ToSocketAddrs;

/// Type of callback which handles requests.
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
    tcp_server: mio::tcp::TcpListener,
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
    ///  - The given address is not a valid socket address.
    ///
    ///  - The given address could not be bound.
    ///
    ///    Reasons for this error include
    ///    - the port already being in use, and
    ///    - the port requiring elevated permissions.
    ///
    pub fn bind<T: ToSocketAddrs>(addr: T) -> Option<HttpServer> {
        match addr.to_socket_addrs() {
            Ok(mut addrs) => {
                match addrs.next() {
                    Some(addr) => {
                        match mio::tcp::TcpListener::bind(&addr) {
                            Ok(server) => {
                                Some(HttpServer {
                                    tcp_server: server,
                                    on_request: None,
                                })
                            },
                            Err(_) => None,
                        }
                    },
                    None => None,
                }
            },
            Err(_) => None,
        }
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

/*
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
