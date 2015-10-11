pub mod v1_1;

pub enum Event {
    /// A HTTP request was received
    Request,

    /// There is input to read on stdin
    Stdin,

    /// It is possible to write to stdout
    Stdout,
}

pub struct Request;
pub struct Response;
