#[macro_use] extern crate log;
extern crate env_logger;
extern crate http2;

use std::thread;

/// Handle a incoming connection.
///
/// This function is run in its own thread each time a connection is created
///
/// # Arguments
///  - `con` - The incoming connection.
///
fn handle_connection(con: &mut http2::HttpConnection) {
    info!("Connection established: {:?}", con);
    while let Ok(_) = con.next_request() {}
}

/// Create a new server and begin listening.
fn main() {
    env_logger::init().unwrap();
    match http2::HttpServer::bind("localhost:80") {
        Err(err) => error!("Error: {:?}", err),
        Ok(server) => server.listen(handle_connection)
    }
}

