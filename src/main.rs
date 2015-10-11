#[macro_use] extern crate log;
extern crate env_logger;
extern crate http;

/// Handle a incoming connection.
///
/// This function is run in its own thread each time a connection is created
///
/// # Arguments
///  - `con` - The incoming connection.
///
fn on_request(request: http::HttpRequest) -> http::HttpResponse {
  http::HttpResponse
}

/// Create a new server and begin listening.
fn main() {
    env_logger::init().unwrap();
    match http::HttpServer::bind("localhost:80") {
        Ok(mut server) => server.listen(Some(Box::new(on_request))),
        Err(err) => error!("{}", err),
    }
}
