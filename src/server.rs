use crate::context::RumContext;
use crate::handler::Handler;
use crate::method::MethodType;
use crate::thread::ThreadPool;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread::available_parallelism;

pub struct RumServer {
    host: String,
    port: i32,
    handler: Handler,
}

impl RumServer {
    pub fn new(host: &str, port: i32) -> RumServer {
        return RumServer {
            host: host.to_string(),
            port: port,
            handler: Handler::new()
        };
    }

    pub fn use_html_template(&mut self, templates_path: &str) {
        self.handler.set_template_engine(templates_path);
    }

    pub fn use_static_assets(&mut self, static_asset_path: &str) {
        self.handler.set_static_assets(static_asset_path);
    }

    pub fn start(self) {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(addr).unwrap();
        println!("**RUM-FRAMEWORK** Listening: {}:{}", self.host, self.port);
        let available_parallelism_size = available_parallelism().unwrap().get();
        let pool_size = if available_parallelism_size < 4 {
            4
        } else {
            available_parallelism_size
        };
        let pool = ThreadPool::new(pool_size);
        let handler = Arc::new(self.handler);
        handler.router.show_routes("");
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let handler = Arc::clone(&handler);
            pool.execute(move || {
                handler.handle_connection(stream);
            });
        }
    }

    fn add_route(&mut self, method_type: MethodType, route: &str, handler: fn(&mut RumContext)) {
        self.handler.add_route(method_type, route, handler);
    }

    pub fn global_middleware(&mut self, handlers: Vec<fn(&mut RumContext)>){
        self.handler.set_middleware("", handlers);
    }

    pub fn middleware(&mut self, route: &str, handlers: Vec<fn(&mut RumContext)>){
        self.handler.set_middleware(route, handlers);
    }

    pub fn get(&mut self, route: &str, handler: fn(&mut RumContext)) {
        self.add_route(MethodType::GET, route, handler);
    }

    pub fn post(&mut self, route: &str, handler: fn(&mut RumContext)) {
        self.add_route(MethodType::POST, route, handler);
    }

    pub fn put(&mut self, route: &str, handler: fn(&mut RumContext)) {
        self.add_route(MethodType::PUT, route, handler);
    }

    pub fn delete(&mut self, route: &str, handler: fn(&mut RumContext)) {
        self.add_route(MethodType::DELETE, route, handler);
    }

    pub fn connect(&mut self, route: &str, handler: fn(&mut RumContext)) {
        self.add_route(MethodType::CONNECT, route, handler);
    }

    pub fn options(&mut self, route: &str, handler: fn(&mut RumContext)) {
        self.add_route(MethodType::OPTIONS, route, handler);
    }

    pub fn trace(&mut self, route: &str, handler: fn(&mut RumContext)) {
        self.add_route(MethodType::TRACE, route, handler);
    }

    pub fn patch(&mut self, route: &str, handler: fn(&mut RumContext)) {
        self.add_route(MethodType::PATCH, route, handler);
    }
}
