#[macro_use] extern crate log;
extern crate env_logger;

use std::net::{TcpListener, TcpStream};
use std::io::Result;
use std::sync::Arc;
use std::thread;

/// A HTTP server
struct HttpServer {
    tcp_listener: TcpListener,
}

impl HttpServer {
    /// Create and bind a new HttpServer
    ///
    /// # Arguments
    ///  - addr - the address to bind
    ///
    fn bind(addr: &str) -> Result<HttpServer> {
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

    /// Run a HttpServer, creating a new thread for each incoming connection
    ///
    /// # Aruments
    ///  - cb - the callback to run when for each connection
    ///
    fn run<F>(&self, cb: F)
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
                    handles.push(thread::spawn(move || (cb)(con)));
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
struct HttpConnection {
    tcp_stream: TcpStream,
}

impl HttpConnection {
    /// Create a new HttpConnection
    ///
    /// # Arguments
    ///  - stream - the TcpStream to encapsulate
    fn new(stream: TcpStream) -> HttpConnection {
        HttpConnection { tcp_stream: stream }
    }
}


fn handle_connection(con: HttpConnection) {
    info!("Connection established: {:?}", con.tcp_stream);
    thread::sleep_ms(1000);
}

fn main() {
    env_logger::init().unwrap();

    match HttpServer::bind("localhost:80") {
        Err(err) => error!("Error: {}", err),
        Ok(server) => server.run(handle_connection)
    }
}
