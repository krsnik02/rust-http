#[macro_use] extern crate log;
extern crate http;
extern crate mio;

use http::HttpServer;
use mio::{EventLoop, EventSet, Token, Handler};
use log::{Log, LogRecord, LogLevel, LogMetadata, SetLoggerError, LogLevelFilter};


// Log everything at info level or above
struct InfoLogger;

impl InfoLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(|max_log_level| {
            max_log_level.set(LogLevelFilter::Info);
            Box::new(InfoLogger)
        })
    }
}

impl Log for InfoLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}



const TOK_SERVER: Token = Token(0);

struct MyHandler(HttpServer);

impl Handler for MyHandler {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<MyHandler>,
             token: Token, _: EventSet) {
        match token {
            TOK_SERVER => {
                let MyHandler(ref mut server) = *self;
                match server.accept() {
                    Ok(Some(con)) => {
                        info!("Accepted connection from {}",
                              con.peer_addr().unwrap());
                        event_loop.shutdown();
                    },
                    Ok(None) => {
                        info!("Not actually ready");
                    },
                    Err(e) => {
                        error!("{:?}", e);
                    },
                }
            },
            _ => panic!("Unrecognized token!"),
        }
    }
}

fn main() {
    InfoLogger::init().unwrap();

    let mut event_loop = mio::EventLoop::new().unwrap();
    let server = http::HttpServer::bind("localhost:8080").unwrap();

    match server.local_addr() {
        Ok(addr) => { info!("Listening on {}", addr); },
        Err(err) => {
            warn!("{}", err);
            info!("Listening on unknown address");
        },
    }

    server.register_self(&mut event_loop, TOK_SERVER).unwrap();
    event_loop.run(&mut MyHandler(server)).unwrap();
}
