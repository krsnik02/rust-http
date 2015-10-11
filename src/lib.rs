#[macro_use] extern crate log;
extern crate mio;

pub mod http;
mod backend;

pub type HttpServer = http::Server<backend::TCPBackend>;
pub type HttpsServer = http::Server<backend::SSLBackend>;
