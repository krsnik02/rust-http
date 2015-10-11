#[macro_use] extern crate log;
extern crate env_logger;
extern crate net;

use std::io::{BufReader, Stdin};

use net::HttpServer;
use net::http::{Request, Response};

// A HTTP request was received
fn on_request(request: Request, _: &mut HttpServer) -> Response {
    unimplemented!()
}

// There is data to be read on Stdin
fn on_stdin(stdin: &mut BufReader<Stdin>, server: &mut HttpServer) {
    // Shutdown on any keyboard input
    server.shutdown();
}

fn main() {
    env_logger::init().unwrap();
    match HttpServer::bind("localhost:80") {
        Ok(server) => {
            server.on_request(&on_request);
            server.on_stdin(&on_stdin);
            server.run_event_loop();
        },
        Err(e) => error!("{}", e),
    }
}
