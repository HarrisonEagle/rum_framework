use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::str::FromStr;
use std::thread::available_parallelism;
use crate::context::RumContext;
use crate::method::MethodType;
use crate::router::Router;
use crate::status_code;
use crate::thread::ThreadPool;
use std::sync::Arc;
use tera::Tera;


pub struct RumServer {
    host: String,
    port: i32,
    router: Router,
    static_asset_path: Option<String>,
    template_engine: Option<Tera>
}

struct RootRouter {
    router: Router,
    static_asset_path: Option<String>,
    template_engine: Option<Tera>
}

impl RumServer {

    pub fn new(host: &str, port: i32) -> RumServer {
        return RumServer{
            host: host.to_string(),
            port: port,
            router: Router::new(),
            static_asset_path: None,
            template_engine: None,
        };
    }

    pub fn use_html_template(&mut self, templates_path: &str){
        self.template_engine = match Tera::new(templates_path) {
            Ok(t) => Some(t),
            Err(e) => {
                panic!("Reading path in {} failed!\n{}\n", templates_path, e.to_string());
            }
        };
    }

    pub fn use_static_assets(&mut self, static_asset_path: &str){
        self.static_asset_path = Some(static_asset_path.to_string());
    }

    pub fn start(self){
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(addr).unwrap();
        println!("**RUM-FRAMEWORK** Listening: {}:{}", self.host, self.port);
        let available_parallelism_size = available_parallelism().unwrap().get();
        let pool_size = if available_parallelism_size < 4  { 4 } else { available_parallelism_size };
        let pool = ThreadPool::new(pool_size);
        let root_router = Arc::new(RootRouter{
            router: self.router,
            static_asset_path: self.static_asset_path,
            template_engine: self.template_engine
        });
        root_router.router.show_routes("");
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let router =  Arc::clone(&root_router);
            pool.execute(move || {
                handle_connection(stream, router);
            });
        }
    }

    fn add_route(&mut self, method_type: MethodType, route: &str, handler: fn(&mut RumContext)) {
        let mut route_segs: Vec<&str> = route.trim_end_matches('/').split('/').collect();
        if route_segs[0] != ""{
            route_segs.insert(0, "");
        }
        self.router.modify(method_type,route_segs, 0,  handler);
    }

    pub fn get(&mut self, route: &str, handler: fn(&mut RumContext)){
        self.add_route(MethodType::GET, route, handler);
    }

    pub fn post(&mut self, route: &str, handler: fn(&mut RumContext)){
        self.add_route(MethodType::POST, route, handler);
    }

    pub fn put(&mut self, route: &str, handler: fn(&mut RumContext)){
        self.add_route(MethodType::PUT, route, handler);
    }

    pub fn delete(&mut self, route: &str, handler: fn(&mut RumContext)){
        self.add_route(MethodType::DELETE, route, handler);
    }

    pub fn connect(&mut self, route: &str, handler: fn(&mut RumContext)){
        self.add_route(MethodType::CONNECT, route, handler);
    }

    pub fn options(&mut self, route: &str, handler: fn(&mut RumContext)){
        self.add_route(MethodType::OPTIONS, route, handler);
    }

    pub fn trace(&mut self, route: &str, handler: fn(&mut RumContext)){
        self.add_route(MethodType::TRACE, route, handler);
    }

    pub fn patch(&mut self, route: &str, handler: fn(&mut RumContext)){
        self.add_route(MethodType::PATCH, route, handler);
    }
    
}

fn handle_connection(mut stream: TcpStream, root_router: Arc<RootRouter>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let requests = String::from_utf8_lossy(&buffer[..]);
    let mut http_method_str = "";
    let mut route = "";
    let mut http_ver = "";
    let mut request_header_parsed = false;
    let mut request_body = String::new();
    let context = &mut RumContext::new(root_router.template_engine.as_ref());
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
            println!("{}", key);
            context.set_request_header(key, value);
        }else {
            request_body = format!("{}\r\n{}", request_body, line);
        }
    }
    context.set_request_body(request_body);
    
    let (response, http_status) = match MethodType::from_str(http_method_str) {
        Ok(http_method_type) => {
            let mut route_segs: Vec<&str> = route.trim_end_matches('/').split('/').collect();
            if route_segs[0] != ""{
                route_segs.insert(0, "");
            }
            let route_seg_slice = &route_segs[..];
            let last_key = route_segs[route_segs.len() - 1];
            let route_info = root_router.router.get_full_route(http_method_type, route_seg_slice);
            match route_info {
                Some(info) => {
                    route_seg_slice[0];
                    //root_router.router.exec_middleware(info.0, 0);
                    info.1(context);
                    context.get_response(http_ver)
                },
                None => {
                    // NEED?
                    let static_path = root_router.static_asset_path.as_ref();
                    if static_path.is_some() {
                        let file_path = format!("{}/{}", *(static_path.unwrap()), last_key);
                        context.file(status_code::OK, &file_path);
                        context.get_response(http_ver)
                    }else{
                        context.default_404(http_ver)
                    }
                }  
            }
        },
        Err(_) => {
            println!("Unknown Method!");
            context.default_400(http_ver)
        },
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("|{}| {} {}", http_method_str, route , http_status);

}
