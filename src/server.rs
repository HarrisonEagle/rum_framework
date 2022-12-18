use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::str::FromStr;
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
        println!("**RUM-FRAMEWORK** Listening: {}:{}", self.host, self.port);
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
    let mut http_method_str = "";
    let mut route = "";
    let mut http_ver = "";
    let mut request_header_parsed = false;
    let mut request_body = "".to_string();
    for (index,line) in requests.lines().enumerate() {
        if line.len() == 0 {
            request_header_parsed = true;
            continue;
        }
        if index == 0 {
            let mut iter = line.splitn(3," ");
            http_method_str = iter.next().unwrap();
            route = iter.next().unwrap();
            http_ver = iter.next().unwrap();
        }else if !request_header_parsed{
            let mut iter = line.splitn(2,": ");
            let key = iter.next().unwrap();
            let value = iter.next().unwrap();
        }else {
            request_body = format!("{}\r\n{}", request_body, line);
        }
    }
    let mut http_status = status_code::from_status_code(status_code::OK);
    let mut response_body = "".to_string();
    let mut response_header = "\r\n".to_string();
    match MethodType::from_str(http_method_str) {
        Ok(http_method_type) => {
            let route_info = router.get_full_route(http_method_type, route);
        match route_info {
            Some(info) => {
                router.exec_middleware(info.0, 0);
                let response = info.1(Context {  });
                http_status = response.http_status;
                response_body = response.response_body;
                response_header = format!("Content-Type: {}\r\n", response.content_type);
            },
            None => {
                // NEED?
                let response = Response::FILE(status_code::OK, route.trim_matches('/'));
                http_status = response.http_status;
                response_body = response.response_body;
                response_header = format!("Content-Type: {}\r\n", response.content_type);
            }
    }
        },
        Err(_) => { },
    }
    let response = format!("{} {}\r\n{}\r\n{}", http_ver, http_status, response_header, response_body);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("|{}| {} {}", http_method_str, route ,http_status);

}
