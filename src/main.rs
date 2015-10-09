#[macro_use] extern crate log;
extern crate env_logger;

use std::net::TcpListener;

fn main() {
    env_logger::init().unwrap();

    match TcpListener::bind("localhost:80") {
        Err(err) => {
            // Unable to bind socket
            error!("Error: {}", err)
        },
        Ok(listener) => {
            // Successfully bound socket
            info!("Success: {:?}", listener)
        }
    }
}
