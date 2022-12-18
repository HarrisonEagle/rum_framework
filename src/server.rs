use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::thread::available_parallelism;
use crate::router::Context;
use crate::router::MethodType;
use crate::router::Response;
use crate::router::Router;
use crate::status_code;
use crate::thread::ThreadPool;
use std::sync::Arc;
pub struct RumServer {
    host: String,
    port: i32,
    router: Router,
}

impl RumServer {

    pub fn new(host: &str, port: i32) -> RumServer {
        return RumServer{
            host: host.to_string(),
            port: port,
            router: Router::new()
        };
    }

    pub fn start(self){
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(addr).unwrap();
        println!("Listening: {}:{}", self.host, self.port);
        let available_parallelism_size = available_parallelism().unwrap().get();
        let pool_size = if available_parallelism_size < 4  { 4 } else { available_parallelism_size };
        let pool = ThreadPool::new(pool_size);
        let router = Arc::new(self.router);
        router.show_routes("");
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let router =  Arc::clone(&router);
            pool.execute(move || {
                handle_connection(stream, router);
            });
        }
    }

    pub fn test(&self, method_type: MethodType, route: &str){
        self.router.get_handler(method_type, route)
    }

    fn add_route(&mut self, method_type: MethodType, route: &str, handler: fn(Context) -> Response) {
        let mut route_segs: Vec<&str> = route.trim_end_matches('/').split('/').collect();
        if route_segs[0] != ""{
            route_segs.insert(0, "");
        }
        self.router.modify(method_type,route_segs, 0,  handler);
    }

    pub fn get(&mut self, route: &str, handler: fn(Context) -> Response){
        self.add_route(MethodType::GET, route, handler);
    }

    pub fn post(&mut self, route: &str, handler: fn(Context) -> Response){
        self.add_route(MethodType::POST, route, handler);
    }

    pub fn put(&mut self, route: &str, handler: fn(Context) -> Response){
        self.add_route(MethodType::PUT, route, handler);
    }

    pub fn delete(&mut self, route: &str, handler: fn(Context) -> Response){
        self.add_route(MethodType::DELETE, route, handler);
    }

    pub fn connect(&mut self, route: &str, handler: fn(Context) -> Response){
        self.add_route(MethodType::CONNECT, route, handler);
    }

    pub fn options(&mut self, route: &str, handler: fn(Context) -> Response){
        self.add_route(MethodType::OPTIONS, route, handler);
    }

    pub fn trace(&mut self, route: &str, handler: fn(Context) -> Response){
        self.add_route(MethodType::TRACE, route, handler);
    }

    pub fn patch(&mut self, route: &str, handler: fn(Context) -> Response){
        self.add_route(MethodType::PATCH, route, handler);
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
    let status = status_code::from_status_code(status_code::OK);
    let body = "<h1>hello1</h1>";
    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status, body);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
