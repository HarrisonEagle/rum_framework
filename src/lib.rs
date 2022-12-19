mod server;
mod thread;
pub mod router;
pub mod status_code;
pub mod context;

pub mod rum {
    use crate::server::RumServer;

    pub fn new(host: &str, port: i32) -> RumServer{
        return RumServer::new(host, port);
    }
}


    