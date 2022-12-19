pub mod context;
mod handler;
mod method;
mod response;
pub mod router;
mod server;
pub mod status_code;
mod thread;
mod lib_test;

pub mod rum {
    use crate::server::RumServer;

    pub fn new(host: &str, port: i32) -> RumServer {
        return RumServer::new(host, port);
    }
}
