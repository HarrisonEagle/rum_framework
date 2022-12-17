mod server;
mod thread;
mod router;
pub mod status_code;
pub mod rum {
    use crate::server::RumServer;

    pub fn new(host: &str, port: i32) -> RumServer{
        return RumServer::new(host, port);
    }
}


    