use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::thread::available_parallelism;
use crate::router::Router;
use crate::thread::ThreadPool;
use std::sync::Arc;
pub struct RumServer {
    host: String,
    port: i32,
}

impl RumServer {

    pub fn new(host: &str, port: i32) -> RumServer {
        return RumServer{
            host: host.to_string(),
            port: port
        };
    }

    pub fn start(self){
        let rum = self;
        let addr = format!("{}:{}", rum.host, rum.port);
        let listener = TcpListener::bind(addr).unwrap();
        let available_parallelism_size = available_parallelism().unwrap().get();
        let pool_size = if available_parallelism_size < 4  { 4 } else { available_parallelism_size };
        let pool = ThreadPool::new(pool_size);
        let router = Arc::new(Router{});
        
    
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let router =  Arc::clone(&router);
            
            pool.execute(move || {
                handle_connection(stream, router);
            });
        }
    }
    
}

fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let requests = String::from_utf8_lossy(&buffer[..]);
    for line in requests.lines() {
        let kv = line.split(": ");
        println!("{}",line);
        for (i, el) in kv.enumerate() {
            if i == 0 {
                println!("key:{}",el);
            }else {
                println!("value:{}",el);
            }
        }
    }
    let status = "200 OK";
    let body = "<h1>hello1</h1>";
    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status, body);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
