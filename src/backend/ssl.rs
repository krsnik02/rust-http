
use std::io::Result;
use std::net::SocketAddr;

pub struct Backend;

impl super::Backend for Backend {
    type Server = ();
    type Stream = ();

    fn bind(addr: SocketAddr) -> Result<()> {
        unimplemented!()
    }
}
